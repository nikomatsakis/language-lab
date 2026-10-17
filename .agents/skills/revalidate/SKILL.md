---
name: revalidate
description: Revalidate a Language Lab requirement against implementation and tests, replacing its evidence file only when the full requirement is satisfied. Use when language-lab-ci reports missing or stale evidence or the user requests requirement revalidation.
---

# Revalidate a requirement

Work from the repository root. This is a semantic review, not a snapshot refresh.
The checker only detects stale text; passing it does not establish correctness.

For each requested requirement, follow this order:

1. Obtain its exact text and location:
   ```sh
   cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- requirements --requirement REQUIREMENT_ID
   ```
   Read the surrounding specification and referenced requirements to understand
   its contract. Remove `evidence/REQUIREMENT_ID.json` if it exists. Do not use
   the old justification as a substitute for a fresh review.
2. **Before inspecting implementation or test code**, enumerate the full range
   of scenarios and edge cases required by the contract. Write this checklist
   in your working response first. Consider applicable success and failure
   paths, boundaries, state transitions, persistence, atomicity, concurrency,
   scope isolation, and interactions. Do not invent unrelated requirements.
3. Only after recording that checklist, inspect the implementation and tests.
   Trace each scenario to the relevant code and meaningful assertions. Read
   enough surrounding code, helpers, and fixtures to assess what actually runs;
   test names and comments alone are not evidence. Run the relevant tests and
   distinguish tested behavior from claims or plausible implementation.
4. If any required scenario is unimplemented, incorrectly implemented, untested,
   or cannot be verified, **do not generate an evidence file**. Leave the old
   file deleted. Report the missing edge cases and coverage or implementation
   gaps, naming the requirement. A failed check is the correct outcome here.
   Do not weaken the requirement or write optimistic evidence to make CI pass.
   Fixing the implementation is a separate task unless already requested.
5. Only if the entire requirement is implemented and tested, create
   `evidence/REQUIREMENT_ID.json` with exactly these fields:
   ```json
   {
     "requirement": "exact text returned by the requirements command",
     "inputs": [
       {
         "path": "repository/relative/file.rs",
         "line_start": 10,
         "line_end": 20,
         "contents": "exact contents of the inclusive line range"
       }
     ],
     "justification": "Explain how the implementation and tests cover every scenario in the checklist, citing the input paths and ranges. Include the tests run and their results."
   }
   ```
   Use excerpts from both implementation and tests (and material helpers or
   fixtures). Include enough context to support the claims, without copying
   unrelated files. Paths are relative to the repository; lines are one-based
   and inclusive. Preserve line endings and the last newline exactly. Generate
   each input object using:
   ```sh
   cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- input --path FILE --line-start N --line-end M
   ```
   Compose JSON with a serializer; do not manually escape source contents.
   These helper commands extract text only and never certify a requirement.
6. Run the targeted freshness check:
   ```sh
   cargo run --locked --manifest-path language-lab-ci/Cargo.toml -- check --requirement REQUIREMENT_ID
   ```
   If relevant source or requirement text changed during review, reconsider the
   affected scenarios before regenerating excerpts. Report what was validated
   and any requirements left without evidence. Do not imply that the checker
   evaluates the justification or proves semantic coverage.
