# Synthetic v0.1 compatibility fixture

v1.sql and v1.json were generated from the TypeScript implementation at commit 6f6e0cc before its removal. They contain only synthetic records: a corrected tea preference, task, proposed decision, Markdown artifact, unknown action outcome and checkpoint. The all-zero fixture token is deliberately synthetic and has no access to the owner's private instance.

The SQL preserves the original tables and metadata as an archival artifact; the executable does not load it or include a SQLite engine. Native Rust tests import the JSON bundle, replay every original request (including a superseded revision), and compare receipts, operation hashes and exported semantics. The JSON includes the original pre-normalization requests and expected receipts/export. Native numeric tests also exercise floating-point revision inputs and JavaScript whitespace normalization. contracts/mcp-tools.json preserves the supported v1 context tool schemas.

These fixtures are test data, not runtime state or permission to perform any action.
