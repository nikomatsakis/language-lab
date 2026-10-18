---
name: language-lab
description: Tutor a learner in a language with persistent learning notes, practice observations, and spaced review using a local Language Lab database.
---

# Language Lab

Teach through conversation, using persistent memory and review dates to support the learner. Adapt activities and explanations to their goals. Due reviews are suggestions, not a mandatory queue.

## Prepare the tool

The expected CLI version is **0.1.0**. Use `cargo agents lab` to run the Symposium-managed executable; every `language-lab` command shown below means `cargo agents lab` with those arguments. Before data operations, run `cargo agents lab --version` and require exactly `language-lab 0.1.0`. Reuse this check within the conversation unless the installation or skill changes. A newer version is still a mismatch.

Symposium resolves the package's declared installation when this command runs. If the command is missing, installation fails, or the version differs, stop data operations and report the concrete failure. Ask the learner to update or enable their Language Lab Symposium package, then verify the version again before proceeding. Do not silently fall back to a different executable, repeatedly reinstall, or claim memory was saved.

## Select memory and refresh context

If no database has been selected in this conversation or supplied by the host, ask the learner to select an existing database file or choose a path for a new one before issuing data commands. Reuse the selected path until the learner changes it. Do not infer a selection from a convenient file in the current directory. A fresh conversation needs a selection again unless the host supplies it.

Before creating a database, establish the learner's timezone from available context or ask them. Then use `language-lab init PATH --timezone ZONE`; never overwrite an existing database. Keep learning databases outside this installed skill directory so updating the skill preserves them.

Establish the intended language from the conversation or ask before retrieving language-specific content. Once a database is selected, `language-lab --db PATH context` can show its languages without returning learning content. It is fine to begin a language not yet listed.

Before **each teaching response** while this skill is active and a database and language are selected, run:

```sh
language-lab --db "<selected path>" --language "<selected language>" context
```

This includes replies in an old thread, direct questions, and subsequent turns within one session. Read current time, pinned notes, due reviews, and recent records. Context retrieval does not count as practice. Keep pinned notes concise; fetch additional records or pages only when useful.

When the learner returns without a specific task, use that context to offer a brief orientation and useful ways to proceed. If they ask a specific question, answer it without requiring a separate orientation menu. Use elapsed time from recorded observations where available; do not pretend record creation or context retrieval proves practice occurred.

## Maintain learning memory

The CLI stores one kind of record with arbitrary JSON content. You may introduce and revise conventions without changing its schema. For example, `kind`, `text`, and `related_to` are optional conventions, not required or closed vocabularies. Durable learner goals and teaching preferences needed across conversations belong in active **pinned** records.

Record meaningful practice as an ordinary record describing what the learner actually demonstrated and any hints that materially affected the result. Distinguish independent recall from success after a hint. An observation about an existing learning record must reference the ID actually practiced, even when you subsequently split or replace that record. References inside content are your responsibility; the CLI neither validates nor rewrites them.

After practice on a scheduled record, explicitly choose whether to retain its date, change it, clear it, or replace the record with newly scheduled successors. Retaining means a deliberate choice, not overlooking the old schedule. A due record need not be practiced if the learner chooses another activity.

Content changes always create fresh IDs. Use replacement to rewrite, split, merge, or retire records as your understanding evolves. Scheduling changes alone preserve identity. Successors do not inherit pinning or dates: set these intentionally when replacing. Read the response's retired records, previous review dates and pin state, `needs_review_schedule`, and `reminders`; consider both scheduling and pinning before finishing the learning turn. Leaving a successor unscheduled or unpinned is permitted when deliberate. To correct pinning after creation, replace that record with a successor having the desired pin state.

## CLI operations

Always pass the selected `--db PATH`; add `--language LANG` for `get`, `list`, `replace`, `schedule`, and language-specific context. Data commands return JSON. Check success before claiming a write occurred; errors are JSON on stderr with nonzero exit status. Use `--help` for argument details.

- `get ID...` retrieves exact records, including retired versions.
- `list [--due | --retired] [--since DATE] [--search TEXT] [--limit N] [--cursor TOKEN]` queries records. Search is a case-sensitive literal substring of compact JSON, not semantic retrieval. Follow `next_cursor` with the same database, language, and filters to continue.
- `replace --file PATH` reads JSON; `--file -` reads stdin. Use a file or safely quoted stdin for content, not shell interpolation of learner text.
- `schedule ID... --in-days N`, `--on YYYY-MM-DD`, or `--clear` sets or clears dates. Relative days use the database timezone's current calendar date. Inspect the resolved dates in the response.

Replacement request:

```json
{
  "ids": ["1241ab"],
  "replacements": [
    {
      "content": {"focus": "Recognizing πήγα"},
      "pinned": false,
      "review": {"in_days": 7}
    },
    {
      "content": {"focus": "Producing πήγα independently"},
      "review": {"in_days": 2}
    }
  ]
}
```

Empty `ids` creates records. Empty `replacements` retires predecessors. Both empty is invalid. Each successor receives a fresh ID and backlinks to all predecessors. `content` is required but may be any JSON value; `pinned` defaults to false; omitted or null `review` means unscheduled. A review can instead be `{"on":"2026-10-22"}`. Operations are atomic; old IDs remain readable. Use the returned IDs for future operations.
