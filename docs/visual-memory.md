# Visual memory

Mother keeps images as evidence alongside graph memories. The original bytes are stored unchanged and verified by SHA-256. Captions, descriptions and OCR are separate, attributed claims about an image. They never replace it. The design and its decisions are in [the visual memory spec](superpowers/specs/2026-11-22-visual-memory-design.md).

Everything here works on the native redb Garden's v2 graph. The v1 `artifact` record type and its text-only media types are unchanged.

## What is stored

- **Asset.** One row per unique SHA-256, holding the media type, pixel dimensions and any embedded metadata found. The type comes from the bytes themselves, never from a declared value. Accepted formats are JPEG, PNG, WebP, GIF, HEIC/HEIF and AVIF. The bytes live in the existing content-addressed blob tables, inside the same database transaction as everything else.
- **Acquisition.** An append-only record written every time the owner adds a file. It holds the original filename, when it was added, an optional event time and its precision, a source note, and the origin: `captured`, `uploaded`, `imported`, `screenshot` or `ai_generated`. Adding the same bytes twice reuses the asset and appends a second acquisition. Neither record is overwritten.
- **Derivative.** An optional copy with location, device, timestamp, XMP, IPTC and comment metadata removed, linked to its original. The original's hash never changes. Derivatives are supported for JPEG, PNG, WebP and GIF. HEIC, HEIF and AVIF are reported but not rewritten. Removing EXIF also removes JPEG orientation.
- **Use.** An evidence record whose source is the asset. It is linked to an assertion by giving that assertion a new revision. Many assertions can cite one asset without copying it. Per-use context is in the `mother.media:use` extension.

The default ingest cap is 64 MiB per file; `--max-bytes` changes it.

## Owner commands

Stop the owner service before direct database commands, as described in [the local service guide](garden-service.md).

```powershell
mother garden media add --db "C:\private-data\garden.redb" --file "C:\private-photos\beach.jpg" --origin captured --event-time 2026-07-04T18:30:00Z --event-precision minute --strip-metadata
```

```powershell
mother garden media attach --db "C:\private-data\garden.redb" --file "C:\private-photos\beach.jpg" --to <assertion-id> --expected-revision 3 --note "The day we arrived"
```

```powershell
mother garden media attach --db "C:\private-data\garden.redb" --sha256 <sha256> --to <assertion-id> --expected-revision 1
```

```powershell
mother garden media list --db "C:\private-data\garden.redb" --record <assertion-id>
```

```powershell
mother garden media verify --db "C:\private-data\garden.redb"
```

```powershell
mother garden media export --db "C:\private-data\garden.redb" --sha256 <sha256> --out "C:\private-photos\copy.jpg"
```

To move images between Gardens without a full backup:

```powershell
mother garden media export-all --db "C:\private-data\garden.redb" --out "C:\private-exports\garden-media"
```

```powershell
mother garden media import --db "C:\private-data\other.redb" --dir "C:\private-exports\garden-media"
```

`export-all` writes a new private directory with one `<sha256>.<ext>` file per stored image and a `manifest.json` holding each asset's metadata, acquisitions, derivative links and citing evidence IDs. Images whose bytes can't be read, and evidence citing bytes that aren't stored, go in the manifest's `missing` list rather than failing the export. `import` checks every file's digest, size and type against the manifest. It then adds assets, acquisitions (copied unchanged) and derivative links in one transaction. Graph evidence is not imported, so attach the images to memories in the new Garden yourself. Repeating an import adds nothing, and any mismatch commits nothing.

`attach` ingests the file (or reuses a stored asset), creates the evidence and revises the assertion in one transaction. A stale `--expected-revision`, a missing target or invalid bytes commit nothing. `export` refuses to overwrite and verifies the digest first. `verify` re-hashes every stored image and reports missing, corrupt or dangling media without changing anything.

To browse attached images locally:

```powershell
mother garden explore --db "C:\private-data\garden.redb" --media
```

The gallery uses the same single-use loopback link as the explorer, and it never calls out. It shows stripped derivatives where they exist. Originals that carry location or device metadata are withheld unless you add `--originals`.

## Attaching a photo from a conversation

Hosts generally do not pass the original bytes of an uploaded image to an MCP server. A model may see a resized copy, but tool arguments are text, so Mother never treats bytes in a conversation as the original. The flow is:

1. The harness calls `mother_garden_propose` with one evidence item whose source is `{"type":"uri","uri":"urn:mother:media:pending"}`. Its `mother.media:pending` extension (schema version 1) carries the target `{"kind":"assertion","id":...}`, the target's `expectedRevision`, an optional `description` and an optional `eventTime`. The target must be visible to the proposer. The proposal carries no path and no bytes.
2. The harness tells you to finish in Mother. It must not say the photo is saved.
3. Open `mother garden inbox`, select the proposal and press `f`. Paste the path of the original, choose whether to store a stripped copy (`s`) and whether to keep the harness's description (`x`), then press Enter.
4. One transaction ingests the bytes, writes the acquisition, replaces the pending item with real evidence, revises the target and closes the proposal with its receipt. If anything fails, nothing is stored and the proposal stays pending.

The same step without the terminal UI:

```powershell
mother garden media complete --db "C:\private-data\garden.redb" --proposal <proposal-id> --file "C:\private-photos\beach.jpg" --strip-metadata --accept-description
```

Ordinary approval of a pending attachment is refused without blocking the proposal. Dismissing it stores nothing. A pending marker can never be written directly with `mother_garden_apply`.

## Interpretations

A caption, description or OCR result is an assertion with predicate `mother.media:caption`, `mother.media:description` or `mother.media:ocr_text`. It holds a string value, and its evidence must be image evidence. It needs the `mother.media:interpretation` extension (schema version 1):

```json
{"producer":{"kind":"model","name":"<model or person>"},"method":"caption","producedAt":"2026-11-22T10:00:00Z"}
```

For a model producer, confidence, when present, must use `model_inference`. Interpretations go through proposals and owner review like any other graph content. Several can coexist for one image, and retracting one leaves the image and the others untouched. Search indexes the text, never the image. The subject is the subject of the assertion the image supports. Mother never sends an image to an AI service on its own.

## MCP access

Two tools are catalogued in `contracts/media-tools.json`. They are advertised only to credentials holding their scopes. Owners grant the scopes with `mother garden grant --scopes ...`.

| Tool | Scope | Returns |
| --- | --- | --- |
| `mother_garden_media_list` | `media:meta` (needs `garden:read`) | Paged metadata, acquisitions, derivatives and visible uses. No bytes. |
| `mother_garden_media_get` | `media:bytes` (needs `media:meta`) | One image of at most 2 MiB, as MCP image content. |

- Only assets cited by evidence the credential can currently see are disclosed. Hidden assets are absent from listings and counts.
- `media_get` serves the stripped derivative by default. Without a derivative, it serves an original only when no embedded metadata was found. Otherwise the original requires `variant: "original"` and the additional `media:originals` scope.
- Images over the bound are refused, never truncated.
- Shared clients cannot receive `media:bytes` or `media:originals`.
- Returning an image discloses it to the host and usually its model provider. Hosts differ in how they handle image results, and a protocol test does not prove a host shows or forwards them.

## Backups

Full backups include assets, acquisitions and derivative links, and restore validates them with the rest of the bundle. Bundles written before media support restore unchanged. See [garden backups](garden-backups.md). Keep photos, bundles and databases outside Git.
