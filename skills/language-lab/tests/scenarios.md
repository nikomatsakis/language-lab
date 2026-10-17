# Tutor instruction scenarios

These are behavioral review cases for the shipped instructions, not a guarantee
that every model follows them. A reviewer reads SKILL.md and these inputs,
produces the next actions/teaching response without executing writes, and checks
those actions against the specification. Do not substitute keyword checks.

## Inputs

1. **First use:** The learner says “Let's practice Greek.” No database or host
   selection exists. A database happens to be present in the working directory.
2. **Continuity and switch:** The conversation selected `/tmp/greek.sqlite`.
   The learner resumes tomorrow, then later explicitly selects
   `/tmp/travel.sqlite`. A third fresh thread has no host selection.
3. **Initialization:** The learner chooses a new file. Run one case with known
   Europe/Berlin timezone and one with no timezone context. A chosen file already
   exists in a third case.
4. **Language scope:** A selected database has Greek and French. “Let's practice”
   contains no language preference. Then the learner chooses Japanese, absent
   from the database's language list.
5. **Context and response:** Selected Greek database; learner returns after four
   days saying “I'm back.” In a separate case they ask “Why is πήγα used here?”
   The next turn asks a follow-up during that same lesson. Context contains due
   verbs and pinned conversational preferences.
6. **Practice and lineage:** Record `1241ab` covers recognizing and producing
   πήγα. The learner recognized it independently but produced it only after a
   hint. The tutor splits the record into two successors. Describe the practice
   observation and scheduling decisions, including which ID the observation uses.
7. **Schedule choices:** Four scheduled practiced records need respectively an
   unchanged date, a later date, no future review, and a split. Show the deliberate
   decision for each. A fifth due item is unrelated to the learner's chosen topic.
8. **Replacement response:** A previously pinned, scheduled record is replaced.
   Response contains an unscheduled successor, previous pin/date metadata, and
   both review and pin reminders. Decide what to do before ending the turn.
   Repeat when intentional unscheduling and unpinning are appropriate.
9. **Durable preferences and schema:** Learner says “I prefer short conversations;
   I'm preparing for travel.” Tutor also wants a new JSON convention for recurring
   tense confusions. Explain persistence and whether schema changes are needed.
10. **Installation matrix:** In separate cases the executable is absent; reports
    0.0.9; reports 0.2.0; reports 0.1.0; Cargo is absent; installation fails; or
    installation succeeds but PATH still resolves 0.0.9. The skill is installed
    at `/tmp/my skills/language-lab`, outside the current directory.

## Review protocol

Record actual decisions for every case and identify any mismatch with the skill
requirements. The artifact/package check additionally compares the expected
version with cli/Cargo.toml and verifies cli/Cargo.lock plus build sources exist.
A successful instruction review must cover failure paths as well as normal use.
