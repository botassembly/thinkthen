"""Execute shared cases through an explicitly supplied installed CLI binary.

Read actual JSONL details and terminal run facts. Unsupported public controls
remain failed cells; neither Rust results nor fixture answers substitute for
command output. The coordinator supplies THINKTHEN_CLI_BINARY or a second
argument after its loopback port. There is no checkout-binary fallback.
"""
import base64
import json
import os
from pathlib import Path
import re
import sqlite3
import signal
import time
import subprocess
import sys
import tempfile

import c_parity
import parity

ROOT = Path(__file__).resolve().parents[1]
BATCHED = {"decide", "choose", "tag", "score", "filter", "rank", "annotate"}


class ContractGap(ValueError):
    """A required public input cannot be expressed through the command."""


def command(binary, value, settings, home):
    """Translate input forms, without changing questions or expected answers."""
    verb = value["verb"]
    injection = (value.get("operation") or {}).get("injection")
    if injection in ("cancel_token", "expired_deadline"):
        raise ContractGap("CLI signals drain started work; no native token/deadline control")
    if "proxy" in settings:
        raise ContractGap("CLI has no proxy-reservation input field")
    question = value.get("question", {})
    args = [str(binary), verb]
    if value.get("loader"):
        reference = value["reference"]
        if value["loader"] in ("load_named", "named") and (
            not re.fullmatch(r"[a-z][a-z0-9_-]{0,63}", reference)
            or "cwd_refund" in (value.get("setup") or {})
        ):
            raise ContractGap("CLI @reference resolves paths first; no strict named-only loader")
        if value["loader"] in ("load", "file") and "/" not in reference:
            reference = "./" + reference
        operand = reference if reference.startswith("@") else "@" + reference
    elif value.get("literal_operand") is not None:
        operand = value["literal_operand"]
    elif value.get("question_form") == "file":
        operand = "@" + str(home / "fixture-question.json")
    else:
        path = home / "call-question.json"
        body = value.get("raw")
        if body is None:
            # --none is the documented command-only reading option.
            body = c_parity.compact({k: v for k, v in question.items()
                                     if not (verb == "find" and k == "none")})
        path.write_text(body)
        operand = str(path) if verb == "annotate" else "@" + str(path)
    args.append(operand)
    if value.get("literal_threshold") is not None:
        args.extend(["--threshold", str(value["literal_threshold"])])
    if verb == "find" and question.get("none"):
        args.append("--none")
    args.extend(["--details", "--facts"])
    envelope = value.get("context_present") or bool(value.get("candidate_orders"))
    document_files = bool(value.get("paths")) and value.get("source_unit") in (3, 4)
    attached_images = bool(value.get("image_paths")) and not value.get("paths") and len(value["items"]) == 1 and not envelope and isinstance(value["items"][0], (str, type(None)))
    stream = envelope or (not (document_files or attached_images) and (
        bool(value.get("paths")) or verb in ("filter", "rank", "find") or not (
            len(value["items"]) == 1 and isinstance(value["items"][0], str)
        )
    ))
    names = {"base_url": "url", "max_requests": "max-requests-total"}
    for name, value_ in settings.items():
        if name in ("cache", "refresh_cache") and isinstance(value_, bool):
            if value_ and name == "refresh_cache":
                args.append("--refresh-cache")
            elif not value_ and name == "cache" and not any(
                key in settings for key in ("record", "replay", "refresh_cache")
            ):
                args.append("--no-cache")
            continue
        if name == "batch" and (verb not in BATCHED or not stream):
            # Native ignores an engine batch setting on whole-set functions.
            continue
        args.extend(["--" + names.get(name, name.replace("_", "-")), str(value_)])
    if value.get("shared_context") is not None:
        context = home / "shared-context.txt"
        context.write_text(value["shared_context"])
        args.extend(["--context", str(context)])
    if value.get("image_paths") and not value.get("paths"):
        if value.get("media"):
            args.extend(["--image-media", value["media"]])
        for path in value["image_paths"]:
            args.extend(["--image", str(ROOT / path)])
    if value.get("paths") or injection == "recording_read_failure":
        paths = [home / "missing-input"] if injection == "recording_read_failure" else [
            (home if value.get("owned_jsonl") else ROOT) / path
            for path in value["paths"]
        ]
        for path in paths:
            args.extend(["--input", str(path)])
        unit = value.get("source_unit", 1)
        if unit == 5:
            args.append("--jsonl")
        elif unit == 2:
            args.extend(["--window", str(value["window"])])
        else:
            args.extend(["--unit", "line" if unit == 1 else "file"])
        if unit == 4 or value.get("image_reader"):
            args.extend(["--media", "image"])
        return args, ""
    items = value["items"]
    if envelope:
        projected = []
        for at, original in enumerate(items):
            record = {"original": original}
            if value.get("context_present"):
                record["context"] = value["context"]
            if value.get("candidate_orders"):
                record["options"] = value["candidate_orders"][at]
            projected.append(record)
        items = projected
        fields = question.get("on", [])
        if isinstance(fields, str):
            fields = [fields]
        for field in fields or [""]:
            args.extend(["--field", "/original" + field])
        if value.get("context_present"):
            args.extend(["--context-field", "/context"])
        if value.get("candidate_orders"):
            args.extend(["--options", "/options"])
    if value.get("image_paths") and attached_images:
        return args, items[0] or ""
    if verb == "relate":
        return args, c_parity.compact(items)
    if len(items) == 1 and isinstance(items[0], str) and verb not in ("filter", "rank", "find"):
        return args, items[0]
    args.append("--jsonl")
    return args, "".join(c_parity.compact(item) + "\n" for item in items)


