//! Rebuildable, bounded vector cache. It never supplies canonical record text.

pub(super) fn profile_fingerprint() -> String {
    crate::embedding::profile().fingerprint()
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Vector {
    pub start: usize,
    pub end: usize,
    pub values: Vec<f32>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Entry {
    pub partial: bool,
    pub vectors: Vec<Vector>,
}
