//! Independent personal context and local MCP interfaces for M.O.T.H.E.R.
pub mod config;
pub mod contracts;
pub mod error;
pub mod limits;
mod portability;
pub mod store;
/// Default personality seed for a new workspace, embedded in the executable.
pub const DEFAULT_PERSONA: &str = include_str!("../SOUL.md");