def project_row(result, value, at):
    meta = result["meta"]
    assert result["schema"] == "thinkthen.result/2", result
    row = {"value": result["value"], "answer_id": result["answer_id"],
           "origin": {"live": 1, "cache": 2, "replay": 3, None: None}[meta["origin"]],
           "answered_by": meta.get("answered_by"), "observations": len(meta["observations"]),
           "sources": len(meta["question_sources"]),
           "observation_ids": [o.get("observation_id", o.get("failure_id")) for o in meta["observations"]],
           "question_digest": meta.get("question_sha256"), "cache_keys": meta["requests"]}
    row.update({k: result[k] for k in ("input", "file", "first_line", "last_line", "index") if k in result})
    if value.get("context_present") or value.get("candidate_orders"):
        if "input" in row:
            row["input"] = row["input"]["original"]
    if "index" not in row:
        if value["verb"] in ("filter", "rank"):
            indexes = [i for i, original in enumerate(value["items"])
                       if c_parity.equivalent(original, result.get("input"))]
            if len(indexes) != 1:
                raise ContractGap("CLI output omits the original ordinal for repeated records")
            row["index"] = indexes[0]
        else:
            row["index"] = at
    answer = result.get("answer", {})
    row.update({k: answer[k] for k in ("probability", "probabilities") if k in answer})
    question = result.get("question", {})
    row.update({k: question[k] for k in ("name", "wording_version") if k in question})
    row["member_authors"] = [{k: child["question"][k] for k in ("name", "wording_version")
                              if k in child["question"]}
                             for child in result.get("answers", {}).values() if "question" in child]
    row["detail_inputs"] = []
    if value["verb"] == "find":
        row["detail_inputs"] = [{"input": candidate["input"], **candidate.get("source", {})}
                                for candidate in result.get("candidates", []) if candidate.get("input") is not None]
    elif value["verb"] == "relate":
        row["detail_inputs"] = [{"input": original["record"], **{k: original[k] for k in ("file", "first_line", "last_line") if original.get(k) is not None}}
                                if isinstance(original, dict) and "record" in original and "file" in original else {"input": original}
                                for original in result.get("input", [])]
    images = result.get("images")
    image_input = result.get("input")
    if value.get("image_paths") and isinstance(image_input, dict) and "images" in image_input:
        row["input"] = image_input.get("text")
        if images is None:
            images = image_input["images"]
    if images is not None:
        row["images"] = [base64.b64decode(i["base64"]).hex() for i in images]
        row["image_properties"] = [[1 if i["media"] == "image/jpeg" else 2, i["width"], i["height"]]
                                    for i in images]
    if "members" in result:
        row.update(question_name=result["question_name"], usage=meta["usage"], model=meta["model"],
                   context_digest=meta.get("context_sha256"),
                   source_batch_sizes=[s["batch_size"] for s in meta["question_sources"]])
        row["members"] = []
        for member in result["members"]:
            child = member["result"]
            cm = child["meta"]
            cq = child["question"]
            projected = {"name": member["name"], "value": child["value"], "answer_id": child["answer_id"],
                         "author": cq["name"], "probability": child["answer"]["probability"],
                         "usage": cm["usage"], "model": cm["model"],
                         "context_digest": cm.get("context_sha256"),
                         "source_batch_sizes": [s["batch_size"] for s in cm["question_sources"]],
                         "observations": len(cm["observations"]), "sources": len(cm["question_sources"])}
            projected.update({k: cq[k] for k in ("wording_version",) if k in cq})
            if "source" in child:
                projected["source"] = child["source"]
            row["members"].append(projected)
    return row


