//! Loopback Garden reader and local owner attachment review.
use mother::{
    error::{Error, Result},
    garden::store::GardenStore,
    service::{LocalService, attachment},
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
use subtle::ConstantTimeEq;

const MAX_HEAD: usize = 8 * 1024;
const MAX_REVIEW_BODY: usize = 96 * 1024;
const MAX_CONNECTIONS: usize = 32;
const IO_TIMEOUT: Duration = Duration::from_secs(5);
const JS: &str = "text/javascript; charset=utf-8";
const JSON: &str = "application/json";
const TEXT: &str = "text/plain; charset=utf-8";
const JPEG: &str = "image/jpeg";
const OGG: &str = "audio/ogg";
/// Everything the page may load comes from this server; nothing may leave it.
const POLICY: &str = "default-src 'none'; script-src 'self'; worker-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; media-src 'self' blob:; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

macro_rules! asset {
    ($name:literal) => {
        include_bytes!(concat!("../assets/garden-explorer/", $name)).as_slice()
    };
}
fn asset(path: &str) -> Option<(&'static str, &'static [u8])> {
    Some(match path {
        "/" => ("text/html; charset=utf-8", asset!("index.html")),
        "/attachments" => ("text/html; charset=utf-8", asset!("attachments.html")),
        "/attachment-review.js" => (JS, asset!("attachment-review.js")),
        "/trust" => ("text/html; charset=utf-8", asset!("trust.html")),
        "/trust.js" => (JS, asset!("trust.js")),
        "/assets/index.js" => (JS, asset!("assets/index.js")),
        "/assets/bake.worker.js" => (JS, asset!("assets/bake.worker.js")),
        "/glasshouse-pool.jpg" => (JPEG, asset!("glasshouse-pool.jpg")),
        "/cherry-bough.glb" => ("model/gltf-binary", asset!("cherry-bough.glb")),
        "/textures/Ground048_Color.jpg" => (JPEG, asset!("textures/Ground048_Color.jpg")),
        "/textures/Ground048_NormalGL.jpg" => (JPEG, asset!("textures/Ground048_NormalGL.jpg")),
        "/textures/Ground103_Color.jpg" => (JPEG, asset!("textures/Ground103_Color.jpg")),
        "/textures/Ground103_NormalGL.jpg" => (JPEG, asset!("textures/Ground103_NormalGL.jpg")),
        "/audio/manifest.json" => (JSON, asset!("audio/manifest.json")),
        "/audio/footstepOnGrass1Medium.ogg" => (OGG, asset!("audio/footstepOnGrass1Medium.ogg")),
        "/audio/footstepOnGrass2Medium.ogg" => (OGG, asset!("audio/footstepOnGrass2Medium.ogg")),
        "/audio/footstepOnGrass3Medium.ogg" => (OGG, asset!("audio/footstepOnGrass3Medium.ogg")),
        "/audio/footstepOnGrass4Medium.ogg" => (OGG, asset!("audio/footstepOnGrass4Medium.ogg")),
        "/audio/footstepOnGrass5Medium.ogg" => (OGG, asset!("audio/footstepOnGrass5Medium.ogg")),
        // the notices that travel with the valley's code and art
        "/LICENSE.txt" => (TEXT, asset!("LICENSE.txt")),
        "/LICENCE-NOTES.md" => (TEXT, asset!("LICENCE-NOTES.md")),
        "/THIRD-PARTY-NOTICES.md" => (TEXT, asset!("THIRD-PARTY-NOTICES.md")),
        "/CREDITS.md" => (TEXT, asset!("CREDITS.md")),
        "/LICENSES/CC0-1.0.txt" => (TEXT, asset!("LICENSES/CC0-1.0.txt")),
        _ => return None,
    })
}

const HTML: &str = "text/html; charset=utf-8";
const GALLERY_HTML: &[u8] = include_bytes!("../assets/media-gallery/index.html");
const GALLERY_JS: &[u8] = include_bytes!("../assets/media-gallery/gallery.js");
/// Total image bytes one gallery snapshot may hold in memory.
const GALLERY_BYTES: u64 = 256 * 1024 * 1024;
/// Total bytes of full record readings an offline snapshot keeps after releasing the database.
const READING_BYTES: usize = 64 * 1024 * 1024;
const READINGS: &str = "/api/garden/records/";

/// One owner snapshot of attached images, loaded before the database is released.
#[derive(Debug, Default)]
struct Gallery {
    listing: Vec<u8>,
    images: BTreeMap<String, (&'static str, Vec<u8>)>,
}
impl Gallery {
    /// No attached media: what the garden serves when the store holds none it can show.
    fn empty() -> Self {
        Self {
            listing: br#"{"images":[],"withheld":0,"truncated":false,"readOnly":true}"#.to_vec(),
            images: BTreeMap::new(),
        }
    }
}

struct LiveView {
    store: Weak<GardenStore>,
    limit: usize,
    attachment_root: Option<PathBuf>,
}

impl LiveView {
    fn store(&self) -> Result<Arc<GardenStore>> {
        self.store.upgrade().ok_or_else(|| {
            Error::domain("STORE_UNAVAILABLE", "The Garden owner service has stopped.")
        })
    }

    fn reading(&self, id: &str) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(&self.store()?.v1_garden_reader(id)?)?)
    }

    fn snapshot(&self, since: Option<&str>) -> Result<Vec<u8>> {
        let store = self.store()?;
        let mut view = store.v1_garden_view(self.limit)?;
        let generated_at = view["generatedAt"].take();
        let media = store.media_owner_list(None).ok();
        let media_revision = format!("{:x}", Sha256::digest(serde_json::to_vec(&media)?));
        let revision = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&(&view, &media_revision))?)
        );
        if since == Some(&revision) {
            return Ok(br#"{"unchanged":true}"#.to_vec());
        }
        view["generatedAt"] = generated_at;
        view["liveRevision"] = json!(revision);
        view["liveMediaRevision"] = json!(media_revision);
        Ok(serde_json::to_vec(&view)?)
    }
}

fn secret() -> String {
    let mut bytes = [0; 32];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(Debug, PartialEq)]
struct Reply {
    status: u16,
    kind: &'static str,
    body: Cow<'static, [u8]>,
}
impl Reply {
    fn refuse(status: u16, message: &str) -> Self {
        Self {
            status,
            kind: TEXT,
            body: Cow::Owned(message.as_bytes().to_vec()),
        }
    }
}

