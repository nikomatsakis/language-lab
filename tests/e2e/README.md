# Symposium + Codex end-to-end smoke test

Run from a checkout with Rust/rustup, Node/npm, Python 3.9+, Git, and a C compiler.
macOS needs sandbox-exec; Linux needs sandbox-runtime's bubblewrap/socat platform
prerequisites. This is an opt-in integration test, separate from the default CI checks.

```sh
# Fresh temporary home, pinned tools, local source installation; no model calls.
python3 tests/e2e/run.py install

# Faster: copy an already installed Symposium 0.5.1 binary into the test tools.
python3 tests/e2e/run.py install --symposium-binary "$(command -v cargo-agents)"

# Live two-turn test; CODEX_API_KEY must already be set in your environment.
python3 tests/e2e/run.py conversation --symposium-binary "$(command -v cargo-agents)"

# Alternatively explicitly authorize a temporary copy of saved Codex auth.
python3 tests/e2e/run.py conversation --auth-file "$HOME/.codex/auth.json" \
  --symposium-binary "$(command -v cargo-agents)"

python3 -m unittest discover -s tests/e2e -p 'test_*.py'
```

Use `--model NAME` to choose a model, and `--timeout SECONDS` to bound each
subprocess. Without a model flag, Codex uses its default. Live runs consume model
usage. The harness never implicitly reads your saved authentication. An explicitly
provided auth file is copied with mode 0600 and removed after the conversation,
including on failure. Interrupting/killing the harness can prevent cleanup; treat
the retained temporary home as sensitive and remove it after inspection.

The test installs pinned sandbox-runtime and Codex npm packages into its temporary
tools directory. Symposium is installed from its pinned crate version, unless an
explicit local binary is supplied and version-checked. `--codex-binary PATH` also
accepts a native Codex executable of the pinned version, avoiding a download. Host Rust and system build
tools are reused read-only. Tool provisioning and harness-owned source/Git snapshot preparation run before
the OS sandbox and without model credentials. The actual plugin install and
conversation run under sandbox-runtime. Installation uses an empty launcher
directory so srt permits writing the agent configuration it is installing; the
agent runs from the learner directory, where srt protects that metadata. Both
profiles must pass the same read/write isolation probes.
There is no automatic unsandboxed fallback.

The source snapshot is the current `cargo package` archive (including uncommitted
package files). Its Symposium installation URL is rewritten to a temporary local
Git repository. Both the skill and executable therefore come from this snapshot,
not a published Language Lab package or GitHub HEAD. Recommendations and automatic
Symposium updates are disabled in the isolated config. No host plugin is enabled.

Before installation, negative probes must prove the sandbox cannot read a
protected sentinel or write outside the run directory. Personal home directories
and this checkout are denied; only the test directory and selected Rust toolchain
are reallowed there. Runtime network access is limited to Cargo download and model
auth/inference endpoints. npm bootstrap uses the package registry before sandboxing.

The live test starts naturally: “Help me learn Spanish with Language Lab.” It
expects database selection, then resumes that exact session with an explicit new
database path, timezone, Spanish namespace and preferences to remember. Assertions
require a completed turn, observed `cargo agents lab` use, and actual pinned Spanish
records in the selected database. These are smoke assertions, not a judgment of
teaching quality or a guarantee that every preference was recorded accurately.

Every run prints its temporary directory and retains stdout/stderr per command,
Codex JSONL events, the database and a JSON report. The report records tool versions,
source archive hash, temporary Git commit, failures and assertion results. Do not
commit or blindly upload retained homes or transcripts. Remove the printed parent
temporary directory when finished. The production database and config are untouched.
