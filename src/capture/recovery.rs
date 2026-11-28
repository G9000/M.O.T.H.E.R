//! Owner-local capture recovery. Journals retain the original request. Status is
//! established by authenticated reads and never inferred from a lost response.
use super::{
    Inbox, Outcome, Pending, capture_error, json, load, publish, read_json, rejected_admission,
    single_link,
};
use crate::error::{Error, Result};
use serde_json::Value;
use std::{fs, io};
use uuid::Uuid;

fn uncertain(error: &Error) -> bool {
    matches!(
        error.code(),
        "CAPTURE_UNCONFIRMED" | "STORE_UNAVAILABLE" | "TIMEOUT"
    ) || matches!(error, Error::Json(_))
}

impl<F> Inbox<'_, F>
where
    F: Fn(&str, &Value) -> Result<Value>,
{
    pub(super) fn rejection(&self, operation: &Value) -> Result<Option<String>> {
        let id = operation
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or_else(|| capture_error("Invalid capture operation."))?;
        let path = self.state.join(format!("{id}.rejection"));
        single_link(&path)?;
        match read_json(&path, 4096) {
            Ok(failure) => failure["code"]
                .as_str()
                .map(|s| Some(s.to_owned()))
                .ok_or_else(|| capture_error("Invalid capture rejection journal.")),
            Err(Error::Io(e)) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
    pub(super) fn remember_rejection(&self, pending: &Pending, code: &str) -> Result<()> {
        let id = pending.request["operationId"]
            .as_str()
            .and_then(|id| Uuid::parse_str(id).ok())
            .ok_or_else(|| capture_error("Invalid original capture operation."))?;
        publish(
            &self.state.join(format!("{id}.rejection")),
            &json!({"code":code}),
        )
    }

    fn status(&self, id: Uuid, pending: &Pending) -> Result<Value> {
        let op = json!(id.to_string());
        let mut result = json!({"operationId":op});
        let receipt = match (self.call)("mother_get_receipt", &json!({"operationId":op})) {
            Ok(receipt) => receipt,
            Err(e) if e.code() == "NOT_FOUND" => {
                match self.rejection(&op)? {
                    Some(code) => {
                        result["status"] = json!("failed");
                        result["errorCode"] = json!(code);
                        result["nextAction"] = json!(
                            "Review the rejected draft and current target before making a new capture."
                        );
                    }
                    None => {
                        result["status"] = json!("queued");
                        result["nextAction"] = json!(format!(
                            "Run mother capture --policy <policy> retry --operation {id} with the original policy."
                        ));
                    }
                }
                return Ok(result);
            }
            Err(e) => return Err(e),
        };
        if let Err(error) = self.validate_receipt(&receipt, &op, Some(pending)) {
            if error.code() != "OPERATION_REUSED" {
                return Err(error);
            }
            result["status"] = json!("failed");
            result["errorCode"] = json!(error.code());
            result["nextAction"] = json!(
                "This operation already identifies another request. Inspect the original selection and assign a fresh ID only to a different capture."
            );
            return Ok(result);
        }
        let proposal_id = &receipt["proposal"]["id"];
        result["proposalId"] = proposal_id.clone();
        let proposal = match (self.call)("mother_get_proposal", &json!({"id":proposal_id})) {
            Ok(proposal) => proposal,
            Err(error) if matches!(error.code(), "FORBIDDEN" | "NOT_FOUND") => {
                result["status"] = json!("unconfirmed");
                result["errorCode"] = json!(error.code());
                result["nextAction"] = json!(
                    "The submission was received. Ask the owner to check the proposal and current record access."
                );
                return Ok(result);
            }
            Err(error) => return Err(error),
        };
        if proposal["id"] != *proposal_id
            || proposal["generation"] != self.workspace["generation"]
            || proposal["workspaceId"] != self.workspace["id"]
            || proposal["actor"] != self.client_id
        {
            return Err(capture_error(
                "Current proposal does not match this capture's original authority.",
            ));
        }
        match proposal["status"].as_str() {
            Some("pending") => {
                result["status"] = json!("awaiting_review");
                result["nextAction"] =
                    json!("Open the owner review inbox to save or dismiss the proposal.");
            }
            Some("saved" | "dismissed") => {
                let review = &proposal["receipt"];
                if review["proposalId"] != *proposal_id
                    || review["generation"] != self.workspace["generation"]
                    || review["status"] != proposal["status"]
                    || !review["operationId"].is_string()
                {
                    return Err(capture_error("The owner review receipt is incomplete."));
                }
                result["reviewOperationId"] = review["operationId"].clone();
                result["status"] = proposal["status"].clone();
                if proposal["status"] == "saved" {
                    let saved = &review["receipt"];
                    if saved["operationId"] != review["operationId"]
                        || saved["generation"] != review["generation"]
                        || !saved["record"]["id"].is_string()
                        || !saved["record"]["revision"].is_u64()
                    {
                        return Err(capture_error("The canonical save receipt is incomplete."));
                    }
                    result["recordId"] = saved["record"]["id"].clone();
                    result["recordRevision"] = saved["record"]["revision"].clone();
                    result["nextAction"] = json!("Open the saved record by its ID.");
                } else {
                    result["nextAction"] =
                        json!("Review the owner's dismissal. This capture is closed.");
                }
            }
            _ => return Err(capture_error("Unknown owner review status.")),
        }
        Ok(result)
    }

    fn recover_one(
        &self,
        id: Uuid,
        pending: &Pending,
        retry: bool,
        enabled: &impl Fn() -> Result<bool>,
    ) -> Result<Value> {
        self.validate_pending(pending, &json!(id.to_string()))?;
        let result = self.status(id, pending)?;
        if !retry || result["status"] != "queued" {
            return Ok(result);
        }
        match self.send_pending(pending, &json!(id.to_string()), enabled) {
            Ok(Outcome::Paused) => {
                return Err(capture_error(
                    "Capture policy changed before retry. Reload the policy.",
                ));
            }
            Err(e) if !rejected_admission(&e) && e.code() != "CAPTURE_REJECTED" => {
                return Err(e);
            }
            _ => {}
        }
        self.status(id, pending)
    }

    pub(in crate::capture) fn recover(
        &self,
        operation: Option<Uuid>,
        retry: bool,
        enabled: impl Fn() -> Result<bool>,
    ) -> Result<Value> {
        let ids = if let Some(id) = operation {
            vec![id]
        } else {
            let mut ids = Vec::new();
            for entry in fs::read_dir(self.state)? {
                let name = entry?.file_name();
                if let Some(name) = name.to_str().and_then(|n| n.strip_suffix(".json"))
                    && let Ok(id) = Uuid::parse_str(name)
                    && id.to_string() == name
                {
                    ids.push(id);
                }
            }
            ids.sort_unstable();
            ids
        };
        let mut results = Vec::new();
        for id in ids {
            let pending = load(&self.state.join(format!("{id}.json")))?.ok_or_else(|| {
                Error::domain("NOT_FOUND", "No capture journal exists for this operation.")
            })?;
            if operation.is_none()
                && (pending.client != self.client_id
                    || pending.request["workspaceId"] != self.workspace["id"]
                    || pending.request["generation"] != self.workspace["generation"])
            {
                continue;
            }
            let observation = match self.recover_one(id, &pending, retry, &enabled) {
                Ok(value) => value,
                Err(e) if uncertain(&e) => {
                    json!({"operationId":id.to_string(),"status":"unconfirmed","errorCode":e.code(),"nextAction":"Reconnect the original service and check status using the same operation ID."})
                }
                Err(e) => return Err(e),
            };
            results.push(observation);
        }
        Ok(json!({"operations":results}))
    }
}
