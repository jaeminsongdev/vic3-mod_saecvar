"""Check authored M0 contracts and documentation links, without a game or PDX parser."""
import json
from pathlib import Path
import re

SERIES = Path(__file__).resolve().parents[2]
REPO = SERIES.parent
target = json.loads((SERIES / "toolchain/configs/game-target.json").read_text(encoding="utf-8"))
metadata = json.loads((SERIES / "mod/.metadata/metadata.json").read_text(encoding="utf-8"))
assert metadata["game_id"] == target["game_id"]
assert metadata["supported_game_version"] == target["version"]
assert metadata["name"]
dates = SERIES / "mod/common/defines/zz_saecvar_g0_dates.txt"
assert dates.read_bytes().startswith(b"\xef\xbb\xbf"), "PDX script must use UTF-8 BOM"
text = dates.read_text(encoding="utf-8-sig")
for key, value in (("START_DATE", target["campaign"]["start"]), ("END_DATE", target["campaign"]["end"])):
    assert re.findall(rf'\b{key}\s*=\s*"([^"]+)"', text) == [value], key
authored = {p.relative_to(SERIES / "mod").as_posix() for p in (SERIES / "mod").rglob("*") if p.is_file()}
assert authored == {"README.md", ".metadata/metadata.json", "common/defines/zz_saecvar_g0_dates.txt"}, authored
for sha in target["source_sha256"].values():
    assert re.fullmatch("[0-9a-f]{64}", sha)
checked = 0
for md in [REPO / "README.md", *(SERIES / "docs").rglob("*.md"), SERIES / "README.md",
           SERIES / "mod/README.md", SERIES / "toolchain/README.md", SERIES / "tests/README.md",
           SERIES / "tests/runtime/README.md"]:
    for link in re.findall(r'\[[^\]]*\]\(([^)]+)\)', md.read_text(encoding="utf-8-sig")):
        if re.match(r"[a-zA-Z]+:|#", link):
            continue
        path = link.split("#", 1)[0]
        if path:
            assert (md.parent / path).exists(), f"Broken link in {md.relative_to(REPO)}: {link}"
            checked += 1
print(f"M0 authored contracts and {checked} local links: PASS; engine validation is separate")
