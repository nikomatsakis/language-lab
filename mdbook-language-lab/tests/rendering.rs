use mdbook_language_lab::process;
use serde_json::{Value, json};

fn chapter(path: &str, content: &str) -> Value {
    json!({"Chapter": {"path": path, "content": content, "sub_items": []}})
}
fn render(source: &str) -> String {
    let mut book = json!({"items": [chapter("test.md", source)]});
    process(&mut book).unwrap();
    book["items"][0]["Chapter"]["content"]
        .as_str()
        .unwrap()
        .into()
}

#[test]
fn identifier_alphabet_and_required_dot() {
    for id in ["a.b", "0.9", "a-b.c_d", ".", "a..b"] {
        let output = render(&format!("{{req}}`{id}` Text."));
        assert!(output.contains(&format!("id=\"req-{id}\"")));
    }
    for id in ["nodot", "A.b", "é.b", "a b", "a/b", "a:b", "a<b", "a\\b"] {
        let mut book = json!({"items": [chapter("invalid.md", &format!("{{req}}`{id}` Text."))]});
        let error = process(&mut book).unwrap_err();
        assert!(
            error.contains("invalid requirement identifier"),
            "{id}: {error}"
        );
        assert!(error.contains("invalid.md"));
    }
}

#[test]
fn duplicate_and_unknown_diagnostics_identify_sources() {
    for same_chapter in [false, true] {
        let mut book = if same_chapter {
            json!({"items": [chapter("first.md", "{req}`a.b` One.\n\n{req}`a.b` Two.")]})
        } else {
            json!({"items": [chapter("first.md", "{req}`a.b` One."), chapter("deep/second.md", "{req}`a.b` Two.")]})
        };
        let error = process(&mut book).unwrap_err();
        assert!(error.contains("duplicate requirement a.b"));
        assert!(error.contains("first.md"));
        if !same_chapter {
            assert!(error.contains("deep/second.md"));
        }
    }
    let mut book = json!({"items": [chapter("deep/unknown.md", "{missing.rule}`Label`")]});
    let error = process(&mut book).unwrap_err();
    assert!(error.contains("missing.rule"));
    assert!(error.contains("deep/unknown.md"));
}

#[test]
fn references_resolve_before_and_after_nested_definitions() {
    for reversed in [false, true] {
        let source = chapter("guide/deep/start.md", "{a.b}`Go there`");
        let mut parent = chapter("design/root.md", "Parent");
        parent["Chapter"]["sub_items"] = json!([chapter("design/Greek α.md", "{req}`a.b` Rule.")]);
        let items = if reversed {
            vec![parent, source]
        } else {
            vec![source, parent]
        };
        let mut book = json!({"items":items});
        process(&mut book).unwrap();
        let index = usize::from(reversed);
        assert_eq!(
            book["items"][index]["Chapter"]["content"],
            "<a class=\"requirement-reference\" href=\"../../design/Greek%20%CE%B1.html#req-a.b\" title=\"Requirement a.b\">Go there</a>"
        );
    }
    let local = render("{req}`a.b` Rule.\n\n{a.b}`local`");
    assert!(local.contains("href=\"#req-a.b\" title=\"Requirement a.b\">local</a>"));
}

#[test]
fn literal_markdown_contexts_remain_byte_identical() {
    let cases = [
        "```markdown\n{req}`fake.id`\n{missing.id}`reference`\n```\n",
        "~~~\n{req}`fake.id`\n~~~\n",
        "    {req}`fake.id`\n\n    {missing.id}`reference`\n",
        "``{req}`fake.id` `` and ``{missing.id}`reference` ``\n",
        "\\{req}`fake.id` and \\{missing.id}`reference`\n",
        "[{req}`fake.id`](https://example.com)\n",
        "![{req}`fake.id`](image.png)\n",
        "[{missing.id}`reference`][target]\n\n[target]: page.md\n",
        "![{missing.id}`reference`][image]\n\n[image]: image.png\n",
    ];
    for source in cases {
        assert_eq!(render(source), source);
    }
    // An even number of backslashes escapes the slash, not the role brace.
    assert!(render("\\\\{req}`real.id`").starts_with("\\\\<a "));
}

#[test]
fn html_labels_escape_all_special_characters() {
    let output = render("{req}`a.b` Text.\n\n{a.b}`<img src=x onerror='bad' title=\"bad\"> &`");
    assert!(
        output.contains("&lt;img src=x onerror=&#39;bad&#39; title=&quot;bad&quot;&gt; &amp;</a>")
    );
    assert!(!output.contains("<img"));
}

#[test]
fn preserves_bytes_outside_roles_including_unicode_and_crlf() {
    let prefix = "# Ελληνικά\r\n\r\n  **Before**\t";
    let suffix = "\tAfter  \r\n\r\n[link](target)\r\n{note}`untouched`\r\n";
    let badge = render("{req}`a.b`");
    assert_eq!(
        render(&format!("{prefix}{{req}}`a.b`{suffix}")),
        format!("{prefix}{badge}{suffix}")
    );
    assert_eq!(
        render("No roles\r\nUnicode: πήγα 🦀\n"),
        "No roles\r\nUnicode: πήγα 🦀\n"
    );
}

#[test]
fn badges_are_visible_accessible_and_offline_with_focus_styles() {
    let output = render("{req}`record.identity` Text.\n\n{record.identity}`identity`");
    assert!(output.contains("id=\"req-record.identity\" href=\"#req-record.identity\" aria-label=\"Requirement record.identity\""));
    assert!(output.contains("<span class=\"requirement-kind\">REQ</span>"));
    assert!(output.contains("<span class=\"requirement-name\">record.identity</span>"));
    for forbidden in ["<script", "<img", "src=", "http:", "https:", "onclick="] {
        assert!(!output.contains(forbidden));
    }
    let css = include_str!("../../theme/requirements.css");
    assert!(!css.contains("url("));
    assert!(!css.contains("@import"));
    assert!(css.contains(".requirement-badge:focus-visible, .requirement-reference:focus-visible { outline: 3px solid #d97706; outline-offset: 3px; }"));
    assert!(!css.contains("display: none"));
    assert!(!css.contains("visibility: hidden"));
    let config = include_str!("../../book.toml");
    assert!(config.contains("additional-css = [\"theme/requirements.css\"]"));
}
