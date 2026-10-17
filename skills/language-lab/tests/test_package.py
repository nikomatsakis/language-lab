"""Check the installed skill's package contract, not pedagogical wording.

Run from any directory: python3 skills/language-lab/tests/test_package.py
Behavioral instruction review is recorded separately in review.md.
"""
import json
from pathlib import Path
import re
import subprocess
import unittest


SKILL = Path(__file__).resolve().parents[1]


class PackageTests(unittest.TestCase):
    def test_locked_embedded_package_and_expected_version(self):
        # Cargo validates the manifest and lockfile and discovers build targets.
        result = subprocess.run(
            ["cargo", "metadata", "--locked", "--offline", "--no-deps",
             "--format-version", "1", "--manifest-path", str(SKILL / "cli/Cargo.toml")],
            check=True, capture_output=True, text=True,
        )
        subprocess.run(
            ["cargo", "build", "--locked", "--offline", "--manifest-path",
             str(SKILL / "cli/Cargo.toml")],
            check=True, capture_output=True, text=True,
        )
        packages = json.loads(result.stdout)["packages"]
        package = next(p for p in packages if p["name"] == "language-lab")
        expected = re.search(r"The expected CLI version is \*\*([^*]+)\*\*",
                             (SKILL / "SKILL.md").read_text())
        self.assertIsNotNone(expected, "Skill must declare the expected version")
        self.assertEqual(expected.group(1), package["version"])
        self.assertTrue((SKILL / "cli/Cargo.lock").is_file())
        binaries = [t for t in package["targets"] if "bin" in t["kind"]]
        self.assertTrue(binaries, "Embedded package needs an executable target")
        for target in package["targets"]:
            source = Path(target["src_path"])
            self.assertTrue(source.is_file())
            self.assertTrue(source.is_relative_to(SKILL / "cli"),
                            "Skill must ship its own build targets")


if __name__ == "__main__":
    unittest.main()
