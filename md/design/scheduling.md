# Time and review scheduling

A review belongs to a specific record ID. Content replacement creates new identity; rescheduling preserves identity. Calendar-day intervals intentionally avoid requiring the tutor to perform date arithmetic.

## Clock and dates

{req}`time.instant` Each command must sample a single current instant used consistently for its timestamps and relative-date calculations.

{req}`time.today` The CLI must derive today’s date from that instant in the database’s timezone.

{req}`time.relative` `in_days: N` and `--in-days N` must resolve by adding N calendar days to today in that timezone.

{req}`time.interval-domain` Relative intervals must be nonnegative integers; zero must mean today.

{req}`time.date-format` Explicit dates must be valid Gregorian calendar dates in `YYYY-MM-DD` form, with years 0001 through 9999.

{req}`time.overflow` An interval whose resulting date falls outside years 0001 through 9999 must fail without mutation.

{req}`time.due` An active scheduled record must be due exactly when its review date is on or before today.

A calendar day is not a fixed 24-hour duration. Tests should cover daylight-saving changes and leap days. Past explicit dates are valid and immediately due.

## Schedule operations

{req}`schedule.one-date` Each active record must have at most one pending review date.

{req}`schedule.mode` `schedule` must require exactly one of `--in-days`, `--on`, and `--clear`.

{req}`schedule.active` Scheduling must fail if any supplied ID is missing, repeated, retired, or outside the selected language.

{req}`schedule.atomic` Scheduling multiple IDs must update all of them atomically or leave all unchanged.

{req}`schedule.identity` Setting or clearing a review date must preserve the record’s ID and immutable fields.

{req}`schedule.overwrite` Setting a review date must replace any existing pending date for that record.

{req}`schedule.clear` Clearing a review must leave the record unscheduled, including when it was already unscheduled.

{req}`schedule.response` Scheduling must return a `records` array in input order, each entry containing `id` and its resolved `review_on` date, or null when cleared.

{req}`schedule.entry` A non-null replacement `review` must contain exactly one of `in_days` or `on`, with the same validation as the corresponding schedule option.

{req}`schedule.no-auto-review` Reading a due record or creating a practice observation must not automatically clear or change a review date.

The tutor decides whether to reschedule, clear, or replace after practice. The CLI does not infer mastery or select intervals.
