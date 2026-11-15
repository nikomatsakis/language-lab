# CLI contract

The executable is named `language-lab`.

## Command surface

```text
language-lab --version
language-lab init PATH --timezone ZONE
language-lab --db PATH [--language LANG] context [--limit N]
language-lab --db PATH --language LANG get ID...
language-lab --db PATH --language LANG list [--due | --retired]
    [--since DATE] [--search TEXT] [--limit N] [--cursor TOKEN]
language-lab --db PATH --language LANG replace --file FILE
language-lab --db PATH --language LANG schedule ID...
    (--in-days N | --on DATE | --clear)
```

> **`A-cli-database-explicit`** (accepted): **Cli database explicit**
> Property: Every data command other than initialization must require an explicit `--db` path.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-language-required`** (accepted): **Cli language required**
> Property: `get`, `list`, `replace`, and `schedule` must require `--language`.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-no-prompt`** (accepted): **Cli no prompt**
> Property: The CLI must not prompt interactively for database selection, language selection, or confirmation.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-json-output`** (accepted): **Cli json output**
> Property: A successful data command must emit exactly one JSON value on stdout.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-success-status`** (accepted): **Cli success status**
> Property: A successful command must exit with status zero.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-error`** (accepted): **Cli error**
> Property: A failed command must exit nonzero and emit a JSON object containing `error.code` and `error.message` on stderr, leaving stdout empty.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-file-input`** (accepted): **Cli file input**
> Property: `replace --file PATH` must read one UTF-8 JSON request from that file.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-stdin-input`** (accepted): **Cli stdin input**
> Property: `replace --file -` must read one UTF-8 JSON request from stdin.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-strict-envelope`** (accepted): **Cli strict envelope**
> Property: Unknown fields in CLI-owned request objects must be rejected; keys inside `content` must remain unrestricted.
> Authority: `R-atomic-updates`
> - Criterion `behavior` [test]: Submit missing fields, wrong types, duplicate or missing predecessors, conflicting review modes, and foreign or retired IDs. Abort event insertion with a SQLite trigger and compare the full context with its baseline. Expected outcome follows the property above.

> **`A-cli-help`** (accepted): **Cli help**
> Property: `--help` must document the available subcommands and each subcommand’s accepted arguments.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-no-extra-operands`** (accepted): **Cli no extra operands**
> Property: Missing required arguments, unknown options, and unexpected positional arguments must fail without database mutation.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-version`** (accepted): **Cli version**
> Property: `language-lab --version` must print `language-lab VERSION`, with VERSION equal to the Cargo package version, and exit zero without opening a database.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-operands`** (accepted): **Cli operands**
> Property: `get` and `schedule` must require at least one ID.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-error-codes`** (accepted): **Cli error codes**
> Property: Error codes must distinguish `invalid_input`, `database_missing`, `database_invalid`, `schema_unsupported`, `record_missing`, `record_retired`, and `storage_error`; invalid arguments, JSON, ranges, or cursors use `invalid_input`, and IDs outside the selected language use `record_missing`.
> Authority: `R-agent-interface`
> - Criterion `behavior` [test]: Invoke the compiled CLI with piped input. Exercise every subcommand, help and version, file and stdin JSON, missing operands, conflicting flags, invalid input, and missing or foreign records; inspect exit status, both output streams, and database bytes. Expected outcome follows the property above.

> **`A-cli-init-response`** (accepted): **Cli init response**
> Property: Successful initialization must return `database` (the supplied path) and `timezone` (the validated identifier).
> Authority: `R-local-memory`
> - Criterion `behavior` [test]: Initialize a temporary database, reopen persisted records, and try invalid zones, existing paths, unsupported schemas, empty files, and foreign SQLite files. Compare file bytes before and after rejected operations. Expected outcome follows the property above.

Help and version output are human-readable and exempt from the data-command JSON contract. Messages may evolve. Existing-path initialization and filesystem/SQLite failures use `storage_error`.

## Response shapes

`get` returns a `records` array. `list` returns a `records` array and `next_cursor`. `context` returns clock and language information and, when selected, `language`, `pinned`, `due`, and `recent`; due and recent each use the list response shape. `init` returns the selected database path and timezone. Mutation responses follow replacement (`A-replace-response-created`) and scheduling (`A-schedule-response`) requirements.

The CLI need not offer a generic SQL interface, a separate practice command, or a notebook editor. Its public contract is the small set of operations above.