/// Offline links are single-use. Live links are kept alongside the owner-private endpoint.
struct Site {
    authority: [String; 2],
    origin: [String; 2],
    launch: Mutex<Option<String>>,
    session: Mutex<Option<String>>,
    snapshot: Vec<u8>,
    /// Offline only: each shown record's full reading, taken with the snapshot.
    readings: BTreeMap<String, Vec<u8>>,
    gallery: Mutex<Option<Gallery>>,
    live: Option<LiveView>,
    /// True when "/" is the media gallery page rather than the garden.
    gallery_page: bool,
}
impl Site {
    fn new(port: u16, launch: String, snapshot: Vec<u8>) -> Self {
        let authority = [format!("127.0.0.1:{port}"), format!("localhost:{port}")];
        Self {
            origin: authority.clone().map(|a| format!("http://{a}")),
            authority,
            launch: Mutex::new(Some(launch)),
            session: Mutex::new(None),
            snapshot,
            readings: BTreeMap::new(),
            gallery: Mutex::new(None),
            live: None,
            gallery_page: false,
        }
    }
    /// The media gallery page on its own (`--media`).
    fn with_gallery(port: u16, launch: String, gallery: Gallery) -> Self {
        let mut site = Self::new(port, launch, gallery.listing.clone());
        site.gallery = Mutex::new(Some(gallery));
        site.gallery_page = true;
        site
    }
    /// The garden, with its attached media served under the same session for the photo grove.
    fn with_media(port: u16, launch: String, snapshot: Vec<u8>, gallery: Gallery) -> Self {
        let mut site = Self::new(port, launch, snapshot);
        site.gallery = Mutex::new(Some(gallery));
        site
    }
    fn signed_in(&self, bearer: Option<&str>) -> bool {
        let session = self.session.lock().unwrap_or_else(|e| e.into_inner());
        matches(session.as_deref(), bearer)
    }
    /// Gallery routes; `None` falls through to the shared session and snapshot routes.
    fn gallery_reply(&self, method: &str, path: &str, bearer: Option<&str>) -> Option<Reply> {
        if method != "GET" || !(self.gallery_page || path.starts_with("/api/media")) {
            return None;
        }
        if path.starts_with("/api/media") && !self.signed_in(bearer) {
            return Some(Reply::refuse(
                401,
                "Open the gallery from your launch link.",
            ));
        }
        let mut stored = self.gallery.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(live) = &self.live {
            // Refresh the owner decision before each live read. Listing a memory needs
            // metadata only. A byte request loads just the requested authorized variant.
            let wanted = path.strip_prefix("/api/media/").unwrap_or("");
            match live
                .store()
                .and_then(|store| gallery_selection(&store, false, Some(wanted)))
            {
                Ok(current) => *stored = Some(current),
                Err(_) => return Some(Reply::refuse(503, "Garden media is unavailable.")),
            }
        }
        let gallery = stored.as_ref()?;
        let page = |kind, body: &'static [u8]| Reply {
            status: 200,
            kind,
            body: Cow::Borrowed(body),
        };
        Some(match (method, path) {
            ("GET", "/") if self.gallery_page => page(HTML, GALLERY_HTML),
            ("GET", "/gallery.js") if self.gallery_page => page(JS, GALLERY_JS),
            ("GET", "/api/media") => Reply {
                status: 200,
                kind: JSON,
                body: Cow::Owned(gallery.listing.clone()),
            },
            ("GET", path) if path.starts_with("/api/media") => {
                let sha = path.strip_prefix("/api/media/").unwrap_or_default();
                match gallery.images.get(sha) {
                    Some((kind, bytes)) => Reply {
                        status: 200,
                        kind,
                        body: Cow::Owned(bytes.clone()),
                    },
                    None => Reply::refuse(404, "Not found."),
                }
            }
            ("GET", _) if self.gallery_page && asset(path).is_some() => {
                Reply::refuse(404, "Not found.")
            }
            _ => return None,
        })
    }

    fn respond(&self, head: &str) -> Reply {
        let (headers, body) = head.split_once("\r\n\r\n").unwrap_or((head, ""));
        let mut lines = headers.split("\r\n");
        let mut request = lines.next().unwrap_or_default().split(' ');
        let (Some(method), Some(path), Some("HTTP/1.1" | "HTTP/1.0"), None) = (
            request.next(),
            request.next(),
            request.next(),
            request.next(),
        ) else {
            return Reply::refuse(400, "Bad request.");
        };
        let header = |name: &str| {
            lines.clone().find_map(|line| {
                let (key, value) = line.split_once(':')?;
                key.eq_ignore_ascii_case(name).then(|| value.trim())
            })
        };
        // A page on another site can reach this port, but not with this name or origin.
        if !header("host").is_some_and(|host| self.authority.iter().any(|a| a == host)) {
            return Reply::refuse(403, "This garden only answers at its own local address.");
        }
        let origin = header("origin");
        if origin.is_some_and(|origin| !self.origin.iter().any(|o| o == origin)) {
            return Reply::refuse(403, "Requests from other sites are refused.");
        }
        let bearer = header("authorization").and_then(|value| value.strip_prefix("Bearer "));
        // (the page reads its own drawing options, such as ?quality=low, from the address)
        let (path, query) = path.split_once('?').unwrap_or((path, ""));
        if let Some(reply) = self.gallery_reply(method, path, bearer) {
            return reply;
        }
        match (method, path) {
            ("GET", "/")
                if self
                    .live
                    .as_ref()
                    .is_some_and(|live| live.attachment_root.is_some()) =>
            {
                let html = String::from_utf8_lossy(asset!("index.html")).replacen(
                    r#"<button class="btn glass" aria-label="Reading Path" id="pathBtn""#,
                    r#"<a class="btn glass" href="/attachments" aria-label="Attachment review" title="Attachment review" style="color:inherit;text-decoration:none"><svg class="ico" viewBox="0 0 24 24" aria-hidden="true"><path d="m8 12 5-5a3 3 0 0 1 4 4l-7 7a4 4 0 0 1-6-6l7-7"/></svg></a><a class="btn glass" href="/trust" aria-label="Agent trust" title="Agent trust" style="color:inherit;text-decoration:none"><svg class="ico" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3 5 6v5c0 4.5 3 8.3 7 10 4-1.7 7-5.5 7-10V6z"/></svg></a><button class="btn glass" aria-label="Reading Path" id="pathBtn""#,
                    1,
                );
                Reply {
                    status: 200,
                    kind: HTML,
                    body: Cow::Owned(html.into_bytes()),
                }
            }
            ("GET", path) if asset(path).is_some() => {
                let (kind, body) = asset(path).expect("checked above");
                Reply {
                    status: 200,
                    kind,
                    body: Cow::Borrowed(body),
                }
            }
            ("POST", "/api/session") => {
                if origin.is_none() {
                    return Reply::refuse(403, "Requests from other sites are refused.");
                }
                let mut launch = self.launch.lock().unwrap_or_else(|e| e.into_inner());
                if !matches(launch.as_deref(), bearer) {
                    return Reply::refuse(401, "This link has already been used or is not valid.");
                }
                if self.live.is_none() {
                    *launch = None;
                }
                let mut session = self.session.lock().unwrap_or_else(|e| e.into_inner());
                let session = session.get_or_insert_with(secret);
                let body = Cow::Owned(json!({ "session": session }).to_string().into_bytes());
                Reply {
                    status: 200,
                    kind: JSON,
                    body,
                }
            }
            ("GET", "/api/trust") | ("POST", "/api/trust") => {
                if !self.signed_in(bearer) {
                    return Reply::refuse(401, "Open Agent trust from the live Garden.");
                }
                let Some(live) = &self.live else {
                    return Reply::refuse(404, "Agent trust requires the live Garden.");
                };
                let result = live.store().and_then(|store| {
                    let activity = |store: &GardenStore| match &live.attachment_root {
                        Some(root) => {
                            attachment::owner_request(store, root, "activity", &json!({}))
                        }
                        None => Ok(json!({"items": []})),
                    };
                    if method == "GET" {
                        let mut overview = store.trust_overview()?;
                        overview["activity"] = activity(&store)?["items"].take();
                        return Ok(overview);
                    }
                    if origin.is_none() {
                        return Err(Error::domain(
                            "FORBIDDEN",
                            "A same-origin owner request is required.",
                        ));
                    }
                    if header("content-type") != Some("application/json")
                        || body.len() > MAX_REVIEW_BODY
                    {
                        return Err(Error::domain(
                            "INVALID_INPUT",
                            "Expected a bounded JSON body.",
                        ));
                    }
                    let body: serde_json::Value = serde_json::from_str(body)?;
                    match body["action"].as_str() {
                        Some("set") => {
                            let mode: mother::garden::store::trust::TrustMode =
                                serde_json::from_value(body["mode"].clone()).map_err(|_| {
                                    Error::domain("INVALID_INPUT", "Unknown trust mode.")
                                })?;
                            let number = |key: &str| {
                                body[key].as_u64().ok_or_else(|| {
                                    Error::domain("INVALID_INPUT", format!("Missing {key}."))
                                })
                            };
                            let policy = store.set_trust_policy(
                                body["clientId"].as_str().ok_or_else(|| {
                                    Error::domain("INVALID_INPUT", "Missing clientId.")
                                })?,
                                mode,
                                u32::try_from(number("maxAutoCommitsPerDay")?).map_err(|_| {
                                    Error::domain("INVALID_INPUT", "Limit too large.")
                                })?,
                                number("maxAutoBytesPerDay")?,
                                number("expectedRevision")?,
                            )?;
                            Ok(json!({"policy": policy}))
                        }
                        Some("retract") => match &live.attachment_root {
                            Some(root) => attachment::owner_request(&store, root, "retract", &body),
                            None => Err(Error::domain(
                                "NOT_FOUND",
                                "Attachment activity is unavailable.",
                            )),
                        },
                        _ => Err(Error::domain("INVALID_INPUT", "Unknown trust action.")),
                    }
                });
                match result {
                    Ok(value) => Reply {
                        status: 200,
                        kind: JSON,
                        body: Cow::Owned(value.to_string().into_bytes()),
                    },
                    Err(error) => {
                        let status = match error.code() {
                            "FORBIDDEN" => 403,
                            "NOT_FOUND" => 404,
                            "CONFLICT" => 409,
                            "STORE_UNAVAILABLE" => 503,
                            _ => 400,
                        };
                        Reply {
                            status,
                            kind: JSON,
                            body: Cow::Owned(
                                json!({"code":error.code(),"message":error.to_string()})
                                    .to_string()
                                    .into_bytes(),
                            ),
                        }
                    }
                }
            }
            ("GET", "/api/attachments") | ("POST", "/api/attachments") => {
                if !self.signed_in(bearer) {
                    return Reply::refuse(401, "Open Attachment review from the live Garden.");
                }
                let Some(live) = &self.live else {
                    return Reply::refuse(404, "Attachment review requires the live Garden.");
                };
                let Some(root) = &live.attachment_root else {
                    return Reply::refuse(404, "Attachment review is unavailable.");
                };
                let result = live.store().and_then(|store| {
                    if method == "GET" {
                        return attachment::owner_request(&store, root, "list", &json!({}));
                    }
                    if origin.is_none() {
                        return Err(Error::domain(
                            "FORBIDDEN",
                            "A same-origin owner request is required.",
                        ));
                    }
                    if header("content-type") != Some("application/json")
                        || body.len() > MAX_REVIEW_BODY
                    {
                        return Err(Error::domain(
                            "INVALID_INPUT",
                            "Expected a bounded JSON body.",
                        ));
                    }
                    let body: serde_json::Value = serde_json::from_str(body)?;
                    let action = body["action"]
                        .as_str()
                        .ok_or_else(|| Error::domain("INVALID_INPUT", "Missing review action."))?;
                    attachment::owner_request(&store, root, action, &body)
                });
                match result {
                    Ok(value) => Reply {
                        status: 200,
                        kind: JSON,
                        body: Cow::Owned(value.to_string().into_bytes()),
                    },
                    Err(error) => {
                        let status = match error.code() {
                            "FORBIDDEN" => 403,
                            "NOT_FOUND" => 404,
                            "CONFLICT" => 409,
                            "RATE_LIMITED" => 429,
                            "STORE_UNAVAILABLE" => 503,
                            _ => 400,
                        };
                        let message = if status == 503 {
                            "Attachment review could not reconcile the operation. Refresh its status.".into()
                        } else {
                            error.to_string()
                        };
                        Reply {
                            status,
                            kind: JSON,
                            body: Cow::Owned(
                                json!({"code":error.code(),"message":message})
                                    .to_string()
                                    .into_bytes(),
                            ),
                        }
                    }
                }
            }
            ("GET", "/api/garden") => {
                if !self.signed_in(bearer) {
                    return Reply::refuse(401, "Open the garden from the link in your terminal.");
                }
                let snapshot = match &self.live {
                    None => self.snapshot.clone(),
                    Some(live) => match live.snapshot(query.strip_prefix("since=")) {
                        Ok(snapshot) => snapshot,
                        Err(_) => return Reply::refuse(503, "The live Garden is unavailable."),
                    },
                };
                Reply {
                    status: 200,
                    kind: JSON,
                    body: Cow::Owned(snapshot),
                }
            }
            ("GET", path) if path.starts_with(READINGS) => {
                if !self.signed_in(bearer) {
                    return Reply::refuse(401, "Open the garden from the link in your terminal.");
                }
                let id = &path[READINGS.len()..];
                let reading = match &self.live {
                    None => self.readings.get(id).cloned().ok_or(None),
                    Some(live) => live.reading(id).map_err(|e| Some(e.code() == "NOT_FOUND")),
                };
                match reading {
                    Ok(body) => Reply {
                        status: 200,
                        kind: JSON,
                        body: Cow::Owned(body),
                    },
                    // Unknown, malformed and outside-the-snapshot IDs all read alike.
                    Err(None | Some(true)) => Reply::refuse(404, "Not found."),
                    Err(Some(false)) => Reply::refuse(503, "The live Garden is unavailable."),
                }
            }
            ("GET" | "POST", _) => Reply::refuse(404, "Not found."),
            _ => Reply::refuse(405, "Only reading is supported."),
        }
    }
}

