#!/usr/bin/env python3
"""Isolated local-source Symposium/Codex smoke test (Python 3.9+, macOS/Linux)."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import signal
import sqlite3
import subprocess
import sys
import tarfile
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
SRT_VERSION = "0.0.80"
CODEX_VERSION = "0.163.0"
SYMPOSIUM_VERSION = "0.5.1"


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def events_from(text):
    events = []
    for line in text.splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue  # srt may emit its own startup diagnostics
        if isinstance(event, dict) and "type" in event:
            events.append(event)
    require(not any(e["type"] in ("error", "turn.failed") for e in events),
            "Codex reported an error/failed turn; inspect the event log")
    require(any(e["type"] == "turn.completed" for e in events), "No completed Codex turn")
    return events


def assistant_text(events):
    return "\n".join(e.get("item", {}).get("text", "") for e in events
                     if e.get("item", {}).get("type") == "agent_message")


def check_database(path):
    require(path.is_file(), "The tutor did not create the selected database")
    with sqlite3.connect(path.as_uri() + "?mode=ro", uri=True) as db:
        require(db.execute("SELECT timezone FROM metadata").fetchone() == ("Europe/Berlin",),
                "Wrong learner timezone")
        rows = db.execute("SELECT content FROM records WHERE language='es' AND pinned=1 AND retired_at IS NULL").fetchall()
        require(rows, "No active pinned Spanish learning preferences were saved")
        require(any(isinstance(value, (str, list, dict)) and bool(value)
                    for value in (json.loads(r[0]) for r in rows)),
                "Pinned records contain no substantive preference content")
        return {"active_pinned_spanish_records": len(rows)}


class Harness:
    def __init__(self, args):
        self.args = args
        self.base = Path(tempfile.mkdtemp(prefix="lab-e2e-", dir="/tmp")).resolve()
        self.run_dir = self.base / "run"
        self.run_dir.mkdir(mode=0o700)
        self.home = self.run_dir / "home"
        self.tools = self.run_dir / "tools"
        self.work = self.run_dir / "learner"
        self.artifacts = self.run_dir / "artifacts"
        for p in [self.home, self.tools / "bin", self.work, self.artifacts,
                  self.run_dir / "tmp", self.run_dir / "installer", self.home / ".codex", self.home / ".symposium"]:
            p.mkdir(parents=True, exist_ok=True)
        # Start clean: no shell startup files, API credentials, Git config, or agent config.
        self.env = {"HOME": str(self.home), "CODEX_HOME": str(self.home / ".codex"),
                    "SYMPOSIUM_HOME": str(self.home / ".symposium"),
                    "CARGO_HOME": str(self.home / ".cargo"),
                    "TMPDIR": str(self.run_dir / "tmp"), "LANG": "en_US.UTF-8",
                    "PATH": "/usr/bin:/bin:/usr/sbin:/sbin", "GIT_CONFIG_NOSYSTEM": "1",
                    "GIT_CONFIG_GLOBAL": "/dev/null", "CARGO_TERM_COLOR": "never"}
        self.report = {"mode": args.mode, "status": "running", "run_directory": str(self.run_dir),
                       "versions": {"srt": SRT_VERSION, "codex": CODEX_VERSION,
                                    "symposium": SYMPOSIUM_VERSION}}
        print("Run directory:", self.run_dir, flush=True)

    def command(self, name, argv, cwd=None, env=None, timeout=None, sandbox=False, installer=False):
        print("Running:", name, flush=True)
        argv = [str(a) for a in argv]
        if sandbox and installer:
            # Installation intentionally writes agent configuration. Keep srt's
            # cwd-specific metadata protection on the empty installer directory;
            # the filesystem/network boundary still covers the whole process tree.
            argv = ["/bin/sh", "-c", "cd " + shlex.quote(str(cwd or self.work)) + " && exec " + shlex.join(argv)]
            cwd = self.run_dir / "installer"
        if sandbox:
            argv = [str(self.srt), "--settings", str(self.settings), "--", *argv]
        log = self.artifacts / name
        started = time.monotonic()
        with log.with_suffix(".stdout").open("w") as out, log.with_suffix(".stderr").open("w") as err:
            process = subprocess.Popen(argv, cwd=cwd or self.work, env=env or self.env,
                                       stdout=out, stderr=err, start_new_session=True)
            try:
                code = process.wait(timeout=timeout or self.args.timeout)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
                raise RuntimeError(name + " timed out; process group terminated")
            except BaseException:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait()
                raise
        self.report.setdefault("commands", []).append({"name": name, "exit_code": code,
                                                       "seconds": round(time.monotonic() - started, 2)})
        require(code == 0, f"{name} failed ({code}); inspect {log}.stderr")
        return log.with_suffix(".stdout").read_text()

    def bootstrap(self):
        cargo = Path(subprocess.check_output(["rustup", "which", "cargo"], text=True).strip())
        toolchain = cargo.parent.parent
        node = Path(subprocess.check_output(["node", "-p", "process.execPath"], text=True).strip())
        npm = node.parent.parent / "lib/node_modules/npm/bin/npm-cli.js"
        require(npm.is_file(), "Cannot locate npm next to Node; install a standard Node distribution")
        shutil.copy2(node, self.tools / "bin/node")
        self.env["PATH"] = os.pathsep.join([str(self.tools / "bin"), str(self.tools / "npm/node_modules/.bin"),
                                           str(cargo.parent), self.env["PATH"]])
        self.report["rust_toolchain"] = str(toolchain)
        # Downloaded tools are pinned and installed only beneath this fresh run directory.
        packages = [f"@anthropic-ai/sandbox-runtime@{SRT_VERSION}"]
        if not self.args.codex_binary:
            packages.append(f"@openai/codex@{CODEX_VERSION}")
        self.command("bootstrap-npm", [node, npm, "install", "--prefix", self.tools / "npm",
                                      "--no-audit", "--no-fund", *packages])
        self.srt = self.tools / "npm/node_modules/.bin/srt"
        self.codex = self.tools / "npm/node_modules/.bin/codex"
        if self.args.codex_binary:
            self.codex = self.tools / "bin/codex"
            shutil.copy2(Path(self.args.codex_binary).resolve(), self.codex)
            self.report["codex_provisioning"] = "copied explicitly supplied native binary"
        else:
            self.report["codex_provisioning"] = "npm install"
        if self.args.symposium_binary:
            shutil.copy2(Path(self.args.symposium_binary).resolve(), self.tools / "bin/cargo-agents")
            self.report["symposium_provisioning"] = "copied explicitly supplied binary"
        else:
            self.command("bootstrap-symposium", [cargo, "install", "symposium", "--version", SYMPOSIUM_VERSION,
                                                "--locked", "--root", self.tools])
            self.report["symposium_provisioning"] = "cargo install"
        version = self.command("symposium-version", ["cargo", "agents", "--version"])
        require(SYMPOSIUM_VERSION in version, "Unexpected Symposium version")
        version = self.command("codex-version", [self.codex, "--version"])
        require(CODEX_VERSION in version, "Unexpected Codex version")
        self.settings = self.run_dir / "sandbox.json"
        self.settings.write_text(json.dumps({
            "filesystem": {"denyRead": ["/Users", "/home", "/root", str(Path.home()), str(ROOT), str(self.base)],
                           "allowRead": [str(self.run_dir), str(toolchain)],
                           "allowWrite": [str(self.run_dir)], "denyWrite": []},
            "network": {"allowedDomains": ["index.crates.io", "static.crates.io", "crates.io",
                                            "api.openai.com", "chatgpt.com", "*.chatgpt.com", "auth.openai.com"],
                        "deniedDomains": []}}, indent=2))
        denied = self.base / "protected-sentinel"
        denied.write_text("must not be readable")
        outside_write = self.base / "outside-write"
        probe = (f"if cat {shlex.quote(str(denied))} >/dev/null 2>&1; then exit 31; fi; "
                 f"if touch {shlex.quote(str(outside_write))} 2>/dev/null; then exit 32; fi; echo sandbox-ok")
        output = self.command("sandbox-probe", ["/bin/sh", "-c", probe], sandbox=True)
        require("sandbox-ok" in output and not outside_write.exists(), "Sandbox isolation probe failed")
        self.command("installer-sandbox-probe", ["/bin/sh", "-c", probe], sandbox=True, installer=True)
        self.report["sandbox_probes_passed"] = True

    def install(self):
        # Cargo's include list snapshots current source, including uncommitted edits.
        out = subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT)
        metadata = json.loads(out)
        package = next(p for p in metadata["packages"] if p["name"] == "language-lab")
        # Packaging needs the host's downloaded dependencies but receives no model credentials.
        package_env = {k: v for k, v in os.environ.items() if k not in ("CODEX_API_KEY", "OPENAI_API_KEY")}
        self.command("snapshot", ["cargo", "package", "-p", "language-lab", "--locked", "--offline",
                                  "--allow-dirty", "--no-verify"], cwd=ROOT, env=package_env)
        archive = Path(metadata["target_directory"]) / "package" / f"language-lab-{package['version']}.crate"
        self.report["source_archive_sha256"] = hashlib.sha256(archive.read_bytes()).hexdigest()
        staging = self.run_dir / "registry"
        staging.mkdir()
        with tarfile.open(archive) as bundle:
            for entry in bundle.getmembers():
                require((entry.isfile() or entry.isdir()) and not Path(entry.name).is_absolute()
                        and ".." not in Path(entry.name).parts, "Unsafe package entry")
            bundle.extractall(staging)
        source = staging / f"language-lab-{package['version']}"
        manifest = source / "SYMPOSIUM.toml"
        text, count = re.subn(r'^git = "[^"]+"$', 'git = ' + json.dumps(source.as_uri()),
                             manifest.read_text(), flags=re.MULTILINE)
        require(count == 1, "Expected one Git installation source in Symposium manifest")
        manifest.write_text(text)
        for name, command in [("snapshot-git-init", ["git", "init"]),
                              ("snapshot-git-add", ["git", "add", "."]),
                              ("snapshot-git-commit", ["git", "-c", "user.name=Language Lab Test",
                                                       "-c", "user.email=test@localhost", "commit", "-m", "Local test snapshot"])]:
            self.command(name, command, cwd=source)
        self.report["local_source_commit"] = self.command("snapshot-commit", ["git", "rev-parse", "HEAD"],
                                                          cwd=source).strip()
        (self.work / "Cargo.toml").write_text('[package]\nname="learner-sandbox"\nversion="0.0.0"\nedition="2021"\n')
        (self.work / "src").mkdir()
        (self.work / "src/lib.rs").write_text("")
        (self.home / ".symposium/config.toml").write_text(
            'auto-update = "off"\n[defaults]\nsymposium-recommendations = false\n'
            '[[agent]]\nname = "codex"\n[[registry]]\nname = "test-local"\npath = ' + json.dumps(str(staging)) + '\n')
        self.command("enable-plugin", ["cargo", "agents", "use", "language-lab"], sandbox=True, installer=True)
        version = self.command("lab-version", ["cargo", "agents", "lab", "--version"], sandbox=True, installer=True)
        require(f"language-lab {package['version']}" in version, "Managed CLI version mismatch")
        installed = list(self.home.glob(".agents/skills/**/SKILL.md")) + list(self.work.glob(".agents/skills/**/SKILL.md"))
        require(any(p.read_text() == (source / "skills/language-lab/SKILL.md").read_text() for p in installed),
                "Symposium did not install the packaged skill")
        self.report["installed_skill_paths"] = [str(p.relative_to(self.run_dir)) for p in installed]

    def conversation(self):
        auth = self.home / ".codex/auth.json"
        env = dict(self.env)
        if self.args.auth_file:
            shutil.copyfile(Path(self.args.auth_file).expanduser(), auth)
            auth.chmod(0o600)
        elif os.environ.get("CODEX_API_KEY"):
            env["CODEX_API_KEY"] = os.environ["CODEX_API_KEY"]
        else:
            raise RuntimeError("Live test needs CODEX_API_KEY or explicit --auth-file; installation smoke test passed")
        try:
            # srt is the outer boundary; this avoids incompatible nested OS sandboxes.
            common = [self.codex, "exec", "--json", "--skip-git-repo-check",
                      "--dangerously-bypass-approvals-and-sandbox"]
            if self.args.model:
                common += ["--model", self.args.model]
            first = self.command("turn-1", common + ["Help me learn Spanish with Language Lab."],
                                 env=env, sandbox=True)
            events = events_from(first)
            session = next((e.get("thread_id") for e in events if e["type"] == "thread.started"), None)
            require(session, "Codex did not return a session ID")
            self.report["session_id"] = session
            reply = assistant_text(events)
            require(re.search(r"database|sqlite|base de datos", reply, re.I), "First reply did not request database selection")
            require(not list(self.work.rglob("*.sqlite")), "Tutor created a database before selection")
            database = self.work / "spanish.sqlite"
            prompt = (f"Create a new database at {database}. My timezone is Europe/Berlin. "
                      "Use es for Spanish. I am a beginner preparing for travel and prefer short conversations. "
                      "Save these preferences for future lessons, then help me get started.")
            followup = [self.codex, "exec", "resume", "--json", "--skip-git-repo-check",
                        "--dangerously-bypass-approvals-and-sandbox"]
            if self.args.model:
                followup += ["--model", self.args.model]
            second = self.command("turn-2", followup + [session, prompt], env=env, sandbox=True)
            second_events = events_from(second)
            commands = "\n".join(str(e.get("item", {}).get("command", "")) for e in events + second_events)
            require("cargo agents lab" in commands, "No observed use of the Symposium-managed CLI")
            self.report["database_assertions"] = check_database(database)
            self.report["model_requested"] = self.args.model
        finally:
            auth.unlink(missing_ok=True)
            # Never retain copied account authentication as a test artifact.

    def execute(self):
        try:
            self.bootstrap()
            self.install()
            if self.args.mode == "conversation":
                self.conversation()
            self.report["status"] = "passed"
        except BaseException as error:
            self.report.update(status="failed", error=str(error) or type(error).__name__)
            raise
        finally:
            (self.artifacts / "report.json").write_text(json.dumps(self.report, indent=2) + "\n")
            print("Report:", self.artifacts / "report.json", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["install", "conversation"])
    parser.add_argument("--symposium-binary", help="Copy an existing Symposium 0.5.1 executable instead of rebuilding it")
    parser.add_argument("--codex-binary", help="Copy an existing native Codex 0.163.0 executable instead of downloading it")
    parser.add_argument("--auth-file", help="Explicitly opt in to a temporary copy of Codex auth.json; removed after the run")
    parser.add_argument("--model", help="Codex model override; otherwise use its default")
    parser.add_argument("--timeout", type=int, default=600, help="Timeout per subprocess, seconds")
    args = parser.parse_args()
    try:
        Harness(args).execute()
    except KeyboardInterrupt:
        print("Interrupted; subprocess group terminated", file=sys.stderr)
        return 130
    except (RuntimeError, OSError, subprocess.SubprocessError) as error:
        print(str(error), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
