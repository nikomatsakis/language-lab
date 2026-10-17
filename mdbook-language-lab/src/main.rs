use mdbook_language_lab::process;
use serde_json::Value;
use std::io;
type Result<T> = std::result::Result<T, String>;

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
