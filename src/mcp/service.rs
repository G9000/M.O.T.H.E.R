//! Stdio bridge into the existing local owner; this process never opens its database.
use super::{UNCONFIRMED, garden, input_error};
use mother::{
    error::{Error, Result},
    service::{LocalClient, load_service_config},
};
use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, Implementation, ListToolsResult,
        PaginatedRequestParams, ServerCapabilities, ServerConfig, Tool,
    },
    service::RequestContext,
};
use serde_json::{Value, json};
use std::path::Path;

#[derive(Debug, Clone)]
struct Bridge {
    client: LocalClient,
    tools: Vec<Tool>,
    observes: bool,
    accepts_read_budget: bool,
}
// Only successful record-bearing results count as a read of an actual record.
fn read_event(name: &str, reply: &CallToolResult) -> Option<&'static str> {
    let value = reply.structured_content.as_ref()?;
    let has_record = match name {
        "mother_get_record" => value["id"].is_string(),
        "mother_read_original" if value["source"] == "record" => value["id"].is_string(),
        "mother_garden_get" => value["record"].is_object(),
        "mother_project_resume" => {
            ["confirmedDecision", "checkpoint", "openTask"]
                .iter()
                .any(|k| value[k]["id"].is_string())
                || value["changes"].as_array().is_some_and(|v| !v.is_empty())
        }
        "mother_find_moment"
        | "mother_get_context"
        | "mother_search_context"
        | "mother_garden_query" => ["records", "pinned", "hits"]
            .iter()
            .any(|k| value[k].as_array().is_some_and(|v| !v.is_empty())),
        _ => return None,
    };
    Some(if value["error"]["code"] == "READ_RESULT_TOO_LARGE" {
        "too_large"
    } else if reply.is_error == Some(true) {
        "failed"
    } else if has_record {
        "record"
    } else {
        "empty"
    })
}
impl ServerHandler for Bridge {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(Implementation::new("mother-garden",env!("CARGO_PKG_VERSION"))).with_instructions("M.O.T.H.E.R shared garden through its local owner service. Your harness retains its memory, intelligence, permissions and instruction hierarchy. Read authorized shared context, resolve complete records, and publish only selected attributed content. Pending proposals remain outside canonical recall until local owner approval. MCP cannot grant, approve, migrate, activate or launch. Preserve operation UUIDs after uncertain results and reconcile receipts; this bridge never retries a mutation automatically. Attachment proposals require a dedicated owner grant. Use advertised attachment tools, retain the operation UUID, and require a verified receipt before reporting success. If the host cannot supply original bytes, use the owner file handoff.")
    }
    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<ListToolsResult, ErrorData> {
        let client = self.client.clone();
        let observes = self.observes;
        super::attachment::list_tools(&self.tools, request, move || {
            let caps = client.call("mother_get_capabilities", &json!({}))?;
            if observes {
                let _ = client.call("local_bridge_observe", &json!({"event":"connected"}));
            }
            Ok(caps["mediaAttachmentBridge"]["available"] == true)
        })
        .await
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<CallToolResponse, ErrorData> {
        if !self.tools.iter().any(|tool| tool.name == request.name)
            && !super::attachment::is_tool(&request.name)
        {
            return Err(ErrorData::invalid_params("Unknown tool name.", None));
        }
        let client = self.client.clone();
        let name = request.name.clone();
        let accepts_read_budget = self.accepts_read_budget;
        let result = tokio::task::spawn_blocking(move || {
            let args = Value::Object(request.arguments.unwrap_or_default());
            let budget = super::read_budget(&request.name, &args)?;
            if budget.is_some() && !accepts_read_budget {
                return Err(Error::domain("UNSUPPORTED_VERSION", "This owner service does not support explicit recall budgets. Upgrade the owner and reconnect."));
            }
            let value = if super::attachment::is_tool(&request.name) {
                super::attachment::call(&client, &request.name, args)
            } else {
                client.call(&request.name, &args)
            }?;
            Ok((value, budget))
        })
        .await;
        let reply = match result {
            Ok(Ok((value, budget))) => super::read_output::result_with_budget(&name, value, budget),
            other => {
                let error = match other {
                    Ok(Err(error)) => error,
                    _ => Error::domain("STORE_UNAVAILABLE", UNCONFIRMED),
                };
                let message = if error.code() == "STORE_UNAVAILABLE" {
                    UNCONFIRMED.to_owned()
                } else {
                    error.to_string()
                };
                CallToolResult::structured_error(
                    json!({"error":{"code":error.code(),"message":message,"restartRequired":error.code()=="RESTART_REQUIRED"}}),
                )
            }
        };
        if self.observes
            && let Some(event) = read_event(&name, &reply)
        {
            let client = self.client.clone();
            let _ = tokio::task::spawn_blocking(move || {
                client.call("local_bridge_observe", &json!({"event":event}))
            })
            .await;
        }
        Ok(reply.into())
    }
}
fn remove_read_budget_from_catalog(tools: &mut [Tool]) {
    for tool in tools
        .iter_mut()
        .filter(|t| super::is_budgeted_recall(&t.name))
    {
        let schema = std::sync::Arc::make_mut(&mut tool.input_schema);
        if let Some(fields) = schema.get_mut("properties").and_then(Value::as_object_mut) {
            fields.remove("maxResponseBytes");
        }
    }
}

