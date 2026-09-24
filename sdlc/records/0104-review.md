ACCEPT

# Review of ThinkThen Quick Fix 0104 at 62c89764

Reviewer: fresh read-only Claude session. Scratch copy at the branch commit, deleted afterwards. The worktree was not edited.

## 1. Coverage of stale files

- The check reads only `target/package/thinkthen-<version>.crate`. `catalog.py --crate` reads it, and `tar` unpacks it into a fresh `mktemp -d`. The unpacked `target/package/thinkthen-0.0.1/` directory is never read by the check.
- Cargo 1.93.1 cleans the other files itself. In the scratch copy I planted a 4 MiB random file at `target/package/tmp-crate/thinkthen-0.0.1.crate`, a 4 MiB crate, and a stray file inside `target/package/thinkthen-0.0.1/`. `package` exited 0. Afterwards `tmp-crate` held a 463387-byte file, and the stray file was gone. Only the final crate is written in place without truncation, and the fix removes it.
- No other crate is packaged. The conformance backend is named `conformance-backend` and is not packaged.
- Glob safety: with no match, `sh` passes the literal pattern, and `rm -f` ignores it. Observed: exit 0. The pattern is a fixed relative path under `target/package` after `cd "$REPO"`. It cannot reach outside that folder. A directory named `thinkthen-*.crate` would make `rm` fail and stop the script under `set -e`. That fails closed and is not a realistic case.

## 2. The permanent plant in lint

Keep it in `lint` as written. It is the simplest option.
- The plant adds one 4 MiB file write to a rung that already runs `package`. No extra cargo work.
- A self-test beside `pages-self-test` or `demos-self-test` would have to run `package` twice to prove anything real. That run takes several cargo builds. Testing only the `rm` line alone would prove nothing about cargo.
- Random bytes are the right content. `gzip` may skip trailing zero bytes without an error, so a zero-filled plant might not go red.
- Optional nit: the version `sed` line now appears in both `lint` and `package`. Not worth a change.

## 3. Red and green (scratch copy)

- Fixed branch, `lint`: exit 0.
- `sdlc/scripts/package` restored from origin/main, `lint` rerun: exit 2 with `gzip: stdin: decompression OK, trailing garbage ignored` and `tar: Child returned status 2`.
- Fix restored, `package` over planted crate and tmp file: exit 0.

## 4. Scope and merge

- `git diff --stat origin/main...HEAD` shows five files: `sdlc/scripts/package`, `sdlc/scripts/lint`, the issue, the ticket, and the record. These match the ticket's allowed list.
- The branch sits directly on 23fa28f2. origin/main is 23fa28f2. `git merge-tree --write-tree origin/main HEAD` exits 0.

## 5. Gates (scratch copy, all THINKTHEN_ variables unset)

- `uptime` load average 3.3 to 4.9 before the runs.
- `sdlc/scripts/lint`: exit 0.
- `sdlc/scripts/spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.
