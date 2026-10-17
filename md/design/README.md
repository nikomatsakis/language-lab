# Design

This section is the proposed v1 contract. Each **REQ** badge defines one independently checkable property. “Must” indicates required behavior. Examples and rationale explain the contract but do not introduce additional requirements.

## Reading and referencing requirements

A definition uses a MyST-style role:

```markdown
{req}`record.identity.unique` Each record ID must be unique within its database.
```

A reference uses the requirement ID as its role and a human-readable label as its body:

```markdown
See {record.identity.unique}`database-wide uniqueness`.
```

For example: see {record.identity.unique}`database-wide uniqueness`. Identifiers are stable names for tests and discussion; changing the prose should not gratuitously rename them.

## Scope and choices

The storage model is one learner per database, many language namespaces, arbitrary JSON records, immutable content, replacement lineage, optional pinning, and optional calendar-day review scheduling. There are no built-in vocabulary, grammar, notebook, or practice record types.

The following details make the conversational design precise for a first implementation: lowercase hexadecimal IDs with a minimum of six characters; exact language namespaces; replacement-based pin changes; UTC timestamps and local calendar dates; deterministic pagination; explicit CLI errors; and conservative failures when replacing retired records. These are design choices specified here, not claims of existing implementation.

No spaced-repetition algorithm is prescribed. No semantic search, automatic propagation of practice evidence, multi-user service, hook integration, or recovery/restore command is required by v1. Historical records remain available for manual reconstruction.

## Verification

CLI requirements should be exercised against temporary databases, including failed operations and subsequent reads. Date tests should use a controllable clock and include a daylight-saving transition. Replacement tests should verify both the returned response and the persisted history. Skill requirements are observable workflow checks, suitable for scripted learner scenarios. Renderer requirements have Rust tests and a full mdBook build.
