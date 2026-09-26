#!/usr/bin/env python3
"""The speed test of ticket 0145. README.md gives each mode and the list rule.

usage: measure.py gate [LIST] | plan BENCH | live BENCH NAME
Each command runs in a private home made here, with --no-cache. Its counts come from `status --json` there.
Output text, standard error and the key are never kept. `live` runs only through probes/speed/job.sh.
"""
import csv, glob, hashlib, json, os, shlex, shutil, statistics, subprocess, sys, tempfile, time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
WORK = HERE / "workloads"
LOOPBACK_KEY = "loopback-not-a-key"
SONGS_SHA256 = "3250fed3858ab12481d7e42b70d5eeeb18292db8a7580ec397a91bcd89bcdffd"
TARGET = "The text is the title of a song by the Beatles. It appears on the album Abbey Road."
EVIDENCE = "The text names a song by the Beatles that appears on the album Abbey Road."
SIZES = (8000, 24000, 40000, 56000)
PRICE = 0.042  # dollars a million input tokens, as recorded in the batching evidence record
CAP = 1_800_000
SOURCES = ("crates/thinkthen/src", "crates/thinkthen/Cargo.toml", "Cargo.toml", "Cargo.lock")  # what the binary is built from
MADE = set()


def refuse(message):
    print(f"speed: {message}", file=sys.stderr)
    sys.exit(2)


def made_folder():
    """A private folder this run makes, and the only kind remove() deletes."""
    path = tempfile.mkdtemp(prefix="thinkthen-speed-")
    MADE.add(path)
    return path


def remove(path):
    if path not in MADE:
        refuse("refused to delete a folder this run did not make")
    MADE.discard(path)
    shutil.rmtree(path)


def run(binary, args, stdin, env, cwd):
    """One command in a private home: its requests and tokens from status, its wall time, lines and exit."""
    home = made_folder()
    try:
        base = {**plain(), "HOME": home}
        start = time.monotonic()
        done = subprocess.run([str(binary), *args, "--no-cache"], input=stdin, stdout=subprocess.PIPE,
                              stderr=subprocess.DEVNULL, env={**base, **env}, cwd=cwd)
        seconds = round(time.monotonic() - start, 3)
        status = subprocess.run([str(binary), "status", "--json"], stdout=subprocess.PIPE, env=base, check=True)
        total = json.loads(status.stdout)["usage"]["total"]
    finally:
        remove(home)
    return {"requests_sent": total["requests_sent"], "input_tokens": total["input_tokens"],
            "output_tokens": total["output_tokens"], "seconds": seconds,
            "lines": done.stdout.count(b"\n"), "exit": done.returncode}


def prose(text):
    """The lines outside every fenced block."""
    out, fence = [], False
    for line in text.splitlines():
        if line.lstrip().startswith(("```", "~~~")):
            fence = not fence
        elif not fence:
            out.append(line)
    return out


def landed(number):
    for path in glob.glob(str(ROOT / "sdlc" / "tickets" / f"{number}-*.md")):
        status = next((l for l in prose(Path(path).read_text(encoding="utf-8")) if l.startswith("Status:")), "")
        if status.startswith("Status: landed"):
            return True
    return False


def judge(row, got):
    """The failure sentences for one gate row, empty when it keeps the list rule."""
    name, sent, items, floor = row["function"], got["requests_sent"], row["items"], row["floor"]
    noun = "request" if sent == 1 else "requests"
    if got["exit"] != 0:
        return [f"speed: {name} exited {got['exit']}."]
    entry = row.get("list")
    if entry is None:
        if sent <= floor:
            return []
        return [f"speed: {name} sent {sent} {noun} for {items} items where {floor} fits. Batch it, or list it with its ticket."]
    ticket = entry["number"] or entry["ticket"]
    if entry["number"] and landed(entry["number"]):
        return [f"speed: {name} is listed for ticket {ticket}, which has landed. Remove its entry."]
    if floor < sent <= items:
        return []
    tail = "Remove its entry." if sent <= floor else "Check its workload."
    return [f"speed: {name} is listed for ticket {ticket} but sent {sent} {noun} for {items} items. {tail}"]


