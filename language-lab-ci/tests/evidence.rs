use language_lab_ci::{discover, extract, lines, snapshot, validate};
use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let f = Self(std::env::temp_dir().join(format!(
            "language-lab-ci-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )));
        fs::create_dir_all(&f.0).unwrap();
        f.write("book.toml", "[book]\nsrc = 'chapters'\n");
        f.write("chapters/SUMMARY.md", "# Summary\n\n- [Contract](nested/contract.md)\n- [Draft]()\n- [External](https://example.com)\n");
        f.write(
            "chapters/nested/contract.md",
            "# Contract\n\n{req}`test.rule` Preserve **all** data.\n",
        );
        f.write("src/code.rs", "first\nsecond\nthird");
        f
    }
    fn write(&self, path: &str, contents: &str) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }
    fn evidence(&self) -> Value {
        json!({"requirement":"Preserve **all** data.", "inputs":[{
            "path":"src/code.rs", "line_start":2, "line_end":3, "contents":"second\nthird"
        }], "justification":"reviewer's explanation"})
    }
    fn save(&self, evidence: &Value) {
        self.write("evidence/test.rule.json", &evidence.to_string());
    }
    fn errors(&self) -> Vec<String> {
        validate(&self.0, &discover(&self.0).unwrap()[0])
    }
    fn cli(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_language-lab-ci"))
            .args(args)
            .arg("--root")
            .arg(&self.0)
            .output()
            .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn extracts_multiline_markdown_and_ignores_examples() {
    let source = "# Header\n\n```md\n{req}`fake.fenced` Nope.\n```\n\n    {req}`fake.indented` Nope.\n\n``{req}`fake.inline` ``\n\n\\{req}`fake.escaped` Nope.\n\n[{req}`fake.link`](url)\n\n![{req}`fake.image`](image.png)\n\n{req}`real.rule` Keep **all** data,\nincluding {other.rule}`references`.\n\nRationale is separate.\n";
    let requirements = extract("chapter.md", source).unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].id, "real.rule");
    assert_eq!(
        requirements[0].requirement,
        "Keep **all** data,\nincluding {other.rule}`references`."
    );
    assert_eq!(
        source.lines().nth(requirements[0].line - 1).unwrap(),
        "{req}`real.rule` Keep **all** data,"
    );
}

#[test]
fn rejects_ambiguous_or_empty_definitions() {
    for source in [
        "{req}`test.rule`",
        "prefix {req}`test.rule` Text.",
        "# {req}`test.rule` Text.",
        "{req}`test.one` Text {req}`test.two` more.",
        "{req}`../bad` Text.",
    ] {
        assert!(extract("chapter.md", source).is_err(), "{source}");
    }
}

#[test]
fn discovers_only_listed_chapters_and_rejects_duplicates() {
    let f = Fixture::new();
    f.write("chapters/unused.md", "{req}`unused.rule` Not in book.");
    assert_eq!(discover(&f.0).unwrap().len(), 1);
    f.write(
        "chapters/SUMMARY.md",
        "- [First](nested/contract.md)\n- [Second](second.md)\n",
    );
    f.write("chapters/second.md", "{req}`test.rule` Duplicate.");
    assert!(discover(&f.0).unwrap_err().contains("duplicate"));
}

#[test]
fn discovery_fails_closed_on_missing_chapter_bad_config_and_empty_book() {
    let f = Fixture::new();
    f.write("chapters/SUMMARY.md", "- [Missing](missing.md)");
    assert!(discover(&f.0).is_err());
    f.write("chapters/SUMMARY.md", "# Nothing");
    assert!(discover(&f.0).unwrap_err().contains("no requirements"));
    f.write("book.toml", "[book]\nsrc = 42");
    assert!(discover(&f.0).is_err());
}

#[test]
fn exact_evidence_passes_and_justification_is_not_evaluated() {
    let f = Fixture::new();
    let mut evidence = f.evidence();
    for justification in ["", "This explanation is deliberately not a valid proof."] {
        evidence["justification"] = justification.into();
        f.save(&evidence);
        assert!(f.errors().is_empty());
        assert!(f.cli(&["check"]).status.success());
    }
}

#[test]
fn missing_stale_and_malformed_evidence_request_revalidation() {
    let f = Fixture::new();
    let errors = f.errors();
    assert!(errors[0].contains("test.rule"));
    assert!(errors[0].contains("revalidate this requirement using the revalidate skill"));
    f.write("evidence/test.rule.json", "{");
    assert!(f.errors()[0].contains("invalid evidence JSON"));
    let mut evidence = f.evidence();
    evidence["requirement"] = "Old requirement".into();
    evidence["inputs"][0]["contents"] = "old contents".into();
    f.save(&evidence);
    let errors = f.errors();
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().all(|e| e.contains("revalidate")));
}

