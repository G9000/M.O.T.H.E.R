//! Pinned local sentence embeddings; only explicit installation accesses the network.
use crate::{
    config,
    error::{Error, Result},
};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};
use tokenizers::Tokenizer;
use uuid::Uuid;

/// The fixed sentence encoder used by the disposable vector cache.
pub const MODEL_ID: &str = "sentence-transformers/all-MiniLM-L6-v2";
/// Immutable upstream revision whose asset digests are checked before every load.
pub const MODEL_REVISION: &str = "1110a243fdf4706b3f48f1d95db1a4f5529b4d41";
/// Cache identity for tokenization, pooling, normalization and window policy.
pub const PROFILE: &str =
    "minilm-l6-v2-candle-f32-mean-l2-window192-overlap32-sentence96-pair192-max32-v2";
/// Number of coordinates produced by the pinned encoder.
pub const DIMENSIONS: usize = 384;
const WINDOW_TOKENS: usize = 192;
const WINDOW_OVERLAP: usize = 32;
const MAX_WINDOWS: usize = 32;
const MAX_SENTENCE_TOKENS: usize = 96;
const MAX_QUERY_TOKENS: usize = 256;
// Canonical records are bounded upstream; also bound this module independently.
const MAX_TEXT_BYTES: usize = 256 * 1024;
const CLS_TOKEN: u32 = 101;
const SEP_TOKEN: u32 = 102;

struct Asset {
    name: &'static str,
    size: u64,
    sha256: &'static str,
}
const ASSETS: [Asset; 3] = [
    Asset {
        name: "model.safetensors",
        size: 90_868_376,
        sha256: "53aa51172d142c89d9012cce15ae4d6cc0ca6895895114379cacb4fab128d9db",
    },
    Asset {
        name: "config.json",
        size: 612,
        sha256: "953f9c0d463486b10a6871cc2fd59f223b2c70184f49815e7efbcab5d8908b41",
    },
    Asset {
        name: "tokenizer.json",
        size: 466_247,
        sha256: "be50c3628f2bf5bb5e3a7f17b1f74611b2561a3a27eeab05e5aa30f411572037",
    },
];

fn unavailable() -> Error {
    Error::domain("MODEL_UNAVAILABLE", "Local embedding model is unavailable")
}
fn install_error(message: &'static str) -> Error {
    Error::domain("MODEL_UNAVAILABLE", message)
}

/// Resolve the local model directory without opening files or making network requests.
///
/// # Errors
/// Returns a sanitized configuration failure if no absolute directory can be resolved.
pub fn model_directory() -> Result<PathBuf> {
    let path = match std::env::var_os("MOTHER_MODEL_DIR") {
        Some(value) => PathBuf::from(value),
        None => config::data_directory()
            .map_err(|_| unavailable())?
            .join("models")
            .join("all-MiniLM-L6-v2"),
    };
    if !path.is_absolute() {
        return Err(unavailable());
    }
    Ok(path)
}

fn open_asset(dir: &Path, asset: &Asset) -> Result<File> {
    let path = dir.join(asset.name);
    let metadata = fs::symlink_metadata(&path).map_err(|_| unavailable())?;
    if !metadata.is_file() || metadata.len() != asset.size {
        return Err(unavailable());
    }
    let file = File::open(path).map_err(|_| unavailable())?;
    let metadata = file.metadata().map_err(|_| unavailable())?;
    if !metadata.is_file() || metadata.len() != asset.size {
        return Err(unavailable());
    }
    Ok(file)
}

fn valid_digest(bytes: &[u8], asset: &Asset) -> bool {
    bytes.len() as u64 == asset.size && format!("{:x}", Sha256::digest(bytes)) == asset.sha256
}

fn read_asset(dir: &Path, asset: &Asset) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(asset.size as usize);
    open_asset(dir, asset)?
        .take(asset.size + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| unavailable())?;
    if !valid_digest(&bytes, asset) {
        return Err(unavailable());
    }
    Ok(bytes)
}

