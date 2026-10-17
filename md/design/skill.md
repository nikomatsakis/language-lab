# Tutor skill

The skill is the teaching policy supplied to the model. The CLI supplies storage and time. These requirements specify instructions shipped in the skill. Scenario-based reviews test how those instructions apply; they do not guarantee compliance by every host or model.

## Selection and continuity

{req}`skill.database-choice` If no database has been selected in the conversation, the skill must instruct the tutor to ask the learner to select an existing database or choose a path for a new one before issuing data commands.

{req}`skill.reuse-choice` Once selected, the skill must instruct the tutor to reuse that database path until the learner changes it.

{req}`skill.new-timezone` Before creating a database, the skill must instruct the tutor to establish the learner’s timezone from available context or by asking.

{req}`skill.language-choice` Before retrieving language-specific content, the skill must instruct the tutor to establish the intended language from the conversation or by asking.

{req}`skill.context` Before each teaching response while the skill is active and a database and language are selected, the skill must instruct the tutor to call `context` for that selection.

{req}`skill.resume` When the learner returns without a specific task, the skill must instruct the tutor to use available context to offer a brief orientation and ways to proceed.

{req}`skill.direct-question` The skill must instruct the tutor to address a specific learner question without requiring completion of a separate orientation menu.

A new thread requires selecting the database again unless the host supplies that selection. Persistent learning memory belongs to the database, not the thread. Hooks are optional future support for context refresh.

## Learning memory

{req}`skill.practice-evidence` When recording practice, the skill must instruct the tutor to describe what the learner demonstrated and any hints that materially affected the result.

{req}`skill.practice-reference` The skill must instruct the tutor to make a practice observation about an existing record reference the ID actually practiced, even if that record is subsequently replaced.

{req}`skill.schedule-choice` After practice on a scheduled record, the skill must instruct the tutor to explicitly choose to retain, change, clear, or replace its schedule.

{req}`skill.replacement-reminders` The skill must instruct the tutor to consider replacement scheduling and pinning reminders before finishing the learning turn.

{req}`skill.pin-context` The skill must instruct the tutor to represent durable learner goals and teaching preferences needed across conversations in active pinned records.

{req}`skill.no-taxonomy` The skill must permit the tutor to introduce and revise JSON content conventions without requiring a CLI schema change.

“Consider” means make a deliberate choice, including leaving a successor unscheduled. Reviewing a due record is not mandatory when the learner wants another activity. Conventions such as `kind`, `text`, and `related_to` are suggestions rather than a closed schema.

## Distribution and installation

{req}`skill.package` The main tutor skill must live under `skills/language-lab/` and include a Cargo project at `skills/language-lab/cli/` with its lockfile and all source needed to build the CLI.

{req}`skill.version-check` Before using the CLI, the skill must instruct the tutor to compare `language-lab --version` against the exact package version expected by the skill.

{req}`skill.install` If the binary is missing or its version differs, the skill must instruct the tutor to run `cargo install --locked --path` pointing to its embedded project.

{req}`skill.install-verify` After installation, the skill must instruct the tutor to verify the exact binary version and stop data operations if installation fails or the expected version is not available.

{req}`skill.version-sync` The version expected by the skill must match the embedded Cargo package version.
