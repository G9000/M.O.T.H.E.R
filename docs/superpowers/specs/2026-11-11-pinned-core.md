# Pinned core memories

**Problem.** What a harness sees first is chosen by recency and category balance. Once there are many memories, the most important facts about the owner can be missing from a first read.

**Decision.** `mother_pin_record` marks up to twelve memories or decisions as core. Every context read returns them in full (bounded per record) outside the preview limit and all filters. Pins are stored in an additive table, not on the record, so older executables can still read every record.

**Unchanged.** Record, receipt and core table shapes, preview selection and counts, search, and the AnimaOS required-tool list.

**Not included.** Owner terminal view of pins, pinning through the review inbox, ordering pins by importance, and pins in search results.

**Acceptance.** `tests/pins.rs` and the pinning stdio test in `tests/mcp.rs`.
