# Time windows for context and search

**Problem.** A harness can ask for a kind or a substring, but not for "what changed since yesterday", so recent work is found only by reading everything newest first.

**Decision.** `mother_get_context` and `mother_search_context` take optional `updatedSince` (inclusive) and `updatedBefore` (exclusive) on each record's last update time. Inputs are RFC 3339 with an offset or a date; they are normalized to the stored UTC millisecond form and echoed back.

**Unchanged.** The v1 schema, record shapes, inventory and unsettled-work totals, and behaviour when neither field is sent.

**Not included.** Point-in-time ("as of") reads, windows on creation time or on dates mentioned in content, and CLI flags.

**Acceptance.** `tests/time_window.rs` and the time-window test in `src/search/tests.rs`.
