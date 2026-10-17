# Introduction

Language Lab gives an LLM language tutor durable memory and a clock. The learner uses an existing conversation interface; a skill guides the tutor, and a local Rust command-line tool stores its memory in a SQLite file.

A returning learner might hear: “We’ve been practicing descriptions of past events. A few verb forms could use refreshing. Would you like a short conversation, a review, or something new?” The learner can also ask a specific question and get straight to it. Continuing an old thread and starting a new one use the same learning database.

## Augment the tutor

Language learning resists a fixed taxonomy. Recognizing a Greek verb, producing its aorist form, and deciding when to use that tense are related but distinct abilities. The tutor should be able to discover those distinctions through practice, reorganize its notes, and tailor its approach to the learner.

Language Lab therefore has one content type: a **record** containing arbitrary JSON. A record can describe a learning topic, a practice observation, learner preferences, or something the tutor invents. The CLI does not assign meaning to fields such as `kind` or `related_to` inside that JSON.

The surrounding machinery is deliberately small:

- Short IDs identify immutable versions of records.
- Replacement links preserve the history of rewrites, splits, and merges.
- Optional review dates support spaced practice.
- Optional pinning keeps important context available when the tutor resumes.

Review intervals and teaching choices belong to the tutor. The CLI handles calendar arithmetic and finds scheduled records again.

## A local tool first

The package is a Rust CLI with bundled SQLite and a companion skill. One database serves one learner and can contain multiple languages. The skill asks which database to use. No account service, dedicated app UI, MCP server, or lifecycle hook is required for this design.

A future open-plugin or Symposium package can distribute the same components. Host-specific hooks may eventually make context refresh more reliable, but they are not a dependency of the initial experience.

## Status of this book

The tutor skill lives in `skills/language-lab/`, with its Rust CLI embedded in `cli/`. The `mdbook-language-lab` crate renders this book’s requirements; `language-lab-ci` checks their evidence. The [user’s guide](guide.md) describes installation and use, and [Design](design/README.md) defines the testable contract.
