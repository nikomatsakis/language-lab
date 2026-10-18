"""Verify the distributed Cargo archive, skill version, and Symposium wiring."""
import json
from pathlib import Path
import re
import subprocess
import tarfile
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]


class PackageTests(unittest.TestCase):
    def test_archive_builds_and_versions_match(self):
        metadata = json.loads(subprocess.check_output(
            ["cargo", "metadata", "--locked", "--offline", "--no-deps",
             "--format-version", "1", "--manifest-path", str(ROOT / "Cargo.toml")],
            text=True,
        ))
        package = next(p for p in metadata["packages"] if p["name"] == "language-lab")
        subprocess.run(["cargo", "package", "-p", "language-lab", "--locked",
                        "--offline", "--allow-dirty", "--no-verify"], cwd=ROOT, check=True)
        archive = Path(metadata["target_directory"]) / "package" / (
            "language-lab-" + package["version"] + ".crate")
        with tempfile.TemporaryDirectory(prefix="language-lab-package-") as directory:
            with tarfile.open(archive) as bundle:
                for entry in bundle.getmembers():
                    self.assertTrue(entry.isfile() or entry.isdir())
                    self.assertFalse(Path(entry.name).is_absolute())
                    self.assertNotIn("..", Path(entry.name).parts)
                bundle.extractall(directory)
            extracted = Path(directory) / ("language-lab-" + package["version"])
            for path in ["Cargo.lock", "src/main.rs", "src/lib.rs",
                         "SYMPOSIUM.toml", "skills/language-lab/SKILL.md"]:
                self.assertTrue((extracted / path).is_file(), path)
            self.assertFalse((extracted / "language-lab-ci").exists())
            self.assertFalse((extracted / ".agents").exists())
            skill = (extracted / "skills/language-lab/SKILL.md").read_text()
            expected = re.search(r"The expected CLI version is \*\*([^*]+)\*\*", skill)
            self.assertIsNotNone(expected)
            self.assertEqual(expected.group(1), package["version"])
            subprocess.run(["cargo", "build", "--locked", "--offline"],
                           cwd=extracted, check=True, capture_output=True)
            binary = extracted / "target/debug/language-lab"
            self.assertEqual(subprocess.check_output([str(binary), "--version"], text=True),
                             "language-lab " + package["version"] + "\n")


if __name__ == "__main__":
    unittest.main()
