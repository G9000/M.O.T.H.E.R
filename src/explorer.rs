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
        }
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

/// Read one snapshot, release the database, then serve it on loopback until interrupted.
pub(crate) fn run(db: &Path, limit: usize, open: bool) -> Result<()> {
    let view = GardenStore::open_active(db)?.v1_garden_view(limit)?;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let port = listener.local_addr()?.port();
    let launch = secret();
    let url = format!("http://127.0.0.1:{port}/#{launch}");
    let site = Arc::new(Site::new(port, launch, serde_json::to_vec(&view)?));
    println!(
        "{}",
        json!({"url": url, "shown": view["shown"], "total": view["total"], "readOnly": true})
    );
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
