use language_lab_ci::{discover, snapshot, validate};
use std::{collections::BTreeMap, path::PathBuf};

const HELP: &str = "language-lab-ci check [--root PATH] [--requirement ID]\nlanguage-lab-ci requirements [--root PATH] [--requirement ID]\nlanguage-lab-ci input [--root PATH] --path FILE --line-start N --line-end N\n\nDefaults to check in the current directory. Ranges are one-based and inclusive.";

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1).peekable();
    if args.peek().is_some_and(|a| a == "--help" || a == "-h") {
        println!("{HELP}");
        return Ok(());
    }
    let command = if args.peek().is_some_and(|a| !a.starts_with('-')) {
        args.next().unwrap()
    } else {
        "check".into()
    };
    let allowed: &[&str] = match command.as_str() {
        "check" | "requirements" => &["--root", "--requirement"],
        "input" => &["--root", "--path", "--line-start", "--line-end"],
        _ => return Err(format!("unknown command {command}\n{HELP}")),
    };
    let mut options = BTreeMap::new();
    while let Some(key) = args.next() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown option {key}\n{HELP}"));
        }
        let value = args
            .next()
            .filter(|v| !v.starts_with("--"))
            .ok_or_else(|| format!("missing value for {key}"))?;
        if options.insert(key.clone(), value).is_some() {
            return Err(format!("duplicate option {key}"));
        }
    }
    let root = PathBuf::from(options.get("--root").map(String::as_str).unwrap_or("."));
    if command == "input" {
        let get = |key: &str| options.get(key).ok_or_else(|| format!("missing {key}"));
        let start = get("--line-start")?
            .parse()
            .map_err(|_| "invalid --line-start")?;
        let end = get("--line-end")?
            .parse()
            .map_err(|_| "invalid --line-end")?;
        let input = snapshot(&root, get("--path")?, start, end)?;
        println!(
            "{}",
            serde_json::to_string_pretty(&input).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    let mut requirements = discover(&root)?;
    if let Some(id) = options.get("--requirement") {
        requirements.retain(|r| &r.id == id);
        if requirements.is_empty() {
            return Err(format!("unknown requirement {id}"));
        }
    }
    if command == "requirements" {
        println!(
            "{}",
            serde_json::to_string_pretty(&requirements).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    let errors: Vec<_> = requirements
        .iter()
        .flat_map(|r| validate(&root, r))
        .collect();
    if !errors.is_empty() {
        return Err(format!(
            "{}\nEvidence check failed: {} issue(s) across {} requirement(s).",
            errors.join("\n"),
            errors.len(),
            requirements.len()
        ));
    }
    println!(
        "Evidence is current for {} requirement(s). Justifications were not evaluated.",
        requirements.len()
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