def execute(binary, value, settings, home, env, backend=None):
    args, stdin = command(binary, value, settings, home)
    if value.get("held_cancel"):
        process = drain_signal(args, stdin, home, env, backend, value["cli_signal"])
    else:
        process = subprocess.run(args, input=stdin, text=True, capture_output=True,
                                 cwd=home, env=env, timeout=120)
    assert "sk-conformance-loopback" not in process.stdout + process.stderr, "key leaked"
    if "exit" in value.get("expect", {}):
        assert process.returncode == value["expect"]["exit"], (process.returncode, process.stderr)
    if value.get("expect", {}).get("no_result"):
        assert not process.stdout, "refused question emitted a result"
    if value.get("expect", {}).get("secrecy"):
        output = process.stdout + process.stderr
        material = [value.get("raw"), value["question"].get(value["verb"]), *value["items"]]
        for secret in material:
            if secret is not None:
                assert c_parity.compact(secret) not in output, "question material leaked"
                if isinstance(secret, str) and len(secret) >= 8:
                    assert secret not in output, "question material leaked"
    details = [json.loads(line) for line in process.stdout.splitlines()]
    diagnostic = []
    facts = None
    lines = process.stderr.splitlines()
    for at, line in enumerate(lines):
        if line.startswith("{"):
            candidate = json.loads(line)
            assert candidate["schema"] == "thinkthen.run/1", candidate
            assert facts is None, "duplicate terminal facts"
            assert at == len(lines) - 1, "run facts must be the last diagnostic line"
            facts = candidate
        else:
            diagnostic.append(line.removeprefix("thinkthen: "))
    message = "\n".join(diagnostic)
    if len(diagnostic) == 2 and diagnostic[-1].startswith("stopped at record "):
        assert facts and facts.get("stopped", {}).get("at") is not None, "stop diagnostic omitted stopped facts"
        at = facts["stopped"]["at"]
        count = facts["records"]
        noun = "record" if count == 1 else "records"
        assert diagnostic[-1].startswith(f"stopped at record {at}; {count} {noun} finished"), diagnostic
        message = diagnostic[0]
    rows = [project_row(detail, value, at) for at, detail in enumerate(details)]
    completed = process.returncode in (0, 1, 3, 6) and (bool(details) or process.returncode == 0)
    if completed:
        assert facts is not None, "CLI omitted terminal facts"
        out = {"code": 0, "schema": "thinkthen.result/2", "rows": rows, **facts}
        out["schema"] = "thinkthen.result/2"
    else:
        code = {2: 1, 4: 2, 5: 4, 70: 6, -2: 5, -15: 5, 130: 5, 143: 5}.get(process.returncode)
        assert code is not None, (process.returncode, diagnostic)
        out = {"code": code, "message": message, "completed": rows}
        if facts:
            out.update({k: facts[k] for k in ("records", "requests_sent")})
            if facts.get("stopped", {}).get("at") is not None:
                out["stopped_at"] = facts["stopped"]["at"] - 1
    if value.get("held_cancel"):
        assert process.returncode == -value["cli_signal"], process
        assert len(details) == 1 and rows[0]["input"] == value["items"][0], details
        assert rows[0]["index"] == 0 and rows[0]["value"] is True, rows
        assert len(rows[0]["answer_id"]) == 64 and rows[0]["observations"] == rows[0]["sources"] == 1, rows
        assert facts["records"] == facts["requests_sent"] == 1 and facts["retries"] == 0, facts
        assert len(facts["call_id"]) == 64, facts
        assert facts["stopped"] == {"cause":"cancelled","retryable":False}, facts
        assert all(len(identity) == 64 for identity in rows[0]["observation_ids"]), rows
        assert diagnostic == ["stopped by a signal; 1 record finished"], diagnostic
    return out, details


