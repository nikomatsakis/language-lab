//! Persistent, language-scoped records. All command time comes from the supplied instant.
use chrono::{DateTime, Datelike, Days, NaiveDate, Utc};
use chrono_tz::Tz;
use rusqlite::{params, Connection, OpenFlags, TransactionBehavior};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashSet, path::Path};

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: &'static str,
    pub message: String,
}
pub type Result<T> = std::result::Result<T, Error>;
pub fn invalid(message: impl Into<String>) -> Error {
    Error {
        code: "invalid_input",
        message: message.into(),
    }
}
fn error(code: &'static str, message: impl Into<String>) -> Error {
    Error {
        code,
        message: message.into(),
    }
}
impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        error("storage_error", e.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        error("storage_error", e.to_string())
    }
}
fn fresh_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..12].to_owned()
}
fn language(s: &str) -> Result<()> {
    if s.is_empty() {
        Err(invalid("language must be nonempty"))
    } else {
        Ok(())
    }
}
fn date(s: &str) -> Result<NaiveDate> {
    let d =
        NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| invalid("invalid calendar date"))?;
    if !(1..=9999).contains(&d.year()) || d.format("%Y-%m-%d").to_string() != s {
        return Err(invalid("date must be YYYY-MM-DD, years 0001..9999"));
    }
    Ok(d)
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub language: String,
    pub created_at: String,
    pub replaces: Vec<String>,
    pub content: Value,
    pub pinned: bool,
    pub review_on: Option<String>,
    pub retired_at: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Replacement {
    pub ids: Vec<String>,
    pub replacements: Vec<Entry>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    #[serde(deserialize_with = "required_value")]
    pub content: Value,
    #[serde(default)]
    pub pinned: bool,
    pub review: Option<Review>,
}
fn required_value<'de, D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Value, D::Error> {
    let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
    parse_content(raw.get()).map_err(serde::de::Error::custom)
}
// Value's arbitrary-precision deserializer reserves an internal object key.
// Decode containers as raw members so every user object key remains opaque.
fn parse_content(text: &str) -> serde_json::Result<Value> {
    match text.trim_start().as_bytes().first() {
        Some(b'{') => {
            let members: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
                serde_json::from_str(text)?;
            members
                .into_iter()
                .map(|(key, raw)| Ok((key, parse_content(raw.get())?)))
                .collect::<serde_json::Result<serde_json::Map<String, Value>>>()
                .map(Value::Object)
        }
        Some(b'[') => {
            let items: Vec<Box<serde_json::value::RawValue>> = serde_json::from_str(text)?;
            items
                .into_iter()
                .map(|raw| parse_content(raw.get()))
                .collect::<serde_json::Result<Vec<_>>>()
                .map(Value::Array)
        }
        _ => serde_json::from_str(text),
    }
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    #[serde(default, deserialize_with = "present")]
    pub in_days: Option<u64>,
    #[serde(default, deserialize_with = "present")]
    pub on: Option<String>,
}
fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> std::result::Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Filters {
    pub due: bool,
    pub retired: bool,
    pub since: Option<String>,
    pub search: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    database: String,
    language: String,
    filters: Filters,
    cutoff: String,
    offset: usize,
}

