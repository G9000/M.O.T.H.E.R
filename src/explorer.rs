//! Read-only loopback web view of one owner snapshot of the memory garden.
//! It serves embedded files and one JSON snapshot; it never writes and never calls out.
use mother::{
    error::{Error, Result},
    garden::store::GardenStore,
};
use rand::RngCore;
use serde_json::json;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
    path::Path,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};
use subtle::ConstantTimeEq;

const MAX_HEAD: usize = 8 * 1024;
const MAX_CONNECTIONS: usize = 32;
const IO_TIMEOUT: Duration = Duration::from_secs(5);
const JS: &str = "text/javascript; charset=utf-8";
const JSON: &str = "application/json";
const TEXT: &str = "text/plain; charset=utf-8";
const JPEG: &str = "image/jpeg";
const OGG: &str = "audio/ogg";
/// Everything the page may load comes from this server; nothing may leave it.
const POLICY: &str = "default-src 'none'; script-src 'self'; worker-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; media-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

macro_rules! asset {
    ($name:literal) => {
        include_bytes!(concat!("../assets/garden-explorer/", $name)).as_slice()
    };
}
fn asset(path: &str) -> Option<(&'static str, &'static [u8])> {
    Some(match path {
        "/" => ("text/html; charset=utf-8", asset!("index.html")),
        "/assets/index.js" => (JS, asset!("assets/index.js")),
        "/assets/bake.worker.js" => (JS, asset!("assets/bake.worker.js")),
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

/// One owner snapshot of attached images, loaded before the database is released.
#[derive(Debug, Default)]
struct Gallery {
    listing: Vec<u8>,
    images: BTreeMap<String, (&'static str, Vec<u8>)>,
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

/// One launch: a single-use link token that is traded for the only session.
struct Site {
    authority: [String; 2],
    origin: [String; 2],
    launch: Mutex<Option<String>>,
    session: Mutex<Option<String>>,
    snapshot: Vec<u8>,
    gallery: Option<Gallery>,
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
            gallery: None,
        }
    }
    fn with_gallery(port: u16, launch: String, gallery: Gallery) -> Self {
        let mut site = Self::new(port, launch, gallery.listing.clone());
        site.gallery = Some(gallery);
        site
    }
    fn signed_in(&self, bearer: Option<&str>) -> bool {
        let session = self.session.lock().unwrap_or_else(|e| e.into_inner());
        matches(session.as_deref(), bearer)
    }
    /// Gallery routes; `None` falls through to the shared session and snapshot routes.
    fn gallery_reply(&self, method: &str, path: &str, bearer: Option<&str>) -> Option<Reply> {
        let gallery = self.gallery.as_ref()?;
        let page = |kind, body: &'static [u8]| Reply {
            status: 200,
            kind,
            body: Cow::Borrowed(body),
        };
        Some(match (method, path) {
            ("GET", "/") => page(HTML, GALLERY_HTML),
            ("GET", "/gallery.js") => page(JS, GALLERY_JS),
            ("GET", "/api/media") if self.signed_in(bearer) => Reply {
                status: 200,
                kind: JSON,
                body: Cow::Owned(gallery.listing.clone()),
            },
            ("GET", path) if path.starts_with("/api/media") => {
                if !self.signed_in(bearer) {
                    return Some(Reply::refuse(
                        401,
                        "Open the gallery from the link in your terminal.",
                    ));
                }
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
            ("GET", _) if asset(path).is_some() => Reply::refuse(404, "Not found."),
            _ => return None,
        })
    }

    fn respond(&self, head: &str) -> Reply {
        let mut lines = head.split("\r\n");
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
        let path = path.split_once('?').map_or(path, |(path, _)| path);
        if let Some(reply) = self.gallery_reply(method, path, bearer) {
            return reply;
        }
        match (method, path) {
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
                *launch = None;
                let session = secret();
                let body = Cow::Owned(json!({ "session": session }).to_string().into_bytes());
                *self.session.lock().unwrap_or_else(|e| e.into_inner()) = Some(session);
                Reply {
                    status: 200,
                    kind: JSON,
                    body,
                }
            }
            ("GET", "/api/garden") => {
                let session = self.session.lock().unwrap_or_else(|e| e.into_inner());
                if !matches(session.as_deref(), bearer) {
                    return Reply::refuse(401, "Open the garden from the link in your terminal.");
                }
                Reply {
                    status: 200,
                    kind: JSON,
                    body: Cow::Owned(self.snapshot.clone()),
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
    while !head.windows(4).any(|w| w == b"\r\n\r\n") {
        let read = stream.read(&mut chunk).ok()?;
        if read == 0 || head.len() + read > MAX_HEAD {
            return None;
        }
        head.extend_from_slice(&chunk[..read]);
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
    let list = store
        .media_owner_list(None)
        .map_err(mother::garden::config::domain_error)?;
    let mut gallery = Gallery::default();
    let (mut shown, mut withheld, mut total, mut truncated) = (Vec::new(), 0u64, 0u64, false);
    for entry in list["assets"].as_array().into_iter().flatten() {
        let asset = &entry["asset"];
        let uses = entry["uses"].as_array().map_or(0, Vec::len);
        if uses == 0 {
            continue;
        }
        let clean = asset["embeddedMetadata"]
            .as_array()
            .is_some_and(Vec::is_empty);
        let (variant, sha) = match entry["derivatives"][0].as_str() {
            Some(derivative) => ("derivative", derivative),
            None if clean || originals => {
                ("original", asset["sha256"].as_str().unwrap_or_default())
            }
            None => {
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
        let bytes = store
            .media_owner_bytes(sha)
            .map_err(mother::garden::config::domain_error)?;
        total += bytes.len() as u64;
        if total > GALLERY_BYTES {
            truncated = true;
            break;
        }
        gallery.images.insert(sha.to_owned(), (kind, bytes));
        shown.push(
            json!({"sha256":sha,"originalSha256":asset["sha256"],"variant":variant,
            "mediaType":kind,"width":asset["width"],"height":asset["height"],
            "acquisitions":entry["acquisitions"],"uses":uses}),
        );
    }
    gallery.listing = serde_json::to_vec(
        &json!({"images":shown,"withheld":withheld,"truncated":truncated,"readOnly":true}),
    )?;
    Ok(gallery)
}

/// Read one snapshot, release the database, then serve it on loopback until interrupted.
pub(crate) fn run(db: &Path, limit: usize, open: bool, media: Option<bool>) -> Result<()> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let port = listener.local_addr()?.port();
    let launch = secret();
    let url = format!("http://127.0.0.1:{port}/#{launch}");
    let site = if let Some(originals) = media {
        let gallery = gallery(&GardenStore::open_active(db)?, originals)?;
        let listing: serde_json::Value = serde_json::from_slice(&gallery.listing)?;
        println!(
            "{}",
            json!({"url": url, "images": listing["images"].as_array().map_or(0, Vec::len),
                "withheld": listing["withheld"], "readOnly": true})
        );
        Arc::new(Site::with_gallery(port, launch, gallery))
    } else {
        let view = GardenStore::open_active(db)?.v1_garden_view(limit)?;
        println!(
            "{}",
            json!({"url": url, "shown": view["shown"], "total": view["total"], "readOnly": true})
        );
        Arc::new(Site::new(port, launch, serde_json::to_vec(&view)?))
    };
    std::io::stdout().flush()?;
    eprintln!(
        "The link works once. The garden is a snapshot taken just now; press Ctrl+C to close it."
    );
    if open {
        open_browser(&url);
    }
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
    fn gallery_withholds_originals_with_embedded_metadata_unless_asked() {
        use mother::garden::{
            access::GardenAccess,
            contracts::BatchRequest,
            media_tools::{
                AcquisitionInput, AttachRequest, AttachSource, IngestOptions, MediaOrigin,
            },
        };
        let dir = tempfile::tempdir().unwrap();
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