def table(path):
    return [json.loads(line) for line in Path(path).read_text(encoding="utf-8").splitlines() if line.strip()]


def gate(path=HERE / "functions.jsonl"):
    env = {"THINKTHEN_BASE_URL": os.environ["THINKTHEN_BASE_URL"], "THINKTHEN_API_KEY": LOOPBACK_KEY}
    failures = []
    for row in table(path):
        got = run(os.environ["THINKTHEN_BIN"], [row["function"], *row["args"]],
                  (WORK / row["input"]).read_bytes(), env, WORK)
        print(f"{row['function']}\t{row['items']}\t{got['requests_sent']}\t{row['floor']}")
        failures += judge(row, got)
    for failure in failures:
        print(failure, file=sys.stderr)
    return 1 if failures else 0


def plain():
    return {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}


def git(*args, cwd=ROOT):
    return subprocess.run(["git", *args], cwd=cwd, stdout=subprocess.PIPE, env=plain(),
                          check=True).stdout.decode().strip()


def check(bench):
    """The live preconditions. Each refuses at exit 2 before any request."""
    binary = ROOT / "target" / "debug" / "thinkthen"
    if os.environ.get("THINKTHEN_BASE_URL"):
        refuse("THINKTHEN_BASE_URL is set; the speed run measures the built-in address")
    if git("status", "--porcelain"):
        refuse("the checkout has uncommitted changes; commit them so the run names its build")
    head = int(git("log", "-1", "--format=%ct", "--", *SOURCES))
    if not binary.exists() or binary.stat().st_mtime < head:
        refuse("target/debug/thinkthen is missing or older than the last commit to its sources; build it first")
    songs = Path(bench) / "data" / "songs.tsv"
    if not songs.exists():
        refuse(f"{songs} is missing")
    with open(songs, encoding="utf-8", newline="") as f:
        titles = "".join(r["title"] + "\n" for r in csv.DictReader(f, delimiter="\t"))
    if hashlib.sha256(titles.encode()).hexdigest() != SONGS_SHA256:
        refuse(f"the titles in {songs} do not hash to {SONGS_SHA256}")
    return binary, titles.encode()


def helps(binary, verb, flag):
    done = subprocess.run([str(binary), verb, "--help"], stdout=subprocess.PIPE, env=plain())
    return flag.encode() in done.stdout


def arms(binary, verb):
    return [("default", [])] + ([("batch 1", ["--batch", "1"])] if helps(binary, verb, "--batch") else [])


def jsonl(rows):
    return "".join(json.dumps(r, ensure_ascii=False) + "\n" for r in rows).encode()


def catalog_text(bench):
    return subprocess.run([sys.executable, str(Path(bench) / "scripts/run/catalog.py")], stdout=subprocess.PIPE,
                          env=plain(), check=True).stdout.decode()


def padded(source, size):
    """Catalog lines, repeated under numbered headers, cut at a line boundary at or under size bytes."""
    lines = source.splitlines(keepends=True)
    text, copy = "", 1
    while True:
        for line in [f"Copy {copy}\n", *lines]:
            if len((text + line).encode()) > size:
                return text
            text += line
        copy += 1


