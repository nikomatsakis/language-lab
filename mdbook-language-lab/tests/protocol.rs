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
