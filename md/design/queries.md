# Queries and context

## Exact retrieval

{req}`get.exact` `get ID...` must return records in requested order, including retired records.

{req}`get.missing` If any requested ID is missing or outside the selected language, `get` must fail rather than return a partial result.

## Listing

{req}`list.active-default` `list` must select only active records by default.

{req}`list.retired` `list --retired` must select only retired records.

{req}`list.due` `list --due` must select active records satisfying {time.due}`the due-date rule`.

{req}`list.incompatible` Combining `--due` and `--retired` must fail.

{req}`list.since` `--since DATE` must include records created at or after midnight on that date in the database’s timezone.

{req}`list.search` `--search TEXT` must match case-sensitive literal substrings of compact JSON content serialized with object keys recursively sorted lexicographically and Unicode characters unescaped.

{req}`list.intersection` Compatible list filters must be combined by intersection.

{req}`list.order` Lists without `--due` must sort by creation timestamp descending, then ID ascending.

{req}`list.due-order` Due lists must sort by review date ascending, then creation timestamp ascending, then ID ascending.

{req}`list.limit` `--limit` must accept integers from 1 through 1000, defaulting to 20.

{req}`list.pagination` Each list response must contain `records` and a `next_cursor`, which is null when no further matching records exist.

{req}`list.cursor` In an unchanged database, following cursors with the same namespace and filters must enumerate each matching record exactly once in the specified order.

{req}`list.cursor-invalid` A malformed cursor or a cursor used with different namespace or filters must fail with a structured error.

Pagination does not promise a snapshot across concurrent mutations; restart listing when a consistent fresh view is needed. The search definition is intentionally simple and reproducible, not linguistic or semantic search.

## Context

{req}`context.clock` `context` must return `now`, `today`, and `timezone` using {time.instant}`the command’s clock snapshot`.

{req}`context.languages` `context` must return sorted language names represented by any stored record, including retired records.

{req}`context.unselected` Without `--language`, `context` must return clock and language information without returning record content.

{req}`context.pinned` With a selected language, `context` must return every active pinned record in that language, ordered by creation timestamp descending and ID ascending.

{req}`context.due` With a selected language, `context` must return the first page of due records using the list ordering and limit.

{req}`context.recent` With a selected language, `context` must return the first page of active records using the default list ordering and limit.

{req}`context.continuation` The due and recent sections must each expose their matching list continuation cursor.

{req}`context.pin-limit` The context limit must not truncate pinned records or subtract them from due or recent page sizes.

{req}`context.read-only` Retrieving context must not modify learning records, schedules, or activity timestamps.

A record may appear in multiple sections; its ID establishes that it is the same record. Context does not claim a “last practice” time because it cannot interpret arbitrary `kind` values. The tutor can derive that from its own observations. Full pinned context is deliberate: the skill should keep it concise.