#[test]
fn edits_to_requirement_and_source_invalidate_evidence() {
    let f = Fixture::new();
    f.save(&f.evidence());
    f.write(
        "chapters/nested/contract.md",
        "{req}`test.rule` Preserve every byte.",
    );
    assert!(f.errors()[0].contains("requirement text differs"));
    f.write(
        "chapters/nested/contract.md",
        "{req}`test.rule` Preserve **all** data.",
    );
    f.write("src/code.rs", "inserted\nfirst\nsecond\nthird");
    assert!(f.errors()[0].contains("no longer matches"));
}

#[test]
fn line_ranges_preserve_endings_unicode_and_final_newline() {
    assert_eq!(lines("α\r\nβ\r\nγ", 1, 2).unwrap(), "α\r\nβ\r\n");
    assert_eq!(lines("α\r\nβ\r\nγ", 3, 3).unwrap(), "γ");
    assert_eq!(lines("\n", 1, 1).unwrap(), "\n");
    for (text, start, end) in [
        ("", 1, 1),
        ("a\n", 0, 1),
        ("a\nb", 2, 1),
        ("a\n", 2, 2),
        ("a", 1, usize::MAX),
    ] {
        assert!(lines(text, start, end).is_err());
    }
}

#[test]
fn schema_requires_fields_and_nonempty_inputs() {
    let f = Fixture::new();
    for field in ["requirement", "inputs", "justification"] {
        let mut evidence = f.evidence();
        evidence.as_object_mut().unwrap().remove(field);
        f.save(&evidence);
        assert!(!f.errors().is_empty());
    }
    let mut evidence = f.evidence();
    evidence["inputs"] = json!([]);
    f.save(&evidence);
    assert!(f.errors()[0].contains("at least one"));
    evidence = f.evidence();
    evidence["extra"] = true.into();
    f.save(&evidence);
    assert!(!f.errors().is_empty());
    for n in [json!(0), json!(-1), json!(1.5), json!(999)] {
        evidence = f.evidence();
        evidence["inputs"][0]["line_start"] = n;
        f.save(&evidence);
        assert!(!f.errors().is_empty());
    }
}

#[test]
fn input_files_must_exist_be_utf8_and_stay_inside_repository() {
    let f = Fixture::new();
    for path in ["missing.rs", "../outside", "/etc/passwd", ""] {
        assert!(snapshot(&f.0, path, 1, 1).is_err());
    }
    fs::write(f.0.join("binary"), [0xff]).unwrap();
    assert!(snapshot(&f.0, "binary", 1, 1).is_err());
}

#[cfg(unix)]
#[test]
fn escaping_symlinks_are_rejected() {
    let f = Fixture::new();
    let outside = Fixture::new();
    std::os::unix::fs::symlink(outside.0.join("src/code.rs"), f.0.join("escape")).unwrap();
    assert!(
        snapshot(&f.0, "escape", 1, 1)
            .unwrap_err()
            .contains("escapes")
    );
}

#[test]
fn cli_helpers_return_exact_text_and_failures_aggregate() {
    let f = Fixture::new();
    let output = f.cli(&["requirements", "--requirement", "test.rule"]);
    assert!(output.status.success());
    let data: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(data[0]["requirement"], "Preserve **all** data.");
    let output = f.cli(&[
        "input",
        "--path",
        "src/code.rs",
        "--line-start",
        "2",
        "--line-end",
        "3",
    ]);
    assert!(output.status.success());
    let data: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(data, f.evidence()["inputs"][0]);
    assert!(
        !f.0.join("evidence").exists(),
        "helpers must not write attestations"
    );
    f.write(
        "chapters/nested/contract.md",
        "{req}`test.rule` First.\n\n{req}`test.other` Second.",
    );
    let output = f.cli(&["check"]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("test.rule") && stderr.contains("test.other"));
    assert!(stderr.contains("2 issue(s)"));
}

#[test]
fn cli_filter_does_not_hide_unknown_requirement() {
    let f = Fixture::new();
    f.save(&f.evidence());
    assert!(
        f.cli(&["check", "--requirement", "test.rule"])
            .status
            .success()
    );
    for args in [
        vec!["check", "--requirement", "unknown.rule"],
        vec!["check", "--unknown", "x"],
        vec!["check", "--requirement"],
        vec!["input"],
        vec!["bogus"],
    ] {
        assert!(!f.cli(&args).status.success(), "{args:?}");
    }
}

