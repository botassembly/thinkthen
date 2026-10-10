"""Shared fixtures through a versioned installed SwiftPM consumer.

Routine selects existing routine IDs; full executes every required case at a candidate.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import shutil
import sys
import tempfile
ROOT = Path(__file__).resolve().parents[4]
sys.path[:0] = [str(ROOT / "conformance"), str(ROOT / "conformance/children")]
from children import child_env
from session_projection import project_session
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--binary", type=Path, required=True)
parser.add_argument("--case", action="append", default=[])
parser.add_argument("--routine", action="store_true")
args = parser.parse_args()
selected = set(args.case)
if args.routine:
    import parity
    required_ids = parity.required_cases(parity.inventory(), "swift")
    selected.update(line for line in (ROOT / "conformance/routine-ids.txt").read_text().splitlines() if line in required_ids)
    selected.update(("complete-decide", "complete-find", "complete-annotate", "files-annotate", "images-decide", "image-file-decide", "settings-cache-off-sends-again", "settings-replay-answers-from-the-folder-alone", "declared-object-context", "context-null", "recognize-record-contexts"))
native = Path(shutil.which("swift")).resolve().parents[2] / "usr/lib/swift/linux"
def native_parity(consumer, command):
    # Shared recipes/assertions are read-only; execution uses each actual named consumer.
    import sqlite3
    sys.path.insert(0, str(ROOT / "conformance"))
    import parity, c_parity, c_images
    cases = {c["id"]: c for c in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    named = {c["id"]: c for c in json.loads((ROOT / "conformance/named-inputs.json").read_text())["cases"]}
    failures = []
    required = parity.required_cases(parity.inventory(), consumer)
    if selected and not selected.issubset(required):
        raise ValueError("unknown required case selector: " + repr(selected - required.keys()))
    for row in required.values():
        if selected and row["id"] not in selected:
            continue
        failure = None
        try:
            value = c_parity.document(row, cases, named)
            with tempfile.TemporaryDirectory(prefix=f"thinkthen-{consumer}-parity-") as folder:
                home = Path(folder)
                env = child_env(home=folder,
                                PATH=os.environ.get("PATH", "/usr/bin:/bin"),
                                LC_ALL="C.UTF-8",
                                DOTNET_CLI_TELEMETRY_OPTOUT="1",
                                LD_LIBRARY_PATH=str(native))
                backend = c_parity.Backend(ROOT / "target/debug/conformance-backend", env)
                try:
                    env.update(THINKTHEN_API_KEY="sk-conformance-loopback", LIQUIDAI_API_KEY="sk-conformance-loopback", OPENROUTER_API_KEY="sk-conformance-loopback", PERPLEXITY_API_KEY="sk-conformance-loopback")
                    c_parity.prepare(home, value)
                    identities = []
                    for original in value.get("steps", [value]):
                        step = dict(original)
                        if step.get("copy_store"):
                            (home / "refreshed").mkdir()
                            with sqlite3.connect(home / "saved/thinkthen.sqlite") as source, sqlite3.connect(home / "refreshed/thinkthen.sqlite") as target:
                                source.backup(target)
                        if step.get("damage_store"):
                            with sqlite3.connect(home / "saved/thinkthen.sqlite") as db:
                                db.execute("UPDATE answers SET answer='damaged fixture answer'")
                        if value.get("image_variants"):
                            backend.close()
                            backend = c_parity.Backend(ROOT / "target/debug/conformance-backend", env)
                            c_parity.prepare(home, step)
                        for key in ("paths", "image_paths"):
                            if key in step and not step.get("owned_jsonl"):
                                step[key] = [str(ROOT / path) for path in (step[key] or [])]
                        if (step.get("operation") or {}).get("injection") == "recording_read_failure":
                            step.update(paths=[str(home / "missing-input")], source_unit=1)
                        settings = {"cache": False, "model": "jev-latest" if "steps" in value else "jev-1.13.0", "batch": 1, "max_retries": 0}
                        settings.update(step.get("settings", {}))
                        settings["base_url"] = f"http://127.0.0.1:{backend.port}/{original['arm'] if value.get('image_variants') else value['arm']}"
                        replacements = {"$FOLDER": "saved", "$REFRESH": "refreshed", "$PROFILE": "profile.json"}
                        settings = {k: str(home / replacements[v]) if isinstance(v, str) and v in replacements else v for k, v in settings.items()}
                        if row["kind"] in ("images", "image-location"):
                            settings["record"] = str(home / "recorded")
                        def invoke(given):
                            input_file = home / "fixture.json"
                            input_file.write_text(c_parity.compact(step))
                            args = command + [str(input_file), c_parity.compact(given)]
                            if step.get("held_cancel"):
                                running = subprocess.Popen(args, env=env, cwd=home, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                                try:
                                    assert backend.read("wait 1") == "wait 1"
                                    running.stdin.write("!")
                                    running.stdin.flush()
                                    signal = running.stdout.readline()
                                    assert signal == "cancel-fired\n", (signal, running.stderr.read() if running.poll() is not None else "signal not received")
                                    backend.process.stdin.write("release\n")
                                    backend.process.stdin.flush()
                                    stdout, stderr = running.communicate(timeout=60)
                                    output = subprocess.CompletedProcess(args, running.returncode, stdout, stderr)
                                finally:
                                    backend.process.stdin.write("release\n")
                                    backend.process.stdin.flush()
                                    if running.poll() is None:
                                        running.kill()
                                        running.wait()
                            else:
                                output = subprocess.run(args, env=env, cwd=home, capture_output=True, text=True, timeout=120 if value.get("image_variants") else 60)
                            assert output.returncode == 0 and not output.stderr, (consumer, row["id"], output.returncode, output.stderr[:1000])
                            return project_session(json.loads(output.stdout), step)
                        before = int(backend.read("count"))
                        got = invoke(settings)
                        if value.get("identity_steps"):
                            identities.append(got)
                        if step.get("stored_answers") == 0:
                            path = home / "saved/thinkthen.sqlite"
                            if path.exists():
                                with sqlite3.connect(path) as db:
                                    assert db.execute("SELECT count(*) FROM answers").fetchone()[0] == 0
                        if value.get("image_variants"):
                            c_images.assert_images(original, got, json.loads(backend.read("capture"))["bodies"])
                        if step.get("owned_jsonl") and step.get("incremental"):
                            capture = json.loads(backend.read("capture"))["bodies"]
                            assert capture, got
                            request = json.loads(capture[0])
                            expected = {f"q{i+1}": {"type": "noul", "instructions": f"The text is {c_parity.compact(item)}. {step['question']['decide']}"} for i, item in enumerate(step["items"][:2])}
                            assert request["questions"] == expected
                        c_parity.assertions(row, original, got, int(backend.read("count")) - (before if step.get("count_delta") else 0))
                        if row["kind"] in ("images", "image-location"):
                            before = int(backend.read("count"))
                            saved = invoke({**{k:v for k,v in settings.items() if k != "record"}, "replay": str(home / "recorded")})
                            c_parity.assertions(row, original, saved, int(backend.read("count")))
                            assert saved["requests_sent"] == 0 and int(backend.read("count")) == before
                            assert saved["rows"][0]["answer_id"] == got["rows"][0]["answer_id"]
                    if value.get("identity_steps"):
                        ids = [entry["rows"][0]["observation_ids"] for entry in identities]
                        assert ids[0] == ids[1] == ids[2] and ids[3] != ids[0] and ids[4] == ids[3] and ids[5] == ids[0]
                        assert len({entry["call_id"] for entry in identities}) == 6
                        assert len({identities[i]["rows"][0]["answer_id"] for i in (0,1,2,5)}) == 1
                        assert identities[3]["rows"][0]["answer_id"] == identities[4]["rows"][0]["answer_id"] != identities[0]["rows"][0]["answer_id"]
                finally:
                    backend.close()
        except (AssertionError, ValueError, KeyError, TypeError, AttributeError, IndexError, subprocess.SubprocessError, OSError) as error:
            failure = type(error).__name__ + ": " + str(error)
            failures.append((row["id"], failure))
            print(f"{consumer} fixture {row['id']} failed: {failure[:1500]}", file=sys.stderr)
        print("parity: " + json.dumps({"consumer": consumer, "case": row["id"], "checks": row.get("checks", ["named", "runtime"]), "status": "fail" if failure else "pass"}), flush=True)
    print(f"{consumer} native fixture failures: {len(failures)}")
    return bool(failures)

raise SystemExit(native_parity("swift", [str(args.binary.resolve())]))
