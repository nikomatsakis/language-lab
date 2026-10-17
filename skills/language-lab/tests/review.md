# Independent tutor instruction review

Reviewed 2026-10-17 by the spec-review subagent. I read only SKILL.md and the
scenario inputs before writing these decisions; no expected answers were read.
These are dry-run actions derived from the shipped instructions. No learner
database commands or writes were executed. This checks the instruction contract,
not universal compliance by an arbitrary model or host.

1. **First use.** Check the CLI version before any data operation. Greek is the
   intended language, but no database has been selected. Ask: “Would you like to
   open an existing learning database or create one? Which path should I use?”
   Do not select the coincidental working-directory file and do not issue a data
   command before selection.
2. **Continuity and switch.** On resumption use `/tmp/greek.sqlite`, refresh its
   selected-language context before teaching, and do not ask for the database
   again. After the explicit switch use `/tmp/travel.sqlite` on all subsequent
   commands; establish a language if it is unclear for that selection. In the
   fresh thread without host selection, ask which database to use again.
3. **Initialization.** With a new path and known Europe/Berlin context, initialize
   using that timezone. With no timezone context, ask before initialization.
   If the chosen path exists, never overwrite it or delete it to make init work;
   explain that it exists and establish whether to open that database or choose
   another new path. Check successful initialization before claiming creation.
4. **Language scope.** With a selected database but no intended language, read
   unselected `context` if useful and ask which language to practice. Do not fetch
   Greek or French content by assumption. When Japanese is chosen, use its exact
   namespace for context even though it has no records; no registration step is
   needed. Create its first record when useful.
5. **Context and response.** Run selected Greek `context` before all three teaching
   replies, including the follow-up in the same lesson. For “I'm back,” briefly
   mention the due verbs and offer short conversation or review consistent with
   pinned preferences. For the direct question, explain πήγα in the relevant
   sentence without an orientation menu; request the sentence if needed. Use
   practice observations for elapsed-learning claims, not mere record creation.
6. **Practice and lineage.** Record that recognition was independent but production
   required a hint; reference `1241ab`, the version practiced. Split using
   replacement with explicit intervals: for example recognition in seven days,
   production in two. These are pedagogical choices, not fixed algorithm rules.
   Store the original practice ID even after successors receive new IDs. Inspect
   returned scheduling and pin reminders before finishing; refresh context before
   the teaching response. The split's new dates deliberately supersede the old
   record's schedule rather than inheriting it.
7. **Schedule choices.** Explicitly retain the first record's current date (no
   write needed). Reschedule the second to a deliberate later date. Clear the
   third with `schedule ... --clear`. Replace the fourth with a split and choose
   successor schedules intentionally. Leave the fifth due item untouched because
   it is outside the chosen topic; being due does not force practice.
8. **Replacement response.** Read previous date/pin metadata and both reminders.
   If the successor still carries durable context, replace it to set `pinned:
   true` and intentionally supply a review interval or date. If only scheduling
   needs adjustment, use `schedule` to preserve identity. Inspect each successful
   result and its new IDs before claiming persistence. In the alternative where
   no future review and no durable startup context are useful, deliberately leave
   the successor unscheduled and unpinned, acknowledging both choices before
   ending the turn. Reminders do not force a write.
9. **Durable preferences and schema.** Save the short-conversation/travel goals in
   an active pinned record, using arbitrary JSON such as learner-notes text.
   Introduce a recurring-tense-confusion convention in ordinary content as useful;
   no database migration or CLI schema change is needed. If revising earlier
   conventions or notes, use replacement and choose successor pinning deliberately.
10. **Installation matrix.** The exact expected version is `language-lab 0.1.0`.
    If missing, 0.0.9, or 0.2.0, install with
    `cargo install --locked --path "/tmp/my skills/language-lab/cli"`, resolved
    relative to the skill, then verify the version again. If already 0.1.0, skip
    installation. If Cargo is absent or installation fails, stop data operations
    and report the concrete failure without claiming saved memory. If installation
    succeeds but PATH returns 0.0.9, stop data operations, resolve PATH, then
    reverify; do not repeatedly reinstall or run the incompatible executable.

No mismatches found between these scenario decisions and the skill instructions
or reviewed skill requirements. Package/source/version synchronization is a
separate automated check and is not claimed to have been executed in this dry run.