fn matches(expected: Option<&str>, given: Option<&str>) -> bool {
    match (expected, given) {
        (Some(expected), Some(given)) => expected.as_bytes().ct_eq(given.as_bytes()).into(),
        _ => false,
    }
}

fn read_head(stream: &mut TcpStream) -> Option<String> {
    let mut head = Vec::new();
    let mut chunk = [0; 1024];
    let deadline = Instant::now() + IO_TIMEOUT;
    let end = loop {
        stream
            .set_read_timeout(Some(deadline.checked_duration_since(Instant::now())?))
            .ok()?;
        let read = stream.read(&mut chunk).ok()?;
        if read == 0 {
            return None;
        }
        head.extend_from_slice(&chunk[..read]);
        if let Some(end) = head.windows(4).position(|w| w == b"\r\n\r\n") {
            break end + 4;
        }
        if head.len() > MAX_HEAD {
            return None;
        }
    };
    if end > MAX_HEAD {
        return None;
    }
    let headers = std::str::from_utf8(&head[..end]).ok()?;
    if headers.starts_with("POST /api/attachments ") || headers.starts_with("POST /api/trust ") {
        let mut length = None;
        for line in headers.lines().skip(1) {
            if let Some((name, value)) = line.split_once(':') {
                if name.eq_ignore_ascii_case("transfer-encoding") {
                    return None;
                }
                if name.eq_ignore_ascii_case("content-length") {
                    if length.is_some() {
                        return None;
                    }
                    length = Some(value.trim().parse::<usize>().ok()?);
                }
            }
        }
        let length = length.filter(|length| (1..=MAX_REVIEW_BODY).contains(length))?;
        if head.len() > end + length {
            return None;
        }
        while head.len() < end + length {
            stream
                .set_read_timeout(Some(deadline.checked_duration_since(Instant::now())?))
                .ok()?;
            let remaining = (end + length - head.len()).min(chunk.len());
            let read = stream.read(&mut chunk[..remaining]).ok()?;
            if read == 0 {
                return None;
            }
            head.extend_from_slice(&chunk[..read]);
        }
    } else {
        head.truncate(end);
    }
    String::from_utf8(head).ok()
}

