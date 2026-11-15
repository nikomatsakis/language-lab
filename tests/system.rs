use chrono::{DateTime, Utc};
use language_lab::{Database, Filters, Replacement, Review};
use serde_json::{json, Value};
use tempfile::TempDir;
fn instant(s: &str) -> DateTime<Utc> {
    s.parse().unwrap()
}
const NOW: &str = "2026-10-17T12:00:00Z";
fn setup(zone: &str) -> (TempDir, Database) {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("db");
    Database::init(&p, zone).unwrap();
    let db = Database::open(&p, instant(NOW)).unwrap();
    (dir, db)
}
fn request(v: Value) -> Replacement {
    serde_json::from_value(v).unwrap()
}
fn add(db: &mut Database, lang: &str, content: Value) -> String {
    db.replace(
        lang,
        request(json!({"ids":[],"replacements":[{"content":content}]})),
    )
    .unwrap()["created"][0]
        .as_str()
        .unwrap()
        .into()
}
fn record(db: &Database, lang: &str, id: &str) -> Value {
    db.get(lang, &[id.into()]).unwrap()["records"][0].clone()
}
fn review(days: u64) -> Option<Review> {
    Some(Review {
        in_days: Some(days),
        on: None,
    })
}
/// anchorbook: A-cli-init-response#behavior
/// anchorbook: A-database-sqlite#behavior
/// anchorbook: A-database-init-exclusive#behavior
/// anchorbook: A-database-open-existing#behavior
/// anchorbook: A-database-timezone#behavior
/// anchorbook: A-database-version#behavior
/// anchorbook: A-database-single-learner#behavior
/// anchorbook: A-database-invalid#behavior
#[test]
fn storage_creation_validation_and_persistence() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("db");
    assert_eq!(
        Database::open(&p, instant(NOW)).err().unwrap().code,
        "database_missing"
    );
    assert!(!p.exists());
    assert!(Database::init(&p, "bad/zone").is_err());
    assert!(!p.exists());
    let out = Database::init(&p, "Europe/Berlin").unwrap();
    assert_eq!(out, json!({"database":p,"timezone":"Europe/Berlin"}));
    let bytes = std::fs::read(&p).unwrap();
    assert!(bytes.starts_with(b"SQLite format 3\0"));
    assert_eq!(Database::init(&p, "UTC").unwrap_err().code, "storage_error");
    assert_eq!(bytes, std::fs::read(&p).unwrap());
    let mut db = Database::open(&p, instant(NOW)).unwrap();
    let id = add(&mut db, "el", json!("persisted"));
    drop(db);
    let db = Database::open(&p, instant(NOW)).unwrap();
    assert_eq!(record(&db, "el", &id)["content"], "persisted");
    drop(db);
    let c = rusqlite::Connection::open(&p).unwrap();
    c.execute("UPDATE metadata SET version=2", []).unwrap();
    drop(c);
    let bytes = std::fs::read(&p).unwrap();
    assert_eq!(
        Database::open(&p, instant(NOW)).err().unwrap().code,
        "schema_unsupported"
    );
    assert_eq!(bytes, std::fs::read(&p).unwrap());
    for contents in [b"not sqlite".as_slice(), b"".as_slice()] {
        let other = dir.path().join("invalid");
        std::fs::write(&other, contents).unwrap();
        assert_eq!(
            Database::open(&other, instant(NOW)).err().unwrap().code,
            "database_invalid"
        );
        assert_eq!(std::fs::read(&other).unwrap(), contents);
    }
    let other = dir.path().join("foreign");
    let c = rusqlite::Connection::open(&other).unwrap();
    c.execute("CREATE TABLE unrelated (x)", []).unwrap();
    drop(c);
    let bytes = std::fs::read(&other).unwrap();
    assert_eq!(
        Database::open(&other, instant(NOW)).err().unwrap().code,
        "database_invalid"
    );
    assert_eq!(std::fs::read(&other).unwrap(), bytes);
}
/// anchorbook: A-language-namespace#behavior
/// anchorbook: A-language-exact#behavior
/// anchorbook: A-language-implicit#behavior
/// anchorbook: A-record-identity-unique#behavior
/// anchorbook: A-record-identity-format#behavior
/// anchorbook: A-record-identity-collision#behavior
/// anchorbook: A-record-content-json#behavior
/// anchorbook: A-record-content-opaque#behavior
/// anchorbook: A-record-content-roundtrip#behavior
/// anchorbook: A-record-created#behavior
/// anchorbook: A-record-envelope#behavior
/// anchorbook: A-record-pin-default#behavior
#[test]
fn json_roundtrip_scope_and_collision() {
    let (_dir, mut db) = setup("UTC");
    assert!(db.context(Some(""), 20).is_err());
    assert!(db.list("", Filters::default(), 20, None).is_err());
    assert_eq!(
        db.list("empty", Filters::default(), 20, None).unwrap()["records"],
        json!([])
    );
    let values = vec![
        Value::Null,
        json!(true),
        json!(42),
        json!(2.5),
        json!("πήγα"),
        json!([1, null]),
        json!({"kind":"practice","related_to":["nonexistent"],"z":{"b":2,"a":1}}),
    ];
    for v in values {
        let id = add(&mut db, "el", v.clone());
        assert!(
            id.len() >= 6
                && id
                    .bytes()
                    .all(|x| x.is_ascii_hexdigit() && !x.is_ascii_uppercase())
        );
        let r = record(&db, "el", &id);
        assert_eq!(r["content"], v);
        assert_eq!(r["pinned"], false);
        assert!(r["review_on"].is_null());
        assert_eq!(r["created_at"], "2026-10-17T12:00:00.000000000Z");
        assert_eq!(r.as_object().unwrap().len(), 8);
        assert_eq!(
            db.get("EL", std::slice::from_ref(&id)).unwrap_err().code,
            "record_missing"
        );
    }
    let mut ids = ["abcdef", "abcdef", "123456"].into_iter();
    let a = db
        .replace_with_ids(
            "EL",
            request(json!({"ids":[],"replacements":[{"content":null},{"content":false}]})),
            || ids.next().unwrap().into(),
        )
        .unwrap();
    assert_eq!(a["created"], json!(["abcdef", "123456"]));
    db.replace("EL", request(json!({"ids":["abcdef"],"replacements":[]})))
        .unwrap();
    let mut ids = ["abcdef", "fedcba"].into_iter();
    assert_eq!(
        db.replace_with_ids(
            "EL",
            request(json!({"ids":[],"replacements":[{"content":null}]})),
            || ids.next().unwrap().into()
        )
        .unwrap()["created"],
        json!(["fedcba"])
    );
    assert_eq!(
        db.context(None, 20).unwrap()["languages"],
        json!(["EL", "el"])
    );
}
/// anchorbook: A-replace-fresh-ids#behavior
/// anchorbook: A-replace-backlinks#behavior
/// anchorbook: A-replace-retire#behavior
/// anchorbook: A-replace-event#behavior
/// anchorbook: A-replace-no-inheritance#behavior
/// anchorbook: A-replace-successor-review#behavior
/// anchorbook: A-replace-unscheduled-valid#behavior
/// anchorbook: A-replace-response-created#behavior
/// anchorbook: A-replace-response-retired#behavior
/// anchorbook: A-replace-response-unscheduled#behavior
/// anchorbook: A-replace-response-review-reminder#behavior
/// anchorbook: A-replace-response-pin-reminder#behavior
/// anchorbook: A-replace-response-reminders#behavior
/// anchorbook: A-record-content-immutable#behavior
/// anchorbook: A-record-active#behavior
/// anchorbook: A-record-history-readable#behavior
/// anchorbook: A-record-pin-explicit#behavior
/// anchorbook: A-record-pin-immutable#behavior
#[test]
fn replacement_lifecycle_handoff_events_and_immutable_history() {
    let (dir, mut db) = setup("UTC");
    let a=db.replace("el",request(json!({"ids":[],"replacements":[{"content":"original","pinned":true,"review":{"in_days":0}}]}))).unwrap()["created"][0].as_str().unwrap().to_owned();
    let original = record(&db, "el", &a);
    let result=db.replace("el",request(json!({"ids":[a],"replacements":[{"content":"understood","pinned":true,"review":{"in_days":7}},{"content":"needs work","review":null}]}))).unwrap();
    let ids: Vec<String> = serde_json::from_value(result["created"].clone()).unwrap();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], a);
    assert_ne!(ids[0], ids[1]);
    assert_eq!(
        result["retired"],
        json!([{"id":a,"previous_review_on":"2026-10-17","previous_pinned":true}])
    );
    assert_eq!(result["needs_review_schedule"], json!([ids[1]]));
    assert_eq!(result["reminders"].as_array().unwrap().len(), 2);
    for id in &ids {
        assert_eq!(record(&db, "el", id)["replaces"], json!([a]));
    }
    assert_eq!(record(&db, "el", &ids[0])["review_on"], "2026-10-24");
    assert_eq!(record(&db, "el", &ids[0])["pinned"], true);
    assert_eq!(record(&db, "el", &ids[1])["pinned"], false);
    let old = record(&db, "el", &a);
    for key in [
        "id",
        "language",
        "created_at",
        "content",
        "replaces",
        "pinned",
    ] {
        assert_eq!(old[key], original[key]);
    }
    assert_eq!(old["retired_at"], original["created_at"]);
    assert!(old["review_on"].is_null());
    let merged = db
        .replace(
            "el",
            request(json!({"ids":[ids[1],ids[0]],"replacements":[{"content":"merged"}]})),
        )
        .unwrap();
    let m = merged["created"][0].as_str().unwrap();
    assert_eq!(record(&db, "el", m)["replaces"], json!([ids[1], ids[0]]));
    let rewritten = db
        .replace(
            "el",
            request(json!({"ids":[m],"replacements":[{"content":"merged"}]})),
        )
        .unwrap();
    let n = rewritten["created"][0].as_str().unwrap();
    assert_ne!(n, m);
    let deleted = db
        .replace("el", request(json!({"ids":[n],"replacements":[]})))
        .unwrap();
    assert_eq!(deleted["created"], json!([]));
    assert_eq!(deleted["reminders"], json!([]));
    assert_eq!(
        db.list("el", Filters::default(), 20, None).unwrap()["records"],
        json!([])
    );
    let c = rusqlite::Connection::open(dir.path().join("db")).unwrap();
    let events: i64 = c
        .query_row("SELECT count(*) FROM events", [], |r| r.get(0))
        .unwrap();
    assert_eq!(events, 5);
    let (pred, succ): (String, String) = c
        .query_row(
            "SELECT predecessors,successors FROM events ORDER BY rowid DESC LIMIT 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(serde_json::from_str::<Value>(&pred).unwrap(), json!([n]));
    assert_eq!(succ, "[]");
}
/// anchorbook: A-replace-types#behavior
/// anchorbook: A-replace-input#behavior
/// anchorbook: A-replace-nonempty#behavior
/// anchorbook: A-replace-unique-inputs#behavior
/// anchorbook: A-replace-active-inputs#behavior
/// anchorbook: A-replace-atomic#behavior
/// anchorbook: A-cli-strict-envelope#behavior
#[test]
fn invalid_requests_and_atomic_failures() {
    let (dir, mut db) = setup("UTC");
    let id = add(&mut db, "el", json!("a"));
    db.schedule("el", std::slice::from_ref(&id), review(2))
        .unwrap();
    let baseline = db.context(Some("el"), 20).unwrap();
    for bad in [
        json!({}),
        json!({"ids":[],"replacements":[{}]}),
        json!({"ids":[1],"replacements":[]}),
        json!({"ids":[],"replacements":[{"content":1,"pinned":null}]}),
        json!({"ids":[],"replacements":[{"content":1,"extra":true}]}),
        json!({"ids":[],"replacements":[],"extra":1}),
        json!({"ids":[],"replacements":[{"content":1,"review":{"in_days":-1}}]}),
        json!({"ids":[],"replacements":[{"content":1,"review":{"in_days":1.5}}]}),
        json!({"ids":[],"replacements":[{"content":1,"review":{"extra":3}}]}),
    ] {
        assert!(serde_json::from_value::<Replacement>(bad).is_err());
    }
    for bad in [
        json!({"ids":[],"replacements":[]}),
        json!({"ids":[id,id],"replacements":[]}),
        json!({"ids":[id,"missing"],"replacements":[{"content":1}]}),
        json!({"ids":[id],"replacements":[{"content":1,"review":{}}]}),
        json!({"ids":[id],"replacements":[{"content":1,"review":{"in_days":1,"on":"2026-01-01"}}]}),
    ] {
        assert!(db.replace("el", request(bad)).is_err());
        assert_eq!(db.context(Some("el"), 20).unwrap(), baseline);
    }
    assert_eq!(
        db.replace("fr", request(json!({"ids":[id],"replacements":[]})))
            .unwrap_err()
            .code,
        "record_missing"
    );
    let c = rusqlite::Connection::open(dir.path().join("db")).unwrap();
    c.execute_batch("CREATE TRIGGER fail_event BEFORE INSERT ON events BEGIN SELECT RAISE(ABORT,'simulated storage failure'); END;").unwrap();
    assert_eq!(
        db.replace(
            "el",
            request(json!({"ids":[id],"replacements":[{"content":"b"}]}))
        )
        .unwrap_err()
        .code,
        "storage_error"
    );
    assert_eq!(db.context(Some("el"), 20).unwrap(), baseline);
    c.execute_batch("DROP TRIGGER fail_event;").unwrap();
    db.replace("el", request(json!({"ids":[id],"replacements":[]})))
        .unwrap();
    assert_eq!(
        db.replace("el", request(json!({"ids":[id],"replacements":[]})))
            .unwrap_err()
            .code,
        "record_retired"
    );
}
/// anchorbook: A-replace-concurrent#behavior
#[test]
fn concurrent_replacement_has_one_winner() {
    let (dir, mut db) = setup("UTC");
    let id = add(&mut db, "el", json!(1));
    drop(db);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let handles = (0..2)
        .map(|_| {
            let p = dir.path().join("db");
            let id = id.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut db = Database::open(&p, instant(NOW)).unwrap();
                barrier.wait();
                db.replace(
                    "el",
                    request(json!({"ids":[id],"replacements":[{"content":2}]})),
                )
            })
        })
        .collect::<Vec<_>>();
    let results = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter_map(|r| r.as_ref().err())
            .next()
            .unwrap()
            .code,
        "record_retired"
    );
}
/// anchorbook: A-time-today#behavior
/// anchorbook: A-time-relative#behavior
/// anchorbook: A-time-interval-domain#behavior
/// anchorbook: A-time-date-format#behavior
/// anchorbook: A-time-overflow#behavior
/// anchorbook: A-time-due#behavior
/// anchorbook: A-schedule-one-date#behavior
/// anchorbook: A-schedule-identity#behavior
/// anchorbook: A-schedule-overwrite#behavior
/// anchorbook: A-schedule-clear#behavior
#[test]
fn calendar_days_dates_and_schedule_identity() {
    for (zone, now, today, next) in [
        (
            "Europe/Berlin",
            "2026-03-28T23:30:00Z",
            "2026-03-29",
            "2026-03-30",
        ),
        (
            "Europe/Berlin",
            "2026-10-24T22:30:00Z",
            "2026-10-25",
            "2026-10-26",
        ),
        ("UTC", "2024-02-28T12:00:00Z", "2024-02-28", "2024-02-29"),
        (
            "Pacific/Apia",
            "2011-12-29T12:00:00Z",
            "2011-12-29",
            "2011-12-30",
        ),
    ] {
        let (dir, _) = setup(zone);
        let mut db = Database::open(&dir.path().join("db"), instant(now)).unwrap();
        assert_eq!(db.today().to_string(), today);
        let id = add(&mut db, "el", json!(1));
        let original = record(&db, "el", &id);
        assert_eq!(
            db.schedule("el", std::slice::from_ref(&id), review(1))
                .unwrap()["records"][0]["review_on"],
            next
        );
        db.schedule("el", std::slice::from_ref(&id), review(0))
            .unwrap();
        assert_eq!(
            db.list(
                "el",
                Filters {
                    due: true,
                    ..Default::default()
                },
                20,
                None
            )
            .unwrap()["records"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        db.schedule("el", std::slice::from_ref(&id), None).unwrap();
        db.schedule("el", std::slice::from_ref(&id), None).unwrap();
        assert_eq!(record(&db, "el", &id), original);
    }
    let (dir, mut db) = setup("UTC");
    let id = add(&mut db, "el", json!(1));
    for s in [
        "0000-01-01",
        "10000-01-01",
        "2026-2-01",
        "2026-02-29",
        "2026-13-01",
        "2026-01-01x",
    ] {
        assert!(db
            .schedule(
                "el",
                std::slice::from_ref(&id),
                Some(Review {
                    on: Some(s.into()),
                    in_days: None
                })
            )
            .is_err());
    }
    for s in ["0001-01-01", "9999-12-31", "2024-02-29"] {
        assert_eq!(
            db.schedule(
                "el",
                std::slice::from_ref(&id),
                Some(Review {
                    on: Some(s.into()),
                    in_days: None
                })
            )
            .unwrap()["records"][0]["review_on"],
            s
        );
    }
    let before = record(&db, "el", &id);
    assert!(db
        .schedule("el", std::slice::from_ref(&id), review(u64::MAX))
        .is_err());
    assert_eq!(record(&db, "el", &id), before);
    let mut db = Database::open(&dir.path().join("db"), instant("9999-12-31T12:00:00Z")).unwrap();
    assert!(db
        .schedule("el", std::slice::from_ref(&id), review(1))
        .is_err());
    assert_eq!(record(&db, "el", &id), before);
}
/// anchorbook: A-schedule-active#behavior
/// anchorbook: A-schedule-atomic#behavior
/// anchorbook: A-schedule-response#behavior
/// anchorbook: A-schedule-no-auto-review#behavior
#[test]
fn scheduling_atomic_and_passive_reads() {
    let (dir, mut db) = setup("UTC");
    let a = add(&mut db, "el", json!(1));
    let b = add(&mut db, "el", json!(2));
    let foreign = add(&mut db, "fr", json!(3));
    let ids = vec![b.clone(), a.clone()];
    let out = db.schedule("el", &ids, review(0)).unwrap();
    assert_eq!(
        out["records"],
        json!([{"id":b,"review_on":"2026-10-17"},{"id":a,"review_on":"2026-10-17"}])
    );
    let before = record(&db, "el", &a);
    for ids in [
        vec![],
        vec![a.clone(), a.clone()],
        vec![a.clone(), "missing".into()],
        vec![a.clone(), foreign],
    ] {
        assert!(db.schedule("el", &ids, review(5)).is_err());
        assert_eq!(record(&db, "el", &a), before);
    }
    let c = rusqlite::Connection::open(dir.path().join("db")).unwrap();
    c.execute_batch(&format!("CREATE TRIGGER fail_update BEFORE UPDATE ON records WHEN OLD.id='{a}' BEGIN SELECT RAISE(ABORT,'failure'); END;")).unwrap();
    assert_eq!(
        db.schedule("el", &ids, review(5)).unwrap_err().code,
        "storage_error"
    );
    assert_eq!(record(&db, "el", &b)["review_on"], "2026-10-17");
    c.execute_batch("DROP TRIGGER fail_update;").unwrap();
    db.context(Some("el"), 1).unwrap();
    db.list(
        "el",
        Filters {
            due: true,
            ..Default::default()
        },
        20,
        None,
    )
    .unwrap();
    add(&mut db, "el", json!({"kind":"practice","related_to":[a]}));
    assert_eq!(record(&db, "el", &a), before);
    db.replace("el", request(json!({"ids":[b],"replacements":[]})))
        .unwrap();
    assert_eq!(
        db.schedule("el", &[a.clone(), b], None).unwrap_err().code,
        "record_retired"
    );
    assert_eq!(record(&db, "el", &a), before);
}
/// anchorbook: A-get-exact#behavior
/// anchorbook: A-get-missing#behavior
/// anchorbook: A-list-active-default#behavior
/// anchorbook: A-list-retired#behavior
/// anchorbook: A-list-due#behavior
/// anchorbook: A-list-incompatible#behavior
/// anchorbook: A-list-search#behavior
/// anchorbook: A-list-intersection#behavior
/// anchorbook: A-list-pagination#behavior
/// anchorbook: A-list-cursor#behavior
/// anchorbook: A-list-cursor-invalid#behavior
/// anchorbook: A-list-cursor-database#behavior
/// anchorbook: A-context-clock#behavior
/// anchorbook: A-context-unselected#behavior
/// anchorbook: A-context-pinned#behavior
/// anchorbook: A-context-due#behavior
/// anchorbook: A-context-recent#behavior
/// anchorbook: A-context-continuation#behavior
/// anchorbook: A-context-pin-limit#behavior
/// anchorbook: A-context-read-only#behavior
/// anchorbook: A-context-shape#behavior
#[test]
fn queries_pagination_and_context() {
    let (dir, mut db) = setup("Europe/Berlin");
    let mut generated = ["aaaaaa", "bbbbbb", "cccccc", "dddddd"].into_iter();
    db.replace_with_ids("el",request(json!({"ids":[],"replacements":[{"content":{"z":"πήγα","a":{"z":2,"a":1}},"pinned":true,"review":{"in_days":0}},{"content":"Alpha","pinned":true,"review":{"on":"2026-10-16"}},{"content":"alpha","pinned":true,"review":{"in_days":1}},{"content":"other"}]})),||generated.next().unwrap().into()).unwrap();
    let a = "aaaaaa".to_string();
    let b = "bbbbbb".to_string();
    assert_eq!(
        db.get("el", &[b.clone(), a.clone(), b.clone()]).unwrap()["records"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["bbbbbb", "aaaaaa", "bbbbbb"]
    );
    assert_eq!(
        db.get("el", &[a.clone(), "missing".into()])
            .unwrap_err()
            .code,
        "record_missing"
    );
    for limit in [0, 1001] {
        assert!(db.list("el", Filters::default(), limit, None).is_err());
        assert!(db.context(Some("el"), limit).is_err());
        assert!(db.context(None, limit).is_err());
    }
    for limit in [1, 1000] {
        assert!(db.list("el", Filters::default(), limit, None).is_ok());
    }
    assert!(db
        .list(
            "el",
            Filters {
                due: true,
                retired: true,
                ..Default::default()
            },
            20,
            None
        )
        .is_err());
    let sorted = db.list("el", Filters::default(), 20, None).unwrap();
    assert_eq!(
        sorted["records"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["aaaaaa", "bbbbbb", "cccccc", "dddddd"]
    );
    assert!(sorted["next_cursor"].is_null());
    let due = Filters {
        due: true,
        ..Default::default()
    };
    let page = db.list("el", due.clone(), 1, None).unwrap();
    assert_eq!(page["records"][0]["id"], b);
    let cursor = page["next_cursor"].as_str().unwrap();
    let tomorrow = Database::open(&dir.path().join("db"), instant("2026-10-18T12:00:00Z")).unwrap();
    let second = tomorrow
        .list("el", due.clone(), 1000, Some(cursor))
        .unwrap();
    assert_eq!(second["records"].as_array().unwrap().len(), 1);
    assert_eq!(second["records"][0]["id"], a);
    assert!(second["next_cursor"].is_null());
    assert_eq!(
        tomorrow.list("el", due.clone(), 20, None).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    let copy = dir.path().join("copy");
    std::fs::copy(dir.path().join("db"), &copy).unwrap();
    assert!(Database::open(&copy, instant(NOW))
        .unwrap()
        .list("el", due.clone(), 1, Some(cursor))
        .is_ok());
    let (_other, other) = setup("Europe/Berlin");
    assert_eq!(
        other
            .list("el", due.clone(), 1, Some(cursor))
            .unwrap_err()
            .code,
        "invalid_input"
    );
    assert!(db.list("EL", due.clone(), 1, Some(cursor)).is_err());
    assert!(db.list("el", Filters::default(), 1, Some(cursor)).is_err());
    for bad in ["", "{}", "null", "junk"] {
        assert!(db.list("el", due.clone(), 1, Some(bad)).is_err());
    }
    for (search, count) in [
        ("Alpha", 1),
        ("alpha", 1),
        ("πήγα", 1),
        ("\"a\":{\"a\":1,\"z\":2}", 1),
        ("%", 0),
    ] {
        assert_eq!(
            db.list(
                "el",
                Filters {
                    search: Some(search.into()),
                    ..Default::default()
                },
                20,
                None
            )
            .unwrap()["records"]
                .as_array()
                .unwrap()
                .len(),
            count
        );
    }
    let intersection = db
        .list(
            "el",
            Filters {
                due: true,
                since: Some("2026-10-17".into()),
                search: Some("Alpha".into()),
                retired: false,
            },
            20,
            None,
        )
        .unwrap();
    assert_eq!(intersection["records"].as_array().unwrap().len(), 1);
    assert_eq!(intersection["records"][0]["id"], b);
    assert_eq!(
        db.list(
            "el",
            Filters {
                since: Some("2026-10-18".into()),
                ..Default::default()
            },
            20,
            None
        )
        .unwrap()["records"],
        json!([])
    );
    let bytes = std::fs::read(dir.path().join("db")).unwrap();
    let ctx = db.context(Some("el"), 1).unwrap();
    assert_eq!(ctx["pinned"].as_array().unwrap().len(), 3);
    assert_eq!(ctx["pinned"][0]["id"], a);
    assert_eq!(ctx["recent"]["records"].as_array().unwrap().len(), 1);
    assert_eq!(ctx["due"], page);
    assert!(ctx["recent"]["next_cursor"].is_string());
    assert_eq!(ctx["language"], "el");
    assert_eq!(ctx["timezone"], "Europe/Berlin");
    assert_eq!(ctx["today"], "2026-10-17");
    assert_eq!(ctx["now"], "2026-10-17T12:00:00.000000000Z");
    assert_eq!(std::fs::read(dir.path().join("db")).unwrap(), bytes);
    let unselected = db.context(None, 20).unwrap();
    assert_eq!(unselected.as_object().unwrap().len(), 4);
    for key in ["language", "pinned", "due", "recent"] {
        assert!(unselected.get(key).is_none());
    }
    db.replace("el", request(json!({"ids":[a],"replacements":[]})))
        .unwrap();
    let retired = db
        .list(
            "el",
            Filters {
                retired: true,
                ..Default::default()
            },
            20,
            None,
        )
        .unwrap();
    assert_eq!(retired["records"].as_array().unwrap().len(), 1);
    assert_eq!(retired["records"][0]["id"], a);
    assert_eq!(
        db.context(Some("el"), 1).unwrap()["pinned"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}
/// anchorbook: A-list-since#behavior
/// anchorbook: A-list-order#behavior
/// anchorbook: A-list-due-order#behavior
#[test]
fn local_since_and_order_across_instants() {
    let (dir, mut db) = setup("Europe/Berlin");
    let old = add(&mut db, "el", json!("old"));
    drop(db);
    let mut db = Database::open(&dir.path().join("db"), instant("2026-10-17T22:30:00Z")).unwrap();
    let new = add(&mut db, "el", json!("new"));
    assert_eq!(
        db.list(
            "el",
            Filters {
                since: Some("2026-10-18".into()),
                ..Default::default()
            },
            20,
            None
        )
        .unwrap()["records"][0]["id"],
        new
    );
    let all = db.list("el", Filters::default(), 1, None).unwrap();
    assert_eq!(all["records"][0]["id"], new);
    assert_eq!(
        db.list("el", Filters::default(), 1, all["next_cursor"].as_str())
            .unwrap()["records"][0]["id"],
        old
    );
    db.schedule("el", &[new, old.clone()], review(0)).unwrap();
    assert_eq!(
        db.list(
            "el",
            Filters {
                due: true,
                ..Default::default()
            },
            20,
            None
        )
        .unwrap()["records"][0]["id"],
        old
    );
}
/// anchorbook: A-schedule-entry#behavior
#[test]
fn review_null_keys_are_not_missing_keys() {
    for review in [
        json!({"in_days":null}),
        json!({"on":null}),
        json!({"in_days":null,"on":"2026-10-22"}),
        json!({"in_days":1,"on":null}),
    ] {
        assert!(serde_json::from_value::<Replacement>(
            json!({"ids":[],"replacements":[{"content":null,"review":review}]})
        )
        .is_err());
    }
}
/// anchorbook: A-context-languages#behavior
/// anchorbook: A-replace-cancel-review#behavior
/// anchorbook: A-language-isolation#behavior
/// anchorbook: A-language-empty#behavior
#[test]
fn scope_context_empty_and_events_preserve_handoff() {
    let (dir, mut db) = setup("UTC");
    let foreign=db.replace("fr",request(json!({"ids":[],"replacements":[{"content":"secret","pinned":true,"review":{"in_days":0}}]}))).unwrap()["created"][0].as_str().unwrap().to_owned();
    let empty = db.context(Some("el"), 20).unwrap();
    for section in ["recent", "due"] {
        assert_eq!(empty[section]["records"], json!([]));
        assert!(empty[section]["next_cursor"].is_null());
    }
    assert_eq!(empty["pinned"], json!([]));
    assert_eq!(
        db.list("el", Filters::default(), 20, None).unwrap()["records"],
        json!([])
    );
    let own = add(&mut db, "el", json!("local"));
    let before = record(&db, "fr", &foreign);
    assert_eq!(
        db.replace(
            "el",
            request(json!({"ids":[own,foreign],"replacements":[{"content":null}]}))
        )
        .unwrap_err()
        .code,
        "record_missing"
    );
    assert_eq!(record(&db, "fr", &foreign), before);
    assert_eq!(record(&db, "el", &own)["retired_at"], Value::Null);
    db.replace("fr", request(json!({"ids":[foreign],"replacements":[]})))
        .unwrap();
    let c = rusqlite::Connection::open(dir.path().join("db")).unwrap();
    let (timestamp, retired): (String, String) = c
        .query_row(
            "SELECT timestamp,retired FROM events ORDER BY rowid DESC LIMIT 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(timestamp, "2026-10-17T12:00:00.000000000Z");
    assert_eq!(
        serde_json::from_str::<Value>(&retired).unwrap(),
        json!([{"id":foreign,"previous_pinned":true,"previous_review_on":"2026-10-17"}])
    );
    assert_eq!(
        db.context(None, 20).unwrap()["languages"],
        json!(["el", "fr"])
    );
}
#[test]
fn calendar_since_boundaries() {
    for (zone, now, since, included) in [
        ("Pacific/Apia", "2011-12-30T09:59:59Z", "2011-12-30", false),
        ("Pacific/Apia", "2011-12-30T10:00:00Z", "2011-12-30", true),
        (
            "America/Sao_Paulo",
            "2018-11-04T02:59:59Z",
            "2018-11-04",
            false,
        ),
        (
            "America/Sao_Paulo",
            "2018-11-04T03:00:00Z",
            "2018-11-04",
            true,
        ),
        ("UTC", "2024-02-28T23:59:59Z", "2024-02-29", false),
        ("UTC", "2024-02-29T00:00:00Z", "2024-02-29", true),
    ] {
        let (dir, _) = setup(zone);
        let mut db = Database::open(&dir.path().join("db"), instant(now)).unwrap();
        add(&mut db, "el", json!("boundary"));
        let result = db
            .list(
                "el",
                Filters {
                    since: Some(since.into()),
                    ..Default::default()
                },
                20,
                None,
            )
            .unwrap();
        assert_eq!(
            result["records"].as_array().unwrap().len(),
            usize::from(included),
            "{zone} {now}"
        );
    }
}
#[test]
fn cursor_semantic_validation_and_due_id_ties() {
    let (_dir, mut db) = setup("UTC");
    let mut ids = ["cccccc", "aaaaaa", "bbbbbb"].into_iter();
    db.replace_with_ids("el",request(json!({"ids":[],"replacements":[{"content":1,"review":{"in_days":0}},{"content":2,"review":{"in_days":0}},{"content":3,"review":{"in_days":0}}]})),||ids.next().unwrap().into()).unwrap();
    let filter = Filters {
        due: true,
        ..Default::default()
    };
    let page = db.list("el", filter.clone(), 1, None).unwrap();
    assert_eq!(page["records"][0]["id"], "aaaaaa");
    let cursor = page["next_cursor"].as_str().unwrap();
    let next = db.list("el", filter.clone(), 1, Some(cursor)).unwrap();
    assert_eq!(next["records"][0]["id"], "bbbbbb");
    let last = db
        .list("el", filter.clone(), 1, next["next_cursor"].as_str())
        .unwrap();
    assert_eq!(last["records"][0]["id"], "cccccc");
    assert!(last["next_cursor"].is_null());
    for (key, value) in [
        ("cutoff", json!("2026-02-30")),
        ("offset", json!(999)),
        ("offset", json!(-1)),
    ] {
        let mut bad: Value = serde_json::from_str(cursor).unwrap();
        bad[key] = value;
        assert_eq!(
            db.list("el", filter.clone(), 1, Some(&bad.to_string()))
                .unwrap_err()
                .code,
            "invalid_input"
        );
    }
}
#[test]
fn initialization_filesystem_and_malformed_schema() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(
        Database::init(dir.path(), "UTC").unwrap_err().code,
        "storage_error"
    );
    assert!(dir.path().is_dir());
    let missing = dir.path().join("missing/db");
    assert_eq!(
        Database::init(&missing, "UTC").unwrap_err().code,
        "storage_error"
    );
    assert!(!missing.exists());
    let p = dir.path().join("db");
    Database::init(&p, "UTC").unwrap();
    let c = rusqlite::Connection::open(&p).unwrap();
    c.execute("DROP TABLE events", []).unwrap();
    drop(c);
    let bytes = std::fs::read(&p).unwrap();
    assert_eq!(
        Database::open(&p, instant(NOW)).err().unwrap().code,
        "database_invalid"
    );
    assert_eq!(std::fs::read(&p).unwrap(), bytes);
}
