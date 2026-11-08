# Synthetic v0.1 compatibility fixture

v1.sql and v1.json were generated from the TypeScript implementation at commit 6f6e0cc before its removal. They contain only synthetic records: a corrected tea preference, task, proposed decision, Markdown artifact, unknown action outcome and checkpoint. The all-zero fixture token is deliberately synthetic and has no access to the owner's private instance.

The SQL includes the actual v1 tables, grant hash, usage metadata, operation request hashes and receipts. Rust tests open it directly, authenticate the same fixture client, replay every original request (including a superseded revision), and compare the exported semantics. The JSON includes the original pre-normalization requests and expected receipts/export. contracts/mcp-tools.json captures the original six advertised tools and input schemas.

These fixtures are test data, not runtime state or permission to perform any action.
