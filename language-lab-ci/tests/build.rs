use std::{fs, path::PathBuf, process::Command};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("language-lab-build-{}", std::process::id()));
        fs::create_dir_all(root.join("chapters")).unwrap();
        fs::create_dir_all(root.join("evidence")).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("book.toml"), "[book]\nsrc='chapters'\n").unwrap();
        fs::write(
            root.join("chapters/SUMMARY.md"),
            "- [Contract](contract.md)\n",
        )
        .unwrap();
        fs::write(
            root.join("chapters/contract.md"),
            "{req}`test.build` Build it.\n",
        )
        .unwrap();
        fs::write(root.join("evidence/test.build.json"), serde_json::json!({
            "requirement":"Build it.", "inputs":[{"path":"book.toml","line_start":1,"line_end":1,"contents":"[book]\n"}],"justification":"fixture"
        }).to_string()).unwrap();
        let f = Self(root);
        f.write(
            "Cargo.toml",
            "[package]\nname='build-fixture'\nversion='0.1.0'\nedition='2021'\n[workspace]\n",
        );
        f.write(
            "Cargo.lock",
            "version = 4\n[[package]]\nname = 'build-fixture'\nversion = '0.1.0'\n",
        );
        f.write("src/main.rs", "fn main() {}\n");
        f
    }
    fn write(&self, path: &str, contents: &str) {
        fs::write(self.0.join(path), contents).unwrap();
    }
    fn command(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_language-lab-ci"));
        // The caller's working directory differs from the selected repository.
        cmd.current_dir(self.0.parent().unwrap())
            .args(["check", "--root"])
            .arg(self.0.file_name().unwrap());
        cmd.env("CARGO_NET_OFFLINE", "true");
        cmd
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn full_check_builds_locked_project_and_propagates_failures() {
    let f = Fixture::new();
    let out = f.command().output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    // Compiler errors prove the command actually compiles the selected project.
    f.write(
        "src/main.rs",
        "compile_error!(\"intentional build failure\"); fn main() {}\n",
    );
    let out = f.command().output().unwrap();
    assert!(!out.status.success());
    let error = String::from_utf8_lossy(&out.stderr);
    assert!(error.contains("intentional build failure"));
    assert!(error.contains("Skill build failed"));
    assert!(!String::from_utf8_lossy(&out.stdout).contains("Evidence is current"));
    assert!(
        f.command()
            .args(["--requirement", "test.build"])
            .output()
            .unwrap()
            .status
            .success()
    );
    f.write("src/main.rs", "fn main() {}\n");
    let lock = f.0.join("Cargo.lock");
    let original = fs::read(&lock).unwrap();
    f.write(
        "Cargo.lock",
        "version = 4\n[[package]]\nname = 'build-fixture'\nversion = '9.0.0'\n",
    );
    let before = fs::read(&lock).unwrap();
    let out = f.command().output().unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("--locked"));
    assert_eq!(fs::read(&lock).unwrap(), before);
    fs::remove_file(&lock).unwrap();
    let out = f.command().output().unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("missing"));
    assert!(!lock.exists());
    fs::write(&lock, original).unwrap();
    let empty_path = f.0.join("empty-path");
    fs::create_dir(&empty_path).unwrap();
    let out = f.command().env("PATH", empty_path).output().unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("could not run cargo"));
    fs::remove_file(f.0.join("Cargo.toml")).unwrap();
    let out = f.command().output().unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Cargo.toml"));
}