fn serve(site: &Site, mut stream: TcpStream) {
    if stream.set_read_timeout(Some(IO_TIMEOUT)).is_err()
        || stream.set_write_timeout(Some(IO_TIMEOUT)).is_err()
    {
        return;
    }
    let reply = match read_head(&mut stream) {
        Some(head) => site.respond(&head),
        None => Reply::refuse(400, "Bad request."),
    };
    let reason = match reply.status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        409 => "Conflict",
        429 => "Too Many Requests",
        503 => "Service Unavailable",
        _ => "Method Not Allowed",
    };
    let head = format!(
        "HTTP/1.1 {} {reason}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nContent-Security-Policy: {POLICY}\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nCross-Origin-Resource-Policy: same-origin\r\n\r\n",
        reply.status,
        reply.kind,
        reply.body.len()
    );
    let _ = stream
        .write_all(head.as_bytes())
        .and_then(|()| stream.write_all(&reply.body))
        .and_then(|()| stream.flush());
}

fn open_browser(url: &str) {
    let mut command = if cfg!(windows) {
        let mut command = std::process::Command::new("rundll32");
        command.arg("url.dll,FileProtocolHandler");
        command
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open")
    } else {
        std::process::Command::new("xdg-open")
    };
    if command.arg(url).spawn().is_err() {
        eprintln!("Could not open a browser. Open the link above yourself.");
    }
}

/// Snapshot attached images. Derivatives are shown first; an original carrying
/// embedded location or device data is withheld unless the owner asks for originals.
fn gallery(store: &GardenStore, originals: bool) -> Result<Gallery> {
    gallery_selection(store, originals, None)
}

fn gallery_selection(
    store: &GardenStore,
    originals: bool,
    wanted: Option<&str>,
) -> Result<Gallery> {
    let list = store
        .media_owner_list(None)
        .map_err(mother::garden::config::domain_error)?;
    let mut gallery = Gallery::default();
    let (mut shown, mut withheld, mut total, mut truncated) = (Vec::new(), 0u64, 0u64, false);
    let mut unavailable_memories = std::collections::BTreeSet::new();
    for entry in list["assets"].as_array().into_iter().flatten() {
        let links = || entry["memoryLinks"].as_array().into_iter().flatten();
        let mut mark_unavailable = || {
            for link in links() {
                if let Some(id) = link["memoryId"].as_str() {
                    unavailable_memories.insert(id);
                }
            }
        };
        let asset = &entry["asset"];
        let uses = entry["uses"].as_array().map_or(0, Vec::len);
        if uses == 0 {
            continue;
        }
        let clean = asset["embeddedMetadata"]
            .as_array()
            .is_some_and(Vec::is_empty);
        let original_approved = entry["memoryLinks"]
            .as_array()
            .is_some_and(|links| links.iter().any(|link| link["allowOriginal"] == true));
        let (variant, sha) = match entry["derivatives"][0].as_str() {
            Some(derivative) => ("derivative", derivative),
            None if clean || originals || original_approved => {
                ("original", asset["sha256"].as_str().unwrap_or_default())
            }
            None => {
                mark_unavailable();
                withheld += 1;
                continue;
            }
        };
        let Some(kind) = mother::media::MEDIA_TYPES
            .iter()
            .find(|t| **t == asset["mediaType"])
        else {
            continue;
        };
        let size = asset["sizeBytes"].as_u64().unwrap_or(u64::MAX);
        if total.saturating_add(size) > GALLERY_BYTES {
            mark_unavailable();
            truncated = true;
            continue;
        }
        total += size;
        if wanted.is_none_or(|requested| requested == sha) {
            let bytes = store
                .media_owner_bytes(sha)
                .map_err(mother::garden::config::domain_error)?;
            gallery.images.insert(sha.to_owned(), (kind, bytes));
        }
        let visible_links: Vec<&serde_json::Value> = links()
            .filter(|link| {
                if variant != "original" || clean || originals || link["allowOriginal"] == true {
                    return true;
                }
                if let Some(id) = link["memoryId"].as_str() {
                    unavailable_memories.insert(id);
                }
                false
            })
            .collect();
        shown.push(
            json!({"sha256":sha,"originalSha256":asset["sha256"],"variant":variant,
            "mediaType":kind,"width":asset["width"],"height":asset["height"],
            "acquisitions":entry["acquisitions"],"interpretations":entry["interpretations"],
            "uses":uses,"memoryLinks":visible_links}),
        );
    }
    gallery.listing = serde_json::to_vec(
        &json!({"images":shown,"unavailableMemories":unavailable_memories,"withheld":withheld,"truncated":truncated,"readOnly":true}),
    )?;
    Ok(gallery)
}

