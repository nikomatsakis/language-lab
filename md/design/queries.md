# Queries and context

## Exact retrieval

> **`A-get-exact`** (accepted): **Get exact**
> Property: `get ID...` must return a `records` array in requested order, including retired records and repetitions of requested IDs.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-get-missing`** (accepted): **Get missing**
> Property: If any requested ID is missing or outside the selected language, `get` must fail rather than return a partial result.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

## Listing

> **`A-list-active-default`** (accepted): **List active default**
> Property: `list` must select only active records by default.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-retired`** (accepted): **List retired**
> Property: `list --retired` must select only retired records.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-due`** (accepted): **List due**
> Property: `list --due` must select active records satisfying the due-date rule (`A-time-due`).
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-incompatible`** (accepted): **List incompatible**
> Property: Combining `--due` and `--retired` must fail.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-since`** (accepted): **List since**
> Property: `--since DATE` must include records whose creation timestamp, converted to the database timezone, has a local calendar date on or after DATE.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create records on opposite sides of local midnight, page them by creation order, and give them equal review dates. Check local-date inclusion and oldest-first due ordering. Expected outcome follows the property above.

> **`A-list-search`** (accepted): **List search**
> Property: `--search TEXT` must match case-sensitive literal substrings of compact JSON content serialized with object keys recursively sorted lexicographically and Unicode characters unescaped.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-intersection`** (accepted): **List intersection**
> Property: Compatible list filters must be combined by intersection.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-order`** (accepted): **List order**
> Property: Lists without `--due` must sort by creation timestamp descending, then ID ascending.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create records on opposite sides of local midnight, page them by creation order, and give them equal review dates. Check local-date inclusion and oldest-first due ordering. Expected outcome follows the property above.

> **`A-list-due-order`** (accepted): **List due order**
> Property: Due lists must sort by review date ascending, then creation timestamp ascending, then ID ascending.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create records on opposite sides of local midnight, page them by creation order, and give them equal review dates. Check local-date inclusion and oldest-first due ordering. Expected outcome follows the property above.

> **`A-list-limit`** (accepted): **List limit**
> Property: `--limit` must accept integers from 1 through 1000, defaulting to 20.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create 25 pinned, due records and call list and context without a limit. Assert 20 records per page, continuation cursors, and all 25 pinned records. Expected outcome follows the property above.

> **`A-list-pagination`** (accepted): **List pagination**
> Property: Each list response must contain `records` and a `next_cursor`, which is null when no further matching records exist.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-cursor`** (accepted): **List cursor**
> Property: In an unchanged database, following cursors with the same namespace and filters (with the original due cutoff retained even across midnight) must enumerate each matching record exactly once in the specified order.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-list-cursor-invalid`** (accepted): **List cursor invalid**
> Property: A malformed cursor or a cursor used with a different database, namespace, or filters must fail with a structured error.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

Page size may change between pages; it is not a filter. Due cursors retain their original local-date cutoff.

Pagination does not promise a snapshot across concurrent mutations; restart listing when a consistent fresh view is needed. The search definition is intentionally simple and reproducible, not linguistic or semantic search.

> **`A-list-cursor-database`** (accepted): **List cursor database**
> Property: A cursor must be bound to the database identity assigned at initialization; copying a database preserves that identity.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

## Context

> **`A-context-clock`** (accepted): **Context clock**
> Property: `context` must return `now`, `today`, and `timezone` using the command’s clock snapshot (`A-time-instant`).
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-languages`** (accepted): **Context languages**
> Property: `context` must return a `languages` array of sorted language names represented by any stored record, including retired records.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create pinned, due French content and query an empty Greek namespace, then attempt a mixed-language replacement. Inspect unchanged records, the retirement event, and the global language list. Expected outcome follows the property above.

> **`A-context-unselected`** (accepted): **Context unselected**
> Property: Without `--language`, `context` must return clock and language information without returning record content.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-pinned`** (accepted): **Context pinned**
> Property: With a selected language, `context` must return every active pinned record in that language, ordered by creation timestamp descending and ID ascending.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-due`** (accepted): **Context due**
> Property: With a selected language, `context` must return the first page of due records using the list ordering and limit.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-recent`** (accepted): **Context recent**
> Property: With a selected language, `context` must return the first page of active records using the default list ordering and limit.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-continuation`** (accepted): **Context continuation**
> Property: The due and recent sections must each expose their matching list continuation cursor.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-pin-limit`** (accepted): **Context pin limit**
> Property: The context limit must not truncate pinned records or subtract them from due or recent page sizes.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-read-only`** (accepted): **Context read only**
> Property: Retrieving context must not modify learning records, schedules, or activity timestamps.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-shape`** (accepted): **Context shape**
> Property: Selected context must expose `language`, `pinned` (record array), `due`, and `recent` (each with `records` and `next_cursor`); unselected context must omit these four keys.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create fixed-ID pinned, due, future, and ordinary records with case-distinct and Unicode content. Query filters and boundaries, traverse pages across midnight, reuse cursors against a copied or different database, retire a record, and compare selected and global context and file bytes. Expected outcome follows the property above.

> **`A-context-limit`** (accepted): **Context limit**
> Property: Context must accept the same limit domain and default as `list --limit`.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create 25 pinned, due records and call list and context without a limit. Assert 20 records per page, continuation cursors, and all 25 pinned records. Expected outcome follows the property above.

A record may appear in multiple sections; its ID establishes that it is the same record. Context does not claim a “last practice” time because it cannot interpret arbitrary `kind` values. The tutor can derive that from its own observations. Full pinned context is deliberate: the skill should keep it concise.
