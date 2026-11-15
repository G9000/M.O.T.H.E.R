//! Independent personal context and local MCP interfaces for M.O.T.H.E.R.
pub mod access;
#[cfg(feature = "legacy-sqlite")]
pub mod brief;
pub mod config;
pub mod contracts;
pub mod embedding;
pub mod error;
pub mod garden;
mod handoffs;
#[cfg(feature = "legacy-sqlite")]
mod history;
mod inbox;
pub mod launch;
pub mod limits;
mod pins;
mod portability;
mod private_paths;
pub mod search;
pub mod service;
#[cfg(feature = "legacy-sqlite")]
pub mod store;
#[cfg(feature = "legacy-sqlite")]
pub mod tend;