fn verify_asset(dir: &Path, asset: &Asset) -> Result<()> {
    let mut reader = open_asset(dir, asset)?.take(asset.size + 1);
    let mut buffer = [0u8; 64 * 1024];
    let mut digest = Sha256::new();
    let mut size = 0u64;
    loop {
        let count = reader.read(&mut buffer).map_err(|_| unavailable())?;
        if count == 0 {
            break;
        }
        size += count as u64;
        digest.update(&buffer[..count]);
    }
    if size != asset.size || format!("{:x}", digest.finalize()) != asset.sha256 {
        return Err(unavailable());
    }
    Ok(())
}

fn verify_directory(dir: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(dir).map_err(|_| unavailable())?;
    if !metadata.is_dir() {
        return Err(unavailable());
    }
    for asset in &ASSETS {
        verify_asset(dir, asset)?;
    }
    Ok(())
}

/// Check every installed asset's size and SHA-256 without downloading anything.
///
/// # Errors
/// Returns an error only for a relative directory; absent or corrupt models report `ready: false`.
pub fn status(dir: &Path) -> Result<Value> {
    if !dir.is_absolute() {
        return Err(unavailable());
    }
    let directory_valid = fs::symlink_metadata(dir).is_ok_and(|m| m.is_dir());
    let assets: Vec<Value> = ASSETS.iter().map(|asset| {
        let verified = directory_valid && verify_asset(dir, asset).is_ok();
        json!({"name":asset.name, "verified":verified, "bytes":asset.size, "sha256":asset.sha256})
    }).collect();
    let ready = assets.iter().all(|asset| asset["verified"] == true);
    Ok(
        json!({"model":MODEL_ID, "revision":MODEL_REVISION, "profile":PROFILE,
        "dimensions":DIMENSIONS, "ready":ready, "assets":assets}),
    )
}

