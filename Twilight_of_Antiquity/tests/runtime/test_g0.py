"""Synthetic regression checks for experimental patch safety; no game assets needed."""
import tempfile
import hashlib
import json
import unittest
from pathlib import Path
from unittest.mock import patch
import g0


class PatchSafety(unittest.TestCase):
    def test_unexpected_source_count_fails_closed(self):
        with self.assertRaises(ValueError):
            g0.replace_exact("game_date > 1836.1.1", r"1836\.1\.1", "600.1.1", 6)

    def test_only_selected_date_reference_changes(self):
        src = "year < 1883\nyear >= 1883\nvalue = 1883\n"
        result = g0.replace_exact(src, r"(\byear\s*(?:<|>=)\s*)1883\b", r"\g<1>647", 2)
        self.assertEqual(result, "year < 647\nyear >= 647\nvalue = 1883\n")

    def test_traversal_run_id_rejected_before_inspection(self):
        with self.assertRaises(ValueError):
            g0.create_run(Path("missing"), "date", "../escape")

    def test_existing_run_preserved(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "old").mkdir()
            sentinel = root / "old/evidence.txt"
            sentinel.write_text("keep")
            with patch.object(g0, "LOCAL", root), patch.object(g0, "inventory", return_value={}):
                with self.assertRaises(FileExistsError):
                    g0.create_run(root, "baseline", "old")
            self.assertEqual(sentinel.read_text(), "keep")

    def test_observation_hashes_the_loaded_boundary_probe_without_mutation(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            profile = root / "profile"
            mod = root / "boundary"
            profile.mkdir()
            (mod / "common/defines").mkdir(parents=True)
            dates = mod / "common/defines/zz_saecvar_g0_dates.txt"
            dates.write_text('NGame = { START_DATE = "719.12.30" }', encoding="utf-8-sig")
            settings = profile / "content_load.json"
            settings.write_text(json.dumps({"enabledMods": [{"path": str(mod)}]}), encoding="utf-8")
            before = settings.read_bytes()
            with patch.object(g0, "LOCAL", root / "runs"), patch.object(g0, "inventory", return_value={}), patch.object(g0, "collect"):
                g0.observe_external(root, profile, "observed")
            record = json.loads((root / "runs/observed/run.local.json").read_text(encoding="utf-8"))
            actual = record["loaded_probe_inputs"][0]["selected_file_hashes"]["common/defines/zz_saecvar_g0_dates.txt"]
            self.assertEqual(actual, hashlib.sha256(dates.read_bytes()).hexdigest())
            self.assertEqual(settings.read_bytes(), before)


if __name__ == "__main__":
    unittest.main()
