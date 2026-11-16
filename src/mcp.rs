//! Official MCP SDK adapter for the standalone native garden.
mod discovery;
mod garden;
mod read_output;
mod service;

use mother::error::{Error, Result};
use rmcp::model::Tool;
use serde_json::{Value, json};
use std::path::Path;

const UNCONFIRMED: &str = "The operation could not be confirmed. Reconcile its receipt before retrying with the same operation ID.";

fn input_error() -> Error {
    Error::domain(
        "INVALID_INPUT",
        "Input does not match the context contract.",
    )
}

// Preserve the v1 catalog through the native compatibility view while
// advertising the current MCP schema dialect.
fn advertised_tools() -> Result<Vec<Tool>> {
    let mut tools: Vec<Value> = serde_json::from_str(include_str!("../contracts/mcp-tools.json"))?;
    tools.push(serde_json::from_str(include_str!(
        "../contracts/capabilities-tool.json"
    ))?);
    for tool in &mut tools {
        let schema = tool["inputSchema"]
            .as_object_mut()
            .ok_or_else(input_error)?;
        schema.insert(
            "$schema".into(),
            json!("https://json-schema.org/draft/2020-12/schema"),
        );
    }
    Ok(serde_json::from_value(Value::Array(tools))?)
}

pub async fn serve_garden(config_path: &Path) -> Result<()> {
    serve(config_path).await
}

/// Select only an explicitly configured native transport. Unsupported
/// credentials never open a backend or implicitly convert a configuration.
pub async fn serve(config_path: &Path) -> Result<()> {
    let selected = mother::config::read_json(config_path, 16_384)?;
    match selected["backend"].as_str() {
        Some("garden_service") => service::serve(config_path).await,
        Some("garden_redb") => garden::serve(config_path).await,
        _ => Err(Error::domain(
            "LEGACY_UNAVAILABLE",
            "SQLite credentials are no longer supported. Use explicitly provisioned native garden credentials; existing data is unchanged.",
        )),
    }
}
