"""Fetch pinned upstream sources for an offline community build; compile nothing."""
import json
from pathlib import Path
import subprocess
import sys

versions = json.loads(Path(__file__).with_name("community-versions.json").read_text())
if len(sys.argv) != 2 or sys.argv[1] not in versions:
    sys.exit("usage: scripts/duckdb-prepare --community " + "|".join(versions))
version = sys.argv[1]
cache = Path.home() / ".local/share/thinkthen-build-tools/duckdb-community" / version
for project, revision in versions[version].items():
    destination = cache / project
    command = ["git", "--git-dir=" + str(destination / ".git"), "--work-tree=" + str(destination)]
    if not (destination / ".git").exists():
        destination.mkdir(parents=True, exist_ok=True)
        subprocess.run([*command, "init", "-q", str(destination)], check=True)
    def git(*args):
        return subprocess.check_output([*command, *args], text=True).strip()
    head = subprocess.run([*command, "rev-parse", "--verify", "HEAD"],
                          capture_output=True, text=True)
    if head.returncode or head.stdout.strip() != revision:
        subprocess.run([*command, "fetch", "--depth=1",
                        "https://github.com/duckdb/" + project + ".git", revision], check=True)
        subprocess.run([*command, "checkout", "--detach", "FETCH_HEAD"], check=True)
    if git("rev-parse", "HEAD") != revision:
        sys.exit("upstream revision mismatch: " + project)
print(cache)
