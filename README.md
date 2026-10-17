# Language Lab

A language tutor skill with durable memory and a clock. The tutor uses arbitrary
JSON records in a local SQLite file, with optional pinned context and review dates.

The main skill is [`skills/language-lab`](skills/language-lab/SKILL.md). It includes
the Rust CLI and checks that the installed version matches before using it.

```sh
cargo install --locked --path skills/language-lab/cli
language-lab --version
```

Building needs Rust/Cargo and a C compiler; SQLite is bundled. The skill handles
database selection conversationally. See the [user’s guide](md/guide.md) and
[specification](md/design/README.md), or run `mdbook serve --open`.

## Validation

```sh
cargo test --locked --manifest-path skills/language-lab/cli/Cargo.toml
cargo test --locked --manifest-path mdbook-language-lab/Cargo.toml
cargo test --locked --manifest-path language-lab-ci/Cargo.toml
python3 skills/language-lab/tests/test_package.py
cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- check
mdbook build
```

Evidence records bind requirement text to reviewed source and test excerpts.
When they become stale, use the [revalidate skill](.agents/skills/revalidate/SKILL.md).
The freshness checker does not judge semantic correctness; the reviewer does.
