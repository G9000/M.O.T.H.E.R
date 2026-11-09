//! Exact official missing-alias diagnostics; all other failures remain unknown.
pub(crate) fn is_absent(success: bool, stdout: &[u8], stderr: &str, alias: &str) -> bool {
    let expected = format!("alias {alias} is not known; run create or connect first");
    let diagnostic = stderr.trim();
    !success
        && stdout.is_empty()
        && (diagnostic == expected || diagnostic == format!("Error: {expected}"))
}
#[cfg(test)]
mod tests {
    use super::is_absent;
    #[test]
    fn accepts_observed_bare_and_prefixed_missing_alias_errors() {
        let bare = "alias mother-diagnostic-unregistered is not known; run create or connect first";
        assert!(is_absent(
            false,
            b"",
            bare,
            "mother-diagnostic-unregistered"
        ));
        assert!(is_absent(
            false,
            b"",
            &format!("Error: {bare}\n"),
            "mother-diagnostic-unregistered"
        ));
    }
    #[test]
    fn only_exact_failed_empty_output_for_this_alias_means_absent() {
        let text = "alias owned is not known; run create or connect first";
        assert!(!is_absent(true, b"", text, "owned"));
        assert!(!is_absent(false, b"{}", text, "owned"));
        assert!(!is_absent(false, b"", text, "another"));
        assert!(!is_absent(
            false,
            b"",
            &format!("{text}\nunrelated diagnostic"),
            "owned"
        ));
    }
}
