"""M0/G0 local harness; stdlib only, no PDX parser and no runtime PASS inference.

All writes are confined to a fresh .local/g0 run. Installed game files are read-only.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
from datetime import datetime, timezone

SERIES = Path(__file__).resolve().parents[2]
LOCAL = SERIES / ".local" / "g0"
TARGET = json.loads((SERIES / "toolchain/configs/game-target.json").read_text(encoding="utf-8"))
FILES = (
    "common/defines/00_defines.txt",
    "common/laws/00_governance_principles.txt",
    "events/french_pretenders.txt",
    "common/history/treaties/00_historical_treaties.txt",
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def inventory(install):
    install = install.resolve(strict=True)
    launcher = install / "launcher/launcher-settings.json"
    settings = json.loads(launcher.read_text(encoding="utf-8-sig"))
    if settings["rawVersion"] != TARGET["version"]:
        raise ValueError("Game version changed: review game-target.json and compatibility patch before running")
    game = install / "game"
    for rel, expected in TARGET["source_sha256"].items():
        if digest(game / rel) != expected:
            raise ValueError(f"Source drift: pinned hash does not match {rel}")
    manifest = install.parents[1] / "appmanifest_529340.acf"
    build = re.search(r'"buildid"\s+"(\d+)"', manifest.read_text(encoding="utf-8"))[1]
    if build != TARGET["steam_build_id"]:
        raise ValueError("Steam build changed: review the pinned target before running")
    refs = []
    counts = {}
    for folder in ("common", "events"):
        for path in sorted((game / folder).rglob("*.txt")):
            for number, line in enumerate(path.read_text(encoding="utf-8-sig", errors="replace").splitlines(), 1):
                code = line.split("#", 1)[0]
                if re.search(r"\b(?:game_date|year|date|birth_date|death_date)\s*(?:[<>=]+)", code):
                    rel = path.relative_to(game).as_posix()
                    refs.append({"path": rel, "line": number, "text": code.strip()})
                    counts[rel] = counts.get(rel, 0) + 1
    dlc = []
    for path in sorted((game / "dlc").rglob("*.dlc")):
        text = path.read_text(encoding="utf-8-sig")
        name = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M)
        dlc.append({"descriptor": path.relative_to(game).as_posix(), "name": name[1] if name else None,
                    "sha256": digest(path), "activation": "UNVERIFIED"})
    return {
        "captured_utc": datetime.now(timezone.utc).isoformat(),
        "install_read_only": str(install), "version": settings["rawVersion"], "steam_build_id": build,
        "source_hashes": {rel: digest(game / rel) for rel in FILES},
        "installed_dlc_descriptors": dlc, "date_reference_matches": refs,
        "date_reference_file_counts": counts,
        "scan_limit": "Lexical candidate inventory only; no scope analysis, archives, GUI or full history-date validation.",
    }


def replace_exact(text, pattern, replacement, count):
    updated, actual = re.subn(pattern, replacement, text)
    if actual != count:
        raise ValueError(f"Source drift: expected {count} matches, found {actual}: {pattern}")
    return updated


def create_run(install, mode, run_id):
    if not re.fullmatch(r"[a-zA-Z0-9_-]+", run_id):
        raise ValueError("run ID must contain only letters, digits, underscore or hyphen")
    facts = inventory(install)
    run = LOCAL / run_id
    run.mkdir(parents=True, exist_ok=False)
    profile = run / "profile"
    profile.mkdir()
    enabled = []
    if mode != "baseline":
        mod = profile / "mod/saecvar_g0"
        for rel in (".metadata/metadata.json", "common/defines/zz_saecvar_g0_dates.txt"):
            dest = mod / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(SERIES / "mod" / rel, dest)
        if mode == "boundary":
            dates = mod / "common/defines/zz_saecvar_g0_dates.txt"
            dates.write_text(dates.read_text(encoding="utf-8-sig").replace('"600.1.1"', '"719.12.30"'), encoding="utf-8-sig")
        if mode == "compat":
            for rel in FILES[1:3]:
                src = install / "game" / rel
                original = src.read_text(encoding="utf-8-sig")
                if rel.startswith("common/"):
                    patched = replace_exact(original, r"(\bgame_date\s*>\s*)1836\.1\.1\b", r"\g<1>600.1.1", 6)
                else:
                    patched = replace_exact(original, r"(\byear\s*(?:<|>=)\s*)1883\b", r"\g<1>647", 2)
                dest = mod / rel
                dest.parent.mkdir(parents=True, exist_ok=True)
                dest.write_text(patched, encoding="utf-8-sig")
        # Engine load paths are recorded explicitly, independent of the user's launcher playsets.
        enabled = [{"path": str(mod.resolve()).replace("\\", "/")}]
    write_json(profile / "content_load.json", {"enabledMods": enabled, "disabledDLC": [], "enabledUGC": []})
    write_json(run / "environment.local.json", facts)
    write_json(run / "run.local.json", {
        "id": run_id, "mode": mode, "profile": str(profile.resolve()),
        "campaign": {"start": "719.12.30" if mode == "boundary" else "1836.1.1" if mode == "baseline" else "600.1.1",
                     "end": "1936.1.1" if mode == "baseline" else "720.1.1"},
        "engine_status": "NOT_RUN", "mod_files": {
            p.relative_to(profile).as_posix(): digest(p) for p in sorted(profile.rglob("*")) if p.is_file()
        },
    })
    return run


def load_run(run_id):
    if not re.fullmatch(r"[a-zA-Z0-9_-]+", run_id):
        raise ValueError("invalid run ID")
    run = LOCAL / run_id
    return run, json.loads((run / "run.local.json").read_text(encoding="utf-8"))


def launch(run_id, install):
    run, record = load_run(run_id)
    if (run / "launch.local.json").exists():
        raise ValueError("Run already launched; use a fresh run ID to preserve evidence")
    if record["mode"] == "external":
        raise ValueError("External observation records cannot be launched")
    if os.name == "nt":
        running = subprocess.check_output(["tasklist", "/FI", "IMAGENAME eq victoria3.exe", "/FO", "CSV", "/NH"])
        if b'"victoria3.exe"' in running.lower():
            raise ValueError("Victoria 3 is already running; do not interfere with the existing session")
    facts = json.loads((run / "environment.local.json").read_text(encoding="utf-8"))
    if str(install.resolve()) != facts["install_read_only"]:
        raise ValueError("installation differs from staged source")
    for rel, expected in facts["source_hashes"].items():
        if digest(install / "game" / rel) != expected:
            raise ValueError(f"source changed after stage: {rel}")
    for rel, expected in record["mod_files"].items():
        if digest(Path(record["profile"]) / rel) != expected:
            raise ValueError(f"staged input changed: {rel}")
    command = [str(install / "binaries/victoria3.exe"),
               "-debug_mode", "-gdpr-compliant", f"-userdir={Path(record['profile']).as_posix()}"]
    with (run / "stdout.log").open("wb") as out, (run / "stderr.log").open("wb") as err:
        child_env = dict(os.environ, SteamAppId=TARGET["steam_app_id"], SteamGameId=TARGET["steam_app_id"])
        process = subprocess.Popen(command, cwd=install / "binaries", stdout=out, stderr=err, env=child_env)
    write_json(run / "launch.local.json", {"pid": process.pid, "argv": command,
               "started_utc": datetime.now(timezone.utc).isoformat(), "engine_status": "STARTED_UNVERIFIED"})
    print(f"pid={process.pid}; {run}")


def collect(run_id):
    run, record = load_run(run_id)
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    dest = run / "evidence" / stamp
    dest.mkdir(parents=True, exist_ok=False)
    profile = Path(record["profile"])
    folders = ("logs",) if record["mode"] == "external" else ("logs", "crashes", "exceptions")
    for folder in folders:
        if (profile / folder).is_dir():
            shutil.copytree(profile / folder, dest / folder)
    facts = json.loads((run / "environment.local.json").read_text(encoding="utf-8"))
    install = Path(facts["install_read_only"])
    unchanged = all(digest(install / "game" / rel) == expected for rel, expected in facts["source_hashes"].items())
    saves = [{"path": p.relative_to(profile).as_posix(), "bytes": p.stat().st_size, "sha256": digest(p)}
             for p in profile.rglob("*.v3")]
    write_json(dest / "receipt.local.json", {
        "source_files_unchanged": unchanged, "saves": saves,
        "evidence_files": {p.relative_to(dest).as_posix(): digest(p) for p in sorted(dest.rglob("*")) if p.is_file()},
        "engine_status": "REQUIRES_REVIEW", "note": "Logs and save existence alone do not establish G0 PASS.",
    })
    print(dest)


def observe_external(install, profile, run_id):
    """Read a user-launched profile without changing its settings, playsets or saves."""
    if not re.fullmatch(r"[a-zA-Z0-9_-]+", run_id):
        raise ValueError("invalid run ID")
    profile = profile.resolve(strict=True)
    if not (profile / "content_load.json").is_file():
        raise ValueError("not an existing Victoria 3 profile")
    facts = inventory(install)
    content_load = json.loads((profile / "content_load.json").read_text(encoding="utf-8-sig"))
    loaded_inputs = []
    for item in content_load.get("enabledMods", []):
        if not isinstance(item, dict) or not isinstance(item.get("path"), str):
            raise ValueError("unsupported enabledMods entry")
        mod_path = Path(item["path"])
        if not mod_path.is_absolute():
            mod_path = profile / mod_path
        mod_path = mod_path.resolve(strict=True)
        # Record the actual loaded probe, which may differ from the authored 600-year source.
        selected = (".metadata/metadata.json", "common/defines/zz_saecvar_g0_dates.txt")
        loaded_inputs.append({"root": str(mod_path), "selected_file_hashes": {
            rel: digest(mod_path / rel) for rel in selected if (mod_path / rel).is_file()
        }})
    run = LOCAL / run_id
    run.mkdir(parents=True, exist_ok=False)
    write_json(run / "environment.local.json", facts)
    write_json(run / "run.local.json", {
        "id": run_id, "mode": "external", "profile": str(profile),
        "engine_status": "USER_LAUNCHED_REQUIRES_REVIEW",
        "mod_files": {}, "content_load": content_load, "loaded_probe_inputs": loaded_inputs,
        "authored_mod_hashes": {p.relative_to(SERIES / "mod").as_posix(): digest(p)
                                for p in sorted((SERIES / "mod").rglob("*")) if p.is_file() and p.name != "README.md"},
    })
    collect(run_id)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    stage = sub.add_parser("stage")
    stage.add_argument("--install", type=Path, required=True)
    stage.add_argument("--mode", choices=("baseline", "date", "compat", "boundary"), required=True)
    stage.add_argument("--id", required=True)
    start = sub.add_parser("launch")
    start.add_argument("--install", type=Path, required=True)
    start.add_argument("--id", required=True)
    end = sub.add_parser("collect")
    end.add_argument("--id", required=True)
    external = sub.add_parser("observe")
    external.add_argument("--id", required=True)
    external.add_argument("--install", type=Path, required=True)
    external.add_argument("--profile", type=Path, required=True)
    args = parser.parse_args()
    if args.command == "stage":
        print(create_run(args.install, args.mode, args.id))
    elif args.command == "launch":
        launch(args.id, args.install)
    elif args.command == "collect":
        collect(args.id)
    else:
        observe_external(args.install, args.profile, args.id)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError) as error:
        print(f"G0 failed: {error}", file=sys.stderr)
        sys.exit(1)