def drain_signal(args, stdin, home, env, backend, signum):
    child = subprocess.Popen(args, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                             stderr=subprocess.PIPE, text=True, cwd=home, env=env)
    try:
        child.stdin.write(stdin)
        child.stdin.close()
        child.stdin = None
        assert backend.read("wait 1") == "wait 1", "held first request was not observed"
        os.kill(child.pid, signum)
        deadline = time.monotonic() + 30
        # Pending consumption coordinates delivery. It does not acknowledge the cancellation flag.
        while True:
            assert child.poll() is None, "signal ended the child before the held reply drained"
            statuses = (Path(f"/proc/{child.pid}/task")).glob("*/status")
            pending = [int(line.split()[1], 16) for path in statuses
                       for line in path.read_text().splitlines() if line.startswith(("SigPnd:", "ShdPnd:"))]
            assert pending, "no process signal state"
            if not any(bits & (1 << (signum - 1)) for bits in pending):
                break
            assert time.monotonic() < deadline, "signal delivery did not finish"
            time.sleep(0.01)
        backend.process.stdin.write("release\n")
        backend.process.stdin.flush()
        stdout, stderr = child.communicate(timeout=30)
        return subprocess.CompletedProcess(args, child.returncode, stdout, stderr)
    finally:
        if child.poll() is None:
            child.kill()
            child.communicate()


def settings_for(step, home, port, arm):
    settings = {"cache": False, "model": "jev-1.13.0", "batch": 1, "max_retries": 0,
                **step.get("settings", {})}
    settings["base_url"] = f"http://127.0.0.1:{port}/{step.get('override_arm', arm)}"
    folders = {"$FOLDER": "saved", "$REFRESH": "refreshed", "$PROFILE": "profile.json"}
    return {k: str(home / folders[v]) if isinstance(v, str) and v in folders else v
            for k, v in settings.items()}


def metadata_steps(value):
    steps = value.get("steps", [value])
    if not value.get("metadata"):
        return steps

    def unadorned(question):
        return {k: {n: unadorned(q) for n, q in v.items()} if k == "questions" else v
                for k, v in question.items() if k not in ("name", "wording_version")}

    baseline = {**value, "question": unadorned(value["question"]),
                "count_delta": True, "metadata_only": True, "expect": {},
                "override_arm": "arm/full/capture/v1",
                "settings": {**value.get("settings", {}), "record": "$FOLDER"}}
    baseline.pop("metadata")
    restored = {**value, "count_delta": True, "metadata_only": True,
                "override_arm": "arm/full/capture/v1", "expect": {"requests_sent": 0},
                "settings": {**value.get("settings", {}), "cache": "$FOLDER"}}
    return [*steps, baseline, restored]


