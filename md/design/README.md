# Design

Language Lab supplies durable learning memory and a clock to a conversational
tutor. One SQLite database serves one learner, with multiple exact language
namespaces. Arbitrary JSON records hold observations, goals, and review material.
Replacement preserves history, pinning supplies persistent context, and review
dates use the learner's local calendar.

The CLI exposes initialization, context, retrieval, listing, replacement, and
scheduling. The packaged skill defines how the tutor uses those operations.
The six following chapters preserve the local v1 contract as Anchor Book anchors.

Each anchor names a property, its governing authority, and a verification
criterion. Tests establish command and storage behavior. Review criteria assess
the skill's instructions and representative tutoring scenarios.
