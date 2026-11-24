//! Attachment tool discovery is driven by the credential's current grant.
use mother::{
    error::Result,
    service::{attachment::AttachmentClient, host_file},
};
use serde_json::Value;

pub(super) fn call(client: &dyn AttachmentClient, name: &str, args: Value) -> Result<Value> {
    if name == host_file::IMPORT {
        return host_file::import(client, &args);
    }
    client.request(name, &args)
}
/// The five owner-side protocol tools plus the bridge-side host file adapter.
pub(super) fn is_tool(name: &str) -> bool {
    mother::service::attachment::is_tool(name) || name == host_file::IMPORT
}

pub(super) async fn list_tools(
    base: &[rmcp::model::Tool],
    page: Option<rmcp::model::PaginatedRequestParams>,
    policy: impl FnOnce() -> Result<bool> + Send + 'static,
) -> std::result::Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
    let mut catalog = base.to_vec();
    let available = tokio::task::spawn_blocking(policy)
        .await
        .map_err(|_| rmcp::ErrorData::internal_error("Could not check media policy.", None))?
        .map_err(|_| rmcp::ErrorData::internal_error("Could not check media policy.", None))?;
    if available {
        for contract in [
            include_str!("../../contracts/media-attachment-tools.json"),
            include_str!("../../contracts/media-host-file-tools.json"),
        ] {
            let tools: Vec<rmcp::model::Tool> = serde_json::from_str(contract).map_err(|_| {
                rmcp::ErrorData::internal_error("Could not load media tools.", None)
            })?;
            catalog.extend(tools);
        }
    }
    super::discovery::list_tools(&catalog, page)
}
