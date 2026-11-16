"""Deterministic tests for the live harness's verdict logic; no model calls."""
import importlib.util
import json
from pathlib import Path
import sqlite3
import tempfile
import argparse
import shutil
import sys
import unittest

spec = importlib.util.spec_from_file_location("e2e", Path(__file__).with_name("run.py"))
e2e = importlib.util.module_from_spec(spec)
spec.loader.exec_module(e2e)


class VerdictTests(unittest.TestCase):
    def test_command_timeout_fails_instead_of_hanging(self):
        harness = e2e.Harness(argparse.Namespace(mode="install", timeout=10))
        try:
            with self.assertRaisesRegex(RuntimeError, "timed out"):
                harness.command("timeout", [sys.executable, "-c", "import time; time.sleep(10)"],
                                timeout=0.05)
        finally:
            shutil.rmtree(harness.base)

    def test_events_require_completion_and_reject_failed_turns(self):
        complete = json.dumps({"type": "turn.completed"})
        self.assertEqual(len(e2e.events_from("srt diagnostic\n" + complete)), 1)
        for text in ["not json", '{"type":"turn.started"}',
                     complete + '\n{"type":"turn.failed"}',
                     complete + '\n{"type":"error"}']:
            with self.assertRaises(RuntimeError):
                e2e.events_from(text)

    def test_only_assistant_messages_count_as_replies(self):
        events = [{"item": {"type": "command_execution", "text": "database?"}},
                  {"item": {"type": "agent_message", "text": "Which database?"}}]
        self.assertEqual(e2e.assistant_text(events), "Which database?")

    def test_database_requires_matching_timezone_and_active_spanish_preferences(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "learner.sqlite"
            with self.assertRaises(RuntimeError):
                e2e.check_database(path)
            with sqlite3.connect(path) as db:
                db.executescript('CREATE TABLE metadata(timezone TEXT);'
                                 'INSERT INTO metadata VALUES("UTC");'
                                 'CREATE TABLE records(language TEXT,pinned INTEGER,retired_at TEXT,content TEXT);')
            with self.assertRaises(RuntimeError):
                e2e.check_database(path)
            with sqlite3.connect(path) as db:
                db.execute('UPDATE metadata SET timezone="Europe/Berlin"')
                db.executemany('INSERT INTO records VALUES(?,?,?,?)',
                               [("fr", 1, None, '{}'), ("es", 0, None, '{}'),
                                ("es", 1, "yesterday", '{}')])
            with self.assertRaises(RuntimeError):
                e2e.check_database(path)
            with sqlite3.connect(path) as db:
                db.execute('INSERT INTO records VALUES("es",1,NULL,?)',
                           (json.dumps({"goals": "travel", "level": "beginner"}),))
            self.assertEqual(e2e.check_database(path)["active_pinned_spanish_records"], 1)


if __name__ == "__main__":
    unittest.main()
