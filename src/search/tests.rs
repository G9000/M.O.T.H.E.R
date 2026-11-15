use super::*;

#[test]
fn short_match_keeps_bounded_neighbouring_sentence_context() {
    let text =
        "Header\nMembership may be terminated by written notice. Further monthly fees then cease.";
    let start = text.find("Further").unwrap();
    let (from, to) = context_span(text, start, text.len());
    assert!(text[from..to].contains("written notice"));
    assert!(from <= start);
    assert_eq!(to, text.len());
    let long = format!("{} End.", "a".repeat(500));
    let start = long.find("End").unwrap();
    let (from, to) = context_span(&long, start, long.len());
    assert_eq!(&long[from..to], "End.");
}

#[test]
fn neighbouring_context_survives_line_breaks_and_repeated_punctuation() {
    for separator in [".\n", ".\r\n", ".\n\n", "?! ", "... "] {
        let text = format!(
            "Header\nMembership may be terminated by written notice{separator}Further monthly fees then cease."
        );
        let start = text.find("Further").unwrap();
        let (from, to) = context_span(&text, start, text.len());
        assert!(
            text[from..to].contains("written notice"),
            "{separator:?} lost the previous sentence"
        );
        assert_eq!(to, text.len());
        let end = text.find("notice").unwrap() + "notice".len() + 1;
        let start = text.find("Membership").unwrap();
        let (from, to) = context_span(&text, start, end);
        assert!(
            text[from..to].ends_with("then cease."),
            "{separator:?} lost the next sentence"
        );
        assert!(text[from..to].starts_with("Header"));
    }
    let text = "Is it done?! Yes it is... Then stop.";
    let start = text.find("Yes").unwrap();
    let (from, to) = context_span(text, start, text.find(" Then").unwrap());
    assert_eq!(&text[from..to], text);
    // Text without any words still yields an in-bounds span.
    for text in ["!!!", "\n\n", "... ?"] {
        let (from, to) = context_span(text, 0, text.len());
        assert!(from <= to && to <= text.len());
        let (from, to) = context_span(text, text.len(), text.len());
        assert!(from <= to && to <= text.len());
    }
}
