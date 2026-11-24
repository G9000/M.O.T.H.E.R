//! Acquire an original that the MCP host itself handed to a tool as a file parameter.
//!
//! ChatGPT resolves a declared `openai/fileParams` field to
//! `{download_url, file_id, mime_type?, file_name?}`. This adapter downloads only
//! from the host's own file domains, hashes the actual bytes and reuses the
//! ordinary chunked attachment protocol, so every result still waits for local
//! owner review. It is not a general URL fetcher.
use super::attachment::{AttachmentClient, MAX_BYTES, STATUS, send_original};
use crate::{
    contracts,
    error::{Error, Result, invalid},
};
use serde_json::{Value, json};
use std::{io::Read, time::Duration};
use url::Url;

pub const IMPORT: &str = "mother_media_attachment_import";

/// Hosts that serve ChatGPT file-parameter downloads. Matching is by whole DNS labels.
const HOSTS: [&str; 3] = ["oaiusercontent.com", "chatgpt.com", "openai.com"];
/// Exact third-party storage hosts observed in ChatGPT file-parameter links.
/// Azure storage account names are first-come, so these are never matched by
/// prefix or suffix: each entry is one specific account, added from live evidence.
const EXACT_HOSTS: [&str; 1] = [
    // Observed 2026-11-25 in a live ChatGPT file-parameter download_url (Southeast Asia).
    "oaisdmntprseasia.blob.core.windows.net",
];
const MAX_REDIRECTS: usize = 3;
const TIMEOUT: Duration = Duration::from_secs(120);

fn unavailable(detail: &str) -> Error {
    Error::domain(
        "SOURCE_BYTES_UNAVAILABLE",
        format!(
            "{detail} Ask the owner to attach the original to this conversation, or use Attachment review in the local Garden."
        ),
    )
}

/// Accept only an https URL on a recognised host-file domain, without credentials or ports.
pub(crate) fn allowed(url: &Url) -> bool {
    let host = match url.host() {
        Some(url::Host::Domain(host)) => host.to_ascii_lowercase(),
        _ => return false,
    };
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port().is_none()
        && (EXACT_HOSTS.contains(&host.as_str())
            || HOSTS
                .iter()
                .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}"))))
}

fn host_of(url: &Url) -> String {
    url.host_str().unwrap_or("unknown").to_owned()
}

/// Download at most `MAX_BYTES`, following only redirects that stay on allowed hosts.
fn download(start: &str) -> Result<Vec<u8>> {
    let mut url = Url::parse(start).map_err(|_| unavailable("The host file link is invalid."))?;
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .max_redirects(0)
        .timeout_global(Some(TIMEOUT))
        .http_status_as_error(false)
        .https_only(true)
        .build()
        .into();
    for _ in 0..=MAX_REDIRECTS {
        if !allowed(&url) {
            // Report the host, never the signed link.
            return Err(unavailable(&format!(
                "The host file link points to an unsupported host ({}).",
                host_of(&url)
            )));
        }
        let response = agent
            .get(url.as_str())
            .call()
            .map_err(|_| unavailable("The host file download failed."))?;
        let status = response.status().as_u16();
        if (300..400).contains(&status) {
            let location = response
                .headers()
                .get("location")
                .and_then(|value| value.to_str().ok())
                .ok_or_else(|| unavailable("The host file redirect was incomplete."))?;
            url = url
                .join(location)
                .map_err(|_| unavailable("The host file redirect was invalid."))?;
            continue;
        }
        if status != 200 {
            return Err(unavailable(&format!(
                "The host file download returned HTTP {status}; the link may have expired."
            )));
        }
        let mut bytes = Vec::new();
        response
            .into_body()
            .into_reader()
            .take(MAX_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| unavailable("The host file download was interrupted."))?;
        if bytes.len() as u64 > MAX_BYTES {
            return Err(Error::domain(
                "LIMIT_EXCEEDED",
                "The original exceeds the attachment size limit.",
            ));
        }
        return Ok(bytes);
    }
    Err(unavailable(
        "The host file download redirected too many times.",
    ))
}