def measurements(bench, binary, titles, scratch):
    """(measurement, arm, repeat, label, args, stdin, cwd, records, estimate) for every live command."""
    target = ["filter", TARGET, "--threshold", "0.7"]
    for arm, flags in arms(binary, "filter"):
        for repeat in (1, 2, 3):
            yield "target", arm, repeat, "306 titles", target + flags, titles, None, 306, 300 * 306
    unbatched = ["--batch", "1"] if helps(binary, "filter", "--batch") else []
    yield "throttle", "jobs 16", 1, "306 titles", target + ["--jobs", "16"] + unbatched, titles, None, 306, 300 * 306
    rows = table(HERE / "functions.jsonl")
    extra = [("recognize by step", "recognize", ["--lines", "--relation", "works_for=*:*"], "sentences.txt")]
    if helps(binary, "recognize", "--boundary"):
        extra.append(("recognize by step", "recognize", ["--lines", "--boundary", "run"], "sentences.txt"))
    extra += [("annotate by field", "annotate", [f, "--jsonl"], "records.jsonl")
              for f in ("annotate-title.json", "annotate-album.json")]
    jobs = [("function", r["function"], r["args"], r["input"]) for r in rows] + extra
    for measurement, verb, args, name in jobs:
        stdin = (WORK / name).read_bytes()
        for arm, flags in arms(binary, verb):
            label = " ".join([verb, *args])
            yield measurement, arm, 1, label, [verb, *args, *flags], stdin, WORK, stdin.count(b"\n"), 300 * 24
    cases = [p for pattern in ("functions/*/*-cold.jsonl", "functions/*/*-context.jsonl",
                               "functions/decide/decide-love.jsonl") for p in sorted(glob.glob(str(Path(bench) / pattern)))]
    for path in cases:
        found = table(path)
        if "args" not in found[0]:  # a saved rows file, such as audit/rows-context.jsonl, not a case file
            continue
        records = [r for case in found for r in case["records"]]
        verb = found[0]["function"]
        for arm, flags in arms(binary, verb):
            yield ("bench job", arm, 1, str(Path(path).relative_to(bench)), [verb, *found[0]["args"], *flags],
                   jsonl(records), Path(path).parent, len(records), 300 * len(records))
    groups = {}
    for path in sorted(glob.glob(str(Path(bench) / "questions" / "*.jsonl"))):
        for q in table(path):
            record = {"id": q["id"], "input": q["input"], **({"options": q["options"]} if q.get("options") else {})}
            groups.setdefault((q["function"], q["question"]), []).append(record)
    for (verb, question), records in groups.items():
        args = [verb, question, "--jsonl", "--field", "/input"] + (["--options", "/options"] if verb == "choose" else [])
        for arm, flags in arms(binary, verb):
            yield ("bench questions", arm, 1, f"{verb}: {question}", args + flags, jsonl(records), None,
                   len(records), 300 * len(records))
    first, source = titles.split(b"\n")[0], catalog_text(bench)
    for size in (len(first), *SIZES):
        text = first if size == len(first) else padded(source, size).encode()
        for repeat in (1, 2, 3):
            yield ("evidence size", "single", repeat, f"{len(text)} bytes", ["decide", EVIDENCE], text, None, 1,
                   300 + round(0.516 * len(text)))
    if helps(binary, "filter", "--context"):
        for name, text in (("catalog", source), ("padded", padded(source, SIZES[-1]))):
            path = Path(scratch) / f"context-{name}.txt"
            path.write_text(text, encoding="utf-8")
            for repeat in (1, 2, 3):
                yield ("context", "default", repeat, f"{path.stat().st_size} bytes", target + ["--context", str(path)],
                       titles, None, 306, 300 + round(0.516 * path.stat().st_size) + 40 * 306)


def plan(bench):
    if "THINKTHEN_API_KEY" in os.environ:
        refuse("THINKTHEN_API_KEY is set; plan sends nothing and needs no key")
    toolchain = ("HOME", "CARGO_HOME", "RUSTUP_HOME", "RUSTUP_TOOLCHAIN", "RUSTC_WRAPPER", "SCCACHE_CONF")
    env = {**plain(), **{k: os.environ[k] for k in toolchain if k in os.environ}}
    subprocess.run(["cargo", "build", "--locked", "--quiet", "--package", "thinkthen"], cwd=ROOT, env=env, check=True)
    binary, titles = check(bench)
    scratch = made_folder()
    try:
        total = 0
        for m, arm, repeat, label, args, _, _, records, estimate in measurements(bench, binary, titles, scratch):
            total += estimate
            print(f"{m}\t{arm}\t{repeat}\t{label}\t{records}\t{estimate}\t{shlex.join(args)}")
        print(f"estimated input tokens\t{total}")
    finally:
        remove(scratch)