def run_case(binary, row, value, home):
    if value.get("held_cancel") and "cli_signal" not in value:
        for signum in (signal.SIGINT, signal.SIGTERM):
            signal_home = home / str(signum)
            signal_home.mkdir()
            run_case(binary, row, {**value, "cli_signal":signum, "items":["first evidence", "queued evidence"]}, signal_home)
        return
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(home),
           "XDG_CONFIG_HOME": str(home / "config"), "XDG_CACHE_HOME": str(home / "cache"),
           "XDG_STATE_HOME": str(home / "state"), "LANG": "C.UTF-8", "LC_ALL": "C.UTF-8"}
    c_parity.prepare(home, value)
    # Check admission expressibility before spawning a fixture server.
    for step in metadata_steps(value):
        command(binary, step, settings_for(step, home, 1, value["arm"]), home)
    backend = c_parity.Backend(ROOT / "target/debug/conformance-backend", env)
    env.update({name: "sk-conformance-loopback" for name in
                ("THINKTHEN_API_KEY", "LIQUIDAI_API_KEY", "OPENROUTER_API_KEY", "PERPLEXITY_API_KEY")})
    identities = []
    unadorned = None
    try:
        for step in metadata_steps(value):
            if value.get("metadata") and step.get("question_form") == "file":
                c_parity.prepare(home, step)
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
            settings = settings_for(step, home, backend.port, step["arm"] if value.get("image_variants") else value["arm"])
            if "steps" in value and "model" not in step.get("settings", {}):
                settings["model"] = "jev-latest"
            if row["kind"] in ("images", "image-location"):
                settings["record"] = str(home / "recorded")
            before = int(backend.read("count"))
            if step.get("held_cancel"):
                settings.update(batch=1, jobs=1, base_url=f"http://127.0.0.1:{backend.port}/arm/held/capture/v1")
            got, details = execute(binary, step, settings, home, env, backend)
            count = int(backend.read("count"))
            if "requests_sent" in got:
                assert got["requests_sent"] == count - before, (got, count - before)
            bodies = json.loads(backend.read("capture"))["bodies"]
            if step.get("held_cancel"):
                assert len(bodies) == 1, "signal drain sent a queued suffix"
            if value.get("image_variants"):
                from c_images import assert_images
                assert_images(step, got, bodies)
            c_parity.assertions(row, step, got, count - before if step.get("count_delta") else count)
            if "capture_request" in step["expect"]:
                assert [json.loads(body) for body in bodies] == [step["expect"]["capture_request"]]
            if "per_item_context" in step["expect"]:
                context = step["expect"]["per_item_context"] or "Each question quotes the text it asks about."
                assert bodies and all(json.loads(body)["state"] == context for body in bodies)
            if "selected_item" in step["expect"] and got["code"] == 0:
                selected = c_parity.compact(step["expect"]["selected_item"])
                if step["verb"] == "annotate":
                    selected = c_parity.compact(selected)
                assert any(selected in q["instructions"] or json.loads(body)["state"] == step["expect"]["selected_item"]
                           for body in bodies for q in json.loads(body)["questions"].values()), "selected evidence changed"
            for detail in details:
                for key, wanted in step["expect"].get("resolved_metadata", {}).items():
                    assert detail["question"][key] == wanted, (key, detail)
                if step["expect"].get("no_inferred_metadata"):
                    wanted = step["expect"]["resolved_metadata"]
                    assert not (set(detail["question"]) & {"name", "wording_version"}) - set(wanted)
                for key, member in (("ordered_properties", "properties"), ("required_properties", "required")):
                    if key in step["expect"]:
                        actual = detail["question"]["item_schema"][member]
                        assert (list(actual) if member == "properties" else actual) == step["expect"][key]
            if value.get("metadata") and step is not value:
                if not step.get("metadata"):
                    unadorned = got
                elif unadorned is not None:
                    for bare_row, authored_row in zip(unadorned["rows"], got["rows"], strict=True):
                        for key in ("question_digest", "cache_keys", "observation_ids", "answer_id"):
                            assert bare_row[key] == authored_row[key], (key, bare_row, authored_row)
            if step.get("stored_answers") == 0:
                path = home / "saved/thinkthen.sqlite"
                if path.exists():
                    with sqlite3.connect(path) as db:
                        assert db.execute("SELECT count(*) FROM answers").fetchone()[0] == 0
            if value.get("identity_steps"):
                identities.append(got)
            if row["kind"] in ("images", "image-location"):
                replay = {k: v for k, v in settings.items() if k != "record"}
                replay["replay"] = str(home / "recorded")
                saved, _ = execute(binary, step, replay, home, env)
                assert int(backend.read("count")) == count and saved["requests_sent"] == 0
                c_parity.assertions(row, step, saved, 0)
                assert [r["answer_id"] for r in saved["rows"]] == [r["answer_id"] for r in got["rows"]]
        if identities:
            ids = [v["rows"][0]["observation_ids"] for v in identities]
            assert ids[0] == ids[1] == ids[2] == ids[5] and ids[3] != ids[0] and ids[4] == ids[3]
            assert len({v["call_id"] for v in identities}) == len(identities)
            answers = [v["rows"][0]["answer_id"] for v in identities]
            assert answers[0] == answers[1] == answers[2] == answers[5] and answers[3] == answers[4] != answers[0]
        if "compile" in row.get("checks", []):
            raise ContractGap("shared compile-accessor check needs a written CLI applicability ruling")
    finally:
        backend.close()


