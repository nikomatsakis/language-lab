# CLI contract

The executable is named `language-lab`. The renderer is a separate executable named `mdbook-language-lab`.

## Command surface

```text
language-lab init PATH --timezone ZONE
language-lab --db PATH [--language LANG] context [--limit N]
language-lab --db PATH --language LANG get ID...
language-lab --db PATH --language LANG list [--due | --retired]
    [--since DATE] [--search TEXT] [--limit N] [--cursor TOKEN]
language-lab --db PATH --language LANG replace --file FILE
language-lab --db PATH --language LANG schedule ID...
    (--in-days N | --on DATE | --clear)
```

{req}`cli.database-explicit` Every data command other than initialization must require an explicit `--db` path.

{req}`cli.language-required` `get`, `list`, `replace`, and `schedule` must require `--language`.

{req}`cli.no-prompt` The CLI must not prompt interactively for database selection, language selection, or confirmation.

{req}`cli.json-output` A successful data command must emit exactly one JSON value on stdout.

{req}`cli.success-status` A successful command must exit with status zero.

{req}`cli.error` A failed command must exit nonzero and emit a JSON object containing `error.code` and `error.message` on stderr, leaving stdout empty.

{req}`cli.file-input` `replace --file PATH` must read one UTF-8 JSON request from that file.

{req}`cli.stdin-input` `replace --file -` must read one UTF-8 JSON request from stdin.

{req}`cli.strict-envelope` Unknown fields in CLI-owned request objects must be rejected; keys inside `content` must remain unrestricted.

{req}`cli.help` `--help` must document the available subcommands and each subcommand’s accepted arguments.

{req}`cli.no-extra-operands` Missing required arguments, unknown options, and unexpected positional arguments must fail without database mutation.

Help output is human-readable and exempt from the data-command JSON contract. Error codes should distinguish invalid input, missing database, unsupported schema, missing record, retired record, and storage failure; wording may evolve.

## Response shapes

`get` returns a `records` array. `list` returns a `records` array and `next_cursor`. `context` returns clock and language information and, when selected, `language`, `pinned`, `due`, and `recent`; due and recent each use the list response shape. `init` returns the selected database path and timezone. Mutation responses follow {replace.response.created}`replacement` and {schedule.response}`scheduling` requirements.

The CLI need not offer a generic SQL interface, a separate practice command, or a notebook editor. Its public contract is the small set of operations above.
