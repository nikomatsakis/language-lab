# Replacement and retirement

`replace` takes an object with `ids` and `replacements` arrays. Each replacement entry contains a required `content`, optional boolean `pinned`, and optional `review`. The CLI owns all other envelope fields.

> **`A-replace-types`** (accepted): **Replace types**
> Property: Predecessor IDs must be strings, and each replacement entry must be an object with required `content` (including null as a valid value), optional boolean `pinned`, and optional `review`.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Submit missing fields, wrong types, duplicate or missing predecessors, conflicting review modes, and foreign or retired IDs. Abort event insertion with a SQLite trigger and compare the full context with its baseline. Expected outcome follows the property above.

## Validation and atomicity

> **`A-replace-input`** (accepted): **Replace input**
> Property: The CLI must require both `ids` and `replacements` arrays in a replacement request.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Submit missing fields, wrong types, duplicate or missing predecessors, conflicting review modes, and foreign or retired IDs. Abort event insertion with a SQLite trigger and compare the full context with its baseline. Expected outcome follows the property above.

> **`A-replace-nonempty`** (accepted): **Replace nonempty**
> Property: A replacement request with both arrays empty must fail.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Submit missing fields, wrong types, duplicate or missing predecessors, conflicting review modes, and foreign or retired IDs. Abort event insertion with a SQLite trigger and compare the full context with its baseline. Expected outcome follows the property above.

> **`A-replace-unique-inputs`** (accepted): **Replace unique inputs**
> Property: A replacement request containing the same predecessor ID more than once must fail.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Submit missing fields, wrong types, duplicate or missing predecessors, conflicting review modes, and foreign or retired IDs. Abort event insertion with a SQLite trigger and compare the full context with its baseline. Expected outcome follows the property above.

> **`A-replace-active-inputs`** (accepted): **Replace active inputs**
> Property: Every predecessor must exist and be active in the selected language at the time the transaction commits.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Submit missing fields, wrong types, duplicate or missing predecessors, conflicting review modes, and foreign or retired IDs. Abort event insertion with a SQLite trigger and compare the full context with its baseline. Expected outcome follows the property above.

> **`A-replace-atomic`** (accepted): **Replace atomic**
> Property: Validation failure or storage failure must leave all predecessor records, schedules, and proposed successors unchanged.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Submit missing fields, wrong types, duplicate or missing predecessors, conflicting review modes, and foreign or retired IDs. Abort event insertion with a SQLite trigger and compare the full context with its baseline. Expected outcome follows the property above.

> **`A-replace-concurrent`** (accepted): **Replace concurrent**
> Property: Concurrent replacements of the same active predecessor must permit at most one successful replacement.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Release two threads against the same active predecessor using a barrier. Assert exactly one successful replacement and a record_retired error for the loser. Expected outcome follows the property above.

> **`A-replace-fresh-ids`** (accepted): **Replace fresh ids**
> Property: Every successor must receive a fresh ID, including a one-to-one rewrite with identical content.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-backlinks`** (accepted): **Replace backlinks**
> Property: Each successor’s `replaces` array must contain all and only the predecessor IDs supplied in the request, in input order.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-retire`** (accepted): **Replace retire**
> Property: A successful replacement must retire every predecessor with the transaction’s UTC timestamp.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-event`** (accepted): **Replace event**
> Property: Every successful replacement must durably record its timestamp, predecessor IDs, and successor IDs, including operations with no successors.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

These rules give creation, rewriting, splitting, merging, and deletion the same operation. Since inputs must already exist and successors are fresh, predecessor links cannot form cycles.

## Scheduling and pinning handoff

> **`A-replace-no-inheritance`** (accepted): **Replace no inheritance**
> Property: A successor must not inherit a predecessor’s review date or pin state implicitly.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-cancel-review`** (accepted): **Replace cancel review**
> Property: Retiring a predecessor must remove its pending review from active scheduling while retaining the previous date in the replacement event; the retired record’s `review_on` must be null.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create pinned, due French content and query an empty Greek namespace, then attempt a mixed-language replacement. Inspect unchanged records, the retirement event, and the global language list. Expected outcome follows the property above.

> **`A-replace-successor-review`** (accepted): **Replace successor review**
> Property: A review supplied on a replacement entry must be applied to that successor within the replacement transaction.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-unscheduled-valid`** (accepted): **Replace unscheduled valid**
> Property: Omitting a successor review, or supplying `review: null`, must create an unscheduled successor without failing the operation.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-response-created`** (accepted): **Replace response created**
> Property: A successful replacement response must return successor IDs in replacement-entry order in `created`.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-response-retired`** (accepted): **Replace response retired**
> Property: A successful replacement response must contain `retired` in predecessor order, with `id`, `previous_review_on`, and `previous_pinned` for each predecessor.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-response-unscheduled`** (accepted): **Replace response unscheduled**
> Property: A successful replacement response must include `needs_review_schedule`, containing exactly the unscheduled successor IDs.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-response-review-reminder`** (accepted): **Replace response review reminder**
> Property: When at least one predecessor had a review and at least one successor is unscheduled, the response must include a reminder to choose successor review dates.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-replace-response-pin-reminder`** (accepted): **Replace response pin reminder**
> Property: When at least one predecessor was pinned and successors exist, the response must include a reminder to consider successor pinning.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

Deletion has no successors and therefore no request to schedule or pin successors. A reminder is informational; it does not turn a successful operation into an error.

> **`A-replace-response-reminders`** (accepted): **Replace response reminders**
> Property: Every successful replacement response must contain a `reminders` array of informational strings, empty when no reminder is needed.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

## Example response

```json
{
  "created": ["ab231f", "cd456a"],
  "retired": [
    {
      "id": "1241ab",
      "previous_review_on": "2026-10-20",
      "previous_pinned": false
    }
  ],
  "needs_review_schedule": ["cd456a"],
  "reminders": ["Choose review dates for unscheduled successors."]
}
```

Practice observations remain ordinary records. Replacing the learning record they describe does not rewrite their JSON references; see opaque content (`A-record-content-opaque`).
