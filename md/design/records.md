# Database and records

## Storage and scope

> **`A-database-sqlite`** (accepted): **Database sqlite**
> Property: The CLI must persist its durable learning state in a SQLite database file at the selected path.
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

> **`A-database-bundled`** (accepted): **Database bundled**
> Property: The distributed CLI executable must operate without a separately installed SQLite library or executable.
> Authority: `R-local-memory`
> - Criterion `behavior` [review]: Inspect Cargo feature resolution for rusqlite bundled, then run the packaged CLI in an environment without a SQLite executable or dynamically loaded SQLite dependency. Expected outcome follows the property above.

> **`A-database-init-exclusive`** (accepted): **Database init exclusive**
> Property: Initializing an existing path must fail without modifying its contents.
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

> **`A-database-open-existing`** (accepted): **Database open existing**
> Property: Data commands other than `init` must fail when the selected database does not exist, rather than creating it.
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

> **`A-database-timezone`** (accepted): **Database timezone**
> Property: Initialization must persist a valid IANA timezone identifier supplied through `--timezone`.
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

> **`A-database-version`** (accepted): **Database version**
> Property: Opening an unsupported database schema version must fail without modifying the database.
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

> **`A-database-single-learner`** (accepted): **Database single learner**
> Property: The database must not require a user identifier for any operation.
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

Temporary SQLite journals are permitted; “one file” describes durable storage when commands have completed normally.

> **`A-database-init-failure`** (accepted): **Database init failure**
> Property: Failed initialization must remove any newly created partial database and must never remove or modify a preexisting path.
> Authority: `R-local-memory`
> - Criterion `behavior` [review]: Inspect and execute the injected initialization-failure test. Check cleanup after schema creation fails and preservation of preexisting files. Expected outcome follows the property above.

> **`A-database-invalid`** (accepted): **Database invalid**
> Property: Opening a file that is not a Language Lab database must fail without modifying the file.
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

## Languages

> **`A-language-namespace`** (accepted): **Language namespace**
> Property: Every record must belong to exactly one language namespace.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-language-exact`** (accepted): **Language exact**
> Property: Language names must be nonempty strings compared exactly, without implicit case folding or alias conversion; an explicitly empty `--language` must fail even when querying an empty database.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-language-implicit`** (accepted): **Language implicit**
> Property: Creating the first record in a namespace must make that namespace available without a separate language-registration operation.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-language-isolation`** (accepted): **Language isolation**
> Property: A language-scoped command must not return record content from or mutate records in another namespace; the global language-name list in context is permitted.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create pinned, due French content and query an empty Greek namespace, then attempt a mixed-language replacement. Inspect unchanged records, the retirement event, and the global language list. Expected outcome follows the property above.

> **`A-language-empty`** (accepted): **Language empty**
> Property: Querying a namespace with no records must return empty results rather than fail.
> Authority: `R-scoped-context`
> - Criterion `behavior` [test]: Create pinned, due French content and query an empty Greek namespace, then attempt a mixed-language replacement. Inspect unchanged records, the retirement event, and the global language list. Expected outcome follows the property above.

Use conventional tags such as `el` by agreement with the tutor. The CLI does not maintain a language taxonomy.

## Identity and content

> **`A-record-identity-unique`** (accepted): **Record identity unique**
> Property: Each record ID must be unique within its database, including IDs of retired records.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-identity-format`** (accepted): **Record identity format**
> Property: Generated IDs must consist of at least six lowercase hexadecimal characters.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-identity-collision`** (accepted): **Record identity collision**
> Property: An ID collision must cause another ID to be generated rather than overwrite an existing record.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-content-json`** (accepted): **Record content json**
> Property: The `content` field must accept any JSON value, including objects, arrays, strings, numbers, booleans, and null.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-content-opaque`** (accepted): **Record content opaque**
> Property: The CLI must assign no application semantics to keys or references within `content`.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-content-roundtrip`** (accepted): **Record content roundtrip**
> Property: Reading a stored record must preserve its JSON value, without requiring preservation of input whitespace or object-key ordering.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-content-immutable`** (accepted): **Record content immutable**
> Property: Once an ID has been created, its language, content, creation timestamp, and predecessor links must not change.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-record-created`** (accepted): **Record created**
> Property: The CLI must assign each new record a UTC RFC 3339 creation timestamp using its clock.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-envelope`** (accepted): **Record envelope**
> Property: A retrieved record must expose `id`, `language`, `created_at`, `replaces`, `content`, `pinned`, `review_on`, and `retired_at`.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-active`** (accepted): **Record active**
> Property: A record must be active exactly when `retired_at` is null.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-record-history-readable`** (accepted): **Record history readable**
> Property: Fetching a retired ID must return its original content and predecessor links.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

## Pinning

> **`A-record-pin-default`** (accepted): **Record pin default**
> Property: A new record must default to `pinned: false` when pinning is omitted.
> Authority: `R-adaptable-memory`
> - Criterion `behavior` [test]: Store null, booleans, numbers, Unicode, arrays, nested objects, and opaque references in distinct el and EL namespaces. Inject repeated active and retired IDs and inspect retrieved envelopes. Expected outcome follows the property above.

> **`A-record-pin-explicit`** (accepted): **Record pin explicit**
> Property: A replacement entry with `pinned: true` must create a pinned successor.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

> **`A-record-pin-immutable`** (accepted): **Record pin immutable**
> Property: Changing the pin state of a record must require replacement with a new ID.
> Authority: `R-memory-history`
> - Criterion `behavior` [test]: Create a pinned scheduled record, split it, merge successors in reversed order, rewrite identical content, and retire the result. Inspect predecessor history, successor envelopes, response order, reminders, and persisted events. Expected outcome follows the property above.

Pinning is independent of content and review scheduling. A pinned record may also be scheduled. Content fields such as `kind: "learner-notes"` do not automatically pin it.
