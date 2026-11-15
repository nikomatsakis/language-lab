# Authority

These requirements articulate the purposes already expressed in the local v1 contract.
Their acceptance carries forward that baseline.

> **`R-local-memory`** (accepted): **Portable durable learning memory**
> Statement: A learner must be able to retain and reopen local learning memory using a self-contained CLI, with explicit file selection and protection against overwriting or corrupting existing files.
> Source: The existing local-first CLI and SQLite contract.

> **`R-adaptable-memory`** (accepted): **Adaptable records and stable identity**
> Statement: The tutor must be able to store evolving JSON conventions, identify each immutable record version, and recover the same content without application-level interpretation by storage.
> Source: The introduction’s language-learning examples and existing record contract.

> **`R-scoped-context`** (accepted): **Focused and predictable retrieval**
> Statement: The tutor must retrieve only the selected language’s content, with deterministic queries and bounded context pages while retaining all pinned guidance.
> Source: The existing language, query, pagination, and context contract.

> **`R-memory-history`** (accepted): **Recoverable learning history**
> Statement: Revising learning notes must preserve predecessor history and make pinning and scheduling handoffs explicit to the tutor.
> Source: The existing replacement and retirement contract.

> **`R-atomic-updates`** (accepted): **Consistent mutations under failure and concurrency**
> Statement: A rejected or failed mutation must leave learning memory consistent, and competing replacements must have one winner.
> Source: The existing replacement and scheduling atomicity requirements.

> **`R-calendar-reviews`** (accepted): **Learner-local review dates**
> Statement: Review scheduling must use the learner’s calendar, preserve record identity, and leave interval and mastery decisions to the tutor.
> Source: The existing timezone, clock, and scheduling contract.

> **`R-agent-interface`** (accepted): **Predictable command interface for agents**
> Statement: Agents must invoke explicit noninteractive operations and receive structured results and failures with stable invocation and output conventions.
> Source: The existing CLI command contract and Symposium usage.

> **`R-tutor-continuity`** (accepted): **Continuity and truthful tutoring observations**
> Statement: The packaged skill must refresh context, respect learner choices, preserve demonstrated practice and hints, and use a compatible managed CLI installation.
> Source: The existing tutor skill and distribution contract.

