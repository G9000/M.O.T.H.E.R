# M.O.T.H.E.R contributor guide

This is the owner's personal workspace, independent of AnimaOS. Preserve the README's approved title, centered poem and artwork. Keep its description brief and link to setup; put technical instructions in docs/.

Use the independent Cargo package and locked dependencies. Run `cargo fmt --all -- --check`, `cargo test --locked`, `cargo build --release --locked`, and `cargo clippy --locked --all-targets --all-features -- -D warnings`. Tests use synthetic context and owned temporary directories. The executable needs no Bun, Node or AnimaOS runtime.

Keep personal records, tokens, private exports and runtime databases out of Git. The default private data directory is outside this repository. Do not import real conversations, modify live harness settings, or start external actions as a test. tests/fixtures contains clearly synthetic compatibility data only.

The domain contract is src/contracts.rs; contracts/mcp-tools.json preserves the advertised v1 tool schemas. MCP mutations go through Store::write and its SQLite transaction. Preserve atomic revision checks and receipts. Client identity comes from private configuration, never tool arguments. Owner CLI grant/restore operations are not MCP tools.

Preserve the v1 SQLite schema, existing grants, IDs, sources, corrections, retractions and uncertain outcomes. Runtime upgrades open existing stores without changing generation. Restore validates a bundle before creating a fresh destination, commits all state atomically, changes generation, excludes grants and never executes actions. Keep cross-runtime hash/normalization tests.

Owners supply SOUL.md with init --soul or beside the database. Do not embed the maintainer's persona or discover one from an unrelated working directory. Existing workspaces use the stored persona snapshot. Harnesses retain their model, permissions and instruction hierarchy. Protocol tests are not proof that a real host has loaded the connection. See PORTING.md for test mappings and deliberate differences.
