# Language Lab

A language tutor skill with durable memory and a clock. The tutor uses arbitrary
JSON records in a local SQLite file, with optional pinned context and review dates.

The main skill is [`skills/language-lab`](skills/language-lab/SKILL.md). It ships with
the root Rust CLI package and checks that the installed version matches before using it.

```sh
cargo install --locked --path .
language-lab --version
```

Building needs Rust/Cargo and a C compiler; SQLite is bundled. The skill handles
database selection conversationally. See the [user’s guide](md/guide.md) and
[specification](md/design/README.md), or run `mdbook serve --open`.

## Workspace commands

From the repository root:

```sh
cargo lab --help
anchorbook check --config anchorbook.toml
cargo test --workspace --locked
```

`cargo lab` aliases `cargo run --locked -p language-lab --`.
Install the pinned Anchor Book engine using [project guidance](md/working.md).

The root crate ships the CLI, skill, and `SYMPOSIUM.toml` together. The package includes the CLI and tutor skill.

## Symposium

The manifest exposes `skills/language-lab` and a `lab` subcommand backed by a
Cargo installation from this Git repository. After adding this repository to your
Symposium plugin sources and enabling Language Lab, use `cargo agents lab --help`.
The tutor skill uses that managed command; `cargo lab` is the local development alias.
The Git installation sees pushed commits, not uncommitted local edits.

Validate the manifest without installing or enabling it:

```sh
cargo agents plugin validate SYMPOSIUM.toml --no-check-crates
```

## Validation

```sh
cargo test --locked -p language-lab
python3 skills/language-lab/tests/test_package.py
anchorbook check --config anchorbook.toml
mdbook build
```

The CLI specification uses Anchor Book. See [project guidance](md/working.md)
for the pinned engine installation and checks, and the
[identity mapping](md/roadmaps/README.md) for the migration from the former
requirement system.

## End-to-end tests

The opt-in [Symposium/Codex harness](tests/e2e/README.md) creates an isolated home,
installs the local package through Symposium, and can run a two-turn Spanish
tutoring smoke test under sandbox-runtime. Start with its credential-free
installation mode before running model-backed conversation tests.
