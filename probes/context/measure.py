#!/usr/bin/env python3
"""Grade three capped context runs; print counts only, never answers or a key."""

import csv
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from probes.speed import measure as speed


def refuse(message):
    print(f"context: {message}", file=sys.stderr)
    raise SystemExit(2)


def counted_tokens(total, rows):
    """Require reported usage on every row and match the process totals."""
    if not isinstance(total, dict) or not rows:
        refuse("the backend did not report complete token usage")
    usages = []
    for row in rows:
        meta = row.get("meta") if isinstance(row, dict) else None
        usage = meta.get("usage") if isinstance(meta, dict) else None
        if not isinstance(usage, dict):
            refuse("the backend did not report complete token usage")
        usages.append(usage)
    spent = 0
    for field in ("input_tokens", "output_tokens"):
        value = total.get(field)
        shares = [usage.get(field) for usage in usages]
        if type(value) is not int or value < 0 or any(type(part) is not int or part < 0 for part in shares):
            refuse("the backend did not report complete token usage")
        if sum(shares) != value:
            refuse("the result and process token counts differ")
        spent += value
    if spent == 0:
        refuse("the sent request has no reported token usage")
    return spent


def run(binary, titles, catalog, key, scratch):
    home = scratch / "home"
    home.mkdir()
    context = scratch / "catalog.txt"
    context.write_text(catalog, encoding="utf-8")
    results = scratch / "results.jsonl"
    env = {**speed.plain(), "HOME": str(home), "THINKTHEN_API_KEY": os.environ["THINKTHEN_API_KEY"]}
    with results.open("wb") as output:
        command = subprocess.run(
            [str(binary), "decide", "It appears on the album Abbey Road.", "--lines", "--details",
             "--threshold", "0.7", "--no-cache", "--context", str(context)],
            input=titles, stdout=output, stderr=subprocess.DEVNULL, env=env, check=False,
        )
    if command.returncode:
        refuse(f"decide exited {command.returncode}")
    status = subprocess.run([str(binary), "status", "--json"], capture_output=True, env=env, check=True)
    total = json.loads(status.stdout)["usage"]["total"]
    result_rows = [json.loads(line) for line in results.read_bytes().splitlines()]
    spent = counted_tokens(total, result_rows)
    report = subprocess.run(
        [str(binary), "audit", str(results), str(key), "--id", ""],
        capture_output=True, env=speed.plain(), check=False,
    )
    if report.returncode:
        refuse(f"audit exited {report.returncode}")
    rows = [json.loads(line) for line in report.stdout.splitlines()]
    if len(rows) != 1 or rows[0]["rows"] != 306 or rows[0]["labeled"] != 306:
        refuse("audit did not grade all 306 titles in one row")
    grade = rows[0]
    if total["requests_sent"] != 1:
        refuse(f"sent {total['requests_sent']} requests instead of one")
    return {"right": grade["right"], "false_yes": grade["false_yes"],
            "misses": grade["false_no"], "input_tokens": total["input_tokens"],
            "output_tokens": total["output_tokens"], "counted_tokens": spent,
            "requests_sent": total["requests_sent"]}


def main(bench, name):
    binary, titles = speed.check(bench)
    if speed.git("status", "--porcelain", "--untracked-files=no", cwd=bench):
        refuse("the bench checkout has uncommitted changes")
    if not name or any(char not in "abcdefghijklmnopqrstuvwxyz0123456789-" for char in name):
        refuse("NAME needs lowercase letters, digits, or hyphens")
    output = Path(__file__).resolve().parent / "runs" / f"{name}.jsonl"
    if output.exists():
        refuse("the named count record already exists")
    songs = Path(bench) / "data" / "songs.tsv"
    with songs.open(encoding="utf-8", newline="") as source:
        rows = list(csv.DictReader(source, delimiter="\t"))
    if len(rows) != 306 or sum(row["first_album"] == "Abbey Road" for row in rows) != 18:
        refuse("the bench needs 306 titles and 18 Abbey Road first albums")
    catalog = speed.catalog_text(bench)
    spent = 0
    output.parent.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="thinkthen-context-") as temporary:
        root = Path(temporary)
        key = root / "key.jsonl"
        key.write_text("".join(json.dumps({"id": row["title"], "value": row["first_album"] == "Abbey Road"}) + "\n"
                               for row in rows), encoding="utf-8")
        for repeat in range(1, 4):
            if spent >= 120_000:
                refuse(f"stopped at {spent} counted tokens before repeat {repeat}")
            scratch = root / f"repeat-{repeat}"
            scratch.mkdir()
            counts = run(binary, titles, catalog, key, scratch)
            spent += counts["counted_tokens"]
            line = json.dumps({"name": name, "repeat": repeat, "build": speed.git("rev-parse", "HEAD"), **counts})
            with output.open("a", encoding="utf-8") as kept:
                kept.write(line + "\n")
            print(line)
            if counts["right"] < 297:
                refuse(f"repeat {repeat} scored under 297 right")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        refuse("usage: measure.py BENCH NAME")
    main(*sys.argv[1:])
