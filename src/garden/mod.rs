//! Versioned Garden storage, owner-reviewed proposals and an explicitly enabled native v1 view.
pub mod access;
pub mod config;
pub mod contracts;
pub mod history;
pub mod indexes;
pub mod migrate;
pub mod store;

/// Optional learned v1 search over native canonical records and disposable native vectors.
pub mod v1_search {
    pub use super::store::compat::learned::{
        EncodedPassage, EncodedPassages, Encoder, NativeSearch,
    };
}

/// Trusted owner review validation shared by the local UI and qualified graph writer.
pub mod inbox {
    pub use super::store::inbox::{OWNER_ACTOR, validate_reviewed_items};
}

/// Optional search preparation for trusted local adapters.
pub mod search {
    pub use super::store::search::{EmbeddingVectors, PreparationReport, Vectorizer};
}
/// Visual evidence: owner ingest and attachment types, plus scoped MCP media tools.
pub mod media_tools {
    pub use super::store::media::{
        Acquisition, AcquisitionInput, AttachRequest, AttachSource, AttachmentReview,
        DEFAULT_MAX_BYTES, DerivativeReport, GET_TOOL, INTERPRETATION_EXTENSION,
        INTERPRETATION_PREDICATES, IngestOptions, IngestReport, LIST_TOOL, MAX_REPLY_BYTES,
        MediaAsset, MediaOrigin, PENDING_EXTENSION, PENDING_URI, PendingAttachment, USE_EXTENSION,
        add_capabilities,
    };
}
/// Full portable snapshots and trusted owner blob ingestion.
pub mod backup {
    pub use super::store::backup::{BackupReport, BackupRestoreStage, BlobInfo};
}