def main():
    if os.environ.get("THINKTHEN_CONFORMANCE_IDS") is not None:
        raise ValueError("CLI strict parity refuses a case selector")
    binary = sys.argv[2] if len(sys.argv) == 3 else os.environ.get("THINKTHEN_CLI_BINARY")
    if len(sys.argv) not in (2, 3) or not binary:
        raise ValueError("usage: cli_parity.py LOOPBACK_PORT [INSTALLED_BINARY]; THINKTHEN_CLI_BINARY may supply the binary")
    binary = Path(binary).resolve(strict=True)
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise ValueError("the installed command is not executable")
    rows = parity.required_cases(parity.inventory(), "cli")
    cases = {r["id"]: r for r in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    named = {r["id"]: r for r in json.loads((ROOT / "conformance/named-inputs.json").read_text())["cases"]}
    failures = 0
    for row in rows.values():
        error = None
        try:
            value = c_parity.document(row, cases, named)
            if row.get("cli_boundary") == "question-file":
                value["expect"] = row["expect"]
            if row["id"] in ("29-usage-json-text", "31-usage-rank-blank-question"):
                value["literal_operand"] = cases[row["id"]]["question"]["decide"]
                if row["id"] == "29-usage-json-text":
                    value["literal_threshold"] = 90
            with tempfile.TemporaryDirectory(prefix="thinkthen-cli-parity-") as tmp:
                home = Path(tmp) / "case"
                home.mkdir()
                run_case(binary, row, value, home)
        except (AssertionError, KeyError, ValueError, TypeError, OSError, subprocess.SubprocessError) as failure:
            error = f"{type(failure).__name__}: {failure}"
        if error:
            failures += 1
            print(f"CLI fixture {row['id']} failed: {error}", file=sys.stderr)
        print("parity: " + json.dumps({"consumer": "cli", "case": row["id"],
              "checks": row.get("checks", ["named", "runtime"]),
              "status": "fail" if error else "pass"}), flush=True)
    print(f"CLI shared fixture results: {len(rows) - failures} passed, {failures} failed")
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
