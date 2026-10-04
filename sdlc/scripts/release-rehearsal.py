#!/usr/bin/env python3
"""Require a completed rehearsal of the exact release commit."""

import json
import os
import re
import subprocess
import sys


WORKFLOW_PATH = ".github/workflows/release.yml"


def pages(text):
    decoder = json.JSONDecoder()
    parsed = []
    offset = 0
    while offset < len(text):
        if text[offset].isspace():
            offset += 1
            continue
        page, offset = decoder.raw_decode(text, offset)
        if (not isinstance(page, dict) or type(page.get("total_count")) is not int
                or page["total_count"] < 0 or not isinstance(page.get("workflow_runs"), list)
                or any(not isinstance(run, dict) for run in page["workflow_runs"])):
            raise ValueError("invalid page")
        parsed.append(page)
    if not parsed:
        raise ValueError("missing pages")
    return parsed


def eligible(run, sha, repository):
    branch = run.get("head_branch")
    if (not isinstance(branch, str)
            or (branch != "main" and re.fullmatch(r"release/[0-9]+\.[0-9]+", branch) is None)
            or run.get("head_sha") != sha or run.get("event") != "workflow_dispatch"
            or run.get("status") != "completed" or run.get("conclusion") != "success"):
        return False
    path = run.get("path")
    if not isinstance(path, str):
        return False
    parts = path.split("@")
    if len(parts) > 2 or (len(parts) == 2 and parts[1] not in (branch, f"refs/heads/{branch}")):
        return False
    return parts[0] in (WORKFLOW_PATH, f"{repository}/{WORKFLOW_PATH}")


def check(sha, repository):
    if (re.fullmatch(r"[0-9a-f]{40}", sha) is None
            or re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository) is None):
        raise ValueError("invalid query identity")
    endpoint = (f"repos/{repository}/actions/workflows/release.yml/runs?head_sha={sha}"
                "&status=success&event=workflow_dispatch&per_page=100")
    result = subprocess.run(["gh", "api", "--method", "GET", "--paginate", "-H",
                             "Accept: application/vnd.github+json", endpoint],
                            capture_output=True, text=True, timeout=60, check=True)
    # Parse every page before evaluating proof, including pages following a success.
    return any(eligible(run, sha, repository) for page in pages(result.stdout)
               for run in page["workflow_runs"])


def main():
    sha = os.environ.get("GITHUB_SHA", "")
    try:
        success = check(sha, os.environ.get("GITHUB_REPOSITORY", ""))
    except (OSError, ValueError, subprocess.SubprocessError):
        print(f"release-workflow: could not read the rehearsal runs for {sha}", file=sys.stderr)
        return 1
    if not success:
        print(f"release-workflow: no successful rehearsal ran on {sha}; "
              "dispatch rehearse mode on that commit first", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