/// Read one snapshot, release the database, then serve it on loopback until interrupted.
/// `gallery_only` serves the media gallery page instead of the garden; `originals` also shows
/// originals that carry embedded location or device data, in either.
pub(crate) fn run(
    db: &Path,
    limit: usize,
    open: bool,
    gallery_only: bool,
    originals: bool,
) -> Result<()> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let port = listener.local_addr()?.port();
    let launch = secret();
    let url = format!("http://127.0.0.1:{port}/#{launch}");
    let site = if gallery_only {
        let gallery = gallery(&GardenStore::open_active(db)?, originals)?;
        let listing: serde_json::Value = serde_json::from_slice(&gallery.listing)?;
        println!(
            "{}",
            json!({"url": url, "images": listing["images"].as_array().map_or(0, Vec::len),
                "withheld": listing["withheld"], "readOnly": true})
        );
        Arc::new(Site::with_gallery(port, launch, gallery))
    } else {
        let store = GardenStore::open_active(db)?;
        let view = store.v1_garden_view(limit)?;
        let readings = offline_readings(&store, &view)?;
        // The garden's photo grove: attached media, read in the same visit to the store.
        let media = gallery(&store, originals).unwrap_or_else(|error| {
            eprintln!("Attached media could not be read for this snapshot: {error}");
            Gallery::empty()
        });
        drop(store);
        let listing: serde_json::Value = serde_json::from_slice(&media.listing)?;
        println!(
            "{}",
            json!({"url": url, "shown": view["shown"], "total": view["total"],
                "images": listing["images"].as_array().map_or(0, Vec::len),
                "withheld": listing["withheld"], "readOnly": true})
        );
        let mut site = Site::with_media(port, launch, serde_json::to_vec(&view)?, media);
        site.readings = readings;
        Arc::new(site)
    };
    std::io::stdout().flush()?;
    eprintln!(
        "The link works once. The garden is a snapshot taken just now; press Ctrl+C to close it."
    );
    if open {
        open_browser(&url);
    }
    serve_site(listener, site)
}

