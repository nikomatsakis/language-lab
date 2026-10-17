# mdbook-language-lab

HTML preprocessor for the Language Lab specification. It uses mdBook’s JSON
stdin/stdout protocol and renders local, accessible requirement badges.

- Define: `{req}` followed immediately by a backtick-delimited requirement ID.
- Reference: `{requirement.id}` followed by a backtick-delimited text label.

For example:

```markdown
{req}`record.unique` IDs must be unique.
See {record.unique}`the uniqueness rule`.
```

Definitions must be unique. Missing references and invalid definition IDs fail
the build. Literal code examples are preserved. IDs contain a dot and use
lowercase ASCII letters, digits, dots, underscores, or hyphens.

The parent book.toml runs this crate through Cargo and loads
`../theme/requirements.css` (relative to this crate). For another book, install
the executable and copy that stylesheet into the book:

```toml
[preprocessor.language-lab]
renderers = ["html"]

[output.html]
additional-css = ["theme/requirements.css"]
```

From this directory:

```sh
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

This crate renders the specification; it does not implement the language-learning CLI.
