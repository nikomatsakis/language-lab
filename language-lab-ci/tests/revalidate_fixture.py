"""Prepare isolated cases for a fresh agent to exercise the revalidate skill.

Run with an empty destination directory. Reviewers should see the requirement
before opening implementation/tests. This script does not write attestations.
"""
import json
from pathlib import Path
import shutil
import sys

repo = Path(__file__).resolve().parents[2]
destination = Path(sys.argv[1])
destination.mkdir(parents=True, exist_ok=False)
for case, implementation, cases in [
    ("complete", "return max(0, min(10, value))", [(-10**40, 0), (-1, 0), (0, 0), (1, 1), (9, 9), (10, 10), (11, 10), (10**40, 10)]),
    ("missing-tests", "return max(0, min(10, value))", [(1, 1)]),
    ("incorrect", "return min(10, value)", [(-1, 0), (0, 0), (5, 5), (10, 10), (11, 10)]),
]:
    root = destination / case
    for folder in ["md", "tests", "evidence"]:
        (root / folder).mkdir(parents=True, exist_ok=True)
    (root / "book.toml").write_text('[book]\nsrc = "md"\n')
    (root / "md/SUMMARY.md").write_text('- [Contract](contract.md)\n')
    (root / "md/contract.md").write_text('{req}`sample.clamp` For an integer input, clamp must return zero below zero, ten above ten, and the input unchanged from zero through ten inclusive.\n')
    (root / "implementation.py").write_text(f'def clamp(value):\n    {implementation}\n')
    (root / "tests/test_clamp.py").write_text(
        'import unittest\nfrom implementation import clamp\n\nclass ClampTests(unittest.TestCase):\n'
        '    def test_cases(self):\n'
        f'        for value, expected in {cases!r}:\n'
        '            with self.subTest(value=value):\n'
        '                self.assertEqual(clamp(value), expected)\n'
    )
    (root / "evidence/sample.clamp.json").write_text(json.dumps({
        "requirement": "stale", "inputs": [], "justification": "Old review: everything is good."
    }))
    for crate in ["language-lab-ci", "mdbook-language-lab"]:
        shutil.copytree(repo / crate, root / crate, ignore=shutil.ignore_patterns("target", ".git"))
print(destination)