pub(super) async fn serve(path: &Path) -> Result<()> {
    let path = path.to_owned();
    let bridge = tokio::task::spawn_blocking(move || -> Result<Bridge> {
        let client = LocalClient::new(load_service_config(&path)?)?;
        let available = client.call("mother_get_capabilities", &json!({}))?;
        let enabled = available["v1"]["available"]
            .as_bool()
            .ok_or_else(input_error)?;
        let graph_available = available["garden"]["available"]
            .as_bool()
            .ok_or_else(input_error)?;
        let mut tools = if graph_available {
            let granted: Vec<&str> = available["garden"]["availableTools"]
                .as_array()
                .ok_or_else(input_error)?
                .iter()
                .filter_map(Value::as_str)
                .collect();
            garden::with_media(garden::advertised_tools(enabled)?, &granted)?
        } else if enabled {
            garden::advertised_context_tools()?
        } else {
            return Err(input_error());
        };
        if available["v1"]["readOriginal"] != 1 {
            tools.retain(|t| t.name.as_ref() != "mother_read_original");
        }
        if available["v1"]["momentLookup"] != 1 {
            tools.retain(|t| t.name.as_ref() != "mother_find_moment");
        }
        let accepts_read_budget = available["v1"]["explicitReadBudget"] == 1;
        if !accepts_read_budget {
            remove_read_budget_from_catalog(&mut tools);
        }
        if available["v1"]["projectResume"] != 1 {
            tools.retain(|t| {
                !matches!(
                    t.name.as_ref(),
                    "mother_project_resume" | "mother_mark_project_seen"
                )
            });
        }
        Ok(Bridge {
            client,
            tools,
            observes: available["bridgeObservations"] == 1,
            accepts_read_budget,
        })
    })
    .await
    .map_err(|_| Error::domain("STORE_UNAVAILABLE", UNCONFIRMED))??;
    bridge
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|_| Error::domain("STORE_UNAVAILABLE", "Local service MCP could not start."))?
        .waiting()
        .await
        .map_err(|_| Error::domain("STORE_UNAVAILABLE", "Local service MCP stopped."))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn older_owner_catalog_keeps_cursors_and_omits_unsupported_budget() {
        let mut catalog = garden::advertised_context_tools().unwrap();
        for tool in catalog
            .iter()
            .filter(|t| super::super::is_budgeted_recall(&t.name))
        {
            let field = &tool.input_schema["properties"]["maxResponseBytes"];
            assert_eq!(field["minimum"], 4096);
            assert_eq!(field["maximum"], 12288);
        }
        remove_read_budget_from_catalog(&mut catalog);
        for tool in catalog
            .iter()
            .filter(|t| super::super::is_budgeted_recall(&t.name))
        {
            let fields = &tool.input_schema["properties"];
            assert!(fields.get("maxResponseBytes").is_none());
            assert_eq!(fields["cursor"]["type"], "string");
        }
    }
}