// Created exclusively by this invocation, with a fixed UUID-named sibling path.
struct StagingDirectory(PathBuf);
impl Drop for StagingDirectory {
    fn drop(&mut self) {
        // Never follow a replaced top-level link when cleaning up an interrupted installation.
        if fs::symlink_metadata(&self.0).is_ok_and(|m| m.is_dir()) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
}

/// Explicitly download and verify the pinned public model into a new local directory.
///
/// Existing valid installations are reused. Invalid existing paths are never repaired or
/// removed automatically. No record or query text is included in any network request.
///
/// # Errors
/// Reports a sanitized failure for inaccessible destinations, failed downloads, mismatched
/// assets, or an existing invalid installation. Each download has a five-minute deadline
/// and fails sooner if the connection stalls for a minute.
pub fn install(dir: &Path) -> Result<Value> {
    if !dir.is_absolute() {
        return Err(install_error("Model directory must be absolute"));
    }
    match fs::symlink_metadata(dir) {
        Ok(_) => {
            verify_directory(dir).map_err(|_| install_error("Existing model directory is invalid; choose a new directory or move it aside before installing"))?;
            return status(dir);
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err(install_error("Cannot inspect model destination")),
    }
    let parent = dir.parent().ok_or_else(unavailable)?;
    fs::create_dir_all(parent)
        .map_err(|_| install_error("Cannot create model parent directory"))?;
    let staging_path = parent.join(format!(".mother-model-install-{}", Uuid::new_v4()));
    fs::create_dir(&staging_path)
        .map_err(|_| install_error("Cannot create model staging directory"))?;
    let staging = StagingDirectory(staging_path);
    let agent = ureq::Agent::config_builder()
        .https_only(true)
        .max_redirects(5)
        .timeout_global(Some(Duration::from_secs(300)))
        .timeout_connect(Some(Duration::from_secs(30)))
        // A stall limit, restarted on every read. A total body budget here would
        // cap the 91 MB asset at one minute and fail on ordinary connections.
        .timeout_per_read(Some(Duration::from_secs(60)))
        .build()
        .new_agent();
    for asset in &ASSETS {
        let url = format!(
            "https://huggingface.co/{MODEL_ID}/resolve/{MODEL_REVISION}/{}",
            asset.name
        );
        let mut response = agent.get(&url).call().map_err(|_| {
            install_error("Pinned model download failed; retry explicit installation")
        })?;
        let mut reader = response.body_mut().as_reader().take(asset.size + 1);
        let mut file = config::create_private_file(&staging.0.join(asset.name))
            .map_err(|_| install_error("Cannot create staged model asset"))?;
        let mut digest = Sha256::new();
        let mut size = 0u64;
        let mut buffer = [0u8; 64 * 1024];
        loop {
            let count = reader
                .read(&mut buffer)
                .map_err(|_| install_error("Pinned model download was interrupted"))?;
            if count == 0 {
                break;
            }
            size += count as u64;
            if size > asset.size {
                return Err(install_error(
                    "Downloaded model asset exceeds its pinned size",
                ));
            }
            digest.update(&buffer[..count]);
            file.write_all(&buffer[..count])
                .map_err(|_| install_error("Cannot write staged model asset"))?;
        }
        if size != asset.size || format!("{:x}", digest.finalize()) != asset.sha256 {
            return Err(install_error(
                "Downloaded model asset does not match its pinned size and digest",
            ));
        }
        file.sync_all()
            .map_err(|_| install_error("Cannot flush staged model asset"))?;
    }
    // A competing successful installer may have published the same verified assets.
    if fs::symlink_metadata(dir).is_ok() {
        verify_directory(dir).map_err(|_| {
            install_error("Model destination appeared during installation and is invalid")
        })?;
        return status(dir);
    }
    if fs::rename(&staging.0, dir).is_err() {
        verify_directory(dir)
            .map_err(|_| install_error("Cannot publish verified model installation"))?;
    }
    status(dir)
}

/// A bounded set of overlapping token windows with explicit omitted coverage.
#[derive(Debug)]
pub(crate) struct PassagePlan {
    pub passages: Vec<Passage>,
    pub partial: bool,
}
/// Byte offsets always reference the original title-newline-content string.
#[derive(Debug)]
pub(crate) struct Passage {
    pub start: usize,
    pub end: usize,
    token_ids: Vec<u32>,
}

pub(crate) struct LocalEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
}
impl std::fmt::Debug for LocalEmbedder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalEmbedder")
            .field("model", &MODEL_ID)
            .finish_non_exhaustive()
    }
}
impl LocalEmbedder {
    pub(crate) fn load(dir: &Path) -> Result<Self> {
        let metadata = fs::symlink_metadata(dir).map_err(|_| unavailable())?;
        if !dir.is_absolute() || !metadata.is_dir() {
            return Err(unavailable());
        }
        // Parse and load the exact verified bytes, never reopen them after validation.
        let weights = read_asset(dir, &ASSETS[0])?;
        let config_bytes = read_asset(dir, &ASSETS[1])?;
        let tokenizer_bytes = read_asset(dir, &ASSETS[2])?;
        let config: Config = serde_json::from_slice(&config_bytes).map_err(|_| unavailable())?;
        if config.hidden_size != DIMENSIONS || config.max_position_embeddings < MAX_QUERY_TOKENS {
            return Err(unavailable());
        }
        let mut tokenizer = Tokenizer::from_bytes(&tokenizer_bytes).map_err(|_| unavailable())?;
        tokenizer.with_padding(None);
        tokenizer.with_truncation(None).map_err(|_| unavailable())?;
        if tokenizer.token_to_id("[CLS]") != Some(CLS_TOKEN)
            || tokenizer.token_to_id("[SEP]") != Some(SEP_TOKEN)
        {
            return Err(unavailable());
        }
        let vb = VarBuilder::from_buffered_safetensors(weights, DType::F32, &Device::Cpu)
            .map_err(|_| unavailable())?;
        let model = BertModel::load(vb, &config).map_err(|_| unavailable())?;
        Ok(Self { model, tokenizer })
    }

