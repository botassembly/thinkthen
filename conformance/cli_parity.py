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
    if value.get("context_present"):
        raise ContractGap("CLI has shared --context FILE, no per-record context input")
    if injection in ("cancel_token", "expired_deadline") or value.get("held_cancel"):
        raise ContractGap("CLI signals drain started work; no native token/deadline control")
    if "proxy" in settings:
        raise ContractGap("CLI has no proxy-reservation input field")
    if value.get("image_paths") and len(value["items"]) > 1:
        raise ContractGap("CLI --image attachments cannot accompany record framing")
    question = value.get("question", {})
    if value.get("candidate_orders"):
        raise ContractGap("CLI image captions cannot carry per-record candidate lists")
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
    if verb == "find" and question.get("none"):
        args.append("--none")
    args.extend(["--details", "--facts"])
    document_files = bool(value.get("paths")) and value.get("source_unit") in (3, 4)
    attached_images = bool(value.get("image_paths")) and not value.get("paths")
    stream = not (document_files or attached_images) and (
        bool(value.get("paths")) or verb in ("filter", "rank", "find") or not (
            len(value["items"]) == 1 and isinstance(value["items"][0], str)
        )
    )
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
        if name == "max_request_bytes" and verb == "find":
            raise ContractGap("CLI find exposes no max-request-bytes setting")
        args.extend(["--" + names.get(name, name.replace("_", "-")), str(value_)])
    if value.get("shared_context") is not None:
        if verb not in BATCHED:
            raise ContractGap("CLI whole-set function exposes no shared context")
        context = home / "shared-context.txt"
        context.write_text(value["shared_context"])
        args.extend(["--context", str(context)])
    if value.get("image_paths") and not value.get("paths"):
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
    if value.get("image_paths"):
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
    row["detail_inputs"] = result.get("inputs", [])
    if "images" in result:
        row["images"] = [base64.b64decode(i["base64"]).hex() for i in result["images"]]
        row["image_properties"] = [[1 if i["media"] == "image/jpeg" else 2, i["width"], i["height"]]
                                    for i in result["images"]]
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


def execute(binary, value, settings, home, env):
    args, stdin = command(binary, value, settings, home)
    process = subprocess.run(args, input=stdin, text=True, capture_output=True,
                             cwd=home, env=env, timeout=120)
    assert "sk-conformance-loopback" not in process.stdout + process.stderr, "key leaked"
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
    rows = [project_row(detail, value, at) for at, detail in enumerate(details)]
    completed = process.returncode in (0, 1, 3, 6) and (bool(details) or process.returncode == 0)
    if completed:
        assert facts is not None, "CLI omitted terminal facts"
        out = {"code": 0, "schema": "thinkthen.result/2", "rows": rows, **facts}
        out["schema"] = "thinkthen.result/2"
    else:
        code = {2: 1, 4: 2, 5: 4, 70: 6, -2: 5, -15: 5, 130: 5, 143: 5}.get(process.returncode)
        assert code is not None, (process.returncode, diagnostic)
        out = {"code": code, "message": "\n".join(diagnostic), "completed": rows}
        if facts:
            out.update({k: facts[k] for k in ("records", "requests_sent")})
            if facts.get("stopped", {}).get("at") is not None:
                out["stopped_at"] = facts["stopped"]["at"] - 1
    return out, details


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
            got, details = execute(binary, step, settings, home, env)
            count = int(backend.read("count"))
            if "requests_sent" in got:
                assert got["requests_sent"] == count - before, (got, count - before)
            bodies = json.loads(backend.read("capture"))["bodies"]
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
