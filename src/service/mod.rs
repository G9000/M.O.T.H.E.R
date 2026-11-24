//! One native database owner with authenticated, bounded local IPC clients.
pub mod attachment;
mod local;
mod protocol;

pub use local::{LocalClient, LocalService, ServiceClientConfig, ServiceInfo, load_service_config};