    pub(crate) fn query(&self, text: &str) -> Result<Vec<f32>> {
        if text.len() > MAX_TEXT_BYTES {
            return Err(unavailable());
        }
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|_| unavailable())?;
        let ids = encoding.get_ids();
        if ids.len() <= 2 || ids.len() > MAX_QUERY_TOKENS || text.trim().is_empty() {
            return Err(unavailable());
        }
        self.encode(ids)
    }

    pub(crate) fn passages(&self, text: &str) -> Result<PassagePlan> {
        if text.len() > MAX_TEXT_BYTES {
            return Err(unavailable());
        }
        let encoding = self
            .tokenizer
            .encode(text, false)
            .map_err(|_| unavailable())?;
        plan_passages(text, encoding.get_ids(), encoding.get_offsets())
    }

    pub(crate) fn embed(&self, passage: &Passage) -> Result<Vec<f32>> {
        self.encode(&passage.token_ids)
    }

    fn encode(&self, ids: &[u32]) -> Result<Vec<f32>> {
        if ids.is_empty() || ids.len() > MAX_QUERY_TOKENS {
            return Err(unavailable());
        }
        let forward = || -> candle_core::Result<Vec<f32>> {
            let input = Tensor::new(ids, &Device::Cpu)?.unsqueeze(0)?;
            let types = input.zeros_like()?;
            // Windows are unpadded, so every token participates in attention and pooling.
            let mask = input.ones_like()?;
            let hidden = self.model.forward(&input, &types, Some(&mask))?;
            let pooling_mask = mask.to_dtype(DType::F32)?.unsqueeze(2)?;
            let sums = hidden.broadcast_mul(&pooling_mask)?.sum(1)?;
            let pooled = sums.broadcast_div(&pooling_mask.sum(1)?)?;
            pooled.squeeze(0)?.to_vec1::<f32>()
        };
        normalize(forward().map_err(|_| unavailable())?)
    }
}

fn normalize(mut values: Vec<f32>) -> Result<Vec<f32>> {
    if values.len() != DIMENSIONS || values.iter().any(|v| !v.is_finite()) {
        return Err(unavailable());
    }
    let norm = values
        .iter()
        .map(|&v| f64::from(v).powi(2))
        .sum::<f64>()
        .sqrt();
    if !norm.is_finite() || norm <= 1e-12 {
        return Err(unavailable());
    }
    for value in &mut values {
        *value = (f64::from(*value) / norm) as f32;
    }
    let normalized_norm = values.iter().map(|&v| f64::from(v).powi(2)).sum::<f64>();
    if !normalized_norm.is_finite() || (normalized_norm - 1.0).abs() > 1e-5 {
        return Err(unavailable());
    }
    Ok(values)
}

// Duplicate token sequences carry identical model input, so keep their first provenance.
// Returning false means a distinct candidate could not fit the inference budget.
fn append_passage(
    text: &str,
    ids: &[u32],
    offsets: &[(usize, usize)],
    tokens: std::ops::Range<usize>,
    passages: &mut Vec<Passage>,
) -> Result<bool> {
    let content_ids = &ids[tokens.clone()];
    if passages
        .iter()
        .any(|passage| passage.token_ids[1..passage.token_ids.len() - 1] == *content_ids)
    {
        return Ok(true);
    }
    if passages.len() == MAX_WINDOWS {
        return Ok(false);
    }
    let start = offsets[tokens.start].0;
    let end = offsets[tokens.end - 1].1;
    if start >= end || text.get(start..end).is_none() {
        return Err(unavailable());
    }
    let mut token_ids = Vec::with_capacity(content_ids.len() + 2);
    token_ids.push(CLS_TOKEN);
    token_ids.extend_from_slice(content_ids);
    token_ids.push(SEP_TOKEN);
    passages.push(Passage {
        start,
        end,
        token_ids,
    });
    Ok(true)
}

// Deterministic sentence-like boundaries, not a language-specific parser. Whole-record
// windows remain authoritative for coverage when punctuation is absent or ambiguous.
fn sentence_ranges(text: &str, offsets: &[(usize, usize)]) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut start_byte = 0;
    for end_byte in text
        .char_indices()
        .filter(|(_, c)| matches!(c, '.' | '!' | '?' | '\n' | '\r' | '。' | '！' | '？'))
        .map(|(index, c)| index + c.len_utf8())
        .chain(std::iter::once(text.len()))
    {
        let start = offsets.partition_point(|&(_, end)| end <= start_byte);
        let end = offsets.partition_point(|&(start, _)| start < end_byte);
        if start < end {
            ranges.push(start..end);
        }
        start_byte = end_byte;
    }
    ranges
}