/// Describe what the host sent for `file` by shape only: never values, links or IDs.
fn received(file: &Value) -> String {
    let shape = match file {
        Value::Null => "no file argument".to_owned(),
        Value::String(text) if text.starts_with("file") => {
            "a bare file ID string, not a host file object; the host did not authorize this file for the tool".to_owned()
        }
        Value::String(_) => "a plain string, not a host file object".to_owned(),
        Value::Object(fields) => {
            let mut keys: Vec<&str> = fields.keys().map(String::as_str).collect();
            keys.sort_unstable();
            let keys: Vec<String> = keys
                .iter()
                .take(8)
                .map(|key| key.chars().take(24).collect())
                .collect();
            let id = match fields.get("file_id") {
                Some(Value::String(id)) if file_id(id) => format!("valid ({})", id_shape(id)),
                Some(Value::String(id)) => format!("rejected ({})", id_shape(id)),
                Some(_) => "not a string".to_owned(),
                None => "missing".to_owned(),
            };
            let link = if fields.get("download_url").is_some_and(Value::is_string) {
                "present"
            } else {
                "missing"
            };
            format!(
                "an object with keys [{}], file_id {id}, download_url {link}",
                keys.join(", ")
            )
        }
        other => format!("a JSON {}", if other.is_array() { "array" } else { "scalar" }),
    };
    format!("The host did not provide a downloadable file: it sent {shape}.")
}

/// Keep only the final path component of a host-supplied name.
fn base_name(name: &str) -> Option<String> {
    let name = name.rsplit(['/', '\\']).next()?.trim();
    (!name.is_empty() && name != "." && name != "..").then(|| name.to_owned())
}

/// Host file IDs are opaque provenance, never a path, command or source of a link.
/// They follow the attachment service's source-locator bounds.
fn file_id(id: &str) -> bool {
    !id.trim().is_empty() && id.len() <= 480 && !id.chars().any(char::is_control)
}

/// Describe an identifier by length and format category, never by value.
fn id_shape(id: &str) -> String {
    let category = if id.trim().is_empty() {
        "empty".to_owned()
    } else if id.chars().any(char::is_control) {
        "contains control characters".to_owned()
    } else if id.starts_with("file_") || id.starts_with("file-") {
        "OpenAI file ID".to_owned()
    } else if let Some((scheme, _)) = id.split_once("://").filter(|(scheme, _)| {
        (1..=16).contains(&scheme.len())
            && scheme
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"+.-".contains(&b))
    }) {
        format!("URI-like with scheme {scheme}")
    } else {
        "opaque".to_owned()
    };
    let bound = if id.len() > 480 {
        ", over 480 bytes"
    } else {
        ""
    };
    format!("length {}, {category}{bound}", id.len())
}

/// Store the host ID as a source locator the attachment service accepts:
/// `scheme://rest` becomes `scheme:rest`; anything carrying a query or
/// fragment (which could hold a token) is replaced by a fingerprint.
fn locator(id: &str) -> String {
    let id = id.trim();
    if id.contains(['?', '#']) {
        use sha2::{Digest, Sha256};
        return format!("host-file-sha256:{:x}", Sha256::digest(id.as_bytes()));
    }
    id.replacen("://", ":", 1).replace("://", "/")
}

/// Prepare, stream and submit a host-provided original for owner review.
pub fn import(client: &dyn AttachmentClient, args: &Value) -> Result<Value> {
    import_with(client, args, download)
}

