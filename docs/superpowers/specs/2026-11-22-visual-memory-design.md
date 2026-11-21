# Visual memory design

Status: implemented for M1–M11 (tracked in
[issues #8–#16, #18 and #20](https://github.com/G9000/M.O.T.H.E.R/issues?q=is%3Aissue+%22Visual+memory%3A%22)). The owner and harness guide is
[visual memory](../../visual-memory.md). The build differs from this plan where noted:

- A pending attachment uses the existing URI source locator with
  `urn:mother:media:pending` plus a `mother.media:pending` extension, so the v2 wire
  schema in `contracts/garden-tools-v2.json` is unchanged. The canonical writer
  refuses the marker; only an owner attachment review replaces it.
- The media tools live in their own catalogue, `contracts/media-tools.json`.
- A third scope, `media:originals`, gates originals. Without a derivative, an
  original is served by default only when no embedded metadata was found.
- An interpretation's subject is the subject entity of the assertion the image
  supports. There is no separate image entity.
- The terminal UI lists a target's existing attachments in the attachment-review
  step. `garden media list --record` covers any assertion.
- The explorer shows images through a separate `--media` gallery mode, not
  inside the 3D garden.
- Derivatives come from lossless container rewriting for JPEG, PNG, WebP and GIF.
  HEIC, HEIF, AVIF and recording metadata is reported but not removed.
- Recordings (M10) are streamed into the existing single-transaction blob store
  under the owner's size cap, rather than staged across transactions. Audio of
  at most 2 MiB is returned over MCP as audio content; video never is.

Principle: preserve the original bytes as evidence, then build attributed,
searchable interpretations around them. An interpretation never replaces or
overrides the original.

## What already exists

These are implemented today and must be reused, not rebuilt.

| Capability | Where | Notes |
| --- | --- | --- |
| Content-addressed blob store | `src/garden/backup.rs` (`put_blob`, `garden_blob_metadata`, `garden_blob_chunks`) | SHA-256 over the full stream, 1 MiB chunks, one durable redb write transaction. Identical bytes share one key, so deduplication is inherent. Empty input is rejected. |
| Owner-only ingestion | `mother garden blob --db … --file …` (`src/garden_cli.rs`) | Reads only an owner-supplied path. MCP drafts cannot request a filesystem read. |
| Artifact evidence locator | `SourceLocator::Artifact { sha256, sizeBytes, mediaType }` in `src/garden/contracts.rs` | v2 evidence can cite ingested bytes. Many evidence records can cite one blob. Validation checks the hash and size shape only; it does not prove the blob is present. |
| Backup/restore integrity | `validate_blobs`, `artifact_refs` in `src/garden/backup.rs`; [garden backups](../../garden-backups.md) | Every artifact reference in current/historical records, receipts, proposals and reviews must match stored bytes, size and digest, or backup/restore fails. |
| Namespaced extensions | v2 drafts, [v2 contracts](../../garden-v2-contracts.md) | Bounded, versioned JSON payloads preserved across updates. Suitable for per-reference context. |
| Reviewed capture | `mother_garden_propose`, owner inbox | Proposals are outside canonical recall until the owner approves. |

Gaps: no provenance beyond hash/size/chunks; no way to discover media as an
object ("which memories cite this photo?"); no read path for bytes; no model for
captions/OCR; no explorer display of user media; v1 `artifact` records accept only
`text/plain` and `text/markdown`.

## Decisions

### D1. Keep image bytes inside redb

The existing blob tables give atomic ingest+link transactions, one-handle service
ownership and verified backups with no extra file inventory. Content-addressed
files on disk would add a second authority, partial-write and orphan states, and
a new backup inventory. Revisit only for video, where single-transaction ingest of
multi-gigabyte files is impractical (see D9).

Add a configurable per-asset ingest cap (proposed default 64 MiB) so one image
cannot stall the write transaction or bloat backups unexpectedly.

### D2. Work in the v2 graph only

v1 schemas are archival compatibility artifacts. The v1 `artifact` media-type
allowlist stays unchanged. Media features are graph features gated by graph
scopes. A v1-only credential gains nothing.

### D3. Separate the asset, its acquisitions, and its uses

- **Media asset**: one row per SHA-256 (the existing blob metadata), plus a
  verified media type and optional pixel dimensions derived by the owner tool.
- **Acquisition record** (new, append-only, owner-written): each time the owner
  ingests bytes, record original filename, acquisition time, optional event time
  with precision, source description, and origin: `captured`, `uploaded`,
  `imported`, `screenshot` or `ai_generated`. The same bytes ingested twice keep
  both acquisition records; neither is overwritten.
- **Use**: an evidence record with `SourceLocator::Artifact`, attached to
  assertions and entities through the existing graph. Per-use context (crop,
  page, why it is relevant) goes in a `mother.media.use` extension.

Acquisition records are owner-private provenance, not graph claims. They are
returned only to credentials that can read at least one evidence record citing
that asset.

### D4. Verify media type; never trust a declared one

The owner tool sniffs the format from magic bytes and records it. The initial
allowlist is JPEG, PNG, WebP, GIF, HEIC/HEIF and AVIF. A mismatch between a
declared `mediaType` and the sniffed type is rejected at link time. Bytes are never
decoded or rendered by the service except by the explicit owner tools that read
dimensions or metadata.

### D5. Location and device metadata

Photos often embed GPS coordinates, timestamps and device serials. Ingest reports
what embedded metadata was found. The original is always stored unchanged. The
owner may additionally ingest a metadata-stripped **derivative** (a new blob with
its own hash) linked to the original by a `derivedFrom` acquisition field. Read
paths default to the derivative when one exists, and return the original only
when the request explicitly asks for it and the grant allows originals.

### D6. Two new scopes, both off by default

- `media:meta` — list and read asset metadata, acquisition records and uses for
  assets cited by visible evidence. Requires `garden:read`. No bytes.
- `media:bytes` — retrieve bytes through a bounded MCP tool. Requires
  `media:meta`. Optionally limited to derivatives only. Never available to shared
  clients by default.

Rationale: returning a photo to a harness sends it to that model provider. That
is disclosure, and it needs its own explicit owner decision, separate from text
recall. Grants remain owner CLI operations, never MCP tools.

### D7. Interpretations are attributed claims, proposed and reviewed

Captions, descriptions, OCR text and detected people/places are **assertions**
whose evidence is the image's evidence record. They carry the producing model or
person, time and method in a `mother.media.interpretation` extension. They enter
through `mother_garden_propose` and the owner inbox, so they are never canonical
until reviewed. Several interpretations of one image can coexist; corrections and
retractions use the existing lifecycle. Interpretation text, not the image, is
what semantic indexing sees. Nothing in this design sends an image to an AI model
on the service's own initiative.

### D8. Portability

Backups already carry blobs and enforce reference integrity. Additions:

- include acquisition records and derivatives in backups and restore validation;
- add `garden media verify`: re-hash every stored blob and report missing or
  corrupt assets and dangling references without failing the whole store;
- the v1 export is unchanged; a graph media export (later) writes a manifest plus
  files named by hash, with an explicit missing-asset list;
- import/restore never fetches URI sources and never fabricates bytes for a
  missing reference.

### D9. Video and audio (designed for, not scheduled)

The asset/acquisition/use split, scopes and interpretation model are
media-agnostic. Large-file ingest will need chunked multi-transaction staging
with an atomic final publish, or a verified external store. That decision is
deferred until there is a concrete need.

## Integration constraints

These limit what can be promised to the owner.

- **Hosts rarely pass original image bytes to MCP servers.** A model may see an
  uploaded photo, but tool arguments are text. Base64 in arguments is capped by
  host message limits and the image the model saw is often already resized or
  recompressed. Mother must not treat such bytes as the original.
- **Returning an image from an MCP tool** (an `image` content block) delivers it
  to the host and usually to the model provider. Behavior differs by host; some
  ignore or downscale image results. Protocol tests do not prove a host renders
  or forwards them.
- **No host is assumed to expose a stable file ID** for an uploaded image that
  Mother could later fetch.

Therefore the authoritative path for the original is always an owner-supplied
file, and the harness's role is to propose the link.

## Owner flow: attach a photo from a conversation

1. In a conversation, the owner shares a photo and says it belongs with an
   existing memory.
2. The harness calls `mother_garden_propose` with a **pending attachment**: the
   target record, its description of the photo, and the event date if known. No
   bytes. The proposal says the original is awaited.
3. The harness tells the owner to finish in Mother; it does not claim the photo
   is saved.
4. The owner opens `mother garden inbox`, selects the proposal and supplies the
   original file path. Mother hashes it, sniffs the type, reports embedded
   location metadata and offers a stripped derivative.
5. On approval, one transaction ingests the bytes (or reuses the existing blob),
   writes the acquisition record, creates the evidence record and links it to the
   target, then closes the proposal with its receipt. The harness's description
   becomes an attributed interpretation only if the owner also accepts it.
6. Dismissing the proposal stores nothing.

A direct owner CLI path (`garden media attach --file … --to <record>`) does the
same without a harness.

## Phased plan and draft issues

Each issue lists dependencies and the regression tests that must pass. All tests
use synthetic images generated in owned temporary directories. No real photos
appear in the repository or in issues.

### Phase 1 — Persistence and provenance (P0)

**M1. Media asset metadata and acquisition records** ([#8](https://github.com/G9000/M.O.T.H.E.R/issues/8)) — no dependencies.
Sniffed media type and dimensions on assets; append-only acquisition table;
ingest cap; `garden media add` returning hash, type and acquisition ID.
Acceptance: re-ingesting identical bytes reuses the blob and appends a second
acquisition; type mismatch and over-cap input are rejected without partial rows;
existing stores open unchanged without a generation change.
Tests: dedup, append-only provenance, cap, sniffing, interrupted ingest leaves no
staged chunks, existing blob-backup tests still pass.

**M2. Owner attach to an existing record** ([#9](https://github.com/G9000/M.O.T.H.E.R/issues/9)) — depends on M1.
`garden media attach` creates evidence plus link in one revision-checked
transaction; a new revision of the target, never an in-place edit.
Acceptance: stale expected revision fails cleanly; history shows the prior
revision intact; restored-generation blocking holds.
Tests: atomicity under injected failure, revision conflicts, many records citing
one asset.

**M3. Backup, restore and verification of media** ([#10](https://github.com/G9000/M.O.T.H.E.R/issues/10)) — depends on M1.
Acquisition records and derivatives included and validated; `garden media verify`
reports missing/corrupt assets and dangling references.
Acceptance: corrupt chunk detected by hash; missing asset reported by reference;
restore still excludes grants and changes generation.
Tests: extend `garden_backup` and `garden_backup_corruption`.

### Phase 2 — Bounded retrieval (P1)

**M4. `media:meta` scope and metadata discovery tool** ([#11](https://github.com/G9000/M.O.T.H.E.R/issues/11)) — depends on M1, M2.
Bounded, paged listing of assets cited by visible evidence and their uses;
contract added to `contracts/garden-tools-v2.json`.
Acceptance: assets cited only by invisible evidence are absent; v1-only and
graph-read-only credentials get nothing; discovery reports the tool only when
authorized.
Tests: ACL fail-closed cases, paging bounds, contract schema test.

**M5. `media:bytes` scope and byte retrieval** ([#13](https://github.com/G9000/M.O.T.H.E.R/issues/13)) — depends on M4, M6.
Returns an MCP image block for one asset, bounded by size, derivative-first.
Acceptance: requires explicit grant; shared clients denied by default; originals
need explicit request and permission; over-limit assets return a structured
refusal, not truncated bytes.
Tests: scope matrix, size bound, derivative preference, digest match on returned
bytes.

**M6. Location metadata report and stripped derivatives** ([#12](https://github.com/G9000/M.O.T.H.E.R/issues/12)) — depends on M1.
Report embedded GPS/device metadata at ingest; optional derivative linked by
`derivedFrom`.
Acceptance: original bytes unchanged (hash identical); derivative has no location
fields; backup carries both.
Tests: synthetic images with and without EXIF GPS.

### Phase 3 — Reviewed conversational capture (P1)

**M7. Pending attachment proposals and inbox completion** ([#14](https://github.com/G9000/M.O.T.H.E.R/issues/14)) — depends on M2.
Proposal shape for "attach awaited original"; inbox prompts for the file and
completes ingest+link+close atomically.
Acceptance: MCP cannot supply a path or bytes for the original; dismissal stores
nothing; approval receipt maps the evidence ID; attribution preserved.
Tests: inbox approval/dismissal, attribution, restored-generation blocking.

### Phase 4 — Interpretations and display (P2)

**M8. Attributed interpretations** ([#15](https://github.com/G9000/M.O.T.H.E.R/issues/15)) — depends on M7.
Caption/OCR/description assertions with `mother.media.interpretation`
attribution, reviewed via inbox, indexed as text.
Acceptance: multiple interpretations coexist; retraction removes one from recall
without touching the image; no automatic model calls.
Tests: recall shows interpretation with attribution, never as owner-asserted fact.

**M9. Explorer and terminal display** ([#16](https://github.com/G9000/M.O.T.H.E.R/issues/16)) — depends on M4.
Owner-local explorer renders derivatives through the existing CSP; terminal UI
shows metadata and opens the file only on owner request.
Acceptance: no external requests; served only on the local owner surface.

### Deferred

**M10. Video and audio ingest** — design only (D9).
**M11. Graph media export with file manifest** — after M3.

## Out of scope

Automatic AI processing of images, cloud storage or sync, face recognition, any
change to v1 schemas or fixtures, and any import of real conversations or photos
as tests.