fn plan_passages(text: &str, ids: &[u32], offsets: &[(usize, usize)]) -> Result<PassagePlan> {
    if ids.len() != offsets.len() {
        return Err(unavailable());
    }
    let mut passages = Vec::new();
    let mut start_token = 0;
    let mut covered = 0;
    // Scan past duplicate windows even when all slots are occupied: repeated content
    // has already been represented and must not hide a later unique clause.
    while start_token < ids.len() {
        let end_token = (start_token + WINDOW_TOKENS).min(ids.len());
        if !append_passage(text, ids, offsets, start_token..end_token, &mut passages)? {
            break;
        }
        covered = end_token;
        if end_token == ids.len() {
            break;
        }
        start_token = end_token - WINDOW_OVERLAP;
    }
    let partial = covered < ids.len();
    // Short passages reduce dilution by unrelated surrounding text. Adjacent pairs
    // preserve relations split over sentence boundaries, using the same original tokens.
    if passages.len() < MAX_WINDOWS {
        let sentences = sentence_ranges(text, offsets);
        for (index, sentence) in sentences.iter().enumerate() {
            if sentence.len() <= MAX_SENTENCE_TOKENS {
                append_passage(text, ids, offsets, sentence.clone(), &mut passages)?;
            }
            if index > 0 {
                let pair = sentences[index - 1].start..sentence.end;
                if pair.len() <= WINDOW_TOKENS {
                    append_passage(text, ids, offsets, pair, &mut passages)?;
                }
            }
            if passages.len() == MAX_WINDOWS {
                break;
            }
        }
    }
    Ok(PassagePlan { passages, partial })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_model_is_sanitized_and_status_is_offline() {
        let root = tempfile::tempdir().expect("create synthetic model directory");
        let missing = root.path().join("private-path-must-not-escape");
        let error = LocalEmbedder::load(&missing).expect_err("missing assets must not load");
        assert_eq!(error.code(), "MODEL_UNAVAILABLE");
        assert!(!error.to_string().contains("private-path"));
        assert_eq!(status(&missing).expect("offline status")["ready"], false);
        fs::create_dir(&missing).expect("create corrupt model directory");
        fs::write(missing.join("config.json"), b"{}").expect("write synthetic corrupt asset");
        assert!(LocalEmbedder::load(&missing).is_err());
        assert_eq!(status(&missing).expect("corrupt status")["ready"], false);
        assert!(
            install(&missing).is_err(),
            "existing invalid directory must fail before network access"
        );
        assert_eq!(
            fs::read(missing.join("config.json")).expect("preserved asset"),
            b"{}"
        );
    }

    #[test]
    fn digest_validation_rejects_same_length_corruption_and_extra_bytes() {
        let root = tempfile::tempdir().expect("create synthetic assets");
        let bytes = b"abc";
        let asset = Asset {
            name: "fixture",
            size: 3,
            sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        };
        fs::write(root.path().join(asset.name), bytes).expect("write valid fixture");
        assert!(verify_asset(root.path(), &asset).is_ok());
        assert_eq!(
            read_asset(root.path(), &asset).expect("verified bytes"),
            bytes
        );
        fs::write(root.path().join(asset.name), b"abd").expect("write corrupt fixture");
        assert!(verify_asset(root.path(), &asset).is_err());
        assert!(read_asset(root.path(), &asset).is_err());
        fs::write(root.path().join(asset.name), b"abcd").expect("write oversized fixture");
        assert!(verify_asset(root.path(), &asset).is_err());
    }

    fn synthetic_plan(tokens: usize) -> PassagePlan {
        let text = "x ".repeat(tokens);
        let ids: Vec<_> = (0..tokens).map(|i| 1000 + i as u32).collect();
        let offsets: Vec<_> = (0..tokens).map(|i| (2 * i, 2 * i + 1)).collect();
        plan_passages(&text, &ids, &offsets).expect("valid synthetic token offsets")
    }

    #[test]
    fn windows_cover_tail_overlap_and_explicitly_report_the_cap() {
        let plan = synthetic_plan(350);
        assert!(!plan.partial);
        assert_eq!(plan.passages.len(), 2);
        assert_eq!(plan.passages[0].start, 0);
        assert_eq!(plan.passages[0].end, 383);
        assert_eq!(plan.passages[1].start, 320);
        assert_eq!(plan.passages[1].end, 699);
        assert_eq!(plan.passages[0].token_ids.len(), 194);
        assert_eq!(plan.passages[0].token_ids.first(), Some(&CLS_TOKEN));
        assert_eq!(plan.passages[0].token_ids.last(), Some(&SEP_TOKEN));
        let full = synthetic_plan(5152);
        assert_eq!(full.passages.len(), 32);
        assert!(!full.partial);
        let partial = synthetic_plan(5153);
        assert_eq!(partial.passages.len(), 32);
        assert!(partial.partial);
        assert_eq!(partial.passages.last().expect("last window").end, 10303);
        assert!(synthetic_plan(0).passages.is_empty());
    }

    fn fixture_tokens(text: &str) -> (Vec<u32>, Vec<(usize, usize)>) {
        let mut vocabulary = std::collections::HashMap::new();
        let mut ids = Vec::new();
        let mut offsets = Vec::new();
        let mut cursor = 0;
        for word in text.split_whitespace() {
            let start = cursor + text[cursor..].find(word).expect("fixture word exists");
            let end = start + word.len();
            let next = vocabulary.len() as u32 + 1000;
            ids.push(*vocabulary.entry(word).or_insert(next));
            offsets.push((start, end));
            cursor = end;
        }
        (ids, offsets)
    }

    #[test]
    fn repeated_prefix_keeps_late_sentence_and_adjacent_pair() {
        let prefix = "Garden paths are lined with flowering plants and carefully watered soil. "
            .repeat(1000);
        let first = "Membership may be terminated at any time by sending written notice to the service provider.";
        let second = "Further monthly fees then cease.";
        let text = format!("Service notes\n{prefix}\n{first} {second}");
        let (ids, offsets) = fixture_tokens(&text);
        let plan = plan_passages(&text, &ids, &offsets).expect("bounded multiresolution passages");
        assert!(
            !plan.partial,
            "repeated raw windows must not exhaust the coverage budget"
        );
        assert!(plan.passages.len() <= MAX_WINDOWS);
        assert!(
            plan.passages.iter().any(|p| &text[p.start..p.end] == first),
            "late sentence must receive a focused embedding"
        );
        let pair = format!("{first} {second}");
        assert!(
            plan.passages.iter().any(|p| text[p.start..p.end] == pair),
            "late adjacent pair must preserve the relationship between sentences"
        );
        let unique: std::collections::HashSet<_> = plan
            .passages
            .iter()
            .map(|p| p.token_ids.as_slice())
            .collect();
        assert_eq!(
            unique.len(),
            plan.passages.len(),
            "identical model inputs must not waste inference slots"
        );
    }

    #[test]
    fn sentence_enrichment_and_unique_raw_windows_share_one_total_cap() {
        let text: String = (0..1000).map(|i| format!("Item{i} ends. ")).collect();
        let (ids, offsets) = fixture_tokens(&text);
        let plan = plan_passages(&text, &ids, &offsets).expect("bounded sentence enrichment");
        assert_eq!(plan.passages.len(), MAX_WINDOWS);
        assert!(
            !plan.partial,
            "raw windows cover the full text even when enrichment is capped"
        );
        let long_single_sentence = synthetic_plan(6000);
        assert_eq!(long_single_sentence.passages.len(), MAX_WINDOWS);
        assert!(
            long_single_sentence.partial,
            "unique raw content beyond the cap must remain explicit"
        );
    }

    #[test]
    fn byte_offsets_preserve_original_unicode_text() {
        let text = "Title\ncafé 東京🙂";
        let start = text.find("café").expect("synthetic token");
        let plan = plan_passages(
            text,
            &[1000, 1001],
            &[(start, start + 5), (start + 6, text.len())],
        )
        .expect("original byte offsets");
        assert_eq!(
            &text[plan.passages[0].start..plan.passages[0].end],
            "café 東京🙂"
        );
        assert!(plan_passages("🙂", &[1], &[(1, 4)]).is_err());
        assert!(plan_passages("x", &[1], &[]).is_err());
    }

    #[test]
    fn normalization_rejects_bad_vectors_and_produces_unit_length() {
        for invalid in [
            vec![0.; DIMENSIONS],
            vec![f32::NAN; DIMENSIONS],
            vec![f32::INFINITY; DIMENSIONS],
            vec![1.; DIMENSIONS - 1],
        ] {
            assert!(normalize(invalid).is_err());
        }
        let normalized = normalize(vec![2.; DIMENSIONS]).expect("finite nonzero embedding");
        let norm: f32 = normalized.iter().map(|v| v * v).sum();
        assert!((norm - 1.).abs() < 1e-5);
    }
}
