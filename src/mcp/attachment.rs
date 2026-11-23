//! ChatGPT supplies a short-lived download URL through its file parameter.
use super::input_error;
use mother::{
    error::{Error, Result},
    service::{LocalClient, attachment::MAX_BYTES},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::Mutex,
};
use url::Url;
use uuid::Uuid;

static DOWNLOADING: Mutex<()> = Mutex::new(());

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Proposal {
    proposal_id: String,
    memory_id: String,
    file: HostFile,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HostFile {
    download_url: String,
    file_id: String,
    file_name: Option<String>,
    mime_type: Option<String>,
}

fn allowed_url(raw: &str) -> Result<Url> {
    if raw.len() > 16_384
        || raw
            .chars()
            .any(|c| c.is_control() || matches!(c, '"' | '\\'))
    {
        return Err(input_error());
    }
    let url = Url::parse(raw).map_err(|_| input_error())?;
    let host = url.host_str().unwrap_or_default();
    if url.scheme() != "https"
        || !host.ends_with(".oaiusercontent.com")
        || url.port().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(Error::domain(
            "INVALID_INPUT",
            "Select a ChatGPT-hosted file with a supported HTTPS download address.",
        ));
    }
    Ok(url)
}

fn failed() -> Error {
    Error::domain(
        "INVALID_INPUT",
        "The selected file could not be downloaded within the 2 MiB limit. Select the file again for a fresh link.",
    )
}

fn download(url: &Url) -> Result<Vec<u8>> {
    // Use the OS TLS client without a shell, curlrc, redirects or inherited proxy settings.
    // Feed the URL on stdin so signed query strings never appear on a process command line.
    #[cfg(windows)]
    let curl =
        PathBuf::from(std::env::var_os("SystemRoot").ok_or_else(failed)?).join("System32/curl.exe");
    #[cfg(not(windows))]
    let curl = PathBuf::from("/usr/bin/curl");
    let mut cmd = Command::new(curl);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    let mut child = cmd
        .args([
            "-q",
            "--silent",
            "--fail",
            "--globoff",
            "--proto",
            "=https",
            "--proxy",
            "",
            "--max-time",
            "25",
            "--connect-timeout",
            "5",
            "--max-filesize",
            &MAX_BYTES.to_string(),
            "--write-out",
            "%{http_code}",
            "--config",
            "-",
        ])
        .env_remove("SSLKEYLOGFILE")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| failed())?;
    let result = (|| {
        writeln!(
            child.stdin.take().ok_or_else(failed)?,
            "url = \"{}\"",
            url.as_str()
        )
        .map_err(|_| failed())?;
        let mut bytes = Vec::new();
        child
            .stdout
            .take()
            .ok_or_else(failed)?
            .take((MAX_BYTES + 4) as u64)
            .read_to_end(&mut bytes)
            .map_err(|_| failed())?;
        if bytes.len() > MAX_BYTES + 3 || bytes.len() <= 3 || !bytes.ends_with(b"200") {
            return Err(failed());
        }
        bytes.truncate(bytes.len() - 3);
        Ok(bytes)
    })();
    if result.is_err() {
        let _ = child.kill();
    }
    if !child.wait().map_err(|_| failed())?.success() {
        return Err(failed());
    }
    result
}

pub(super) fn propose(client: &LocalClient, args: Value) -> Result<Value> {
    let p: Proposal = serde_json::from_value(args).map_err(|_| input_error())?;
    for id in [&p.proposal_id, &p.memory_id] {
        if Uuid::parse_str(id).map_err(|_| input_error())?.to_string() != *id {
            return Err(input_error());
        }
    }
    let url = allowed_url(&p.file.download_url)?;
    let _slot = DOWNLOADING
        .try_lock()
        .map_err(|_| Error::domain("RESOURCE_LIMIT", "Another media transfer is in progress."))?;
    let target = client.call(
        "local_media_target",
        &json!({"memoryId":p.memory_id.to_string()}),
    )?;
    let bytes = download(&url)?;
    mother::service::attachment::submit(
        client,
        json!({"proposalId":p.proposal_id,"memoryId":p.memory_id,
        "expectedRevision":target["revision"],"fileId":p.file.file_id,"fileName":p.file.file_name,"mimeType":p.file.mime_type}),
        &bytes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn file_url_is_https_on_the_host_file_domain() {
        assert!(allowed_url("https://files.oaiusercontent.com/file?sig=abc").is_ok());
        for url in [
            "file:///private/media",
            "https://localhost/",
            "http://files.oaiusercontent.com/a",
            "https://files.oaiusercontent.com.example.org/a",
            "https://name@files.oaiusercontent.com/a",
            "https://files.oaiusercontent.com:8080/a",
            "https://files.oaiusercontent.com/a\nheader",
            "https://files.oaiusercontent.com/a#fragment",
        ] {
            assert!(allowed_url(url).is_err());
        }
    }
}
