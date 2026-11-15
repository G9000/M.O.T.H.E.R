//! Native discovery pages fit bounded hosts without changing the advertised catalog.
use rmcp::{
    ErrorData,
    model::{ListToolsResult, PaginatedRequestParams, Tool},
};
use sha2::{Digest, Sha256};

// Includes the next cursor, with 16 KiB left for the JSON-RPC envelope below a 64 KiB frame.
const PAGE_BYTES: usize = 48 * 1024;

fn invalid_cursor() -> ErrorData {
    ErrorData::invalid_params(
        "Native discovery cursor is invalid or stale; restart discovery.",
        None,
    )
}

pub(super) fn list_tools(
    tools: &[Tool],
    request: Option<PaginatedRequestParams>,
) -> Result<ListToolsResult, ErrorData> {
    let catalog = serde_json::to_vec(tools).map_err(|_| {
        ErrorData::internal_error("Native discovery catalog cannot be encoded.", None)
    })?;
    let hash = format!("{:x}", Sha256::digest(catalog));
    let cursor = |offset| format!("g1:{hash}:{offset}");
    let mut pages = Vec::new();
    let mut start = 0;
    while start < tools.len() {
        let mut accepted = None;
        for end in start + 1..=tools.len() {
            let mut page = ListToolsResult::with_all_items(tools[start..end].to_vec());
            page.next_cursor = (end < tools.len()).then(|| cursor(end));
            let size = serde_json::to_vec(&page)
                .map_err(|_| {
                    ErrorData::internal_error("Native discovery page cannot be encoded.", None)
                })?
                .len();
            if size > PAGE_BYTES {
                break;
            }
            accepted = Some((end, page));
        }
        let (end, page) = accepted.ok_or_else(|| {
            ErrorData::internal_error(
                "A native discovery descriptor exceeds the page budget.",
                None,
            )
        })?;
        pages.push((start, page));
        start = end;
    }
    let offset = match request.and_then(|request| request.cursor) {
        None => 0,
        Some(value) => {
            if value.len() > 96 {
                return Err(invalid_cursor());
            }
            let mut parts = value.split(':');
            if parts.next() != Some("g1") || parts.next() != Some(hash.as_str()) {
                return Err(invalid_cursor());
            }
            let raw = parts.next().ok_or_else(invalid_cursor)?;
            let offset: usize = raw.parse().map_err(|_| invalid_cursor())?;
            if parts.next().is_some()
                || offset == 0
                || offset >= tools.len()
                || raw != offset.to_string()
            {
                return Err(invalid_cursor());
            }
            offset
        }
    };
    if tools.is_empty() {
        return Ok(ListToolsResult::with_all_items(Vec::new()));
    }
    pages
        .into_iter()
        .find_map(|(start, page)| (start == offset).then_some(page))
        .ok_or_else(invalid_cursor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[test]
    fn real_native_catalogs_complete_in_bounded_pages_without_schema_changes() {
        for v1 in [false, true] {
            let expected = crate::mcp::garden::advertised_tools(v1).unwrap();
            let mut request = None;
            let mut all = Vec::new();
            let mut pages = 0;
            loop {
                let page = list_tools(&expected, request.clone()).unwrap();
                assert_eq!(
                    serde_json::to_value(list_tools(&expected, request).unwrap()).unwrap(),
                    serde_json::to_value(&page).unwrap()
                );
                assert!(!page.tools.is_empty());
                assert!(serde_json::to_vec(&page).unwrap().len() <= PAGE_BYTES);
                let frame = json!({"jsonrpc":"2.0","id":"x".repeat(12 * 1024),"result":page});
                assert!(serde_json::to_vec(&frame).unwrap().len() < 64 * 1024);
                all.extend(page.tools);
                pages += 1;
                match page.next_cursor {
                    None => break,
                    Some(cursor) => {
                        request = Some(PaginatedRequestParams::default().with_cursor(Some(cursor)))
                    }
                }
                assert!(
                    pages < 4,
                    "current native catalog must fit the admitted host page count"
                );
            }
            assert!(pages > 1);
            println!(
                "native discovery v1={v1}: {pages} pages, {} tools",
                all.len()
            );
            assert_eq!(
                serde_json::to_value(all).unwrap(),
                serde_json::to_value(expected).unwrap()
            );
        }
    }

    #[test]
    fn malformed_noncanonical_out_of_range_and_stale_cursors_are_rejected() {
        let tools = crate::mcp::garden::advertised_tools(true).unwrap();
        let first = list_tools(&tools, None).unwrap();
        let valid = first.next_cursor.unwrap();
        let hash = valid.split(':').nth(1).unwrap();
        for cursor in [
            String::new(),
            "unknown".into(),
            "x".repeat(97),
            format!("g1:{hash}:0"),
            format!("g1:{hash}:1"),
            format!("g1:{hash}:01"),
            format!("g1:{hash}:+1"),
            format!("g1:{hash}:{}", tools.len()),
            format!("{valid}:extra"),
            format!("g1:{}:1", "0".repeat(64)),
        ] {
            assert!(
                list_tools(
                    &tools,
                    Some(PaginatedRequestParams::default().with_cursor(Some(cursor.clone())))
                )
                .is_err(),
                "{cursor}"
            );
        }
        let mut changed = tools;
        changed[0].description = Some("Changed current catalog".into());
        assert!(
            list_tools(
                &changed,
                Some(PaginatedRequestParams::default().with_cursor(Some(valid)))
            )
            .is_err()
        );
    }

    #[test]
    fn an_oversized_descriptor_fails_closed_and_an_empty_catalog_terminates() {
        let mut tools = crate::mcp::garden::advertised_tools(true).unwrap();
        tools.last_mut().unwrap().description = Some("x".repeat(PAGE_BYTES).into());
        assert!(list_tools(&tools, None).is_err());
        let empty = list_tools(&[], None).unwrap();
        assert_eq!(
            serde_json::to_value(&empty).unwrap()["tools"],
            Value::Array(Vec::new())
        );
        assert!(empty.next_cursor.is_none());
    }
}
