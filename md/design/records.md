# Database and records

## Storage and scope

{req}`database.sqlite` The CLI must persist its durable learning state in a SQLite database file at the selected path.

{req}`database.bundled` The distributed CLI executable must operate without a separately installed SQLite library or executable.

{req}`database.init.exclusive` Initializing an existing path must fail without modifying its contents.

{req}`database.open.existing` Commands other than `init` must fail when the selected database does not exist, rather than creating it.

{req}`database.timezone` Initialization must persist a valid IANA timezone identifier supplied through `--timezone`.

{req}`database.version` Opening an unsupported database schema version must fail without modifying the database.

{req}`database.single-learner` The database must not require a user identifier for any operation.

Temporary SQLite journals are permitted; “one file” describes durable storage when commands have completed normally.

## Languages

{req}`language.namespace` Every record must belong to exactly one language namespace.

{req}`language.exact` Language names must be nonempty strings compared exactly, without implicit case folding or alias conversion.

{req}`language.implicit` Creating the first record in a namespace must make that namespace available without a separate language-registration operation.

{req}`language.isolation` A language-scoped command must not read or mutate records in another namespace.

{req}`language.empty` Querying a namespace with no records must return empty results rather than fail.

Use conventional tags such as `el` by agreement with the tutor. The CLI does not maintain a language taxonomy.

## Identity and content

{req}`record.identity.unique` Each record ID must be unique within its database, including IDs of retired records.

{req}`record.identity.format` Generated IDs must consist of at least six lowercase hexadecimal characters.

{req}`record.identity.collision` An ID collision must cause another ID to be generated rather than overwrite an existing record.

{req}`record.content.json` The `content` field must accept any JSON value, including objects, arrays, strings, numbers, booleans, and null.

{req}`record.content.opaque` The CLI must assign no application semantics to keys or references within `content`.

{req}`record.content.roundtrip` Reading a stored record must preserve its JSON value, without requiring preservation of input whitespace or object-key ordering.

{req}`record.content.immutable` Once an ID has been created, its language, content, creation timestamp, and predecessor links must not change.

{req}`record.created` The CLI must assign each new record a UTC RFC 3339 creation timestamp using its clock.

{req}`record.envelope` A retrieved record must expose `id`, `language`, `created_at`, `replaces`, `content`, `pinned`, `review_on`, and `retired_at`.

{req}`record.active` A record must be active exactly when `retired_at` is null.

{req}`record.history.readable` Fetching a retired ID must return its original content and predecessor links.

## Pinning

{req}`record.pin.default` A new record must default to `pinned: false` when pinning is omitted.

{req}`record.pin.explicit` A replacement entry with `pinned: true` must create a pinned successor.

{req}`record.pin.immutable` Changing the pin state of a record must require replacement with a new ID.

Pinning is independent of content and review scheduling. A pinned record may also be scheduled. Content fields such as `kind: "learner-notes"` do not automatically pin it.
