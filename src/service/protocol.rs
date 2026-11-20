//! Bounded length-delimited authenticated local requests with fixed I/O deadlines.
use crate::{
    contracts::Auth,
    error::{Error, Result},
};
use interprocess::local_socket::Stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    io::{self, Read, Write},
    thread,
    time::{Duration, Instant},
};

pub(super) const MAX_FRAME: usize = 4 * 1024 * 1024;
pub(super) const IO_TIMEOUT: Duration = Duration::from_millis(500);
pub(super) const POLL_INTERVAL: Duration = Duration::from_millis(2);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Hello {
    pub version: u32,
    pub client_id: String,
    pub challenge: String,
    pub service_id: String,
    pub workspace_id: String,
    pub generation: String,
    pub address: String,
}
impl Hello {
    pub fn context(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum OwnerProof {
    Proof { version: u32, mac: [u8; 32] },
    Unauthorized,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Request {
    pub version: u32,
    pub service_id: String,
    pub workspace_id: String,
    pub generation: String,
    pub auth: Auth,
    pub method: String,
    pub arguments: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Response {
    Ok { value: Value },
    Error { code: String, message: String },
}
impl Response {
    pub fn from_result(result: Result<Value>) -> Self {
        match result {
            Ok(value) => Self::Ok { value },
            Err(error) => Self::Error {
                code: error.code().to_owned(),
                message: if error.code() == "STORE_UNAVAILABLE" {
                    "Service outcome is unconfirmed. Reconcile the original operation UUID.".into()
                } else if let Error::Domain { message, .. } = error {
                    message
                } else {
                    error.to_string()
                },
            },
        }
    }
    pub fn into_result(self) -> Result<Value> {
        match self {
            Self::Ok { value } => Ok(value),
            Self::Error { code, message } => {
                let code = match code.as_str() {
                    "INVALID_INPUT" => "INVALID_INPUT",
                    "UNSUPPORTED_VERSION" => "UNSUPPORTED_VERSION",
                    "UNSUPPORTED_EXTENSION" => "UNSUPPORTED_EXTENSION",
                    "LIMIT_EXCEEDED" => "LIMIT_EXCEEDED",
                    "INVALID_ENDPOINT" => "INVALID_ENDPOINT",
                    "UNAUTHORIZED" => "UNAUTHORIZED",
                    "FORBIDDEN" => "FORBIDDEN",
                    "WORKSPACE_MISMATCH" => "WORKSPACE_MISMATCH",
                    "NOT_FOUND" => "NOT_FOUND",
                    "CONFLICT" => "CONFLICT",
                    "OPERATION_REUSED" => "OPERATION_REUSED",
                    "GENERATION_MISMATCH" => "GENERATION_MISMATCH",
                    "RESTART_REQUIRED" => "RESTART_REQUIRED",
                    "RECALL_PROFILE_UNAVAILABLE" => "RECALL_PROFILE_UNAVAILABLE",
                    "STALE_GENERATION" => "STALE_GENERATION",
                    "RESOURCE_LIMIT" => "RESOURCE_LIMIT",
                    "REFERENCE_NOT_FOUND" => "REFERENCE_NOT_FOUND",
                    "KIND_MISMATCH" => "KIND_MISMATCH",
                    "DUPLICATE_RECORD" => "DUPLICATE_RECORD",
                    "COMPAT_NOT_INITIALIZED" => "COMPAT_NOT_INITIALIZED",
                    "MIGRATION_FENCED" => "MIGRATION_FENCED",
                    "MODEL_UNAVAILABLE" => "MODEL_UNAVAILABLE",
                    "MODEL_BUSY" => "MODEL_BUSY",
                    "SEARCH_CACHE" => "SEARCH_CACHE",
                    "FULL_BACKUP_REQUIRED" => "FULL_BACKUP_REQUIRED",
                    _ => "STORE_UNAVAILABLE",
                };
                Err(Error::domain(code, message))
            }
        }
    }
}

/// Named pipes otherwise retain unread writes in an unbounded background flush pool.
/// All frame consumption is acknowledged before the server closes a successful response.
#[derive(Debug)]
pub(super) struct Connection(pub Stream);
impl Drop for Connection {
    fn drop(&mut self) {
        #[cfg(windows)]
        {
            let Stream::NamedPipe(stream) = &self.0;
            stream.inner().assume_flushed();
        }
    }
}

fn io_error() -> Error {
    Error::domain(
        "STORE_UNAVAILABLE",
        "Local service outcome is unconfirmed. Reconcile the original operation UUID.",
    )
}

fn read_exact_until(stream: &mut Stream, mut bytes: &mut [u8], deadline: Instant) -> Result<()> {
    while !bytes.is_empty() {
        if Instant::now() >= deadline {
            return Err(Error::domain(
                "STORE_UNAVAILABLE",
                format!(
                    "Local read deadline expired with {} bytes remaining.",
                    bytes.len()
                ),
            ));
        }
        match stream.read(bytes) {
            // PIPE_NOWAIT's ERROR_NO_DATA can become BrokenPipe, which interprocess
            // downgrades to an empty read. Both idle and closed pipes stay deadline-bound.
            #[cfg(windows)]
            Ok(0) => thread::sleep(POLL_INTERVAL),
            #[cfg(not(windows))]
            Ok(0) => return Err(io_error()),
            Ok(count) => {
                bytes = &mut bytes[count..];
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(POLL_INTERVAL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => {
                return Err(Error::domain(
                    "STORE_UNAVAILABLE",
                    format!("Local frame read failed: {error}"),
                ));
            }
        }
    }
    Ok(())
}

fn write_all_until(stream: &mut Stream, mut bytes: &[u8], deadline: Instant) -> Result<()> {
    while !bytes.is_empty() {
        if Instant::now() >= deadline {
            return Err(io_error());
        }
        // PIPE_NOWAIT can report a zero-length send when the entire requested write
        // does not fit. The portable listener's Windows pipe buffer hint is 512 bytes.
        #[cfg(windows)]
        let chunk = &bytes[..bytes.len().min(512)];
        #[cfg(not(windows))]
        let chunk = bytes;
        match stream.write(chunk) {
            Ok(0) => thread::sleep(POLL_INTERVAL),
            Ok(count) => {
                bytes = &bytes[count..];
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(POLL_INTERVAL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => {
                return Err(Error::domain(
                    "STORE_UNAVAILABLE",
                    format!("Local frame write failed: {error}"),
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn read_frame<T: for<'de> Deserialize<'de>>(stream: &mut Stream) -> Result<T> {
    read_frame_with_timeout(stream, IO_TIMEOUT)
}

pub(super) fn read_frame_with_timeout<T: for<'de> Deserialize<'de>>(
    stream: &mut Stream,
    timeout: Duration,
) -> Result<T> {
    let deadline = Instant::now() + timeout;
    let mut length = [0; 4];
    read_exact_until(stream, &mut length, deadline)
        .map_err(|error| Error::domain("STORE_UNAVAILABLE", format!("Frame header: {error}")))?;
    let length = u32::from_be_bytes(length) as usize;
    if length == 0 || length > MAX_FRAME {
        return Err(Error::domain(
            "LIMIT_EXCEEDED",
            "Local frame exceeds the supported size limit.",
        ));
    }
    let mut bytes = vec![0; length];
    read_exact_until(stream, &mut bytes, deadline)
        .map_err(|error| Error::domain("STORE_UNAVAILABLE", format!("Frame body: {error}")))?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub(super) fn write_frame<T: Serialize>(stream: &mut Stream, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    if bytes.len() > MAX_FRAME {
        return Err(Error::domain(
            "LIMIT_EXCEEDED",
            "Local frame exceeds the supported size limit.",
        ));
    }
    let deadline = Instant::now() + IO_TIMEOUT;
    let length = u32::try_from(bytes.len()).map_err(|_| io_error())?;
    write_all_until(stream, &length.to_be_bytes(), deadline)?;
    write_all_until(stream, &bytes, deadline)
}

pub(super) fn acknowledge(stream: &mut Stream) -> Result<()> {
    write_all_until(stream, &[1], Instant::now() + IO_TIMEOUT)
}
pub(super) fn read_acknowledgement(stream: &mut Stream) -> Result<()> {
    let mut ack = [0];
    read_exact_until(stream, &mut ack, Instant::now() + IO_TIMEOUT)?;
    if ack != [1] {
        return Err(io_error());
    }
    Ok(())
}
