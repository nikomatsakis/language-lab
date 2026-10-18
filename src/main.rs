use clap::{ArgGroup, Parser, Subcommand};
use language_lab::{invalid, Database, Filters, Result, Review};
use std::{io::Read, path::PathBuf};
#[derive(Parser)]
#[command(
    version,
    about = "Persistent learning records and calendar-based review scheduling"
)]
struct Cli {
    #[arg(long, global = true)]
    db: Option<PathBuf>,
    #[arg(long, global = true)]
    language: Option<String>,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Create a new database without overwriting existing files.
    Init {
        path: PathBuf,
        #[arg(long)]
        timezone: String,
    },
    /// Read the current clock, languages, pinned records, due reviews and recent records.
    Context {
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Retrieve exact IDs, including historical records.
    Get {
        #[arg(required=true,num_args=1..)]
        ids: Vec<String>,
    },
    /// Query active or retired records.
    List {
        #[arg(long, conflicts_with = "retired")]
        due: bool,
        #[arg(long)]
        retired: bool,
        #[arg(long)]
        since: Option<String>,
        #[arg(long)]
        search: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Atomically create, replace, split, merge or retire records from JSON. Use - for stdin.
    Replace {
        #[arg(long)]
        file: PathBuf,
    },
    /// Set or clear the pending review for active records.
    #[command(group(ArgGroup::new("mode").required(true).args(["in_days","on","clear"])))]
    Schedule {
        #[arg(required=true,num_args=1..)]
        ids: Vec<String>,
        #[arg(long)]
        in_days: Option<u64>,
        #[arg(long)]
        on: Option<String>,
        #[arg(long)]
        clear: bool,
    },
}
fn execute(cli: Cli) -> Result<serde_json::Value> {
    let now = chrono::Utc::now();
    if let Some(lang) = &cli.language {
        if lang.is_empty() {
            return Err(invalid("language must be nonempty"));
        }
    }
    if let Command::Init { path, timezone } = cli.command {
        return Database::init(&path, &timezone);
    }
    let path = cli.db.ok_or_else(|| invalid("--db is required"))?;
    let lang = cli.language.as_deref();
    if !matches!(cli.command, Command::Context { .. }) && lang.is_none() {
        return Err(invalid("--language is required"));
    }
    let mut db = Database::open(&path, now)?;
    match cli.command {
        Command::Context { limit } => db.context(lang, limit),
        Command::Get { ids } => db.get(lang.unwrap(), &ids),
        Command::List {
            due,
            retired,
            since,
            search,
            limit,
            cursor,
        } => db.list(
            lang.unwrap(),
            Filters {
                due,
                retired,
                since,
                search,
            },
            limit,
            cursor.as_deref(),
        ),
        Command::Replace { file } => {
            let mut input = String::new();
            if file.as_os_str() == "-" {
                std::io::stdin()
                    .read_to_string(&mut input)
                    .map_err(|e| invalid(e.to_string()))?;
            } else {
                input = std::fs::read_to_string(file).map_err(|e| {
                    if e.kind() == std::io::ErrorKind::InvalidData {
                        invalid(e.to_string())
                    } else {
                        e.into()
                    }
                })?;
            }
            let request = serde_json::from_str(&input).map_err(|e| invalid(e.to_string()))?;
            db.replace(lang.unwrap(), request)
        }
        Command::Schedule {
            ids,
            in_days,
            on,
            clear,
        } => db.schedule(
            lang.unwrap(),
            &ids,
            if clear {
                None
            } else {
                Some(Review { in_days, on })
            },
        ),
        Command::Init { .. } => unreachable!(),
    }
}
fn fail(error: language_lab::Error) -> ! {
    eprintln!("{}", serde_json::json!({"error":error}));
    std::process::exit(1)
}
fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e)
            if matches!(
                e.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            ) =>
        {
            print!("{e}");
            return;
        }
        Err(e) => fail(invalid(e.to_string())),
    };
    match execute(cli) {
        Ok(value) => println!("{value}"),
        Err(e) => fail(e),
    }
}
