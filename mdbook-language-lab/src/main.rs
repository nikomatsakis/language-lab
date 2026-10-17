use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use serde_json::Value;
use std::{collections::BTreeMap, io, ops::Range, path::Path};

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
struct Role {
    range: Range<usize>,
    name: String,
    label: String,
}

fn valid_id(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-_".contains(&b))
}

// Let the Markdown parser identify inline code, so fenced/indented examples and
// literal code spans are never mistaken for roles. Preserve all other source bytes.
fn roles(source: &str) -> Vec<Role> {
    let mut result = Vec::new();
    let mut nested_link = 0;
    for (event, range) in Parser::new(source).into_offset_iter() {
        match event {
            Event::Start(Tag::Link { .. } | Tag::Image { .. }) => nested_link += 1,
            Event::End(TagEnd::Link | TagEnd::Image) => nested_link -= 1,
            Event::Code(label) if nested_link == 0 => {
                let prefix = &source[..range.start];
                if !prefix.ends_with('}') {
                    continue;
                }
                let Some(start) = prefix.rfind('{') else {
                    continue;
                };
                let escapes = prefix[..start]
                    .bytes()
                    .rev()
                    .take_while(|b| *b == b'\\')
                    .count();
                if escapes % 2 != 0 {
                    continue;
                }
                let name = &prefix[start + 1..prefix.len() - 1];
                if name == "req" || (valid_id(name) && name.contains('.')) {
                    result.push(Role {
                        range: start..range.end,
                        name: name.into(),
                        label: label.into_string(),
                    });
                }
            }
            _ => {}
        }
    }
    result
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn href(from: &str, to: &str, id: &str) -> String {
    if from == to {
        return format!("#req-{id}");
    }
    let from: Vec<_> = from.split('/').collect();
    let target = Path::new(to)
        .with_extension("html")
        .to_string_lossy()
        .replace('\\', "/");
    let to: Vec<_> = target.split('/').collect();
    let common = from[..from.len() - 1]
        .iter()
        .zip(&to)
        .take_while(|(a, b)| a == b)
        .count();
    let path = format!(
        "{}{}",
        "../".repeat(from.len() - 1 - common),
        to[common..].join("/")
    );
    let encoded: String = path
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"/-._~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect();
    format!("{encoded}#req-{id}")
}

fn chapters(items: &mut Value, f: &mut impl FnMut(&mut Value) -> Result<()>) -> Result<()> {
    let items = items.as_array_mut().ok_or("book items must be an array")?;
    for item in items {
        if let Some(ch) = item.get_mut("Chapter") {
            f(ch)?;
            if let Some(children) = ch.get_mut("sub_items") {
                chapters(children, f)?;
            }
        }
    }
    Ok(())
}

fn process(book: &mut Value) -> Result<()> {
    // mdBook 0.4 used sections; 0.5 uses items. Preserve the input envelope.
    let key = if book.get("items").is_some() {
        "items"
    } else {
        "sections"
    };
    let items = book.get_mut(key).ok_or("missing book items")?;
    let mut definitions = BTreeMap::new();
    chapters(items, &mut |ch| {
        let source = ch["content"].as_str().ok_or("missing chapter content")?;
        for role in roles(source).into_iter().filter(|r| r.name == "req") {
            let path = ch["path"]
                .as_str()
                .ok_or("requirement in chapter without output path")?;
            if !valid_id(&role.label) || !role.label.contains('.') {
                return Err(format!(
                    "{path}: invalid requirement identifier {:?}",
                    role.label
                ));
            }
            if let Some(previous) = definitions.insert(role.label.clone(), path.to_owned()) {
                return Err(format!(
                    "{path}: duplicate requirement {} (first defined in {previous})",
                    role.label
                ));
            }
        }
        Ok(())
    })?;
    chapters(items, &mut |ch| {
        let source = ch["content"].as_str().ok_or("missing chapter content")?;
        let mut output = source.to_owned();
        for role in roles(source).into_iter().rev() {
            let path = ch["path"]
                .as_str()
                .ok_or("role in chapter without output path")?;
            let definition = role.name == "req";
            let id = if definition { &role.label } else { &role.name };
            let target = definitions
                .get(id)
                .ok_or_else(|| format!("{path}: unknown requirement {id}"))?;
            let html = if definition {
                format!(
                    "<a class=\"requirement-badge\" id=\"req-{id}\" href=\"#req-{id}\" aria-label=\"Requirement {id}\"><span class=\"requirement-kind\">REQ</span><span class=\"requirement-name\">{id}</span></a>"
                )
            } else {
                format!(
                    "<a class=\"requirement-reference\" href=\"{}\" title=\"Requirement {id}\">{}</a>",
                    escape(&href(path, target, id)),
                    escape(&role.label)
                )
            };
            output.replace_range(role.range, &html);
        }
        ch["content"] = output.into();
        Ok(())
    })
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        if args == ["supports", "html"] {
            return Ok(());
        }
        return Err("usage: mdbook-language-lab [supports html]".into());
    }
    let input: Value = serde_json::from_reader(io::stdin()).map_err(|e| e.to_string())?;
    let pair = input
        .as_array()
        .filter(|p| p.len() == 2)
        .ok_or("expected [context, book]")?;
    if pair[0]["renderer"] != "html" {
        return Err("only the html renderer is supported".into());
    }
    let mut book = pair[1].clone();
    process(&mut book)?;
    serde_json::to_writer(io::stdout(), &book).map_err(|e| e.to_string())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("mdbook-language-lab: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn chapter(path: &str, content: &str) -> Value {
        json!({"Chapter": {"path":path,"content":content,"sub_items":[]}})
    }
    #[test]
    fn forward_nested_references_and_escaping() {
        let mut book = json!({"items":[chapter("guide/start.md", "See {record.id}`<identity & history>`.")], "extra":42});
        book["items"][0]["Chapter"]["sub_items"] = json!([chapter(
            "design/model.md",
            "{req}`record.id` Every ID is unique."
        )]);
        process(&mut book).unwrap();
        let text = book["items"][0]["Chapter"]["content"].as_str().unwrap();
        assert!(text.contains("../design/model.html#req-record.id"));
        assert!(text.contains("&lt;identity &amp; history&gt;"));
        assert_eq!(book["extra"], 42);
    }
    #[test]
    fn examples_links_and_escaped_roles_are_literal() {
        let source = "```md\n{req}`fake.fenced`\n```\n\n    {req}`fake.indented`\n\n``{req}`fake.inline` ``\n\n\\{req}`fake.escaped`\n\n[{req}`fake.link`](url)\n\n{req}`real.id`\n";
        let found = roles(source);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].label, "real.id");
    }
    #[test]
    fn invalid_duplicate_and_unknown_fail() {
        for content in [
            "{req}`bad id`",
            "{req}`a.b` {req}`a.b`",
            "{missing.id}`oops`",
        ] {
            let mut book = json!({"items":[chapter("x.md",content)]});
            assert!(process(&mut book).is_err());
        }
    }
    #[test]
    fn same_page_and_legacy_envelope() {
        let mut book = json!({"sections":[chapter("x.md", "{req}`a.b` See {a.b}`rule`.")]});
        process(&mut book).unwrap();
        let content = book["sections"][0]["Chapter"]["content"].as_str().unwrap();
        assert!(content.contains("id=\"req-a.b\""));
        assert!(content.contains("href=\"#req-a.b\" title="));
        assert_eq!(
            href("a/start.md", "other folder/rules.md", "a.b"),
            "../other%20folder/rules.html#req-a.b"
        );
    }
}
