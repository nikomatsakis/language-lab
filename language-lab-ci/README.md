# language-lab-ci

Checks the freshness of requirement evidence. It does **not** judge whether a
justification is sound or whether the cited code implements or tests the contract.
That assessment belongs to the `revalidate` skill in `.agents/skills/revalidate`.

From the repository root:

```sh
cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- check
cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- check --requirement record.identity.unique
cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- requirements
cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- input --path path/to/source.rs --line-start 10 --line-end 20
cargo test --locked --manifest-path language-lab-ci/Cargo.toml
```

Every command accepts `--root PATH` (default: current directory). `requirements`
accepts `--requirement ID` too. Helpers output JSON and never write attestations.

## Requirement text

The checker reads `book.toml` (`book.src`, default `src`) and local chapters linked
from `SUMMARY.md`, including nested chapters. Draft and external links are skipped.
Unlisted Markdown files are not part of the check. It reads authored Markdown,
without running mdBook preprocessors or expanding includes. Keep requirement
definitions directly in listed chapters.

The requirement renderer and checker share the same Markdown-aware role parser.
Code examples, escaped roles, and link/image labels are not definitions. Each
`{req}` definition must start its own paragraph, with exactly one definition in
that paragraph. Its text is the Markdown following the role through the end of
that paragraph, with only surrounding whitespace trimmed. Internal line breaks,
formatting, and reference syntax are preserved. Separate paragraphs are rationale,
not part of the requirement. Unsupported placement and duplicate IDs fail closed.

## Evidence format

For each ID, `evidence/ID.json` must contain exactly:

```json
{
  "requirement": "Exact text returned by the requirements command.",
  "inputs": [
    {
      "path": "path/to/source.rs",
      "line_start": 10,
      "line_end": 20,
      "contents": "Exact selected lines, including their original line endings.\n"
    }
  ],
  "justification": "The reviewer's explanation of complete implementation and test coverage."
}
```

Inputs must be nonempty. Line ranges are one-based and inclusive. LF/CRLF,
Unicode, and presence or absence of the final newline are compared exactly.
An empty file has no selectable lines. Paths must stay inside the repository,
including through symlinks; absolute paths and `..` components are rejected.
The `input` helper avoids hand-copying line ranges or escaping JSON.

Missing/malformed evidence, changed requirement text, missing input files,
invalid ranges, and changed excerpts all fail with instructions to revalidate.
All requirement evidence failures are reported in one run. Structural errors in
the book fail discovery. A book with no definitions fails instead of passing
vacuously. Extra evidence files for removed requirements are ignored.

The `justification` field must be a string, but its contents are deliberately
ignored. Passing only establishes that a previous assessment's text is current;
changes outside the selected excerpts can still affect behavior. Reviewers must
include relevant helpers and tests and must not refresh snapshots mechanically.

The separate **Requirement evidence** GitHub Actions workflow runs the tests and
then this check. It intentionally fails until every requirement has evidence.
Book publishing remains independent so the proposed design can still be read.
