//! Attachment tool discovery is driven by the credential's current grant.
use mother::{error::Result, service::attachment::AttachmentClient};
use serde_json::Value;

pub(super) fn call(client: &dyn AttachmentClient, name: &str, args: Value) -> Result<Value> {
    client.request(name, &args)
}
pub(super) use mother::service::attachment::is_tool;

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
        let tools: Vec<rmcp::model::Tool> =
            serde_json::from_str(include_str!("../../contracts/media-attachment-tools.json"))
                .map_err(|_| {
                    rmcp::ErrorData::internal_error("Could not load media tools.", None)
                })?;
        catalog.extend(tools);
    }
    super::discovery::list_tools(&catalog, page)
}
