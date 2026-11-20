//! Stable submission identity and receipt reconciliation for the existing v1 inbox.
use super::{
    capture_error,
    source::{Candidate, Kind},
};
use crate::{
    config::read_json,
    error::{Error, Result},
    private_paths::single_link,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{io, path::Path};

pub(super) enum Outcome {
    Submitted,
    AlreadyReceived,
    NoUniqueCurrentMatch,
    Paused,
}

pub(super) struct Inbox<'a, F> {
    call: &'a F,
    workspace: Value,
    client_id: &'a str,
    state: &'a Path,
}

#[derive(Deserialize, Serialize)]
struct Pending {
    client: String,
    fingerprint: String,
    request: Value,
}

impl<'a, F> Inbox<'a, F>
where
    F: Fn(&str, &Value) -> Result<Value>,
{
    pub(super) fn new(
        call: &'a F,
        state: &'a Path,
        client_id: &'a str,
        workspace_id: &str,
        generation: &str,
    ) -> Self {
        Self {
            call,
            workspace: json!({"id":workspace_id,"generation":generation}),
            client_id,
            state,
        }
    }

    pub(super) fn submit(
        &self,
        candidate: &Candidate,
        still_enabled: impl FnOnce() -> Result<bool>,
    ) -> Result<Outcome> {
        let operation_id = json!(candidate.operation().to_string());
        if self.received(&operation_id)? {
            return Ok(Outcome::AlreadyReceived);
        }

        let path = self.state.join(format!("{}.json", candidate.operation()));
        let fingerprint = candidate_fingerprint(candidate);
        let pending = if let Some(pending) = load(&path)? {
            pending
        } else {
            let Some(request) = self.prepare(candidate, &operation_id)? else {
                return Ok(Outcome::NoUniqueCurrentMatch);
            };
            let pending = Pending {
                client: self.client_id.to_owned(),
                fingerprint: fingerprint.clone(),
                request,
            };
            persist(&path, &pending)?
        };
        if pending.client != self.client_id
            || pending.fingerprint != fingerprint
            || pending.request["workspaceId"] != self.workspace["id"]
            || pending.request["generation"] != self.workspace["generation"]
            || pending.request["operationId"] != operation_id
        {
            return Err(capture_error(
                "Prior capture state differs from the current source or authority. Do not rerun it under another identity.",
            ));
        }

        // Another invocation can finish while we resolve the target or persist the draft.
        if self.received(&operation_id)? {
            return Ok(Outcome::AlreadyReceived);
        }
        if !still_enabled()? {
            return Ok(Outcome::Paused);
        }
        match (self.call)("mother_propose_memory", &pending.request) {
            Ok(receipt) => {
                self.validate_receipt(&receipt, &operation_id)?;
                Ok(Outcome::Submitted)
            }
            Err(error) if error.code() == "CONFLICT" => Ok(Outcome::NoUniqueCurrentMatch),
            Err(error)
                if error.code() == "STORE_UNAVAILABLE" || matches!(error, Error::Json(_)) =>
            {
                match self.received(&operation_id) {
                    Ok(true) => Ok(Outcome::AlreadyReceived),
                    _ => Err(Error::domain(
                        "CAPTURE_UNCONFIRMED",
                        "The original capture receipt is unconfirmed. Resume the same source and policy to reconcile before retrying.",
                    )),
                }
            }
            Err(error) => Err(error),
        }
    }

    fn received(&self, operation: &Value) -> Result<bool> {
        match (self.call)("mother_get_receipt", &json!({"operationId":operation})) {
            Ok(receipt) => {
                self.validate_receipt(&receipt, operation)?;
                Ok(true)
            }
            Err(error) if error.code() == "NOT_FOUND" => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn validate_receipt(&self, receipt: &Value, operation: &Value) -> Result<()> {
        if receipt["operationId"] != operation.clone()
            || receipt["generation"] != self.workspace["generation"]
            || !receipt["proposal"]["id"].is_string()
        {
            return Err(capture_error(
                "The original operation receipt is not a capture proposal.",
            ));
        }
        Ok(())
    }

    fn prepare(&self, candidate: &Candidate, operation_id: &Value) -> Result<Option<Value>> {
        let source_ref = format!(
            "claude-code://session/{}/turn/{}",
            candidate.session, candidate.turn
        );
        let (kind, status, basis, speaker, uncertainty) = match candidate.kind {
            Kind::Decision | Kind::Correction => (
                "decision",
                "confirmed",
                "owner_statement",
                "Owner (user)",
                "Owner explicitly confirmed the decision in this source turn.",
            ),
            Kind::Milestone => (
                "memory",
                "active",
                "owner_statement",
                "Owner (user)",
                "Owner-reported milestone, independently unverified.",
            ),
            Kind::Observation => (
                "memory",
                "active",
                "model_inference",
                "Claude Code assistant",
                "Agent observation, unverified interpretation.",
            ),
        };
        let source = format!(
            "{source_ref} | {speaker} | {} | {uncertainty}",
            candidate.timestamp.to_rfc3339()
        );
        // The v1 structured source contract is HTTPS-only. Keep this local reference in
        // human-readable attribution so it remains visible after approval.
        let content = format!("{}\n\nClaude Code source: {source}", candidate.summary);
        let mut draft = json!({
            "kind":kind, "status":status, "title":candidate.title, "content":content
        });
        if kind == "memory" {
            draft["basis"] = json!(basis);
        }
        let mut request = json!({
            "operationId": operation_id,
            "workspaceId": self.workspace["id"],
            "generation": self.workspace["generation"],
            "expectedRevision": 0,
            "source":source,
            "basis":basis
        });
        let existing = (self.call)(
            "mother_search_context",
            &json!({
                "query":candidate.title, "kind":kind, "mode":"literal", "limit":100
            }),
        )?;
        let matches: Vec<_> = existing["records"]
            .as_array()
            .ok_or_else(|| capture_error("The review inbox search response is invalid."))?
            .iter()
            .filter(|record| {
                record["title"]
                    .as_str()
                    .is_some_and(|title| title.eq_ignore_ascii_case(&candidate.title))
            })
            .collect();
        if candidate.kind == Kind::Correction {
            let [preview] = matches.as_slice() else {
                return Ok(None);
            };
            // Truncated result counts could hide another exact-title match.
            if existing["omitted"].as_u64().unwrap_or(0) != 0 {
                return Ok(None);
            }
            let current = (self.call)("mother_get_record", &json!({"id":preview["id"]}))?;
            if current["kind"] != "decision"
                || current["status"] != "confirmed"
                || current["revision"] != preview["revision"]
                || !current["title"]
                    .as_str()
                    .is_some_and(|title| title.eq_ignore_ascii_case(&candidate.title))
            {
                return Ok(None);
            }
            request["id"] = current["id"].clone();
            request["expectedRevision"] = current["revision"].clone();
            draft["title"] = current["title"].clone();
            draft["references"] = current["references"].clone();
            draft["sources"] = current["sources"].clone();
        } else if !matches.is_empty() {
            // The owner can make a correction explicit. Never silently overwrite an existing record.
            return Ok(None);
        }
        request["record"] = draft;
        Ok(Some(request))
    }
}

fn candidate_fingerprint(candidate: &Candidate) -> String {
    let content = json!([
        format!("{:?}", candidate.kind),
        candidate.title,
        candidate.summary,
        candidate.timestamp
    ]);
    format!("{:x}", Sha256::digest(content.to_string().as_bytes()))
}

fn load(path: &Path) -> Result<Option<Pending>> {
    single_link(path)?;
    match read_json(path, 1_000_000) {
        Ok(value) => Ok(Some(serde_json::from_value(value)?)),
        Err(Error::Io(error)) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn persist(path: &Path, pending: &Pending) -> Result<Pending> {
    let parent = path
        .parent()
        .ok_or_else(|| capture_error("The capture state directory is missing."))?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    serde_json::to_writer(temp.as_file_mut(), pending)?;
    temp.as_file().sync_all()?;
    match temp.persist_noclobber(path) {
        Ok(_file) => {
            #[cfg(unix)]
            std::fs::File::open(parent)?.sync_all()?;
            load(path)?.ok_or_else(|| capture_error("The original capture state is unavailable."))
        }
        Err(error) if error.error.kind() == io::ErrorKind::AlreadyExists => {
            load(path)?.ok_or_else(|| capture_error("The original capture state is unavailable."))
        }
        Err(error) => Err(error.error.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{access::ClientAccess, garden::store::GardenStore};
    use std::{cell::RefCell, fs};
    use uuid::Uuid;

    #[test]
    fn interrupted_submission_resumes_original_request_with_receipt_first() {
        let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        let ws = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store =
            GardenStore::initialize(root.path().join("capture.redb"), &ws, &generation).unwrap();
        store.v1_initialize("Synthetic owner").unwrap();
        let auth = store
            .v1_grant(
                "capture-synthetic",
                &["read".into(), "context:propose".into()],
                &ClientAccess::default(),
                None,
            )
            .unwrap();

        // Drop the reply after one committed write. Drop the next write before it reaches
        // storage. In both cases the original invocation also loses the receipt response.
        for committed in [true, false] {
            let candidate = Candidate {
                session: Uuid::new_v4(),
                turn: Uuid::new_v4(),
                timestamp: "2026-09-04T20:00:00Z".parse().unwrap(),
                kind: Kind::Milestone,
                title: if committed {
                    "First milestone"
                } else {
                    "Second milestone"
                }
                .into(),
                summary: "The synthetic check passed.".into(),
            };
            let calls = RefCell::new(Vec::new());
            let unavailable = || Error::domain("STORE_UNAVAILABLE", "Synthetic disconnection");
            let first = |method: &str, args: &Value| {
                let sent = calls
                    .borrow()
                    .iter()
                    .any(|(name, _): &(String, Value)| name == "mother_propose_memory");
                calls.borrow_mut().push((method.into(), args.clone()));
                if method == "mother_propose_memory" {
                    if committed {
                        store.v1_call(&auth, method, args.clone())?;
                    }
                    return Err(unavailable());
                }
                if method == "mother_get_receipt" && sent {
                    return Err(unavailable());
                }
                store.v1_call(&auth, method, args.clone())
            };
            let sink = Inbox::new(&first, root.path(), &auth.client_id, &ws, &generation);
            assert_eq!(
                sink.submit(&candidate, || Ok(true)).err().unwrap().code(),
                "CAPTURE_UNCONFIRMED"
            );
            let written = calls
                .borrow()
                .iter()
                .filter(|(name, _)| name == "mother_propose_memory")
                .map(|(_, args)| args.clone())
                .collect::<Vec<_>>();
            assert_eq!(written.len(), 1);
            assert_eq!(calls.borrow().last().unwrap().0, "mother_get_receipt");

            let resumed = RefCell::new(Vec::new());
            let second = |method: &str, args: &Value| {
                resumed.borrow_mut().push((method.to_owned(), args.clone()));
                store.v1_call(&auth, method, args.clone())
            };
            let sink = Inbox::new(&second, root.path(), &auth.client_id, &ws, &generation);
            let outcome = sink.submit(&candidate, || Ok(true)).unwrap();
            let resumed = resumed.borrow();
            assert_eq!(resumed[0].0, "mother_get_receipt");
            if committed {
                assert!(matches!(outcome, Outcome::AlreadyReceived));
                assert_eq!(resumed.len(), 1);
            } else {
                assert!(matches!(outcome, Outcome::Submitted));
                assert_eq!(resumed.last().unwrap().0, "mother_propose_memory");
                assert_eq!(resumed.last().unwrap().1, written[0]);
            }
        }
        assert_eq!(
            store.v1_inbox_list().unwrap()["proposals"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }
}
