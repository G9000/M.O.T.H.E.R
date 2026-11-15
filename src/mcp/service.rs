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
}
impl ServerHandler for Bridge {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(Implementation::new("mother-garden",env!("CARGO_PKG_VERSION"))).with_instructions("M.O.T.H.E.R shared garden through its local owner service. Your harness retains its memory, intelligence, permissions and instruction hierarchy. Read authorized shared context, resolve complete records, and publish only selected attributed content. Pending proposals remain outside canonical recall until local owner approval. MCP cannot grant, approve, migrate, activate or launch. Preserve operation UUIDs after uncertain results and reconcile receipts; this bridge never retries a mutation automatically.")
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(self.tools.clone()))
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<CallToolResponse, ErrorData> {
        if !self.tools.iter().any(|tool| tool.name == request.name) {
            return Err(ErrorData::invalid_params("Unknown tool name.", None));
        }
        let client = self.client.clone();
        let result = tokio::task::spawn_blocking(move || {
            client.call(
                &request.name,
                &Value::Object(request.arguments.unwrap_or_default()),
            )
        })
        .await;
        let reply = match result {
            Ok(Ok(value)) => CallToolResult::structured(value),
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
        Ok(reply.into())
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
        Ok(Bridge {
            client,
            tools: garden::advertised_tools(enabled)?,
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