/// Full readings of the snapshot's records, kept so the reader works after the database is
/// released. Past the byte budget, later records keep only their snapshot preview.
fn offline_readings(
    store: &GardenStore,
    view: &serde_json::Value,
) -> Result<BTreeMap<String, Vec<u8>>> {
    let ids: Vec<&str> = view["records"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| r["id"].as_str())
        .collect();
    let mut kept = BTreeMap::new();
    let mut total = 0;
    for reading in store.v1_garden_readers(&ids)? {
        let bytes = serde_json::to_vec(&reading)?;
        total += bytes.len();
        if total > READING_BYTES {
            eprintln!("Some long records show only their beginning in this offline snapshot.");
            break;
        }
        if let Some(id) = reading["recordId"].as_str() {
            kept.insert(id.to_owned(), bytes);
        }
    }
    Ok(kept)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LiveAddress {
    port: u16,
    token: String,
}

impl LiveAddress {
    fn announce(&self, open: bool) -> Result<()> {
        let url = format!("http://127.0.0.1:{}/#{}", self.port, self.token);
        println!("{}", json!({"url": url, "live": true, "readOnly": true}));
        std::io::stdout().flush()?;
        if open {
            open_browser(&url);
        }
        Ok(())
    }
}

pub(crate) struct LiveLaunch {
    path: PathBuf,
    address: LiveAddress,
}

impl LiveLaunch {
    pub(crate) fn announce(&self, open: bool) -> Result<()> {
        self.address.announce(open)
    }
}

impl Drop for LiveLaunch {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn live_address_path(endpoint: &Path, service_id: &str) -> Result<PathBuf> {
    let id = uuid::Uuid::parse_str(service_id)
        .map_err(|_| Error::domain("INVALID_ENDPOINT", "Invalid local service identity."))?;
    Ok(endpoint.with_added_extension(format!("{id}.explorer")))
}

pub(crate) fn open_live(endpoint: &Path, open: bool) -> Result<()> {
    mother::config::check_private_output(endpoint)?;
    let info: mother::service::ServiceInfo =
        serde_json::from_value(mother::config::read_json(endpoint, 16_384)?)?;
    let path = live_address_path(endpoint, &info.service_id)?;
    mother::config::check_private_output(&path)?;
    if !path.try_exists()? {
        return Err(Error::domain(
            "NOT_FOUND",
            "This owner has no live Garden. Start the owner with --explore to enable it.",
        ));
    }
    let address: LiveAddress = serde_json::from_value(mother::config::read_json(&path, 1024)?)?;
    if address.port == 0
        || address.token.len() != 64
        || !address.token.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(Error::domain(
            "INVALID_ENDPOINT",
            "Invalid live Garden address.",
        ));
    }
    address.announce(open)
}

pub(crate) fn start_live(service: &LocalService, endpoint: &Path) -> Result<LiveLaunch> {
    if !service.info().v1_available {
        return Err(Error::domain(
            "COMPAT_NOT_INITIALIZED",
            "The live Garden requires the native v1 view.",
        ));
    }
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let address = LiveAddress {
        port: listener.local_addr()?.port(),
        token: secret(),
    };
    let path = live_address_path(endpoint, &service.info().service_id)?;
    mother::config::check_private_output(&path)?;
    let parent = path
        .parent()
        .ok_or_else(|| Error::domain("INVALID_ENDPOINT", "Missing owner directory."))?;
    let mut file = tempfile::Builder::new()
        .prefix(".garden-explorer-")
        .tempfile_in(parent)?;
    serde_json::to_writer(file.as_file_mut(), &address)?;
    file.as_file().sync_all()?;
    file.persist_noclobber(&path).map_err(|error| error.error)?;
    let mut site = Site::with_media(
        address.port,
        address.token.clone(),
        Vec::new(),
        Gallery::empty(),
    );
    site.live = Some(LiveView {
        store: service.explorer_store(),
        limit: 500,
        attachment_root: Some(service.attachment_root()?),
    });
    let launch = LiveLaunch { path, address };
    thread::Builder::new()
        .name("mother-explorer".into())
        .spawn(move || {
            if let Err(error) = serve_site(listener, Arc::new(site)) {
                eprintln!("Live Garden listener closed: {error}");
            }
        })?;
    Ok(launch)
}

fn serve_site(listener: TcpListener, site: Arc<Site>) -> Result<()> {
    let busy = Arc::new(AtomicUsize::new(0));
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        if busy.fetch_add(1, Ordering::SeqCst) >= MAX_CONNECTIONS {
            busy.fetch_sub(1, Ordering::SeqCst);
            continue;
        }
        let (site, busy) = (Arc::clone(&site), Arc::clone(&busy));
        thread::spawn(move || {
            serve(&site, stream);
            busy.fetch_sub(1, Ordering::SeqCst);
        });
    }
    Err(Error::domain(
        "STORE_UNAVAILABLE",
        "The garden explorer stopped listening.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> Site {
        Site::new(4100, "launch-token".into(), b"{\"records\":[]}".to_vec())
    }
    fn request(method: &str, path: &str, headers: &[&str]) -> String {
        let mut head = format!("{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:4100\r\n");
        for header in headers {
            head.push_str(header);
            head.push_str("\r\n");
        }
        head + "\r\n"
    }
    const ORIGIN: &str = "Origin: http://127.0.0.1:4100";

    fn session(site: &Site) -> String {
        let reply = site.respond(&request(
            "POST",
            "/api/session",
            &[ORIGIN, "Authorization: Bearer launch-token"],
        ));
        assert_eq!(reply.status, 200);
        let body: serde_json::Value = serde_json::from_slice(&reply.body).unwrap();
        body["session"].as_str().unwrap().to_owned()
    }

    #[test]
    fn owner_attachment_routes_require_a_local_session_and_same_origin_to_write() {
        let temp = tempfile::tempdir().unwrap();
        let store = Arc::new(
            GardenStore::initialize(
                temp.path().join("test.redb"),
                &uuid::Uuid::new_v4().to_string(),
                &uuid::Uuid::new_v4().to_string(),
            )
            .unwrap(),
        );
        let mut site = site();
        site.live = Some(LiveView {
            store: Arc::downgrade(&store),
            limit: 1,
            attachment_root: Some(temp.path().join("synthetic-sessions")),
        });
        let home = site.respond(&request("GET", "/", &[]));
        let html = std::str::from_utf8(&home.body).unwrap();
        assert!(html.contains("Garden atlas"));
        assert!(html.contains(r#"href="/attachments" aria-label="Attachment review""#));
        for asset_path in ["/cherry-bough.glb", "/glasshouse-pool.jpg"] {
            assert_eq!(site.respond(&request("GET", asset_path, &[])).status, 200);
        }
        let path = "/api/attachments";
        assert_eq!(site.respond(&request("GET", path, &[])).status, 401);
        let bearer = format!("Authorization: Bearer {}", session(&site));
        assert_eq!(site.respond(&request("GET", path, &[&bearer])).status, 200);
        let body = json!({"action":"approve","operationId":uuid::Uuid::new_v4().to_string()});
        let kind = "Content-Type: application/json";
        assert_eq!(
            site.respond(&(request("POST", path, &[&bearer, kind]) + &body.to_string()))
                .status,
            403
        );
        assert_eq!(
            site.respond(
                &(request(
                    "POST",
                    path,
                    &[&bearer, "Origin: http://127.0.0.1:4101", kind]
                ) + &body.to_string())
            )
            .status,
            403
        );
        assert_eq!(
            site.respond(&(request("POST", path, &[ORIGIN, kind]) + &body.to_string()))
                .status,
            401
        );
        assert_eq!(
            site.respond(&(request("POST", path, &[&bearer, ORIGIN, kind]) + &body.to_string()))
                .status,
            404
        );
    }

    #[test]
    fn owner_trust_routes_require_a_session_same_origin_and_an_eligible_grant() {
        let temp = tempfile::tempdir().unwrap();
        let store = Arc::new(
            GardenStore::initialize(
                temp.path().join("test.redb"),
                &uuid::Uuid::new_v4().to_string(),
                &uuid::Uuid::new_v4().to_string(),
            )
            .unwrap(),
        );
        store.v1_initialize("Synthetic persona").unwrap();
        for (client, scopes) in [
            ("companion", &["read", "media:attach"][..]),
            ("reader", &["read"][..]),
        ] {
            store
                .v1_grant(
                    client,
                    &scopes.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    &Default::default(),
                    None,
                )
                .unwrap();
        }
        let mut site = site();
        site.live = Some(LiveView {
            store: Arc::downgrade(&store),
            limit: 1,
            attachment_root: Some(temp.path().join("synthetic-sessions")),
        });
        let home = site.respond(&request("GET", "/", &[]));
        assert!(
            std::str::from_utf8(&home.body)
                .unwrap()
                .contains(r#"href="/trust" aria-label="Agent trust""#)
        );
        assert_eq!(site.respond(&request("GET", "/trust", &[])).status, 200);
        assert_eq!(site.respond(&request("GET", "/trust.js", &[])).status, 200);
        let path = "/api/trust";
        assert_eq!(site.respond(&request("GET", path, &[])).status, 401);
        let bearer = format!("Authorization: Bearer {}", session(&site));
        let overview = site.respond(&request("GET", path, &[&bearer]));
        assert_eq!(overview.status, 200);
        let text = std::str::from_utf8(&overview.body).unwrap();
        assert!(text.contains("\"companion\"") && !text.contains("token"));
        let kind = "Content-Type: application/json";
        let set = |client: &str, mode: &str, revision: u64| {
            json!({"action":"set","clientId":client,"mode":mode,"maxAutoCommitsPerDay":5,
                "maxAutoBytesPerDay":1048576,"expectedRevision":revision})
            .to_string()
        };
        let post = |headers: &[&str], body: String| {
            site.respond(&(request("POST", path, headers) + &body))
                .status
        };
        assert_eq!(
            post(&[&bearer, kind], set("companion", "trusted_companion", 0)),
            403
        );
        assert_eq!(
            post(
                &[&bearer, "Origin: http://127.0.0.1:4101", kind],
                set("companion", "trusted_companion", 0)
            ),
            403
        );
        assert_eq!(
            post(&[ORIGIN, kind], set("companion", "trusted_companion", 0)),
            401
        );
        assert_eq!(
            post(
                &[&bearer, ORIGIN, kind],
                set("reader", "trusted_companion", 0)
            ),
            403
        );
        assert_eq!(
            post(&[&bearer, ORIGIN, kind], set("companion", "superuser", 0)),
            400
        );
        assert_eq!(
            post(
                &[&bearer, ORIGIN, kind],
                set("companion", "trusted_companion", 0)
            ),
            200
        );
        assert_eq!(
            post(
                &[&bearer, ORIGIN, kind],
                set("companion", "trusted_companion", 0)
            ),
            409
        );
        assert_eq!(store.trust_policy("companion").unwrap().revision, 1);
        assert_eq!(
            post(
                &[&bearer, ORIGIN, kind],
                set("companion", "manual_review", 1)
            ),
            200
        );
        assert!(!store.trust_policy("companion").unwrap().automatic());
    }

    #[test]
    fn approved_memory_video_is_served_only_in_the_owner_session() {
        let temp = tempfile::tempdir().unwrap();
        let workspace = uuid::Uuid::new_v4().to_string();
        let generation = uuid::Uuid::new_v4().to_string();
        let store = GardenStore::initialize(temp.path().join("test.redb"), &workspace, &generation)
            .unwrap();
        store.v1_initialize("Synthetic persona").unwrap();
        let author = store
            .v1_grant(
                "synthetic",
                &["read", "memory:write"].map(String::from),
                &Default::default(),
                None,
            )
            .unwrap();
        let written = store.v1_call(&author, "mother_write_record", json!({"workspaceId":workspace,"generation":generation,
            "operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Test clip","content":"A synthetic recording.","basis":"owner_statement","status":"active"}})).unwrap();
        let id = written["record"]["id"].as_str().unwrap();
        let clip = include_bytes!("../tests/fixtures/attachment-sample.mp4");
        std::fs::write(temp.path().join("clip.mp4"), clip).unwrap();
        let plan = store
            .plan_attachment(temp.path(), "clip.mp4", id, None, false)
            .unwrap();
        store
            .apply_attachment(temp.path(), &plan, &plan.operation_id)
            .unwrap();
        // FFmpeg writes a tag container even for this synthetic clip. The ordinary
        // session names the affected memory without publishing the original bytes.
        let private = gallery(&store, false).unwrap();
        let listing: serde_json::Value = serde_json::from_slice(&private.listing).unwrap();
        assert_eq!(listing["unavailableMemories"], json!([id]));
        assert!(private.images.is_empty());
        let approval = store
            .plan_attachment(temp.path(), "clip.mp4", id, None, true)
            .unwrap();
        store
            .apply_attachment(temp.path(), &approval, &approval.operation_id)
            .unwrap();
        let site = Site::with_media(
            4100,
            "launch-token".into(),
            vec![],
            gallery(&store, false).unwrap(),
        );
        let media_path = format!("/api/media/{}", plan.sha256);
        assert_eq!(site.respond(&request("GET", &media_path, &[])).status, 401);
        let bearer = format!("Authorization: Bearer {}", session(&site));
        let reply = site.respond(&request("GET", "/api/media", &[&bearer]));
        let listing: serde_json::Value = serde_json::from_slice(&reply.body).unwrap();
        assert_eq!(listing["images"][0]["memoryLinks"][0]["memoryId"], id);
        assert_eq!(
            listing["images"][0]["memoryLinks"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            listing["images"][0]["memoryLinks"][0]["allowOriginal"],
            true
        );
        let reply = site.respond(&request("GET", &media_path, &[&bearer]));
        assert_eq!(reply.kind, "video/mp4");
        assert_eq!(reply.body.as_ref(), clip);
    }

    #[test]
    fn gallery_withholds_originals_with_embedded_metadata_unless_asked() {
        use mother::garden::{
            access::GardenAccess,
            contracts::BatchRequest,
            media_tools::{
                AcquisitionInput, AttachRequest, AttachSource, IngestOptions, MediaOrigin,
            },
        };
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let (ws, generation) = (
            uuid::Uuid::new_v4().to_string(),
            uuid::Uuid::new_v4().to_string(),
        );
        let store = GardenStore::initialize(dir.path().join("g.redb"), &ws, &generation).unwrap();
        let writer = store
            .grant(
                "writer",
                &["garden:read".into(), "garden:write".into()],
                &mother::access::ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        let batch: BatchRequest = serde_json::from_value(json!({"version":2,"workspaceId":ws,"generation":generation,
            "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
            {"itemKey":"p","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:place","displayName":"Synthetic","aliases":[],"extensions":{}}},
            {"itemKey":"m","mutation":"create","expectedRevision":0,"draft":{"kind":"assertion","subject":{"refType":"item","kind":"entity","itemKey":"p"},"predicate":"test:seen","object":{"type":"value","value":{"type":"boolean","value":true}},"validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"evidence":[],"confidence":null,"extensions":{}}}]})).unwrap();
        let memory = store.apply(&writer, &batch).unwrap().receipt.results[1]
            .reference
            .id
            .clone();
        // A JPEG whose EXIF block carries a GPS pointer.
        let mut jpeg = vec![0xFF, 0xD8, 0xFF, 0xE1, 0, 30];
        jpeg.extend_from_slice(&[
            b'E', b'x', b'i', b'f', 0, 0, b'I', b'I', b'*', 0, 8, 0, 0, 0, 1, 0, 0x25, 0x88, 2, 0,
            1, 0, 0, 0, 0, 0, 0, 0,
        ]);
        jpeg.extend_from_slice(&[0xFF, 0xDA, 0, 8, 1, 1, 0, 0, 0x3F, 0, 0x12, 0xFF, 0xD9]);
        let path = dir.path().join("gps.jpg");
        std::fs::write(&path, &jpeg).unwrap();
        store
            .media_attach(&AttachRequest {
                source: AttachSource::File {
                    path: &path,
                    input: AcquisitionInput::new(MediaOrigin::Captured),
                    options: IngestOptions::default(),
                },
                target: memory,
                expected_revision: 1,
                note: None,
            })
            .unwrap();
        let hidden: serde_json::Value =
            serde_json::from_slice(&gallery(&store, false).unwrap().listing).unwrap();
        assert_eq!(
            (
                hidden["images"].as_array().unwrap().len(),
                hidden["withheld"].as_u64()
            ),
            (0, Some(1))
        );
        let shown = gallery(&store, true).unwrap();
        assert_eq!(shown.images.len(), 1);
        assert_eq!(shown.images.values().next().unwrap().1, jpeg);
    }

    #[test]
    fn gallery_images_need_the_single_session_and_stay_local() {
        let gallery = Gallery {
            listing: b"{\"images\":[]}".to_vec(),
            images: BTreeMap::from([("ab".repeat(32), ("image/png", vec![1, 2, 3]))]),
        };
        let site = Site::with_gallery(4100, "launch-token".into(), gallery);
        let page = site.respond(&request("GET", "/", &[]));
        assert_eq!(page.body.as_ref(), GALLERY_HTML);
        assert_eq!(site.respond(&request("GET", "/gallery.js", &[])).kind, JS);
        assert_eq!(
            site.respond(&request("GET", "/assets/index.js", &[]))
                .status,
            404,
            "the 3D explorer is not served in gallery mode"
        );
        let image = format!("/api/media/{}", "ab".repeat(32));
        assert_eq!(site.respond(&request("GET", &image, &[])).status, 401);
        assert_eq!(site.respond(&request("GET", "/api/media", &[])).status, 401);
        let token = session(&site);
        let bearer = format!("Authorization: Bearer {token}");
        let reply = site.respond(&request("GET", &image, &[&bearer]));
        assert_eq!((reply.status, reply.kind), (200, "image/png"));
        assert_eq!(reply.body.as_ref(), [1, 2, 3]);
        assert_eq!(
            site.respond(&request("GET", "/api/media/unknown", &[&bearer]))
                .status,
            404
        );
        assert_eq!(
            site.respond(&request("GET", "/api/media", &[&bearer]))
                .body
                .as_ref(),
            b"{\"images\":[]}"
        );
        assert_eq!(
            site.respond(&request(
                "GET",
                &image,
                &[&bearer, "Origin: https://elsewhere.example"]
            ))
            .status,
            403
        );
    }

    #[test]
    fn the_garden_serves_its_media_under_the_same_single_session() {
        let gallery = Gallery {
            listing: b"{\"images\":[]}".to_vec(),
            images: BTreeMap::from([("cd".repeat(32), ("image/jpeg", vec![7, 8]))]),
        };
        let site = Site::with_media(
            4100,
            "launch-token".into(),
            b"{\"records\":[]}".to_vec(),
            gallery,
        );
        let page = site.respond(&request("GET", "/", &[]));
        assert_eq!(
            page.body.as_ref(),
            asset("/").unwrap().1,
            "the garden, not the gallery page"
        );
        assert_eq!(
            site.respond(&request("GET", "/assets/index.js", &[]))
                .status,
            200
        );
        assert_eq!(
            site.respond(&request("GET", "/gallery.js", &[])).status,
            404
        );
        let image = format!("/api/media/{}", "cd".repeat(32));
        assert_eq!(site.respond(&request("GET", &image, &[])).status, 401);
        assert_eq!(site.respond(&request("GET", "/api/media", &[])).status, 401);
        let token = session(&site);
        let bearer = format!("Authorization: Bearer {token}");
        let reply = site.respond(&request("GET", &image, &[&bearer]));
        assert_eq!(
            (reply.status, reply.kind, reply.body.as_ref()),
            (200, "image/jpeg", [7u8, 8].as_slice())
        );
        assert_eq!(
            site.respond(&request("GET", "/api/garden", &[&bearer]))
                .status,
            200
        );
        assert_eq!(
            site.respond(&request(
                "GET",
                &image,
                &[&bearer, "Origin: https://elsewhere.example"]
            ))
            .status,
            403
        );
        let empty: serde_json::Value = serde_json::from_slice(&Gallery::empty().listing).unwrap();
        assert_eq!(empty["images"], json!([]));
    }

    #[test]
    fn pages_are_served_without_a_token_but_the_snapshot_is_not() {
        let site = site();
        assert_eq!(site.respond(&request("GET", "/", &[])).status, 200);
        assert_eq!(
            site.respond(&request("GET", "/?quality=low", &[])).status,
            200
        );
        assert_eq!(
            site.respond(&request("GET", "/assets/index.js", &[])).kind,
            JS
        );
        assert_eq!(
            site.respond(&request("GET", "/api/garden", &[])).status,
            401
        );
        assert_eq!(
            site.respond(&request(
                "GET",
                "/api/garden",
                &["Authorization: Bearer launch-token"]
            ))
            .status,
            401
        );
        assert_eq!(
            site.respond(&request("GET", "/../Cargo.toml", &[])).status,
            404
        );
    }

    #[test]
    fn the_link_token_works_once_and_only_its_session_reads() {
        let site = site();
        let token = session(&site);
        assert_eq!(token.len(), 64);
        let again = site.respond(&request(
            "POST",
            "/api/session",
            &[ORIGIN, "Authorization: Bearer launch-token"],
        ));
        assert_eq!(again.status, 401);
        let read = site.respond(&request(
            "GET",
            "/api/garden",
            &[&format!("Authorization: Bearer {token}")],
        ));
        assert_eq!((read.status, read.kind), (200, JSON));
        assert_eq!(&*read.body, b"{\"records\":[]}");
        assert_eq!(
            site.respond(&request(
                "GET",
                "/api/garden",
                &["Authorization: Bearer wrong"]
            ))
            .status,
            401
        );
    }

    #[test]
    fn live_snapshots_refresh_and_reopen_under_the_owner() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = uuid::Uuid::new_v4().to_string();
        let generation = uuid::Uuid::new_v4().to_string();
        let store = Arc::new(
            GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation)
                .unwrap(),
        );
        store.v1_initialize("Synthetic persona").unwrap();
        let auth = store
            .v1_grant(
                "synthetic",
                &["read".into(), "memory:write".into()],
                &mother::access::ClientAccess::default(),
                None,
            )
            .unwrap();
        let mut site = site();
        site.live = Some(LiveView {
            store: Arc::downgrade(&store),
            limit: 500,
            attachment_root: None,
        });
        let token = session(&site);
        assert_eq!(session(&site), token);
        let bearer = format!("Authorization: Bearer {token}");
        let read = |path: &str| {
            let reply = site.respond(&request("GET", path, &[&bearer]));
            assert_eq!(reply.status, 200);
            serde_json::from_slice::<serde_json::Value>(&reply.body).unwrap()
        };
        let first = read("/api/garden");
        let poll = format!(
            "/api/garden?since={}",
            first["liveRevision"].as_str().unwrap()
        );
        assert_eq!(read(&poll), json!({"unchanged":true}));
        assert_eq!(first["shown"], 0);
        store.v1_call(&auth, "mother_write_record", json!({
            "workspaceId": workspace,
            "generation": generation,
            "operationId": uuid::Uuid::new_v4().to_string(),
            "expectedRevision": 0,
            "record": {"kind":"memory","title":"Synthetic bloom","content":"One new memory.","status":"active","basis":"owner_statement"}
        })).unwrap();
        let next = read(&poll);
        assert_eq!(next["records"][0]["title"], "Synthetic bloom");
        assert_ne!(next["liveRevision"], first["liveRevision"]);
        drop(store);
        assert_eq!(site.respond(&request("GET", &poll, &[&bearer])).status, 503);
    }

    #[test]
    fn other_sites_other_names_and_writes_are_refused() {
        let site = site();
        let foreign = site.respond(&request(
            "POST",
            "/api/session",
            &[
                "Origin: http://example.test",
                "Authorization: Bearer launch-token",
            ],
        ));
        assert_eq!(foreign.status, 403);
        let no_origin = site.respond(&request(
            "POST",
            "/api/session",
            &["Authorization: Bearer launch-token"],
        ));
        assert_eq!(no_origin.status, 403);
        let rebound =
            site.respond("GET /api/garden HTTP/1.1\r\nHost: garden.example.test:4100\r\n\r\n");
        assert_eq!(rebound.status, 403);
        assert_eq!(site.respond("GET / HTTP/1.1\r\n\r\n").status, 403);
        assert_eq!(
            site.respond(&request("DELETE", "/api/garden", &[])).status,
            405
        );
        assert_eq!(site.respond("nonsense\r\n\r\n").status, 400);
        // Refusals never spend the link.
        session(&site);
    }
}
