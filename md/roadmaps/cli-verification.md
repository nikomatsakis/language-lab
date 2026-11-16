# CLI verification

**Roadmap identifier:** `cli-verification`

## Goal

Keep the local CLI contract executable through Anchor Book criteria and tests.
The assignment covers the existing CLI, storage, and packaged tutor instructions.
Hosted changes are tracked in issue #1.

## Milestone 1. Executable CLI criteria {#cli-verification-1}

**Identifier:** `cli-verification-1`

**Outcome:** Concrete scenarios and linked tests let Anchor Book report the current CLI verification result and fail CI on regressions.

**Status:** Complete

**Dependencies:** The pinned Anchor Book engine and nextest 0.9.151.

**Design basis:** The six local design chapters and their governing authority records.

**Tasks**

- [x] Replace generic criteria with concrete fixtures and observations.
- [x] Associate tests with their criteria.
- [x] Run the supported test verification.
- [x] Add verification to CI.

### Completion criteria

#### Time today {#cli-verification-1-time-today}

Execute the fixture scenarios specified by `A-time-today` and its `behavior` criterion.

- **Anchor coverage:** [A-time-today](../design/scheduling.md#A-time-today) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Time relative {#cli-verification-1-time-relative}

Execute the fixture scenarios specified by `A-time-relative` and its `behavior` criterion.

- **Anchor coverage:** [A-time-relative](../design/scheduling.md#A-time-relative) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Time interval domain {#cli-verification-1-time-interval-domain}

Execute the fixture scenarios specified by `A-time-interval-domain` and its `behavior` criterion.

- **Anchor coverage:** [A-time-interval-domain](../design/scheduling.md#A-time-interval-domain) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Time date format {#cli-verification-1-time-date-format}

Execute the fixture scenarios specified by `A-time-date-format` and its `behavior` criterion.

- **Anchor coverage:** [A-time-date-format](../design/scheduling.md#A-time-date-format) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Time overflow {#cli-verification-1-time-overflow}

Execute the fixture scenarios specified by `A-time-overflow` and its `behavior` criterion.

- **Anchor coverage:** [A-time-overflow](../design/scheduling.md#A-time-overflow) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Time due {#cli-verification-1-time-due}

Execute the fixture scenarios specified by `A-time-due` and its `behavior` criterion.

- **Anchor coverage:** [A-time-due](../design/scheduling.md#A-time-due) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule one date {#cli-verification-1-schedule-one-date}

Execute the fixture scenarios specified by `A-schedule-one-date` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-one-date](../design/scheduling.md#A-schedule-one-date) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule mode {#cli-verification-1-schedule-mode}

Execute the fixture scenarios specified by `A-schedule-mode` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-mode](../design/scheduling.md#A-schedule-mode) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule active {#cli-verification-1-schedule-active}

Execute the fixture scenarios specified by `A-schedule-active` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-active](../design/scheduling.md#A-schedule-active) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule atomic {#cli-verification-1-schedule-atomic}

Execute the fixture scenarios specified by `A-schedule-atomic` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-atomic](../design/scheduling.md#A-schedule-atomic) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule identity {#cli-verification-1-schedule-identity}

Execute the fixture scenarios specified by `A-schedule-identity` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-identity](../design/scheduling.md#A-schedule-identity) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule overwrite {#cli-verification-1-schedule-overwrite}

Execute the fixture scenarios specified by `A-schedule-overwrite` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-overwrite](../design/scheduling.md#A-schedule-overwrite) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule clear {#cli-verification-1-schedule-clear}

Execute the fixture scenarios specified by `A-schedule-clear` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-clear](../design/scheduling.md#A-schedule-clear) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule response {#cli-verification-1-schedule-response}

Execute the fixture scenarios specified by `A-schedule-response` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-response](../design/scheduling.md#A-schedule-response) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule entry {#cli-verification-1-schedule-entry}

Execute the fixture scenarios specified by `A-schedule-entry` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-entry](../design/scheduling.md#A-schedule-entry) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Schedule no auto review {#cli-verification-1-schedule-no-auto-review}

Execute the fixture scenarios specified by `A-schedule-no-auto-review` and its `behavior` criterion.

- **Anchor coverage:** [A-schedule-no-auto-review](../design/scheduling.md#A-schedule-no-auto-review) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Get exact {#cli-verification-1-get-exact}

Execute the fixture scenarios specified by `A-get-exact` and its `behavior` criterion.

- **Anchor coverage:** [A-get-exact](../design/queries.md#A-get-exact) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Get missing {#cli-verification-1-get-missing}

Execute the fixture scenarios specified by `A-get-missing` and its `behavior` criterion.

- **Anchor coverage:** [A-get-missing](../design/queries.md#A-get-missing) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List active default {#cli-verification-1-list-active-default}

Execute the fixture scenarios specified by `A-list-active-default` and its `behavior` criterion.

- **Anchor coverage:** [A-list-active-default](../design/queries.md#A-list-active-default) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List retired {#cli-verification-1-list-retired}

Execute the fixture scenarios specified by `A-list-retired` and its `behavior` criterion.

- **Anchor coverage:** [A-list-retired](../design/queries.md#A-list-retired) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List due {#cli-verification-1-list-due}

Execute the fixture scenarios specified by `A-list-due` and its `behavior` criterion.

- **Anchor coverage:** [A-list-due](../design/queries.md#A-list-due) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List incompatible {#cli-verification-1-list-incompatible}

Execute the fixture scenarios specified by `A-list-incompatible` and its `behavior` criterion.

- **Anchor coverage:** [A-list-incompatible](../design/queries.md#A-list-incompatible) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List since {#cli-verification-1-list-since}

Execute the fixture scenarios specified by `A-list-since` and its `behavior` criterion.

- **Anchor coverage:** [A-list-since](../design/queries.md#A-list-since) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List search {#cli-verification-1-list-search}

Execute the fixture scenarios specified by `A-list-search` and its `behavior` criterion.

- **Anchor coverage:** [A-list-search](../design/queries.md#A-list-search) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List intersection {#cli-verification-1-list-intersection}

Execute the fixture scenarios specified by `A-list-intersection` and its `behavior` criterion.

- **Anchor coverage:** [A-list-intersection](../design/queries.md#A-list-intersection) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List order {#cli-verification-1-list-order}

Execute the fixture scenarios specified by `A-list-order` and its `behavior` criterion.

- **Anchor coverage:** [A-list-order](../design/queries.md#A-list-order) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List due order {#cli-verification-1-list-due-order}

Execute the fixture scenarios specified by `A-list-due-order` and its `behavior` criterion.

- **Anchor coverage:** [A-list-due-order](../design/queries.md#A-list-due-order) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List limit {#cli-verification-1-list-limit}

Execute the fixture scenarios specified by `A-list-limit` and its `behavior` criterion.

- **Anchor coverage:** [A-list-limit](../design/queries.md#A-list-limit) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List pagination {#cli-verification-1-list-pagination}

Execute the fixture scenarios specified by `A-list-pagination` and its `behavior` criterion.

- **Anchor coverage:** [A-list-pagination](../design/queries.md#A-list-pagination) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List cursor {#cli-verification-1-list-cursor}

Execute the fixture scenarios specified by `A-list-cursor` and its `behavior` criterion.

- **Anchor coverage:** [A-list-cursor](../design/queries.md#A-list-cursor) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List cursor invalid {#cli-verification-1-list-cursor-invalid}

Execute the fixture scenarios specified by `A-list-cursor-invalid` and its `behavior` criterion.

- **Anchor coverage:** [A-list-cursor-invalid](../design/queries.md#A-list-cursor-invalid) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### List cursor database {#cli-verification-1-list-cursor-database}

Execute the fixture scenarios specified by `A-list-cursor-database` and its `behavior` criterion.

- **Anchor coverage:** [A-list-cursor-database](../design/queries.md#A-list-cursor-database) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context clock {#cli-verification-1-context-clock}

Execute the fixture scenarios specified by `A-context-clock` and its `behavior` criterion.

- **Anchor coverage:** [A-context-clock](../design/queries.md#A-context-clock) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context languages {#cli-verification-1-context-languages}

Execute the fixture scenarios specified by `A-context-languages` and its `behavior` criterion.

- **Anchor coverage:** [A-context-languages](../design/queries.md#A-context-languages) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context unselected {#cli-verification-1-context-unselected}

Execute the fixture scenarios specified by `A-context-unselected` and its `behavior` criterion.

- **Anchor coverage:** [A-context-unselected](../design/queries.md#A-context-unselected) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context pinned {#cli-verification-1-context-pinned}

Execute the fixture scenarios specified by `A-context-pinned` and its `behavior` criterion.

- **Anchor coverage:** [A-context-pinned](../design/queries.md#A-context-pinned) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context due {#cli-verification-1-context-due}

Execute the fixture scenarios specified by `A-context-due` and its `behavior` criterion.

- **Anchor coverage:** [A-context-due](../design/queries.md#A-context-due) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context recent {#cli-verification-1-context-recent}

Execute the fixture scenarios specified by `A-context-recent` and its `behavior` criterion.

- **Anchor coverage:** [A-context-recent](../design/queries.md#A-context-recent) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context continuation {#cli-verification-1-context-continuation}

Execute the fixture scenarios specified by `A-context-continuation` and its `behavior` criterion.

- **Anchor coverage:** [A-context-continuation](../design/queries.md#A-context-continuation) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context pin limit {#cli-verification-1-context-pin-limit}

Execute the fixture scenarios specified by `A-context-pin-limit` and its `behavior` criterion.

- **Anchor coverage:** [A-context-pin-limit](../design/queries.md#A-context-pin-limit) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context read only {#cli-verification-1-context-read-only}

Execute the fixture scenarios specified by `A-context-read-only` and its `behavior` criterion.

- **Anchor coverage:** [A-context-read-only](../design/queries.md#A-context-read-only) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context shape {#cli-verification-1-context-shape}

Execute the fixture scenarios specified by `A-context-shape` and its `behavior` criterion.

- **Anchor coverage:** [A-context-shape](../design/queries.md#A-context-shape) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Context limit {#cli-verification-1-context-limit}

Execute the fixture scenarios specified by `A-context-limit` and its `behavior` criterion.

- **Anchor coverage:** [A-context-limit](../design/queries.md#A-context-limit) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace types {#cli-verification-1-replace-types}

Execute the fixture scenarios specified by `A-replace-types` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-types](../design/replacement.md#A-replace-types) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace input {#cli-verification-1-replace-input}

Execute the fixture scenarios specified by `A-replace-input` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-input](../design/replacement.md#A-replace-input) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace nonempty {#cli-verification-1-replace-nonempty}

Execute the fixture scenarios specified by `A-replace-nonempty` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-nonempty](../design/replacement.md#A-replace-nonempty) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace unique inputs {#cli-verification-1-replace-unique-inputs}

Execute the fixture scenarios specified by `A-replace-unique-inputs` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-unique-inputs](../design/replacement.md#A-replace-unique-inputs) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace active inputs {#cli-verification-1-replace-active-inputs}

Execute the fixture scenarios specified by `A-replace-active-inputs` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-active-inputs](../design/replacement.md#A-replace-active-inputs) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace atomic {#cli-verification-1-replace-atomic}

Execute the fixture scenarios specified by `A-replace-atomic` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-atomic](../design/replacement.md#A-replace-atomic) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace concurrent {#cli-verification-1-replace-concurrent}

Execute the fixture scenarios specified by `A-replace-concurrent` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-concurrent](../design/replacement.md#A-replace-concurrent) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace fresh ids {#cli-verification-1-replace-fresh-ids}

Execute the fixture scenarios specified by `A-replace-fresh-ids` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-fresh-ids](../design/replacement.md#A-replace-fresh-ids) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace backlinks {#cli-verification-1-replace-backlinks}

Execute the fixture scenarios specified by `A-replace-backlinks` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-backlinks](../design/replacement.md#A-replace-backlinks) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace retire {#cli-verification-1-replace-retire}

Execute the fixture scenarios specified by `A-replace-retire` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-retire](../design/replacement.md#A-replace-retire) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace event {#cli-verification-1-replace-event}

Execute the fixture scenarios specified by `A-replace-event` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-event](../design/replacement.md#A-replace-event) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace no inheritance {#cli-verification-1-replace-no-inheritance}

Execute the fixture scenarios specified by `A-replace-no-inheritance` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-no-inheritance](../design/replacement.md#A-replace-no-inheritance) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace cancel review {#cli-verification-1-replace-cancel-review}

Execute the fixture scenarios specified by `A-replace-cancel-review` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-cancel-review](../design/replacement.md#A-replace-cancel-review) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace successor review {#cli-verification-1-replace-successor-review}

Execute the fixture scenarios specified by `A-replace-successor-review` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-successor-review](../design/replacement.md#A-replace-successor-review) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace unscheduled valid {#cli-verification-1-replace-unscheduled-valid}

Execute the fixture scenarios specified by `A-replace-unscheduled-valid` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-unscheduled-valid](../design/replacement.md#A-replace-unscheduled-valid) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace response created {#cli-verification-1-replace-response-created}

Execute the fixture scenarios specified by `A-replace-response-created` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-response-created](../design/replacement.md#A-replace-response-created) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace response retired {#cli-verification-1-replace-response-retired}

Execute the fixture scenarios specified by `A-replace-response-retired` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-response-retired](../design/replacement.md#A-replace-response-retired) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace response unscheduled {#cli-verification-1-replace-response-unscheduled}

Execute the fixture scenarios specified by `A-replace-response-unscheduled` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-response-unscheduled](../design/replacement.md#A-replace-response-unscheduled) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace response review reminder {#cli-verification-1-replace-response-review-reminder}

Execute the fixture scenarios specified by `A-replace-response-review-reminder` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-response-review-reminder](../design/replacement.md#A-replace-response-review-reminder) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace response pin reminder {#cli-verification-1-replace-response-pin-reminder}

Execute the fixture scenarios specified by `A-replace-response-pin-reminder` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-response-pin-reminder](../design/replacement.md#A-replace-response-pin-reminder) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Replace response reminders {#cli-verification-1-replace-response-reminders}

Execute the fixture scenarios specified by `A-replace-response-reminders` and its `behavior` criterion.

- **Anchor coverage:** [A-replace-response-reminders](../design/replacement.md#A-replace-response-reminders) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli database explicit {#cli-verification-1-cli-database-explicit}

Execute the fixture scenarios specified by `A-cli-database-explicit` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-database-explicit](../design/cli.md#A-cli-database-explicit) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli language required {#cli-verification-1-cli-language-required}

Execute the fixture scenarios specified by `A-cli-language-required` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-language-required](../design/cli.md#A-cli-language-required) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli no prompt {#cli-verification-1-cli-no-prompt}

Execute the fixture scenarios specified by `A-cli-no-prompt` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-no-prompt](../design/cli.md#A-cli-no-prompt) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli json output {#cli-verification-1-cli-json-output}

Execute the fixture scenarios specified by `A-cli-json-output` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-json-output](../design/cli.md#A-cli-json-output) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli success status {#cli-verification-1-cli-success-status}

Execute the fixture scenarios specified by `A-cli-success-status` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-success-status](../design/cli.md#A-cli-success-status) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli error {#cli-verification-1-cli-error}

Execute the fixture scenarios specified by `A-cli-error` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-error](../design/cli.md#A-cli-error) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli file input {#cli-verification-1-cli-file-input}

Execute the fixture scenarios specified by `A-cli-file-input` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-file-input](../design/cli.md#A-cli-file-input) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli stdin input {#cli-verification-1-cli-stdin-input}

Execute the fixture scenarios specified by `A-cli-stdin-input` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-stdin-input](../design/cli.md#A-cli-stdin-input) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli strict envelope {#cli-verification-1-cli-strict-envelope}

Execute the fixture scenarios specified by `A-cli-strict-envelope` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-strict-envelope](../design/cli.md#A-cli-strict-envelope) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli help {#cli-verification-1-cli-help}

Execute the fixture scenarios specified by `A-cli-help` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-help](../design/cli.md#A-cli-help) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli no extra operands {#cli-verification-1-cli-no-extra-operands}

Execute the fixture scenarios specified by `A-cli-no-extra-operands` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-no-extra-operands](../design/cli.md#A-cli-no-extra-operands) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli version {#cli-verification-1-cli-version}

Execute the fixture scenarios specified by `A-cli-version` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-version](../design/cli.md#A-cli-version) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli operands {#cli-verification-1-cli-operands}

Execute the fixture scenarios specified by `A-cli-operands` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-operands](../design/cli.md#A-cli-operands) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli error codes {#cli-verification-1-cli-error-codes}

Execute the fixture scenarios specified by `A-cli-error-codes` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-error-codes](../design/cli.md#A-cli-error-codes) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Cli init response {#cli-verification-1-cli-init-response}

Execute the fixture scenarios specified by `A-cli-init-response` and its `behavior` criterion.

- **Anchor coverage:** [A-cli-init-response](../design/cli.md#A-cli-init-response) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Database sqlite {#cli-verification-1-database-sqlite}

Execute the fixture scenarios specified by `A-database-sqlite` and its `behavior` criterion.

- **Anchor coverage:** [A-database-sqlite](../design/records.md#A-database-sqlite) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Database init exclusive {#cli-verification-1-database-init-exclusive}

Execute the fixture scenarios specified by `A-database-init-exclusive` and its `behavior` criterion.

- **Anchor coverage:** [A-database-init-exclusive](../design/records.md#A-database-init-exclusive) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Database open existing {#cli-verification-1-database-open-existing}

Execute the fixture scenarios specified by `A-database-open-existing` and its `behavior` criterion.

- **Anchor coverage:** [A-database-open-existing](../design/records.md#A-database-open-existing) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Database timezone {#cli-verification-1-database-timezone}

Execute the fixture scenarios specified by `A-database-timezone` and its `behavior` criterion.

- **Anchor coverage:** [A-database-timezone](../design/records.md#A-database-timezone) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Database version {#cli-verification-1-database-version}

Execute the fixture scenarios specified by `A-database-version` and its `behavior` criterion.

- **Anchor coverage:** [A-database-version](../design/records.md#A-database-version) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Database single learner {#cli-verification-1-database-single-learner}

Execute the fixture scenarios specified by `A-database-single-learner` and its `behavior` criterion.

- **Anchor coverage:** [A-database-single-learner](../design/records.md#A-database-single-learner) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Database invalid {#cli-verification-1-database-invalid}

Execute the fixture scenarios specified by `A-database-invalid` and its `behavior` criterion.

- **Anchor coverage:** [A-database-invalid](../design/records.md#A-database-invalid) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Language namespace {#cli-verification-1-language-namespace}

Execute the fixture scenarios specified by `A-language-namespace` and its `behavior` criterion.

- **Anchor coverage:** [A-language-namespace](../design/records.md#A-language-namespace) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Language exact {#cli-verification-1-language-exact}

Execute the fixture scenarios specified by `A-language-exact` and its `behavior` criterion.

- **Anchor coverage:** [A-language-exact](../design/records.md#A-language-exact) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Language implicit {#cli-verification-1-language-implicit}

Execute the fixture scenarios specified by `A-language-implicit` and its `behavior` criterion.

- **Anchor coverage:** [A-language-implicit](../design/records.md#A-language-implicit) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Language isolation {#cli-verification-1-language-isolation}

Execute the fixture scenarios specified by `A-language-isolation` and its `behavior` criterion.

- **Anchor coverage:** [A-language-isolation](../design/records.md#A-language-isolation) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Language empty {#cli-verification-1-language-empty}

Execute the fixture scenarios specified by `A-language-empty` and its `behavior` criterion.

- **Anchor coverage:** [A-language-empty](../design/records.md#A-language-empty) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record identity unique {#cli-verification-1-record-identity-unique}

Execute the fixture scenarios specified by `A-record-identity-unique` and its `behavior` criterion.

- **Anchor coverage:** [A-record-identity-unique](../design/records.md#A-record-identity-unique) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record identity format {#cli-verification-1-record-identity-format}

Execute the fixture scenarios specified by `A-record-identity-format` and its `behavior` criterion.

- **Anchor coverage:** [A-record-identity-format](../design/records.md#A-record-identity-format) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record identity collision {#cli-verification-1-record-identity-collision}

Execute the fixture scenarios specified by `A-record-identity-collision` and its `behavior` criterion.

- **Anchor coverage:** [A-record-identity-collision](../design/records.md#A-record-identity-collision) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record content json {#cli-verification-1-record-content-json}

Execute the fixture scenarios specified by `A-record-content-json` and its `behavior` criterion.

- **Anchor coverage:** [A-record-content-json](../design/records.md#A-record-content-json) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record content opaque {#cli-verification-1-record-content-opaque}

Execute the fixture scenarios specified by `A-record-content-opaque` and its `behavior` criterion.

- **Anchor coverage:** [A-record-content-opaque](../design/records.md#A-record-content-opaque) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record content roundtrip {#cli-verification-1-record-content-roundtrip}

Execute the fixture scenarios specified by `A-record-content-roundtrip` and its `behavior` criterion.

- **Anchor coverage:** [A-record-content-roundtrip](../design/records.md#A-record-content-roundtrip) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record content immutable {#cli-verification-1-record-content-immutable}

Execute the fixture scenarios specified by `A-record-content-immutable` and its `behavior` criterion.

- **Anchor coverage:** [A-record-content-immutable](../design/records.md#A-record-content-immutable) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record created {#cli-verification-1-record-created}

Execute the fixture scenarios specified by `A-record-created` and its `behavior` criterion.

- **Anchor coverage:** [A-record-created](../design/records.md#A-record-created) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record envelope {#cli-verification-1-record-envelope}

Execute the fixture scenarios specified by `A-record-envelope` and its `behavior` criterion.

- **Anchor coverage:** [A-record-envelope](../design/records.md#A-record-envelope) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record active {#cli-verification-1-record-active}

Execute the fixture scenarios specified by `A-record-active` and its `behavior` criterion.

- **Anchor coverage:** [A-record-active](../design/records.md#A-record-active) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record history readable {#cli-verification-1-record-history-readable}

Execute the fixture scenarios specified by `A-record-history-readable` and its `behavior` criterion.

- **Anchor coverage:** [A-record-history-readable](../design/records.md#A-record-history-readable) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record pin default {#cli-verification-1-record-pin-default}

Execute the fixture scenarios specified by `A-record-pin-default` and its `behavior` criterion.

- **Anchor coverage:** [A-record-pin-default](../design/records.md#A-record-pin-default) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record pin explicit {#cli-verification-1-record-pin-explicit}

Execute the fixture scenarios specified by `A-record-pin-explicit` and its `behavior` criterion.

- **Anchor coverage:** [A-record-pin-explicit](../design/records.md#A-record-pin-explicit) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

#### Record pin immutable {#cli-verification-1-record-pin-immutable}

Execute the fixture scenarios specified by `A-record-pin-immutable` and its `behavior` criterion.

- **Anchor coverage:** [A-record-pin-immutable](../design/records.md#A-record-pin-immutable) · `behavior` — Full
- **Evidence:** The current Anchor Book nextest run reports satisfied. The association beside the test identifies its executable assertions. This is finite test coverage, with the scope stated by the criterion.

### Completion record

Verified locally on 2026-11-15 against the working tree with Rust 1.99.0,
Anchor Book revision 9689b00416fd577bfb68a02f1fa310f6088404cf,
and nextest 0.9.151. Run `anchorbook verify --config anchorbook.toml`
to reproduce current results. CI execution on GitHub is pending.
Reports and runner logs are generated in `target/anchorbook-verify`.
Passing tests establish their listed scenarios, with broader assurance limited
by the chosen fixtures and assertions.

## Milestone 2. Review obligations {#cli-verification-2}

**Identifier:** `cli-verification-2`

**Outcome:** Packaging, clock ownership, failure cleanup, and tutor instructions have explicit scenario assessments.

**Status:** Complete

**Dependencies:** Existing source and packaged skill. Native review assessment support is unavailable in the pinned engine.

**Design basis:** Review criteria in the local design chapters.

**Tasks**

- [x] Enumerate concrete review scenarios.
- [x] Inspect the current skill instructions against the tutoring scenarios.
- [x] Complete environment-based packaging and failure-boundary assessments.
- [x] Record explicit manual evidence for each review criterion and the engine limitation.

### Completion criteria

#### Time instant {#cli-verification-2-time-instant}

Trace the CLI entry point to its single Utc::now call and Database::open argument. Check that replacement timestamps, context, and relative-date resolution use that stored instant.

- **Anchor coverage:** [A-time-instant](../design/scheduling.md#A-time-instant) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. src/main.rs samples Utc::now once in execute and passes it to Database::open. The stored now supplies timestamps and today; fixed-instant calendar tests cover the downstream calculations. Native review evaluation remains unavailable.

#### Skill database choice {#cli-verification-2-skill-database-choice}

Consider a fresh conversation with no chosen database and a convenient existing file. The instructions must require the learner’s selection before data operations.

- **Anchor coverage:** [A-skill-database-choice](../design/skill.md#A-skill-database-choice) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Select memory and refresh context” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill reuse choice {#cli-verification-2-skill-reuse-choice}

Consider two successive turns followed by an explicit database change. Check that the selected path persists until the learner changes it.

- **Anchor coverage:** [A-skill-reuse-choice](../design/skill.md#A-skill-reuse-choice) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Select memory and refresh context” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill new timezone {#cli-verification-2-skill-new-timezone}

Consider creating a database with and without a known learner timezone. Require an established timezone before init.

- **Anchor coverage:** [A-skill-new-timezone](../design/skill.md#A-skill-new-timezone) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Select memory and refresh context” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill language choice {#cli-verification-2-skill-language-choice}

Consider an ambiguous language request and a clearly named new language. Require an established language before loading its content.

- **Anchor coverage:** [A-skill-language-choice](../design/skill.md#A-skill-language-choice) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Select memory and refresh context” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill context {#cli-verification-2-skill-context}

Walk through a fresh chat, an old chat, a direct question, and a second teaching turn. Each selected-language teaching reply must first refresh context.

- **Anchor coverage:** [A-skill-context](../design/skill.md#A-skill-context) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Select memory and refresh context” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill resume {#cli-verification-2-skill-resume}

Consider a returning learner with pinned goals and due material but no specific request. Require a brief orientation informed by context.

- **Anchor coverage:** [A-skill-resume](../design/skill.md#A-skill-resume) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Select memory and refresh context” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill direct question {#cli-verification-2-skill-direct-question}

Consider a learner asking about a word while reviews are due. Require answering the question without a mandatory orientation menu.

- **Anchor coverage:** [A-skill-direct-question](../design/skill.md#A-skill-direct-question) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Select memory and refresh context” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill practice evidence {#cli-verification-2-skill-practice-evidence}

Compare independent recall with a correct answer after a hint. Require observations to preserve that distinction.

- **Anchor coverage:** [A-skill-practice-evidence](../design/skill.md#A-skill-practice-evidence) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Maintain learning memory” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill practice reference {#cli-verification-2-skill-practice-reference}

Practice an existing record, then split it. Require the observation to reference the original practiced ID.

- **Anchor coverage:** [A-skill-practice-reference](../design/skill.md#A-skill-practice-reference) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Maintain learning memory” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill schedule choice {#cli-verification-2-skill-schedule-choice}

Practice a scheduled record and examine retain, reschedule, clear, and replace outcomes. Require a deliberate choice for each.

- **Anchor coverage:** [A-skill-schedule-choice](../design/skill.md#A-skill-schedule-choice) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Maintain learning memory” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill replacement reminders {#cli-verification-2-skill-replacement-reminders}

Replace a pinned scheduled predecessor with unscheduled successors. Require consideration of both schedule and pin reminders.

- **Anchor coverage:** [A-skill-replacement-reminders](../design/skill.md#A-skill-replacement-reminders) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Maintain learning memory” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill pin context {#cli-verification-2-skill-pin-context}

Introduce a durable teaching preference and then start a fresh conversation. Require storage as active pinned context.

- **Anchor coverage:** [A-skill-pin-context](../design/skill.md#A-skill-pin-context) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Maintain learning memory” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill no taxonomy {#cli-verification-2-skill-no-taxonomy}

Introduce a new JSON convention for recognition versus production. Check that the skill permits it without a storage schema change.

- **Anchor coverage:** [A-skill-no-taxonomy](../design/skill.md#A-skill-no-taxonomy) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Maintain learning memory” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill package {#cli-verification-2-skill-package}

Run the packaging test, inspect archive contents, and build the extracted crate independently of the workspace.

- **Anchor coverage:** [A-skill-package](../design/skill.md#A-skill-package) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. The packaging test passed, checked required archive files, extracted the crate into a temporary directory, built it independently, and executed its version command. Native review evaluation remains unavailable.

#### Skill version check {#cli-verification-2-skill-version-check}

Consider matching, older, and newer binary versions. Require exact agreement before data operations.

- **Anchor coverage:** [A-skill-version-check](../design/skill.md#A-skill-version-check) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Prepare the tool” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill install {#cli-verification-2-skill-install}

Consider a missing managed command. Require Symposium setup or update and invocation through cargo agents lab.

- **Anchor coverage:** [A-skill-install](../design/skill.md#A-skill-install) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Prepare the tool” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill install verify {#cli-verification-2-skill-install-verify}

Consider failed installation or a version mismatch after updating. Require another exact version check and halted data operations.

- **Anchor coverage:** [A-skill-install-verify](../design/skill.md#A-skill-install-verify) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed the listed scenario against the “Prepare the tool” section of skills/language-lab/SKILL.md. Its explicit instruction supplies the required choice or action. This is a source walkthrough, with no claim of a model-backed session. Native review evaluation remains unavailable.

#### Skill version sync {#cli-verification-2-skill-version-sync}

Compare the expected version in the skill with the package version in Cargo.toml.

- **Anchor coverage:** [A-skill-version-sync](../design/skill.md#A-skill-version-sync) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. The packaging test compares the skill’s expected 0.1.0 with Cargo metadata and the extracted binary’s version; all agree. Native review evaluation remains unavailable.

#### Skill symposium {#cli-verification-2-skill-symposium}

Inspect the Symposium manifest’s skill source, lab command, Git installation, crate name, and package version agreement.

- **Anchor coverage:** [A-skill-symposium](../design/skill.md#A-skill-symposium) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Reviewed SYMPOSIUM.toml against Cargo.toml and the skill. The skill directory, lab command, language-lab executable, Git installation source, and 0.1.0 version agree. Native review evaluation remains unavailable.

#### Database bundled {#cli-verification-2-database-bundled}

Inspect Cargo feature resolution for rusqlite bundled, then run the packaged CLI in an environment without a SQLite executable or dynamically loaded SQLite dependency.

- **Anchor coverage:** [A-database-bundled](../design/records.md#A-database-bundled) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. Cargo enables rusqlite bundled. On macOS, otool -L reports only system libraries and no SQLite library; initialization succeeds with an empty executable search path. Native review evaluation remains unavailable.

#### Database init failure {#cli-verification-2-database-init-failure}

Inspect and execute the injected initialization-failure test. Check cleanup after schema creation fails and preservation of preexisting files.

- **Anchor coverage:** [A-database-init-failure](../design/records.md#A-database-init-failure) · `behavior` — Full
- **Evidence:** Reviewed locally on 2026-11-15. The passing partial_initialization_cleanup_and_existing_file_protection test injects failure after schema creation, asserts an empty directory afterward, and verifies a preexisting file retains its bytes. Native review evaluation remains unavailable.

### Assessment limits

The skill's selection, context refresh, memory, scheduling, and installation
sections explicitly contain the requested instructions. A walkthrough of the
listed scenarios finds the relevant directions, including hints, original
practice IDs, explicit review choices, and version mismatch handling.
This inspection establishes instruction presence. It does not establish
consistent adherence by arbitrary models or hosts. Packaging and runtime
boundary observations above apply to this macOS checkout. Additional platform
coverage and live model evaluations remain future assurance work.


Native review assessment ingestion is deferred until Anchor Book implements it.
The completed review milestone records a manual assessment of this working tree.
The engine lists these claims as unchecked. Changes to the skill, packaging,
clock wiring, or criteria require a fresh manual review and an updated record.
