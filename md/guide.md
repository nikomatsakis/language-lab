# User’s guide

The commands below describe the intended interface. The Language Lab CLI and tutor skill are not yet implemented. The book’s renderer is implemented; instructions for building the book are at the end of this chapter.

## Choose your learning database

Invoke the Language Lab skill in your conversation interface. The tutor asks whether to open an existing database or create one, and which language you want to practice. It retains that selection in the conversation and passes the path explicitly on every command.

For a new database, the intended command is:

```sh
language-lab init "$HOME/Languages.sqlite" --timezone Europe/Berlin
```

The timezone determines the meaning of “today” and “in three days.” The file can hold Greek, French, and any other languages you study. Language namespaces are created by storing their first record; there is no separate registration step.

A database is for one learner. Choose a different file to keep another learner’s history separate. Move or copy it while no CLI command is using it; SQLite may create a journal alongside it while writing.

## Start or resume learning

```sh
language-lab --db "$HOME/Languages.sqlite" --language el context
```

The tutor receives the current time, pinned context, recent records, and due reviews. It can offer a brief orientation or answer your immediate question. Reading context does not count as practice and does not reschedule anything.

You can continue an old chat or start a fresh one. In a fresh chat, select the same file to recover your learning memory. The skill refreshes context before each learning turn, including turns in an old chat; see {skill.context}`the refresh requirement`.

## Store learning memory

The tutor normally runs these commands for you. To see the underlying interface, put this JSON in `new-records.json`:

```json
{
  "ids": [],
  "replacements": [
    {
      "content": {
        "kind": "learner-notes",
        "text": "Learning Greek for travel; prefers brief conversations."
      },
      "pinned": true
    },
    {
      "content": {
        "kind": "learning-item",
        "focus": "Producing πήγα independently"
      },
      "review": { "in_days": 3 }
    }
  ]
}
```

Then submit it:

```sh
language-lab --db "$HOME/Languages.sqlite" --language el replace --file new-records.json
```

The command returns generated IDs. Use the actual returned IDs in subsequent commands; IDs shown below are illustrative. `--file -` accepts the same JSON through stdin. Input files are temporary transfer files, not additional pieces of the database.

`kind` is a tutor convention. `pinned` and `review` are CLI fields with defined behavior. All active pinned records appear in context, so pin durable goals and summaries rather than every practice observation.

## Practice, then revise

After practicing, the tutor can create another ordinary record:

```json
{
  "ids": [],
  "replacements": [
    {
      "content": {
        "kind": "practice",
        "related_to": ["1241ab"],
        "activity": "Describe yesterday’s trip",
        "observation": "Produced πήγα after a hint"
      }
    }
  ]
}
```

The CLI timestamps it. The tutor-created reference points to exactly the version practiced; the CLI does not interpret it.

If recognition is strong but independent production is weak, the tutor may split the old learning record:

```json
{
  "ids": ["1241ab"],
  "replacements": [
    {
      "content": { "focus": "Recognizing πήγα" },
      "review": { "in_days": 7 }
    },
    {
      "content": { "focus": "Producing πήγα independently" },
      "review": { "in_days": 2 }
    }
  ]
}
```

Both successors get fresh IDs and links to `1241ab`. Its old content remains readable. Its pending review stops appearing as due. The response reports its former review date and pin status, and identifies successors that have no schedule. The tutor can intentionally leave a record unscheduled.

## Inspect and adjust

These examples assume `1241ab` is an active ID returned by the CLI:

```sh
language-lab --db "$HOME/Languages.sqlite" --language el get 1241ab
language-lab --db "$HOME/Languages.sqlite" --language el list --due
language-lab --db "$HOME/Languages.sqlite" --language el list --search aorist
language-lab --db "$HOME/Languages.sqlite" --language el list --since 2026-10-01
language-lab --db "$HOME/Languages.sqlite" --language el schedule 1241ab --in-days 3
language-lab --db "$HOME/Languages.sqlite" --language el schedule 1241ab --clear
```

`get` can also retrieve retired records. `list --retired` lists retired records instead of active ones. Pagination returns a `next_cursor` that can be supplied using `--cursor` with the same filters.

There is no separate notebook or practice command. The tutor reads and changes all content through the same record operations. To change pinning, it replaces the record with the desired `pinned` setting. To change only a review date, it uses `schedule`, which preserves identity.

## Build this book

From the repository root, with Rust/Cargo and mdBook installed:

```sh
cargo test --locked --manifest-path mdbook-language-lab/Cargo.toml
mdbook build
mdbook serve --open
```

The configured preprocessor command builds and runs the local crate automatically. Cargo needs dependency access on the first build; rendered badges need no network access. The output is in `book/`. See [Requirement rendering](design/rendering.md) for authoring syntax.

## Publish with GitHub Pages

The `.github/workflows/book.yml` workflow tests the requirement renderer and
builds the book on pull requests. Pushes to the repository’s default branch
publish the generated `book/` directory to GitHub Pages. It can also be run
manually from the default branch using **Actions → Publish book → Run workflow**.

To activate it:

1. Put this book directory at the root of a GitHub repository. Commit
   `book.toml`, `md/`, `theme/`, `.github/`, and the renderer crate’s source and
   `Cargo.lock`. The renderer must be included as ordinary source files, not an
   unconfigured nested Git repository; do not commit `book/` or Cargo’s `target/`.
2. In the repository’s **Settings → Pages → Build and deployment**, select
   **GitHub Actions** as the source.
3. Push to the default branch or run the workflow manually. The deployment job
   reports the published URL through the `github-pages` environment.

The workflow pins mdBook to 0.5.4, builds with stable Rust, and uses the Pages
configuration to determine the deployed base path. Pull requests build without
publishing. No personal access token or separate deployment branch is required.

See [GitHub’s custom Pages workflow documentation](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)
for repository setup.

## Validate requirement evidence

The `language-lab-ci` utility checks whether recorded implementation and test
reviews are still current:

```sh
cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- check
```

Each requirement needs an `evidence/REQUIREMENT_ID.json` file containing its exact
text, source/test line excerpts, and a justification. CI compares the text and
excerpts; it deliberately does not evaluate the justification. Missing evidence
or changed text fails the check and requests revalidation.

Use the repository’s **revalidate** skill to perform that review. It first
requires a scenario and edge-case checklist, then inspection of implementation
and tests. If anything is missing, the reviewer leaves the evidence file absent
and reports the gaps. Do not regenerate snapshots merely to silence CI.

The checker reads requirement paragraphs from chapters listed in `SUMMARY.md`.
Its `requirements` command prints the extracted text, and its `input` command
produces exact line-range snapshots. Run `--help` for their arguments. The
**Requirement evidence** workflow is independent of Pages publication; it is
expected to fail until the proposed requirements have been implemented, tested,
and revalidated.