pub struct Database {
    conn: Connection,
    timezone: Tz,
    identity: String,
    now: DateTime<Utc>,
}
impl Database {
    pub fn init(path: &Path, zone: &str) -> Result<Value> {
        Self::init_with(path, zone, |_| Ok(()))
    }
    fn init_with(
        path: &Path,
        zone: &str,
        after_schema: impl FnOnce(&Connection) -> Result<()>,
    ) -> Result<Value> {
        if path.to_str().is_none() {
            return Err(invalid("database path must be valid UTF-8"));
        }
        let _: Tz = zone.parse().map_err(|_| invalid("invalid IANA timezone"))?;
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        let result = (|| -> Result<()> {
            let mut conn = Connection::open(path)?;
            let tx = conn.transaction()?;
            tx.execute_batch("CREATE TABLE metadata (version INTEGER NOT NULL, timezone TEXT NOT NULL, identity TEXT NOT NULL); CREATE TABLE records (id TEXT PRIMARY KEY, language TEXT NOT NULL, created_at TEXT NOT NULL, predecessors TEXT NOT NULL, content TEXT NOT NULL, pinned INTEGER NOT NULL, review_on TEXT, retired_at TEXT); CREATE TABLE events (timestamp TEXT NOT NULL, predecessors TEXT NOT NULL, successors TEXT NOT NULL, retired TEXT NOT NULL);")?;
            tx.execute(
                "INSERT INTO metadata VALUES (1, ?1, ?2)",
                params![zone, uuid::Uuid::new_v4().to_string()],
            )?;
            after_schema(&tx)?;
            tx.commit()?;
            Ok(())
        })();
        drop(file);
        if result.is_err() {
            let _ = std::fs::remove_file(path);
        }
        result?;
        Ok(json!({"database": path, "timezone": zone}))
    }
    pub fn open(path: &Path, now: DateTime<Utc>) -> Result<Self> {
        if !path.exists() {
            return Err(error("database_missing", "database does not exist"));
        }
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        let metadata = || -> rusqlite::Result<(i64, String, String)> {
            conn.query_row("SELECT version,timezone,identity FROM metadata", [], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
        };
        let (version, zone, identity) =
            metadata().map_err(|e| error("database_invalid", e.to_string()))?;
        if version != 1 {
            return Err(error("schema_unsupported", "unsupported schema version"));
        }
        let timezone = zone
            .parse()
            .map_err(|_| error("database_invalid", "invalid stored timezone"))?;
        let check = || -> rusqlite::Result<()> {
            if conn.query_row("SELECT count(*) FROM metadata", [], |r| r.get::<_, i64>(0))? != 1
                || identity.is_empty()
            {
                return Err(rusqlite::Error::InvalidQuery);
            }
            conn.prepare("SELECT id,language,created_at,predecessors,content,pinned,review_on,retired_at FROM records")?;
            conn.prepare("SELECT timestamp,predecessors,successors,retired FROM events")?;
            Ok(())
        };
        check().map_err(|e| error("database_invalid", e.to_string()))?;
        Ok(Self {
            conn,
            timezone,
            identity,
            now,
        })
    }
    pub fn today(&self) -> NaiveDate {
        self.now.with_timezone(&self.timezone).date_naive()
    }
    fn resolve(&self, review: &Review) -> Result<String> {
        match (&review.in_days, &review.on) {
            (Some(n), None) => {
                let d = self
                    .today()
                    .checked_add_days(Days::new(*n))
                    .ok_or_else(|| invalid("date overflow"))?;
                if !(1..=9999).contains(&d.year()) {
                    return Err(invalid("date overflow"));
                }
                Ok(d.to_string())
            }
            (None, Some(s)) => Ok(date(s)?.to_string()),
            _ => Err(invalid("review requires exactly one of in_days or on")),
        }
    }
    pub fn get(&self, lang: &str, ids: &[String]) -> Result<Value> {
        language(lang)?;
        if ids.is_empty() {
            return Err(invalid("at least one ID required"));
        }
        let records = ids
            .iter()
            .map(|id| read_record(&self.conn, lang, id))
            .collect::<Result<Vec<_>>>()?;
        Ok(json!({"records": records}))
    }
    pub fn replace(&mut self, lang: &str, request: Replacement) -> Result<Value> {
        self.replace_with_ids(lang, request, fresh_id)
    }
    /// ID injection allows deterministic collision tests; candidates must have the public format.
    pub fn replace_with_ids(
        &mut self,
        lang: &str,
        request: Replacement,
        mut generator: impl FnMut() -> String,
    ) -> Result<Value> {
        language(lang)?;
        if request.ids.is_empty() && request.replacements.is_empty() {
            return Err(invalid("empty replacement"));
        }
        unique(&request.ids)?;
        let reviews = request
            .replacements
            .iter()
            .map(|e| e.review.as_ref().map(|r| self.resolve(r)).transpose())
            .collect::<Result<Vec<_>>>()?;
        let timestamp = self.now.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let old = active_records(&tx, lang, &request.ids)?;
        let retired: Vec<Value> = old
            .iter()
            .map(|r| json!({"id":r.id,"previous_review_on":r.review_on,"previous_pinned":r.pinned}))
            .collect();
        let mut created = Vec::new();
        let mut unscheduled = Vec::new();
        for (entry, review) in request.replacements.iter().zip(reviews.iter()) {
            let id = loop {
                let id = generator();
                if id.len() < 6
                    || !id
                        .bytes()
                        .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
                {
                    return Err(invalid("invalid generated ID"));
                }
                if !tx.query_row(
                    "SELECT EXISTS(SELECT 1 FROM records WHERE id=?1)",
                    [&id],
                    |r| r.get::<_, bool>(0),
                )? {
                    break id;
                }
            };
            tx.execute(
                "INSERT INTO records VALUES (?1,?2,?3,?4,?5,?6,?7,NULL)",
                params![
                    id,
                    lang,
                    timestamp,
                    json!(request.ids).to_string(),
                    entry.content.to_string(),
                    entry.pinned,
                    review
                ],
            )?;
            if review.is_none() {
                unscheduled.push(id.clone());
            }
            created.push(id);
        }
        for id in &request.ids {
            tx.execute(
                "UPDATE records SET retired_at=?1,review_on=NULL WHERE id=?2",
                params![timestamp, id],
            )?;
        }
        tx.execute(
            "INSERT INTO events VALUES (?1,?2,?3,?4)",
            params![
                timestamp,
                json!(request.ids).to_string(),
                json!(created).to_string(),
                json!(retired).to_string()
            ],
        )?;
        tx.commit()?;
        let mut reminders = Vec::new();
        if old.iter().any(|r| r.review_on.is_some()) && !unscheduled.is_empty() {
            reminders.push("Choose review dates for unscheduled successors.");
        }
        if old.iter().any(|r| r.pinned) && !created.is_empty() {
            reminders.push("Consider pinning successor records.");
        }
        Ok(
            json!({"created":created,"retired":retired,"needs_review_schedule":unscheduled,"reminders":reminders}),
        )
    }
    pub fn schedule(
        &mut self,
        lang: &str,
        ids: &[String],
        review: Option<Review>,
    ) -> Result<Value> {
        language(lang)?;
        if ids.is_empty() {
            return Err(invalid("at least one ID required"));
        }
        unique(ids)?;
        let resolved = review.as_ref().map(|r| self.resolve(r)).transpose()?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        active_records(&tx, lang, ids)?;
        for id in ids {
            tx.execute(
                "UPDATE records SET review_on=?1 WHERE id=?2",
                params![resolved, id],
            )?;
        }
        tx.commit()?;
        Ok(
            json!({"records":ids.iter().map(|id|json!({"id":id,"review_on":resolved})).collect::<Vec<_>>()}),
        )
    }
    pub fn list(
        &self,
        lang: &str,
        filters: Filters,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<Value> {
        language(lang)?;
        check_limit(limit)?;
        if filters.due && filters.retired {
            return Err(invalid("due and retired are incompatible"));
        }
        let since = filters.since.as_deref().map(date).transpose()?;
        let (cutoff, offset) = if let Some(token) = cursor {
            let c: Cursor = serde_json::from_str(token).map_err(|_| invalid("malformed cursor"))?;
            if c.database != self.identity || c.language != lang || c.filters != filters {
                return Err(invalid("cursor database, language, or filters differ"));
            }
            (date(&c.cutoff)?, c.offset)
        } else {
            (self.today(), 0)
        };
        let mut records = self.all(lang)?;
        records.retain(|r| {
            let active = r.retired_at.is_none();
            if active == filters.retired {
                return false;
            }
            if filters.due
                && !r
                    .review_on
                    .as_ref()
                    .is_some_and(|d| d <= &cutoff.to_string())
            {
                return false;
            }
            if let Some(since) = since {
                let Ok(instant) = DateTime::parse_from_rfc3339(&r.created_at) else {
                    return false;
                };
                if instant.with_timezone(&self.timezone).date_naive() < since {
                    return false;
                }
            }
            if let Some(search) = &filters.search {
                if !r.content.to_string().contains(search) {
                    return false;
                }
            }
            true
        });
        if filters.due {
            records.sort_by(|a, b| {
                a.review_on
                    .cmp(&b.review_on)
                    .then(a.created_at.cmp(&b.created_at))
                    .then(a.id.cmp(&b.id))
            });
        } else {
            records.sort_by(|a, b| b.created_at.cmp(&a.created_at).then(a.id.cmp(&b.id)));
        }
        if offset > records.len() {
            return Err(invalid("cursor offset out of bounds"));
        }
        let end = offset.saturating_add(limit).min(records.len());
        let next = if end < records.len() {
            Some(
                serde_json::to_string(&Cursor {
                    database: self.identity.clone(),
                    language: lang.into(),
                    filters,
                    cutoff: cutoff.to_string(),
                    offset: end,
                })
                .unwrap(),
            )
        } else {
            None
        };
        Ok(json!({"records": &records[offset..end],"next_cursor":next}))
    }
    pub fn context(&self, lang: Option<&str>, limit: usize) -> Result<Value> {
        check_limit(limit)?;
        if let Some(lang) = lang {
            language(lang)?;
        }
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT language FROM records ORDER BY language")?;
        let languages = stmt
            .query_map([], |r| r.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut result = json!({"now":self.now.to_rfc3339_opts(chrono::SecondsFormat::Nanos,true),"today":self.today().to_string(),"timezone":self.timezone.to_string(),"languages":languages});
        if let Some(lang) = lang {
            let mut pinned = self
                .all(lang)?
                .into_iter()
                .filter(|r| r.pinned && r.retired_at.is_none())
                .collect::<Vec<_>>();
            pinned.sort_by(|a, b| b.created_at.cmp(&a.created_at).then(a.id.cmp(&b.id)));
            result["language"] = json!(lang);
            result["pinned"] = json!(pinned);
            result["due"] = self.list(
                lang,
                Filters {
                    due: true,
                    ..Default::default()
                },
                limit,
                None,
            )?;
            result["recent"] = self.list(lang, Filters::default(), limit, None)?;
        }
        Ok(result)
    }
    fn all(&self, lang: &str) -> Result<Vec<Record>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM records WHERE language=?1")?;
        let ids = stmt
            .query_map([lang], |r| r.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        ids.iter()
            .map(|id| read_record(&self.conn, lang, id))
            .collect()
    }
}
fn check_limit(n: usize) -> Result<()> {
    if !(1..=1000).contains(&n) {
        Err(invalid("limit must be 1..1000"))
    } else {
        Ok(())
    }
}
fn unique(ids: &[String]) -> Result<()> {
    if ids.iter().collect::<HashSet<_>>().len() != ids.len() {
        Err(invalid("duplicate IDs"))
    } else {
        Ok(())
    }
}
fn active_records(conn: &Connection, lang: &str, ids: &[String]) -> Result<Vec<Record>> {
    ids.iter()
        .map(|id| {
            let r = read_record(conn, lang, id)?;
            if r.retired_at.is_some() {
                Err(error("record_retired", format!("record {id} is retired")))
            } else {
                Ok(r)
            }
        })
        .collect()
}
fn read_record(conn: &Connection, lang: &str, id: &str) -> Result<Record> {
    let r = conn.query_row("SELECT id,language,created_at,predecessors,content,pinned,review_on,retired_at FROM records WHERE id=?1 AND language=?2",params![id,lang], |r| {
        let predecessors: String = r.get(3)?; let content: String = r.get(4)?;
        Ok((Record{id:r.get(0)?,language:r.get(1)?,created_at:r.get(2)?,replaces:Vec::new(),content:Value::Null,pinned:r.get(5)?,review_on:r.get(6)?,retired_at:r.get(7)?},predecessors,content))
    });
    match r {
        Ok((mut record, predecessors, content)) => {
            record.replaces = serde_json::from_str(&predecessors)
                .map_err(|_| error("database_invalid", "invalid stored predecessors"))?;
            record.content = parse_content(&content)
                .map_err(|_| error("database_invalid", "invalid stored content"))?;
            Ok(record)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Err(error(
            "record_missing",
            format!("record {id} not found in selected language"),
        )),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod initialization_tests {
    use super::*;
    #[test]
    fn partial_initialization_cleanup_and_existing_file_protection() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("db");
        let failure = Database::init_with(&path, "UTC", |_| {
            Err(error("storage_error", "injected failure"))
        });
        assert_eq!(failure.unwrap_err().code, "storage_error");
        assert!(!path.exists());
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
        std::fs::write(&path, b"keep me").unwrap();
        let failure = Database::init_with(&path, "UTC", |_| {
            panic!("must not initialize existing file")
        });
        assert_eq!(failure.unwrap_err().code, "storage_error");
        assert_eq!(std::fs::read(&path).unwrap(), b"keep me");
    }
}
