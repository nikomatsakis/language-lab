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
cargo ci
cargo test --workspace --locked
```

`cargo lab` aliases `cargo run --locked -p language-lab --`; `cargo ci` aliases
`cargo run --locked -p language-lab-ci --`. Arguments are forwarded to the tool,
for example `cargo ci check --requirement skill.package`.

The root crate ships the CLI, skill, and `SYMPOSIUM.toml` together. The package
has no dependency on the repository's supporting workspace crates.

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
cargo test --locked --manifest-path mdbook-language-lab/Cargo.toml
cargo test --locked --manifest-path language-lab-ci/Cargo.toml
python3 skills/language-lab/tests/test_package.py
cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- check
mdbook build
```

Evidence records bind requirement text to reviewed source and test excerpts.
When they become stale, use the [revalidate skill](.agents/skills/revalidate/SKILL.md).
The freshness checker does not judge semantic correctness; the reviewer does.

The GitHub Actions workflow runs these CI checks on pushes and pull requests.
The full `language-lab-ci check` also builds the root CLI with `cargo build
--locked`; `check --requirement ID` checks only that requirement’s evidence.
