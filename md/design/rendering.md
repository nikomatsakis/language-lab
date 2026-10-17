# Requirement rendering

`mdbook-language-lab` is the HTML preprocessor for this book. It recognizes two MyST-style inline roles; it is not a general MyST parser.

```markdown
{req}`example.identifier` A requirement definition would go here.

See {example.identifier}`a descriptive label`.
```

The first form defines an anchor and renders a two-part REQ/name badge. The second links to that anchor, including across chapters. Definition names should describe properties, not chapter numbers. Reference bodies are plain-text labels, not nested Markdown.

## Definitions and references

{req}`render.definition` A definition role must produce a visible badge containing `REQ` and its identifier, with an anchor named `req-IDENTIFIER`.

{req}`render.identifiers` Requirement identifiers must contain a dot and consist only of lowercase ASCII letters, digits, dots, hyphens, and underscores.

{req}`render.reference` A reference role must render its body as a link to the matching definition’s HTML page and anchor.

{req}`render.forward` References must resolve independently of chapter order, including references to later chapters and nested chapters.

{req}`render.duplicate` Duplicate requirement definitions must fail the book build with a diagnostic identifying the duplicate ID and chapter paths.

{req}`render.unknown` An unresolved reference role must fail the book build with a diagnostic identifying its ID and chapter path.

{req}`render.invalid` A definition with an invalid identifier must fail the book build.

{req}`render.examples` Roles inside fenced code blocks, indented code blocks, and literal inline code spans must remain unchanged.

{req}`render.escaped` A role whose opening brace is Markdown-escaped must remain unchanged.

{req}`render.existing-links` Roles within Markdown link or image labels must remain unchanged to avoid nested links.

{req}`render.html-safe` Reference labels must be escaped before being emitted as HTML.

{req}`render.source-preservation` Source text outside recognized roles must remain byte-for-byte unchanged by the preprocessor.

Roles are recognized in normal Markdown inline content. Raw HTML is not an authoring surface for roles. To show literal syntax, use a fenced block or a larger inline-code delimiter as above. Other MyST roles without dotted names are outside this plugin’s scope and are left unchanged.

## Integration and presentation

{req}`render.protocol` The executable must read mdBook’s `[context, book]` JSON from stdin and emit only the transformed book JSON on stdout.

{req}`render.supports` `supports html` must exit zero; unsupported renderer names must exit nonzero.

{req}`render.errors` Preprocessing errors must be written to stderr and produce a nonzero exit status without a partial book on stdout.

{req}`render.offline` Rendered requirement badges must not require network requests or JavaScript.

{req}`render.accessibility` Definition badges must expose the requirement identifier as text and an accessible link name.

{req}`render.focus` Badge and reference links must have visible keyboard-focus styling.

The book loads `theme/requirements.css` for local shields-style badges. Using text and CSS keeps badges readable, selectable, searchable, and independent of an image service. The current renderer speaks the JSON protocol directly and preserves unrelated book metadata.

The integration follows the [mdBook preprocessor protocol](https://rust-lang.github.io/mdBook/for_developers/preprocessors).
