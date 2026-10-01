# 0386: The macOS release smoke passes PostgreSQL find and reports a short DuckDB held count

Status: in progress
Milestone: 0.1

Lane claude-4. Branch `ticket/0386-macos-smoke`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Parent: ticket 0128, phase 3b.

## Outcome

1. On both Macs, the PostgreSQL installed check passes `find_proxy_cases`. The shared loopback proxy, `databases/sqlite/tests/conditional_backend.py`, starts listening without a reverse DNS lookup of its own address. A proxy that prints no port, or exits with an error, fails the step with a reason and the proxy's error output, never silently.
2. The DuckDB held-interrupt check names what happened when a held call never reaches the backend: the count the backend read in its 30-second bound and the child's answer, if it gave one. The 30-second bound stays. The ticket records the cause analysis for the Intel Mac failure and the earlier Intel Mac Python case 47 failure.

## Evidence

- Starts from: rehearsal run 36889643043 at main `39f3b99fa`. Every build and package job passed, and both Linux smoke jobs passed entirely. Both Macs failed one PostgreSQL step, and Intel Mac also failed one DuckDB step.
  - Both Macs (jobs 110481955860 and 110481955924): `databases/postgresql` failed `find_proxy_cases` with no message, 36.3 s (ARM) and 37.0 s (Intel) after `find_inputs` passed. x86 Linux passed the same step in 6.0 s. The step starts `python3 tests/find_cases.py proxy ...` with its output and errors sent to files, waits 5 s for the proxy to print its port, and then runs `[ -n "$proxyport" ]`, which fails without a message. The step's `EXIT` trap then writes `quit` and waits for the proxy to end.
  - The proxy is `ConditionalBackend`, a `ThreadingHTTPServer` bound to `127.0.0.1`. CPython's `HTTPServer.server_bind` calls `socket.getfqdn(host)` before it listens. macOS GitHub runners boot with a `.local` host name that they cannot resolve, and that lookup stalls about 35 s there. Other projects measured it: twisted/twisted issue 12832 ("Github Actions on MacOS 15 runs extremely slowly due to `sockets.getfqdn()`") and caty-ai/caty-gateway issue 19 ("ThreadingHTTPServer startup blocks ~36 s in socket.getfqdn ... measured on the macOS lane"). A 35 s stall before the port line explains the silent failure after 5 s, and the step's total of about 36 s, because the trap waits for the proxy to finish starting.
  - The same proxy slows the SQLite installed step "selected find value, budget and interrupt boundaries", which starts it three times: 38.3 s on ARM Mac and 39.0 s on Intel Mac, against 2.5 s on x86 Linux. That step has a 300 s limit and passed.
  - Intel Mac (job 110481955924): `databases/duckdb/cpp/verify_interrupt.py` line 107, `assert backend.wait(1) == 1`, failed in the first `held` call (`thinkthen_details`). `Backend.wait(least)` asks the conformance backend for its count once it reaches `least`; the backend answers at 30 s at the latest (`conformance/backend/src/arms.rs`, `WAIT_BOUND`). The assertion came 31.0 s after the previous line, so the bound was 30 s, not 1 s, and the backend read no request in that time. The child ended within 0.2 s of its input closing (the next log line), so its query was no longer running: it had ended without a request reaching the backend's count, and the test never read its answer. The previous run, 36864015235, passed the same script on Intel Mac in 2.9 s, and ARM Mac and both Linux jobs passed it on both runs. This is not a load bound; a larger number would not change it.
  - Run 36864015235's Intel Mac Python failure, `47-recognize-C18-relations` with `BackendError: the backend closed the connection before a reply`, did not recur. On run 36889643043 Intel Mac passed `libraries/python`, and case 47 passed in the Python, PostgreSQL and SQLite conformance runs. Both Intel Mac failures are one request that did not complete against the conformance backend, on one job each, and neither repeats.
- Keeps: every PostgreSQL step, its 5 s port wait and its five find modes; the proxy's loopback-only address rule, `Connection: close` reply and counting; the DuckDB check's 30 s bound, the 100 ms cancel promise and every assertion after the count.
- Changes:
  - `databases/sqlite/tests/conditional_backend.py`: the server class names itself from its bound address and skips `socket.getfqdn`.
  - `databases/sqlite/tests/test_values.py`: one regression in which `socket.getfqdn` raises, and the proxy still starts and reports its loopback base.
  - `databases/postgresql/check.sh`, `find_proxy_cases`: a missing port line prints `the find proxy printed no port in 5 s` and the proxy's error file; a failed proxy exit prints the error file.
  - `databases/duckdb/cpp/verify_interrupt.py`: a short count fails with the count, the bound and the child's answer, read with a 1 s limit.
- Proof:
  - On this host, with a `sitecustomize.py` on `PYTHONPATH` that makes `socket.getfqdn` sleep 35 s: main's PostgreSQL installed check on run 36889643043's x86 Linux PostgreSQL archive fails `find_proxy_cases` with no message after about 36 s, as on the runner. The branch passes it. Without the shim, both pass.
  - The new regression fails on main's `conditional_backend.py` and passes on the branch. `test_values.py` passes in full.
  - On the M5, the branch's `ConditionalBackend` starts under the M5's Python and serves one canned reply.
  - A planted DuckDB held arm that never receives the request (the child aimed at a closed port) fails with the new message and the child's answer.
  - `sdlc/scripts/lint` in full, `release-archive-self-test.py`, `release-managed-pair-self-test.py`, `release-registry-self-test.py`, `release-language-tools-self-test.py`, `workflows --self-test`, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` and `sdlc/scripts/tickets`.
  - No GitHub Actions run is dispatched. Ian's next rehearsal proves the runner side.
- Defers: the cause of the two single Intel Mac request failures. If either recurs, the DuckDB message or the Python error names the next step. The other Python test servers under `libraries/` bind `HTTPServer` the same way and pay the same stall on a macOS runner; they pass within their limits, and this ticket leaves them.

## What the build taught us

(added before landing)