#[test]
fn default_source_external_links_and_nested_summary() {
    let f = Fixture::new();
    f.write("book.toml", "[book]\ntitle = 'Test'\n");
    f.write("src/SUMMARY.md", "# Summary\n- [Root](root.md)\n  - [Nested](deep/leaf.md#anchor)\n- [Web](//example.com/book)\n- [Mail](mailto:a@example.com)\n- [URN](urn:example:test)\n- [Draft]()\n- [Anchor](#here)\n");
    f.write("src/root.md", "{req}`test.root` Root.");
    f.write("src/deep/leaf.md", "{req}`test.leaf` Leaf.");
    let ids: Vec<_> = discover(&f.0).unwrap().into_iter().map(|r| r.id).collect();
    assert_eq!(ids, ["test.leaf", "test.root"]);
    f.write("book.toml", "book = 3");
    assert!(discover(&f.0).unwrap_err().contains("table"));
    f.write("book.toml", "[invalid");
    assert!(discover(&f.0).is_err());
}

#[test]
fn all_schema_fields_reject_wrong_types_and_missing_inputs() {
    let f = Fixture::new();
    for (key, value) in [
        ("requirement", json!(null)),
        ("inputs", json!({})),
        ("justification", json!(4)),
    ] {
        let mut evidence = f.evidence();
        evidence[key] = value;
        f.save(&evidence);
        assert!(!f.errors().is_empty(), "{key}");
    }
    for key in ["path", "line_start", "line_end", "contents"] {
        let mut evidence = f.evidence();
        evidence["inputs"][0].as_object_mut().unwrap().remove(key);
        f.save(&evidence);
        assert!(!f.errors().is_empty(), "missing {key}");
        let mut evidence = f.evidence();
        evidence["inputs"][0][key] = json!(null);
        f.save(&evidence);
        assert!(!f.errors().is_empty(), "null {key}");
    }
    let mut evidence = f.evidence();
    evidence["inputs"][0]["extra"] = json!(false);
    f.save(&evidence);
    assert!(!f.errors().is_empty());
    fs::write(f.0.join("evidence/test.rule.json"), [0xff]).unwrap();
    assert!(!f.errors().is_empty());
}

#[test]
fn exact_matching_detects_line_endings_and_ignores_uncited_changes() {
    let f = Fixture::new();
    f.write("src/code.rs", "uncited\nα\r\nβ\r\n");
    let mut evidence = f.evidence();
    evidence["inputs"][0]["contents"] = json!("α\r\nβ\r\n");
    f.save(&evidence);
    assert!(f.errors().is_empty());
    for contents in ["α\nβ\n", "α\r\nβ", "a\r\nβ\r\n"] {
        evidence["inputs"][0]["contents"] = json!(contents);
        f.save(&evidence);
        assert!(!f.errors().is_empty());
    }
    evidence["inputs"][0]["contents"] = json!("α\r\nβ\r\n");
    f.save(&evidence);
    f.write("src/code.rs", "changed uncited\nα\r\nβ\r\n");
    assert!(f.errors().is_empty());
    fs::remove_file(f.0.join("src/code.rs")).unwrap();
    assert!(f.errors()[0].contains("src/code.rs"));
}

#[test]
fn targeted_check_ignores_other_missing_evidence() {
    let f = Fixture::new();
    f.save(&f.evidence());
    f.write(
        "chapters/nested/contract.md",
        "{req}`test.rule` Preserve **all** data.\n\n{req}`other.rule` Unattested.",
    );
    assert!(
        f.cli(&["check", "--requirement", "test.rule"])
            .status
            .success()
    );
    assert!(!f.cli(&["check"]).status.success());
}

#[test]
fn requirement_trims_only_outer_whitespace() {
    let req = extract(
        "chapter.md",
        "{req}`test.rule`  First **word**.  \r\nSecond line.  \r\n\r\nSeparate rationale.",
    )
    .unwrap();
    assert_eq!(req[0].requirement, "First **word**.  \r\nSecond line.");
}

#[cfg(unix)]
#[test]
fn repository_contained_symlink_is_allowed() {
    let f = Fixture::new();
    std::os::unix::fs::symlink("src/code.rs", f.0.join("inside")).unwrap();
    assert_eq!(snapshot(&f.0, "inside", 1, 1).unwrap().contents, "first\n");
}

#[test]
fn parent_relative_chapters_are_allowed_only_inside_repository() {
    let f = Fixture::new();
    f.write("chapters/SUMMARY.md", "- [Shared](../shared/chapter.md)");
    f.write("shared/chapter.md", "{req}`test.shared` Shared contract.");
    assert_eq!(discover(&f.0).unwrap()[0].id, "test.shared");
    f.write("book.toml", "[book]\nsrc = './src/../chapters'");
    assert_eq!(discover(&f.0).unwrap()[0].id, "test.shared");
    let other = Fixture::new();
    let sibling = other.0.file_name().unwrap().to_str().unwrap();
    f.write(
        "chapters/SUMMARY.md",
        &format!("- [Escape](../../{sibling}/chapters/nested/contract.md)"),
    );
    assert!(discover(&f.0).unwrap_err().contains("escapes repository"));
}
