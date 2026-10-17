# Requirement evidence

`language-lab-ci` checks whether a human/agent assessment is still current. It cannot prove that an assessment is correct. The `revalidate` repository skill performs that semantic review before creating evidence.

## Discovery and snapshots

{req}`evidence.discovery` The checker must read `book.src` from `book.toml` (default `src`) and inspect local chapters linked from that directory’s `SUMMARY.md`, including nested chapters, while skipping draft and external links and unlisted files.

{req}`evidence.definitions` The checker must use the renderer’s role parser so code examples, escaped roles, and roles in Markdown link/image labels are not treated as definitions.

{req}`evidence.text` The extracted requirement must be the authored Markdown after its definition role through the end of its paragraph, trimming only outer whitespace.

{req}`evidence.placement` The checker must reject empty requirement text and definitions that are not the sole definition at the start of a paragraph.

{req}`evidence.discovery-errors` Duplicate or invalid IDs, unreadable chapters, invalid book configuration, and books with no requirements must fail discovery.

{req}`evidence.file` Every discovered requirement ID must have a readable UTF-8 JSON file at `evidence/ID.json`.

{req}`evidence.schema` Evidence must have exactly the fields `requirement` (string), `inputs` (nonempty array), and `justification` (string); each input must have exactly `path` (string), `line_start` and `line_end` (positive integers), and `contents` (string).

{req}`evidence.requirement-match` The evidence requirement string must equal the extracted requirement text exactly.

{req}`evidence.range` Input line ranges must be one-based, inclusive, ordered, and within the file’s lines; an empty file must have no selectable lines.

{req}`evidence.contents` Every input’s contents must equal its selected UTF-8 lines exactly, including Unicode, LF versus CRLF, and presence or absence of the final newline.

{req}`evidence.paths` Input paths must be repository-relative without parent components and must resolve inside the repository even after following symlinks.

{req}`evidence.justification` The checker must not evaluate or reject the contents of a string-valued justification, including an empty string.

{req}`evidence.failures` A check with missing, malformed, or stale evidence must fail nonzero and report all evidence failures with the requirement IDs and instructions to revalidate using the revalidate skill.

{req}`evidence.helpers` The `requirements` and `input` helper commands must return extracted text and input snapshots as JSON without writing evidence files.

{req}`evidence.filter` A targeted check must check only the requested requirement and must fail if that requirement does not exist.

## Revalidation and automation

{req}`revalidate.order` The revalidate skill must instruct the reviewer to remove existing evidence and enumerate required scenarios and edge cases before inspecting implementation or test code.

{req}`revalidate.gaps` The revalidate skill must instruct the reviewer to leave evidence absent and report gaps if any required behavior is unimplemented, incorrect, untested, or unverifiable.

{req}`revalidate.complete` The revalidate skill must instruct the reviewer to cite implementation and meaningful tests, run relevant tests, and justify coverage of the scenario checklist before writing evidence and running its targeted freshness check.

{req}`evidence.ci` A GitHub Actions workflow must run checker tests and the full evidence check on pushes and pull requests, failing the job when either fails.

The checker reads authored chapters, not generated preprocessor output. Keep requirements directly in listed chapters. Separate paragraphs are rationale; extra evidence files for removed requirements are ignored. A passing check proves freshness only within the cited ranges. Skill instruction requirements are reviewed with scenario-based evaluations, not claims of universal model compliance.