def live(bench, name):
    binary, titles = check(bench)
    out = HERE / "runs" / name
    if out.exists():
        refuse(f"{out.relative_to(ROOT)} already exists")
    build = {"commit": git("rev-parse", "HEAD"), "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
             "version": subprocess.run([str(binary), "--version"], stdout=subprocess.PIPE, env=plain()).stdout.decode().strip(),
             "profile": "debug", "bench_commit": git("rev-parse", "HEAD", cwd=bench),
             "bench_clean": not git("status", "--porcelain", cwd=bench), "processors": os.cpu_count(),
             "loadavg_start": Path("/proc/loadavg").read_text().strip()}
    out.mkdir(parents=True)
    env = {"THINKTHEN_API_KEY": os.environ.get("THINKTHEN_API_KEY", "")}
    spent, rows, scratch = 0, [], made_folder()
    try:
        for m, arm, repeat, label, args, stdin, cwd, records, estimate in measurements(bench, binary, titles, scratch):
            if spent >= CAP:
                print(f"speed: stopped at {spent} counted input tokens, before {m} {label}", file=sys.stderr)
                break
            got = run(binary, args, stdin, env, cwd)
            spent += got["input_tokens"] or estimate
            rows.append({"measurement": m, "arm": arm, "repeat": repeat, "label": label, "records": records, **got})
            with open(out / "rows.jsonl", "a", encoding="utf-8") as f:
                f.write(json.dumps(rows[-1], ensure_ascii=False) + "\n")
    finally:
        remove(scratch)
        build["loadavg_end"] = Path("/proc/loadavg").read_text().strip()
        (out / "build.json").write_text(json.dumps(build, indent=2) + "\n")
    report(rows)


def report(rows):
    """One Markdown table per measurement, and the target's verdict."""
    for measurement in dict.fromkeys(r["measurement"] for r in rows):
        print(f"\n### {measurement}\n\n| Label | Arm | Runs | Requests | Median s | Range s | Input tokens | Output tokens | Dollars |")
        print("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
        for key in dict.fromkeys((r["label"], r["arm"]) for r in rows if r["measurement"] == measurement):
            runs = [r for r in rows if r["measurement"] == measurement and (r["label"], r["arm"]) == key]
            s = [r["seconds"] for r in runs]
            tokens = runs[0]["input_tokens"]
            print(f"| {key[0]} | {key[1]} | {len(runs)} | {runs[0]['requests_sent']} | {statistics.median(s)} | "
                  f"{min(s)} to {max(s)} | {tokens} | {runs[0]['output_tokens']} | {tokens * PRICE / 1e6:.4f} |")
    target = [r["seconds"] for r in rows if r["measurement"] == "target" and r["arm"] == "default"]
    if target:
        verdict = "met" if statistics.median(target) < 0.5 else "not met"
        print(f"\nTarget, filter over 306 titles under 0.5 s at the default: {verdict} ({', '.join(map(str, target))} s)")


if __name__ == "__main__":
    mode, rest = (sys.argv[1], sys.argv[2:]) if len(sys.argv) > 1 else ("", [])
    if mode == "gate" and len(rest) <= 1:
        sys.exit(gate(*rest))
    if mode == "plan" and len(rest) == 1:
        sys.exit(plan(*rest))
    if mode == "live" and len(rest) == 2:
        sys.exit(live(*rest))
    refuse("usage: measure.py gate [LIST] | plan BENCH | live BENCH NAME")
