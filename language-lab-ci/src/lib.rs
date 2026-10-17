use mdbook_language_lab::{roles, valid_id};
use pulldown_cmark::{Event, Parser, Tag};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Serialize)]
pub struct Requirement {
    pub id: String,
    pub path: String,
    pub line: usize,
    pub requirement: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub path: String,
    pub line_start: usize,
    pub line_end: usize,
    pub contents: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub requirement: String,
    pub inputs: Vec<Input>,
    // Required by the file format, deliberately not interpreted by the checker.
    pub justification: String,
}

/// Resolve only repository-contained files, including after following symlinks.
fn contained(root: &Path, relative: &Path) -> Result<PathBuf> {
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, Component::Normal(_) | Component::CurDir))
    {
        return Err(format!(
            "expected a repository-relative path without '..': {}",
            relative.display()
        ));
    }
    let path = root
        .join(relative)
        .canonicalize()
        .map_err(|e| format!("{}: {e}", relative.display()))?;
    if !path.starts_with(root) {
        return Err(format!("path escapes repository: {}", relative.display()));
    }
    Ok(path)
}

fn read(root: &Path, relative: &Path) -> Result<String> {
    fs::read_to_string(contained(root, relative)?)
        .map_err(|e| format!("{}: {e}", relative.display()))
}

// Chapter links may go up from book.src while remaining within the repository.
// Evidence input paths intentionally use the stricter contained() policy above.
fn read_book(root: &Path, relative: &Path) -> Result<String> {
    if relative.is_absolute() {
        return Err(format!(
            "book paths must be relative: {}",
            relative.display()
        ));
    }
    let path = root
        .join(relative)
        .canonicalize()
        .map_err(|e| format!("{}: {e}", relative.display()))?;
    if !path.starts_with(root) {
        return Err(format!(
            "book path escapes repository: {}",
            relative.display()
        ));
    }
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", relative.display()))
}

/// Extract definitions from their enclosing paragraphs, preserving source Markdown.
pub fn extract(path: &str, source: &str) -> Result<Vec<Requirement>> {
    let paragraphs: Vec<_> = Parser::new(source)
        .into_offset_iter()
        .filter_map(|(event, range)| matches!(event, Event::Start(Tag::Paragraph)).then_some(range))
        .collect();
    let mut requirements = Vec::new();
    let mut used = BTreeSet::new();
    for role in roles(source).into_iter().filter(|r| r.name == "req") {
        if !valid_id(&role.label) || !role.label.contains('.') {
            return Err(format!("{path}: invalid requirement ID {:?}", role.label));
        }
        let paragraph = paragraphs
            .iter()
            .find(|r| r.start <= role.range.start && r.end >= role.range.end)
            .ok_or_else(|| format!("{path}: {} must begin a paragraph", role.label))?;
        if !source[paragraph.start..role.range.start].trim().is_empty()
            || !used.insert(paragraph.start)
        {
            return Err(format!(
                "{path}: {} must be the only definition at the start of its paragraph",
                role.label
            ));
        }
        let text = source[role.range.end..paragraph.end].trim();
        if text.is_empty() {
            return Err(format!("{path}: {} has no requirement text", role.label));
        }
        requirements.push(Requirement {
            id: role.label,
            path: path.into(),
            line: source[..role.range.start]
                .bytes()
                .filter(|b| *b == b'\n')
                .count()
                + 1,
            requirement: text.into(),
        });
    }
    Ok(requirements)
}

/// Read authored chapters listed in the configured book's SUMMARY.md.
pub fn discover(root: &Path) -> Result<Vec<Requirement>> {
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let config: toml::Value = read_book(&root, Path::new("book.toml"))?
        .parse()
        .map_err(|e| format!("book.toml: {e}"))?;
    if config.get("book").is_some_and(|book| !book.is_table()) {
        return Err("book must be a TOML table".into());
    }
    let src = match config.get("book").and_then(|b| b.get("src")) {
        Some(value) => value.as_str().ok_or("book.src must be a string")?,
        None => "src",
    };
    let summary = read_book(&root, &Path::new(src).join("SUMMARY.md"))?;
    let mut chapters = BTreeSet::new();
    for event in Parser::new(&summary) {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            let dest = dest_url.as_ref();
            if dest.is_empty()
                || dest.starts_with('#')
                || dest.starts_with("//")
                || dest.split_once(':').is_some_and(|(scheme, _)| {
                    scheme.starts_with(|c: char| c.is_ascii_alphabetic())
                        && scheme
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
                })
            {
                continue;
            }
            let dest = dest.split('#').next().unwrap();
            chapters.insert(Path::new(src).join(dest));
        }
    }
    let mut found: BTreeMap<String, Requirement> = BTreeMap::new();
    for chapter in chapters {
        let source = read_book(&root, &chapter)?;
        let path = chapter.to_string_lossy().replace('\\', "/");
        for requirement in extract(&path, &source)? {
            if let Some(previous) = found.get(&requirement.id) {
                return Err(format!(
                    "duplicate requirement {} in {} and {}",
                    requirement.id, previous.path, path
                ));
            }
            found.insert(requirement.id.clone(), requirement);
        }
    }
    if found.is_empty() {
        return Err("no requirements found in the book; refusing a vacuous check".into());
    }
    Ok(found.into_values().collect())
}

/// Lines are one-based and inclusive; original CRLF/LF and final newlines matter.
pub fn lines(contents: &str, start: usize, end: usize) -> Result<String> {
    if start == 0 || end < start {
        return Err(format!("invalid inclusive line range {start}..{end}"));
    }
    let lines: Vec<_> = contents.split_inclusive('\n').collect();
    if end > lines.len() {
        return Err(format!(
            "line range {start}..{end} exceeds file length {}",
            lines.len()
        ));
    }
    Ok(lines[start - 1..end].concat())
}

pub fn snapshot(root: &Path, path: &str, start: usize, end: usize) -> Result<Input> {
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let text = read(&root, Path::new(path))?;
    Ok(Input {
        path: path.into(),
        line_start: start,
        line_end: end,
        contents: lines(&text, start, end)?,
    })
}

pub fn validate(root: &Path, requirement: &Requirement) -> Vec<String> {
    let result = (|| -> Result<Vec<String>> {
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let path = format!("evidence/{}.json", requirement.id);
        let evidence: Evidence = serde_json::from_str(&read(&root, Path::new(&path))?)
            .map_err(|e| format!("{path}: invalid evidence JSON: {e}"))?;
        let mut errors = Vec::new();
        if evidence.requirement != requirement.requirement {
            errors.push("requirement text differs from the book".into());
        }
        if evidence.inputs.is_empty() {
            errors.push("inputs must contain at least one source or test excerpt".into());
        }
        for input in evidence.inputs {
            match snapshot(&root, &input.path, input.line_start, input.line_end) {
                Err(error) => errors.push(error),
                Ok(current) if current.contents != input.contents => errors.push(format!(
                    "{}:{}..{} no longer matches contents",
                    input.path, input.line_start, input.line_end
                )),
                Ok(_) => {}
            }
        }
        Ok(errors)
    })();
    result.unwrap_or_else(|e| vec![e]).into_iter().map(|error| {
        format!("{}: {error}; revalidate this requirement using the revalidate skill (.agents/skills/revalidate/SKILL.md)", requirement.id)
    }).collect()
}
