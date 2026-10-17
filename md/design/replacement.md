# Replacement and retirement

`replace` takes an object with `ids` and `replacements` arrays. Each replacement entry contains a required `content`, optional boolean `pinned`, and optional `review`. The CLI owns all other envelope fields.

## Validation and atomicity

{req}`replace.input` The CLI must require both `ids` and `replacements` arrays in a replacement request.

{req}`replace.nonempty` A replacement request with both arrays empty must fail.

{req}`replace.unique-inputs` A replacement request containing the same predecessor ID more than once must fail.

{req}`replace.active-inputs` Every predecessor must exist and be active in the selected language at the time the transaction commits.

{req}`replace.atomic` Validation failure or storage failure must leave all predecessor records, schedules, and proposed successors unchanged.

{req}`replace.concurrent` Concurrent replacements of the same active predecessor must permit at most one successful replacement.

{req}`replace.fresh-ids` Every successor must receive a fresh ID, including a one-to-one rewrite with identical content.

{req}`replace.backlinks` Each successor’s `replaces` array must contain all and only the predecessor IDs supplied in the request, in input order.

{req}`replace.retire` A successful replacement must retire every predecessor with the transaction’s UTC timestamp.

{req}`replace.event` Every successful replacement must durably record its timestamp, predecessor IDs, and successor IDs, including operations with no successors.

These rules give creation, rewriting, splitting, merging, and deletion the same operation. Since inputs must already exist and successors are fresh, predecessor links cannot form cycles.

## Scheduling and pinning handoff

{req}`replace.no-inheritance` A successor must not inherit a predecessor’s review date or pin state implicitly.

{req}`replace.cancel-review` Retiring a predecessor must remove its pending review from active scheduling while retaining the previous date in the replacement event.

{req}`replace.successor-review` A review supplied on a replacement entry must be applied to that successor within the replacement transaction.

{req}`replace.unscheduled-valid` Omitting a successor review, or supplying `review: null`, must create an unscheduled successor without failing the operation.

{req}`replace.response.created` A successful replacement response must return successor IDs in replacement-entry order.

{req}`replace.response.retired` A successful replacement response must report each predecessor’s ID, previous review date, and previous pin state.

{req}`replace.response.unscheduled` A successful replacement response must include `needs_review_schedule`, containing exactly the unscheduled successor IDs.

{req}`replace.response.review-reminder` When at least one predecessor had a review and at least one successor is unscheduled, the response must include a reminder to choose successor review dates.

{req}`replace.response.pin-reminder` When at least one predecessor was pinned and successors exist, the response must include a reminder to consider successor pinning.

Deletion has no successors and therefore no request to schedule or pin successors. A reminder is informational; it does not turn a successful operation into an error.

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

Practice observations remain ordinary records. Replacing the learning record they describe does not rewrite their JSON references; see {record.content.opaque}`opaque content`.
