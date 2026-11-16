# Tutor skill

The skill is the teaching policy supplied to the model. The CLI supplies storage and time. These requirements specify instructions shipped in the skill. Scenario-based reviews test how those instructions apply; they do not guarantee compliance by every host or model.

## Selection and continuity

> **`A-skill-database-choice`** (accepted): **Skill database choice**
> Property: If no database has been selected in the conversation, the skill must instruct the tutor to ask the learner to select an existing database or choose a path for a new one before issuing data commands.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider a fresh conversation with no chosen database and a convenient existing file. The instructions must require the learner’s selection before data operations. Expected outcome follows the property above.

> **`A-skill-reuse-choice`** (accepted): **Skill reuse choice**
> Property: Once selected, the skill must instruct the tutor to reuse that database path until the learner changes it.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider two successive turns followed by an explicit database change. Check that the selected path persists until the learner changes it. Expected outcome follows the property above.

> **`A-skill-new-timezone`** (accepted): **Skill new timezone**
> Property: Before creating a database, the skill must instruct the tutor to establish the learner’s timezone from available context or by asking.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider creating a database with and without a known learner timezone. Require an established timezone before init. Expected outcome follows the property above.

> **`A-skill-language-choice`** (accepted): **Skill language choice**
> Property: Before retrieving language-specific content, the skill must instruct the tutor to establish the intended language from the conversation or by asking.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider an ambiguous language request and a clearly named new language. Require an established language before loading its content. Expected outcome follows the property above.

> **`A-skill-context`** (accepted): **Skill context**
> Property: Before each teaching response while the skill is active and a database and language are selected, the skill must instruct the tutor to call `context` for that selection.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Walk through a fresh chat, an old chat, a direct question, and a second teaching turn. Each selected-language teaching reply must first refresh context. Expected outcome follows the property above.

> **`A-skill-resume`** (accepted): **Skill resume**
> Property: When the learner returns without a specific task, the skill must instruct the tutor to use available context to offer a brief orientation and ways to proceed.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider a returning learner with pinned goals and due material but no specific request. Require a brief orientation informed by context. Expected outcome follows the property above.

> **`A-skill-direct-question`** (accepted): **Skill direct question**
> Property: The skill must instruct the tutor to address a specific learner question without requiring completion of a separate orientation menu.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider a learner asking about a word while reviews are due. Require answering the question without a mandatory orientation menu. Expected outcome follows the property above.

A new thread requires selecting the database again unless the host supplies that selection. Persistent learning memory belongs to the database, not the thread. Hooks are optional future support for context refresh.

## Learning memory

> **`A-skill-practice-evidence`** (accepted): **Skill practice evidence**
> Property: When recording practice, the skill must instruct the tutor to describe what the learner demonstrated and any hints that materially affected the result.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Compare independent recall with a correct answer after a hint. Require observations to preserve that distinction. Expected outcome follows the property above.

> **`A-skill-practice-reference`** (accepted): **Skill practice reference**
> Property: The skill must instruct the tutor to make a practice observation about an existing record reference the ID actually practiced, even if that record is subsequently replaced.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Practice an existing record, then split it. Require the observation to reference the original practiced ID. Expected outcome follows the property above.

> **`A-skill-schedule-choice`** (accepted): **Skill schedule choice**
> Property: After practice on a scheduled record, the skill must instruct the tutor to explicitly choose to retain, change, clear, or replace its schedule.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Practice a scheduled record and examine retain, reschedule, clear, and replace outcomes. Require a deliberate choice for each. Expected outcome follows the property above.

> **`A-skill-replacement-reminders`** (accepted): **Skill replacement reminders**
> Property: The skill must instruct the tutor to consider replacement scheduling and pinning reminders before finishing the learning turn.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Replace a pinned scheduled predecessor with unscheduled successors. Require consideration of both schedule and pin reminders. Expected outcome follows the property above.

> **`A-skill-pin-context`** (accepted): **Skill pin context**
> Property: The skill must instruct the tutor to represent durable learner goals and teaching preferences needed across conversations in active pinned records.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Introduce a durable teaching preference and then start a fresh conversation. Require storage as active pinned context. Expected outcome follows the property above.

> **`A-skill-no-taxonomy`** (accepted): **Skill no taxonomy**
> Property: The skill must permit the tutor to introduce and revise JSON content conventions without requiring a CLI schema change.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Introduce a new JSON convention for recognition versus production. Check that the skill permits it without a storage schema change. Expected outcome follows the property above.

“Consider” means make a deliberate choice, including leaving a successor unscheduled. Reviewing a due record is not mandatory when the learner wants another activity. Conventions such as `kind`, `text`, and `related_to` are suggestions rather than a closed schema.

## Distribution and installation

> **`A-skill-package`** (accepted): **Skill package**
> Property: The root `language-lab` Cargo package must include its CLI source, lockfile, `SYMPOSIUM.toml`, and `skills/language-lab/SKILL.md` in its distributable archive and build independently of the repository workspace.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Run the packaging test, inspect archive contents, and build the extracted crate independently of the workspace. Expected outcome follows the property above.

> **`A-skill-version-check`** (accepted): **Skill version check**
> Property: Before using the CLI, the skill must instruct the tutor to compare `cargo agents lab --version` against the exact package version expected by the skill.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider matching, older, and newer binary versions. Require exact agreement before data operations. Expected outcome follows the property above.

> **`A-skill-install`** (accepted): **Skill install**
> Property: The skill must instruct the tutor to invoke the Symposium-managed `cargo agents lab` command, which resolves its declared installation, and to request package setup or update if the command is unavailable or its version differs.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider a missing managed command. Require Symposium setup or update and invocation through cargo agents lab. Expected outcome follows the property above.

> **`A-skill-install-verify`** (accepted): **Skill install verify**
> Property: After installation, the skill must instruct the tutor to verify the exact binary version and stop data operations if installation fails or the expected version is not available.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Consider failed installation or a version mismatch after updating. Require another exact version check and halted data operations. Expected outcome follows the property above.

> **`A-skill-version-sync`** (accepted): **Skill version sync**
> Property: The version expected by the skill must match the root Cargo package version.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Compare the expected version in the skill with the package version in Cargo.toml. Expected outcome follows the property above.

> **`A-skill-symposium`** (accepted): **Skill symposium**
> Property: The Symposium manifest must expose the tutor skill and a `lab` subcommand backed by an installation of the `language-lab` crate from this repository, with the installation version matching the Cargo package version.
> Authority: `R-tutor-continuity`
> - Criterion `behavior` [review]: Inspect the Symposium manifest’s skill source, lab command, Git installation, crate name, and package version agreement. Expected outcome follows the property above.
