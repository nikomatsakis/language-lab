use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn execute(input: &Value) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mdbook-language-lab"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.to_string().as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn protocol_is_json_only_and_preserves_metadata() {
    let input = json!([{ "renderer": "html" }, {"items":[{"Chapter":{
        "path":"test.md", "content":"{req}`test.identity` Unique.", "sub_items":[]
    }}], "metadata": {"preserve":true}}]);
    let output = execute(&input);
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let book: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(book["metadata"], input[1]["metadata"]);
    assert!(
        book["items"][0]["Chapter"]["content"]
            .as_str()
            .unwrap()
            .contains("requirement-badge")
    );
}

#[test]
fn malformed_input_has_no_partial_stdout() {
    for input in [
        json!({}),
        json!([{"renderer":"html"},{"items":[{"Chapter":{
            "path":"broken.md", "content":"{unknown.rule}`broken`", "sub_items":[]
        }}]}]),
    ] {
        let output = execute(&input);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn renderer_negotiation() {
    for (renderer, success) in [("html", true), ("latex", false)] {
        let output = Command::new(env!("CARGO_BIN_EXE_mdbook-language-lab"))
            .args(["supports", renderer])
            .output()
            .unwrap();
        assert_eq!(output.status.success(), success);
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn semantic_and_protocol_errors_never_publish_partial_books() {
    let cases = [
        (
            json!([{"renderer":"html"},{"items":[
                {"Chapter":{"path":"one.md","content":"{req}`a.b` Rule.","sub_items":[]}},
                {"Chapter":{"path":"two.md","content":"{req}`a.b` Duplicate.","sub_items":[]}}
            ]}]),
            vec!["duplicate", "a.b", "one.md", "two.md"],
        ),
        (
            json!([{"renderer":"html"},{"items":[
                {"Chapter":{"path":"one.md","content":"{req}`a.b` Rule.","sub_items":[]}},
                {"Chapter":{"path":"two.md","content":"{unknown.id}`Missing`", "sub_items":[]}}
            ]}]),
            vec!["unknown.id", "two.md"],
        ),
        (
            json!([{"renderer":"html"},{"items":[
                {"Chapter":{"path":"bad.md","content":"{req}`Invalid.id`", "sub_items":[]}}
            ]}]),
            vec!["invalid requirement", "bad.md"],
        ),
        (json!([{"renderer":"latex"},{"items":[]}]), vec!["html"]),
        (json!([{"renderer":"html"},{"items":{}}]), vec!["array"]),
    ];
    for (input, fragments) in cases {
        let output = execute(&input);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        for fragment in fragments {
            assert!(error.contains(fragment), "{error}");
        }
    }
    let mut child = Command::new(env!("CARGO_BIN_EXE_mdbook-language-lab"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"not JSON").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}
