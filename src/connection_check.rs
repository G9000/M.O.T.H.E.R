use mother::{
    error::{Error, Result},
    service::{LocalClient, load_service_config},
};
use serde_json::{Value, json};
use std::path::Path;

fn probe(client: &LocalClient, report: &mut Value) -> Result<()> {
    let caps = client.call("mother_get_capabilities", &json!({}))?;
    report["hostActivity"] = if caps["bridgeObservations"] == 1 {
        client.call("local_bridge_activity", &json!({}))?
    } else {
        json!({"available":false})
    };
    let query = json!({"limit":1,"order":"recent","includePersona":false,"includePinned":false,"previewMaxChars":0});
    let result = if caps["v1"]["available"] == true {
        client.call("mother_get_context", &query)
    } else {
        Err(Error::domain("FORBIDDEN", "No context grant"))
    };
    let records = match result {
        Ok(value) => value["records"].as_array().map(Vec::len),
        Err(e) if e.code() == "FORBIDDEN" && caps["garden"]["available"] == true => {
            let args = json!({"version":2,"workspaceId":client.info().workspace_id,"generation":client.info().generation,"requiredExtensions":{},"ids":[],"kinds":[],"limit":1});
            client.call("mother_garden_query",&args)?["hits"].as_array().map(Vec::len)
        },
        Err(e) => return Err(e),
    }.ok_or_else(|| Error::domain("INVALID_INPUT","Unexpected read response"))?;
    report["probe"] = json!(if records == 0 { "empty" } else { "record" });
    let activity = &report["hostActivity"];
    report["nextStep"] = json!(match activity["lastRead"]["outcome"].as_str() {
        Some("record") => "A host-requested read returned a record through the bridge.",
        Some("too_large") => "Ask the host for a smaller page with short previews.",
        Some("failed") => "Retry a bounded read in the host and inspect its tool error.",
        Some("empty") =>
            "The last host query returned no records. Check the search and granted records.",
        _ if activity["available"] == false =>
            "Update the owner and bridge to collect host activity.",
        _ if activity["lastHostRequestAt"].is_string() =>
            "Discovery reached the bridge. Ask the host to read one authorized record.",
        _ => "Open the intended host and request a Garden record using this connection.",
    });
    Ok(())
}

pub(crate) fn run(path: &Path) -> Result<()> {
    let mut report = json!({"checkedAt":chrono::Utc::now(),"stage":"configuration","ownerAuthenticated":false,"probe":null,"hostActivity":null});
    let result: Result<()> = (|| {
        let config = load_service_config(path)?;
        report["client"] = json!(config.client_id);
        report["stage"] = json!("owner_connection");
        let client = LocalClient::new(config)?;
        report["ownerAuthenticated"] = json!(true);
        report["stage"] = json!("authorized_read");
        probe(&client, &mut report)?;
        report["stage"] = json!("complete");
        Ok(())
    })();
    if let Err(ref e) = result {
        report["error"] = json!(e.code());
        report["nextStep"] = json!(match (report["stage"].as_str(), e.code()) {
            (Some("configuration"), _) =>
                "Select the owner-issued service configuration for this host.",
            (_, "MIGRATION_FENCED") =>
                "Inspect the owner-private directory permissions and any active cutover fence.",
            (_, "UNAUTHORIZED" | "FORBIDDEN") =>
                "Check the credential, its read grant and owner-private paths.",
            (_, "GENERATION_MISMATCH" | "WORKSPACE_MISMATCH" | "RESTART_REQUIRED") =>
                "Bind this host to the current workspace and owner endpoint.",
            (Some("owner_connection"), _) =>
                "Check that the owner service is running at the configured endpoint.",
            _ => "Inspect the authorized read error and retry the connection check.",
        });
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    result.map_err(|_| {
        Error::domain(
            "CONNECTION_CHECK_FAILED",
            "See the reported stage and next step.",
        )
    })
}