/// `import` with an injectable downloader, used by synthetic tests.
pub(crate) fn import_with(
    client: &dyn AttachmentClient,
    args: &Value,
    download: impl FnOnce(&str) -> Result<Vec<u8>>,
) -> Result<Value> {
    let object = args
        .as_object()
        .ok_or_else(|| invalid("Expected tool arguments"))?;
    if let Some(key) = object.keys().find(|key| {
        !["operationId", "memoryId", "expectedRevision", "file"].contains(&key.as_str())
    }) {
        return Err(invalid(format!("Unexpected argument {key}")));
    }
    let operation = contracts::string(args, "operationId")?;
    let file = &args["file"];
    let id = file["file_id"]
        .as_str()
        .filter(|id| file_id(id))
        .ok_or_else(|| unavailable(&received(file)))?;
    let link = file["download_url"]
        .as_str()
        .ok_or_else(|| unavailable(&received(file)))?;
    // An existing operation is reconciled without downloading again.
    match client.request(STATUS, &json!({"operationId":operation})) {
        Ok(state) if state["status"] != "uploading" => return Ok(state),
        Ok(_) => {}
        Err(error) if error.code() == "NOT_FOUND" => {}
        Err(error) => return Err(error),
    }
    let bytes = download(link)?;
    let name = file["file_name"]
        .as_str()
        .and_then(base_name)
        .unwrap_or_else(|| id.to_owned());
    let mut declared = json!({"fileName": name});
    if let Some(kind) = file["mime_type"].as_str().filter(|kind| {
        ["image/", "audio/", "video/"]
            .iter()
            .any(|prefix| kind.starts_with(prefix))
    }) {
        declared["mediaType"] = json!(kind);
    }
    send_original(
        client,
        &mut std::io::Cursor::new(bytes),
        json!({
            "operationId": operation,
            "memoryId": args["memoryId"],
            "expectedRevision": args["expectedRevision"],
            "sourceLocator": locator(id),
            "file": declared,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn url(text: &str) -> Url {
        Url::parse(text).unwrap()
    }

    #[test]
    fn only_host_file_domains_over_https_are_fetchable() {
        for ok in [
            "https://files.oaiusercontent.com/file-abc?sig=x",
            "https://sdmntprwestus.oaiusercontent.com/files/x",
            "https://chatgpt.com/backend-api/files/x",
            "https://cdn.openai.com/x",
            "https://oaisdmntprseasia.blob.core.windows.net/files/x?sv=2025&sig=redacted",
            "https://OAISDMNTPRSEASIA.blob.core.windows.net/files/x",
        ] {
            assert!(allowed(&url(ok)), "{ok}");
        }
        // Azure Blob Storage is open to anyone: only the exact observed account passes.
        for bad in [
            "http://oaisdmntprseasia.blob.core.windows.net/files/x",
            "https://oaisdmntprseasia.blob.core.windows.net:444/files/x",
            "https://oaisdmntprseasiax.blob.core.windows.net/files/x",
            "https://oaisdmntprnortheu.blob.core.windows.net/files/x",
            "https://evil.oaisdmntprseasia.blob.core.windows.net/files/x",
            "https://oaisdmntprseasia.blob.core.windows.net.evil.test/files/x",
            "https://attacker.blob.core.windows.net/files/x",
            "https://blob.core.windows.net/files/x",
            "https://user:pw@oaisdmntprseasia.blob.core.windows.net/files/x",
        ] {
            assert!(!allowed(&url(bad)), "{bad}");
        }
        for bad in [
            "http://files.oaiusercontent.com/x",
            "https://evil-oaiusercontent.com/x",
            "https://oaiusercontent.com.evil.test/x",
            "https://user:pw@files.oaiusercontent.com/x",
            "https://files.oaiusercontent.com:8443/x",
            "https://127.0.0.1/x",
            "https://[::1]/x",
            "https://localhost/x",
            "file:///C:/Windows/win.ini",
        ] {
            assert!(!allowed(&url(bad)), "{bad}");
        }
    }

    #[test]
    fn names_and_references_are_bounded() {
        assert_eq!(base_name("IMG_4408.mp4").as_deref(), Some("IMG_4408.mp4"));
        assert_eq!(
            base_name("/mnt/data/a\\IMG.mp4").as_deref(),
            Some("IMG.mp4")
        );
        assert_eq!(base_name("dir/"), None);
        assert_eq!(base_name(".."), None);
        assert!(file_id("file_00000000b6f8820790d24bf6796d36f5"));
        assert!(file_id("file-AbC123"));
        // Other host formats are opaque provenance, accepted within bounds.
        assert!(file_id("sediment://file_00000000b6f8820790d24bf6796d36f5"));
        assert!(file_id("chat_upload"));
        assert!(!file_id(""));
        assert!(!file_id("   "));
        assert!(!file_id(&"x".repeat(481)));
        assert!(file_id(&"x".repeat(480)));
        assert!(!file_id("file_abc\u{7}"));
        assert!(!file_id("file_abc\nInjected: header"));
        // Locators satisfy the service rule (no "://", "?" or "#") and never keep a query.
        assert_eq!(locator("file_abc"), "file_abc");
        assert_eq!(locator("sediment://file_abc"), "sediment:file_abc");
        let hashed = locator("https://files.example/x?sig=secret");
        assert!(hashed.starts_with("host-file-sha256:") && !hashed.contains("secret"));
        for id in ["sediment://file_abc", "a://b://c", "x?y", "x#y"] {
            let l = locator(id);
            assert!(!l.contains("://") && !l.contains(['?', '#']), "{l}");
        }
        assert_eq!(
            id_shape("sediment://file_abc"),
            "length 19, URI-like with scheme sediment"
        );
        assert_eq!(id_shape("file_abc"), "length 8, OpenAI file ID");
        assert!(id_shape(&"y".repeat(500)).ends_with("over 480 bytes"));
    }

    #[test]
    fn unresolved_files_are_described_by_shape_without_values() {
        let id = "file_00000000b6f8820790d24bf6796d36f5";
        let bare = received(&json!(id));
        assert!(bare.contains("bare file ID string"), "{bare}");
        assert!(!bare.contains(id));
        let partial = received(&json!({"file_id": id, "name": "IMG_4408.mp4"}));
        assert!(partial.contains("keys [file_id, name]"), "{partial}");
        assert!(partial.contains("file_id valid") && partial.contains("download_url missing"));
        assert!(!partial.contains(id) && !partial.contains("IMG_4408"));
        let link =
            received(&json!({"download_url": "https://files.oaiusercontent.com/x?sig=secret"}));
        assert!(link.contains("file_id missing") && link.contains("download_url present"));
        assert!(!link.contains("secret"));
        assert!(received(&Value::Null).contains("no file argument"));
    }

    struct Recorder(std::sync::Mutex<Vec<String>>);
    impl AttachmentClient for Recorder {
        fn request(&self, method: &str, _: &Value) -> Result<Value> {
            self.0.lock().unwrap().push(method.to_owned());
            if method == STATUS {
                Err(Error::domain("NOT_FOUND", "unknown"))
            } else {
                Err(Error::domain("FORBIDDEN", "not reached"))
            }
        }
    }

    #[test]
    fn missing_or_foreign_links_never_reach_prepare_or_the_network() {
        let client = Recorder(std::sync::Mutex::new(Vec::new()));
        let base = json!({"operationId":"9b2c7a8e-0f51-4a41-9a8d-4f0a2c1d3e5f",
            "memoryId":"1566a505-69d7-475c-a9c7-fcca57e0b8b5","expectedRevision":1});
        let mut bare = base.clone();
        bare["file"] = json!("chat_upload");
        assert_eq!(
            import(&client, &bare).unwrap_err().code(),
            "SOURCE_BYTES_UNAVAILABLE"
        );
        let mut foreign = base.clone();
        foreign["file"] =
            json!({"file_id":"file_abc12","download_url":"https://169.254.169.254/latest"});
        let error = import(&client, &foreign).unwrap_err();
        assert_eq!(error.code(), "SOURCE_BYTES_UNAVAILABLE");
        assert!(error.to_string().contains("169.254.169.254"));
        assert!(!error.to_string().contains("/latest"));
        let mut extra = base;
        extra["url"] = json!("https://example.test");
        assert_eq!(import(&client, &extra).unwrap_err().code(), "INVALID_INPUT");
        // Only the status reconciliation was attempted; nothing was prepared.
        assert_eq!(*client.0.lock().unwrap(), vec![STATUS.to_owned()]);
    }
}
