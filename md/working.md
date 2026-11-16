# Working on Language Lab

The canonical CLI specification is this Anchor Book. The project phase is
recorded in `anchorbook.toml`; prototyping describes this migration iteration.
Existing CLI guarantees remain accepted.

Install the engine at the revision used for this migration:

```sh
cargo install --git https://github.com/anchor-book/anchor-book --rev 9689b00416fd577bfb68a02f1fa310f6088404cf --locked anchor-book
```

Use mdBook 0.5.4 and install the Mermaid preprocessor used by CI:

```sh
cargo install mdbook-mermaid --version 0.17.1 --locked
```

Run the following from the repository root:

```sh
anchorbook contract
anchorbook check --config anchorbook.toml
anchorbook verify --config anchorbook.toml
mdbook build
cargo test --workspace --locked
python3 skills/language-lab/tests/test_package.py
```

The pinned engine supports schema 1 and structural checks. Some integration
features in its design documentation, including the engine version declaration,
are still planned. CI pins its installation by commit.

Author properties using Anchor Book's anchor and authority formats. Consult
the pinned engine's `book/design/anchor-definitions.md`,
`authority-definitions.md`, and `verification-evidence.md`.
The CLI verification roadmap records criteria, current coverage, and remaining reviews.

Use [Requests for Discussion](rfds/index.md) for substantial changes to the
system design. Review the proposal and its proposed requirements and anchors
with the project maintainer. Record agreement against the reviewed revision,
then incorporate the agreed definitions into their owning chapters and
link the implementation roadmap to the adopted criteria.

The pinned engine supports schema 1. Review the
[RFD format](https://github.com/anchor-book/anchor-book/blob/9689b00416fd577bfb68a02f1fa310f6088404cf/book/design/rfd-format.md)
and proposal references manually. Keep proposed definitions inside the
`rfd-change` fences. Enable the `book.sections.rfds` mapping when the
local and CI engine versions support schema 2.

Install the supported test runner with `cargo install cargo-nextest --version
0.9.151 --locked`. Verification reads doc-comment associations beside Rust tests.
Completed roadmap milestones claim their Full coverage criteria, so a missing,
failed, or inconclusive linked test makes verification fail. Review methods are
reported as not evaluated by this engine. Their manual assessments and scope
limits are recorded in the CLI verification roadmap.

Test methods cover observable command and database behavior. Review methods
cover instruction semantics and environment or source properties that require
an assessor. Formal proofs are not required by this CLI baseline.
