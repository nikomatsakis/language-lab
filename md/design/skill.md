# Tutor skill

The skill is the teaching policy supplied to the model. The CLI supplies storage and time. These requirements describe observable behavior when the skill is active; they are not host-enforced guarantees about model behavior.

## Selection and continuity

{req}`skill.database-choice` If no database has been selected in the conversation, the skill must ask the learner to select an existing database or choose a path for a new one before issuing data commands.

{req}`skill.reuse-choice` Once selected, the skill must reuse that database path until the learner changes it.

{req}`skill.new-timezone` Before creating a database, the skill must establish the learner’s timezone from available context or by asking.

{req}`skill.language-choice` Before retrieving language-specific content, the skill must establish the intended language from the conversation or by asking.

{req}`skill.context` Before each teaching response while the skill is active and a database and language are selected, the skill must call `context` for that selection.

{req}`skill.resume` When the learner returns without a specific task, the tutor must use available context to offer a brief orientation and ways to proceed.

{req}`skill.direct-question` A specific learner question must not require completion of a separate orientation menu before the tutor addresses it.

A new thread requires selecting the database again unless the host supplies that selection. Persistent learning memory belongs to the database, not the thread. Hooks are optional future support for context refresh.

## Learning memory

{req}`skill.practice-evidence` When recording practice, the tutor must describe what the learner demonstrated and any hints that materially affected the result.

{req}`skill.practice-reference` A practice observation about an existing record must reference the ID actually practiced, even if that record is subsequently replaced.

{req}`skill.schedule-choice` After practice on a scheduled record, the tutor must explicitly choose to retain, change, clear, or replace its schedule.

{req}`skill.replacement-reminders` The tutor must consider replacement scheduling and pinning reminders before finishing the learning turn.

{req}`skill.pin-context` Durable learner goals and teaching preferences needed across conversations must be represented in active pinned records.

{req}`skill.no-taxonomy` The skill must permit the tutor to introduce and revise JSON content conventions without requiring a CLI schema change.

“Consider” means make a deliberate choice, including leaving a successor unscheduled. Reviewing a due record is not mandatory when the learner wants another activity. Conventions such as `kind`, `text`, and `related_to` are suggestions rather than a closed schema.
