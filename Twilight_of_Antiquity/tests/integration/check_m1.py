"""Exercise the real CLI JSON contract and failure exit codes with synthetic inputs."""
import json
from pathlib import Path
import subprocess
import tempfile

SERIES = Path(__file__).resolve().parents[2]
EXE = SERIES / "toolchain/target/debug/saecvar-cli.exe"
FIXTURES = SERIES / "tests/fixtures/pdx"


def run(path):
    return subprocess.run([str(EXE), "parse", str(path)], capture_output=True, encoding="utf-8", timeout=15)


for filename, code in (("basic.txt", 0), ("recovery.txt", 1), ("unsupported.txt", 1)):
    result = run(FIXTURES / filename)
    assert result.returncode == code, (filename, result.stderr)
    report = json.loads(result.stdout)
    assert report["schema"] == "saecvar.syntax.v0"
    assert report["engine_validation"] == "NOT_RUN"
    assert report["semantic_validation"] == "NOT_IMPLEMENTED"
    assert bool(report["diagnostics"]) == bool(code)
    assert report["source_bytes"] == (FIXTURES / filename).stat().st_size
    for diag in report["diagnostics"]:
        assert 0 <= diag["range"]["start_byte"] <= diag["range"]["end_byte"] <= report["source_bytes"]
        assert diag["range"]["start"]["line"] >= 1
    repeated = run(FIXTURES / filename)
    assert repeated.stdout == result.stdout and repeated.returncode == code

with tempfile.TemporaryDirectory(prefix="saecvar-cli-") as tmp:
    root = Path(tmp)
    unicode_file = root / "한글.txt"
    unicode_file.write_text("이름 = 값", encoding="utf-8")
    result = run(unicode_file)
    assert result.returncode == 0 and json.loads(result.stdout)["file"] == str(unicode_file)
    invalid = root / "invalid.txt"
    invalid.write_bytes(b"\xff\xfe\x00")
    assert run(invalid).returncode == 2
    yaml = root / "localization.yml"
    yaml.write_text('l_english:\n label:0 "Text"\n', encoding="utf-8")
    assert run(yaml).returncode == 2
    assert run(root / "missing.txt").returncode == 2
print("M1 CLI: valid/error/unsupported fixtures, JSON, determinism, Unicode, encoding/dialect/IO errors PASS")
