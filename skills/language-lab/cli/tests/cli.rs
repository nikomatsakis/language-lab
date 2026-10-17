use serde_json::{json, Value};
use std::{
    io::Write,
    process::{Command, Output, Stdio},
};
fn run(args: &[&str], input: Option<&[u8]>) -> Output {
    let mut c = Command::new(env!("CARGO_BIN_EXE_language-lab"));
    c.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    if let Some(input) = input {
        child.stdin.take().unwrap().write_all(input).unwrap();
    }
    child.wait_with_output().unwrap()
}
fn success(out: Output) -> Value {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stderr.is_empty());
    serde_json::from_slice(&out.stdout).unwrap()
}
fn failure(out: Output, code: &str) {
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    let v: Value = serde_json::from_slice(&out.stderr).unwrap();
    assert_eq!(v["error"]["code"], code);
    assert!(v["error"]["message"].is_string());
}
#[test]
fn subprocess_contract() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("db");
    let p = path.to_str().unwrap();
    let version = run(&["--version"], None);
    assert!(version.status.success());
    assert!(version.stderr.is_empty());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap(),
        format!("language-lab {}\n", env!("CARGO_PKG_VERSION"))
    );
    for sub in [
        None,
        Some("init"),
        Some("context"),
        Some("get"),
        Some("list"),
        Some("replace"),
        Some("schedule"),
    ] {
        let mut args = vec![];
        if let Some(sub) = sub {
            args.push(sub);
        }
        args.push("--help");
        let out = run(&args, None);
        assert!(out.status.success());
        assert!(out.stderr.is_empty());
        let text = String::from_utf8(out.stdout).unwrap();
        assert!(text.contains("Usage:"));
        if sub.is_none() {
            for name in ["init", "context", "get", "list", "replace", "schedule"] {
                assert!(text.contains(name));
            }
        }
    }
    failure(run(&["--db", p, "context"], None), "database_missing");
    assert!(!path.exists());
    assert_eq!(
        success(run(&["init", p, "--timezone", "Europe/Berlin"], None)),
        json!({"database":p,"timezone":"Europe/Berlin"})
    );
    let baseline = std::fs::read(&path).unwrap();
    for args in [
        vec!["context"],
        vec!["--db", p, "list"],
        vec!["--db", p, "--language", "", "context"],
        vec!["--db", p, "--language", "el", "get"],
        vec!["--db", p, "--language", "el", "schedule", "--clear"],
        vec!["--db", p, "--language", "el", "schedule", "abcdef"],
        vec![
            "--db",
            p,
            "--language",
            "el",
            "schedule",
            "abcdef",
            "--clear",
            "--in-days",
            "1",
        ],
        vec!["--db", p, "context", "extra"],
        vec!["--db", p, "context", "--unknown"],
        vec![
            "--db",
            p,
            "--language",
            "el",
            "schedule",
            "abcdef",
            "--in-days",
            "-1",
        ],
        vec![
            "--db",
            p,
            "--language",
            "el",
            "schedule",
            "abcdef",
            "--in-days",
            "1.5",
        ],
        vec!["--db", p, "--language", "el", "list", "--due", "--retired"],
        vec!["--db", p, "--language", "el", "replace"],
    ] {
        failure(run(&args, None), "invalid_input");
        assert_eq!(std::fs::read(&path).unwrap(), baseline);
    }
    failure(
        run(&["init", p, "--timezone", "UTC"], None),
        "storage_error",
    );
    assert_eq!(std::fs::read(&path).unwrap(), baseline);
    let args = ["--db", p, "--language", "el", "replace", "--file", "-"];
    for input in [
        b"{}".as_slice(),
        b"{} {}",
        b"\xff",
        br#"{"ids":[],"replacements":[{}]}"#,
    ] {
        failure(run(&args, Some(input)), "invalid_input");
    }
    let created = success(run(
        &args,
        Some(br#"{"ids":[],"replacements":[{"content":null}]}"#),
    ));
    let id = created["created"][0].as_str().unwrap();
    let request = dir.path().join("request.json");
    std::fs::write(
        &request,
        r#"{"ids":[],"replacements":[{"content":{"custom":true}}]}"#,
    )
    .unwrap();
    success(run(
        &[
            "--db",
            p,
            "--language",
            "el",
            "replace",
            "--file",
            request.to_str().unwrap(),
        ],
        None,
    ));
    assert_eq!(
        success(run(&["--db", p, "--language", "el", "get", id, id], None))["records"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    success(run(
        &[
            "--db",
            p,
            "--language",
            "el",
            "schedule",
            id,
            "--in-days",
            "0",
        ],
        None,
    ));
    success(run(
        &[
            "--db",
            p,
            "--language",
            "el",
            "schedule",
            id,
            "--on",
            "2026-01-01",
        ],
        None,
    ));
    success(run(
        &["--db", p, "--language", "el", "schedule", id, "--clear"],
        None,
    ));
    assert_eq!(
        success(run(&["--db", p, "--language", "el", "list"], None))["records"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    success(run(&["--db", p, "context"], None));
    success(run(&["--db", p, "--language", "el", "context"], None));
    failure(
        run(&["--db", p, "--language", "fr", "get", id], None),
        "record_missing",
    );
}
#[test]
fn arbitrary_precision_json_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("db");
    let p = p.to_str().unwrap();
    success(run(&["init", p, "--timezone", "UTC"], None));
    for number in [
        "12345678901234567890123456789012345678901234567890",
        "1.234567890123456789012345678901234567890123456789",
        "1e+999",
    ] {
        let input = format!(r#"{{"ids":[],"replacements":[{{"content":{number}}}]}}"#);
        let out = success(run(
            &["--db", p, "--language", "el", "replace", "--file", "-"],
            Some(input.as_bytes()),
        ));
        let id = out["created"][0].as_str().unwrap();
        let got = success(run(&["--db", p, "--language", "el", "get", id], None));
        assert_eq!(got["records"][0]["content"].to_string(), number);
        assert_eq!(
            success(run(
                &["--db", p, "--language", "el", "list", "--search", number],
                None
            ))["records"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }
}
#[test]
fn default_page_size_is_twenty_for_list_and_context() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("db");
    let p = p.to_str().unwrap();
    success(run(&["init", p, "--timezone", "UTC"], None));
    let input=json!({"ids":[],"replacements":(0..25).map(|i|json!({"content":i,"pinned":true,"review":{"in_days":0}})).collect::<Vec<_>>()}).to_string();
    success(run(
        &["--db", p, "--language", "el", "replace", "--file", "-"],
        Some(input.as_bytes()),
    ));
    let list = success(run(&["--db", p, "--language", "el", "list"], None));
    assert_eq!(list["records"].as_array().unwrap().len(), 20);
    assert!(list["next_cursor"].is_string());
    let ctx = success(run(&["--db", p, "--language", "el", "context"], None));
    for section in ["due", "recent"] {
        assert_eq!(ctx[section]["records"].as_array().unwrap().len(), 20);
        assert!(ctx[section]["next_cursor"].is_string());
    }
    assert_eq!(ctx["pinned"].as_array().unwrap().len(), 25);
}
#[test]
fn arbitrary_content_keys_are_not_deserializer_control_messages() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("db");
    let p = p.to_str().unwrap();
    success(run(&["init", p, "--timezone", "UTC"], None));
    for content in [
        r#"{"$serde_json::private::Number":"123"}"#,
        r#"{"$serde_json::private::Number":"abc","other":2}"#,
        r#"[{"nested":{"$serde_json::private::Number":"123"}}]"#,
        r#"{"$serde_json::private::RawValue":"[1,2]"}"#,
    ] {
        let input = format!(r#"{{"ids":[],"replacements":[{{"content":{content}}}]}}"#);
        let created = success(run(
            &["--db", p, "--language", "el", "replace", "--file", "-"],
            Some(input.as_bytes()),
        ));
        let id = created["created"][0].as_str().unwrap();
        let out = run(&["--db", p, "--language", "el", "get", id], None);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(String::from_utf8(out.stdout)
            .unwrap()
            .contains(&format!("\"content\":{content}")));
    }
}
#[cfg(unix)]
#[test]
fn non_utf8_init_path_is_rejected_before_creation() {
    use std::os::unix::ffi::OsStringExt;
    let dir = tempfile::tempdir().unwrap();
    let path = dir
        .path()
        .join(std::ffi::OsString::from_vec(b"db\xff".to_vec()));
    let out = Command::new(env!("CARGO_BIN_EXE_language-lab"))
        .arg("init")
        .arg(&path)
        .args(["--timezone", "UTC"])
        .output()
        .unwrap();
    failure(out, "invalid_input");
    assert!(!path.exists());
}
