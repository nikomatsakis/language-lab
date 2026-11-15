# Time and review scheduling

A review belongs to a specific record ID. Content replacement creates new identity; rescheduling preserves identity. Calendar-day intervals intentionally avoid requiring the tutor to perform date arithmetic.

## Clock and dates

> **`A-time-instant`** (accepted): **Time instant**
> Property: Each command must sample a single current instant used consistently for its timestamps and relative-date calculations.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [review]: Trace the CLI entry point to its single Utc::now call and Database::open argument. Check that replacement timestamps, context, and relative-date resolution use that stored instant. Expected outcome follows the property above.

> **`A-time-today`** (accepted): **Time today**
> Property: The CLI must derive today’s date from that instant in the database’s timezone.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-time-relative`** (accepted): **Time relative**
> Property: `in_days: N` and `--in-days N` must resolve by adding N calendar days to today in that timezone.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-time-interval-domain`** (accepted): **Time interval domain**
> Property: Relative intervals must be nonnegative integers; zero must mean today.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-time-date-format`** (accepted): **Time date format**
> Property: Explicit dates must be valid Gregorian calendar dates in `YYYY-MM-DD` form, with years 0001 through 9999.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-time-overflow`** (accepted): **Time overflow**
> Property: An interval whose resulting date falls outside years 0001 through 9999 must fail without mutation.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-time-due`** (accepted): **Time due**
> Property: An active scheduled record must be due exactly when its review date is on or before today.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

A calendar day is not a fixed 24-hour duration. Tests should cover daylight-saving changes and leap days. Past explicit dates are valid and immediately due.

## Schedule operations

> **`A-schedule-one-date`** (accepted): **Schedule one date**
> Property: Each active record must have at most one pending review date.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-schedule-mode`** (accepted): **Schedule mode**
> Property: `schedule` must require exactly one of `--in-days`, `--on`, and `--clear`.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-schedule-active`** (accepted): **Schedule active**
> Property: Scheduling must fail if any supplied ID is missing, repeated, retired, or outside the selected language.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Schedule two IDs in reversed order, reject duplicate, missing, foreign, and retired IDs, and inject an update failure. Read due context and add a practice observation; compare original records and schedules. Expected outcome follows the property above.

> **`A-schedule-atomic`** (accepted): **Schedule atomic**
> Property: Scheduling multiple IDs must update all of them atomically or leave all unchanged.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Schedule two IDs in reversed order, reject duplicate, missing, foreign, and retired IDs, and inject an update failure. Read due context and add a practice observation; compare original records and schedules. Expected outcome follows the property above.

> **`A-schedule-identity`** (accepted): **Schedule identity**
> Property: Setting or clearing a review date must preserve the record’s ID and immutable fields.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-schedule-overwrite`** (accepted): **Schedule overwrite**
> Property: Setting a review date must replace any existing pending date for that record.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-schedule-clear`** (accepted): **Schedule clear**
> Property: Clearing a review must leave the record unscheduled, including when it was already unscheduled.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Use fixed instants across Berlin daylight-saving transitions, a leap day, and Apia’s skipped date. Set zero-day, one-day, explicit boundary-year, invalid, and overflowing schedules; clear twice and compare original identity. Expected outcome follows the property above.

> **`A-schedule-response`** (accepted): **Schedule response**
> Property: Scheduling must return a `records` array in input order, each entry containing `id` and its resolved `review_on` date, or null when cleared.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Schedule two IDs in reversed order, reject duplicate, missing, foreign, and retired IDs, and inject an update failure. Read due context and add a practice observation; compare original records and schedules. Expected outcome follows the property above.

> **`A-schedule-entry`** (accepted): **Schedule entry**
> Property: A non-null replacement `review` must contain exactly one of `in_days` or `on`, with the same validation as the corresponding schedule option.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Submit null in_days, null on, both fields, and valid single-field review objects. Check deserialization and scheduling rejection without mutations. Expected outcome follows the property above.

> **`A-schedule-no-auto-review`** (accepted): **Schedule no auto review**
> Property: Reading a due record or creating a practice observation must not automatically clear or change a review date.
> Authority: `R-calendar-reviews`
> - Criterion `behavior` [test]: Schedule two IDs in reversed order, reject duplicate, missing, foreign, and retired IDs, and inject an update failure. Read due context and add a practice observation; compare original records and schedules. Expected outcome follows the property above.

The tutor decides whether to reschedule, clear, or replace after practice. The CLI does not infer mastery or select intervals.
