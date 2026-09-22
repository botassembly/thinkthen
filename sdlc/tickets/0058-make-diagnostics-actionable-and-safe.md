---
flow: build
priority: 39
opens: crates/thinkthen specification demos sdlc/planning sdlc/ratchet.json
---

# 0058: Make diagnostics actionable and safe

Status: landed

## Outcome

Ordinary refusals tell a stranger what failed and what they can change. Every changed backend and transport sentence remains tool-owned and repeats no evidence, backend body, credential, address, parser text, HTTP-library text, or operating-system text. A question-file diagnostic may name its offending local JSON key after safe JSON escaping; it prints no raw control byte. A dry run refuses the same invalid width setting as a live run before it prints a plan.

## Current facts and decisions

The remaining A1 issues are wording faults at established behavior boundaries. Status 400 and an exhausted status 500 carry only a number. Transport failures paste the HTTP library's categories. Question-file shape errors name the wrong rule. Empty evidence states its rule backward. A distribution error prints binary floating-point noise. A singular CSV row says `fields`. A stopped run mentions recordings even when none were requested. A recording path that is a regular file exposes an operating-system sentence. The `set -e` warning names no but omits not sure.

The issue that asked to print a backend's own refusal body conflicts with the settled secrecy rule. This ticket declines that part and adds fixed guidance instead. Backend profiles and local size limits supply more specific preflight guidance in the next ticket. A closed connection can still consume the whole timeout; ticket 0066 owns faster engine cancellation and dead-address behavior. Ian can overturn either placement.

This ticket fixes these complete status lines:

- `thinkthen: the backend answered with status 400: the backend refused the request; check --model and the request size`
- `thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries`

“Allowed attempts” remains true when `--max-retries 0` permits the initial attempt alone. Other retryable 5xx statuses retain their existing status-only form in this ticket.

The engine converts the structured `ureq` error into one private transport kind before it crosses into the command. It never classifies rendered text. The mapping and complete command sentences are:

| Structured source | Kind | Complete line |
| --- | --- | --- |
| `ureq::Error::Timeout(_)` | timeout | `thinkthen: the backend timed out; increase --timeout or try again` |
| `ureq::Error::HostNotFound` | name lookup | `thinkthen: the backend's host could not be found; check --url and the network` |
| `ureq::Error::Io` with `ConnectionRefused` | refused connection | `thinkthen: the backend refused the connection; check that it is running and that --url is correct` |
| `ureq::Error::Io` with `UnexpectedEof`, `ConnectionReset`, `ConnectionAborted`, or `BrokenPipe` | premature close | `thinkthen: the backend closed the connection before a reply; try again or change --max-retries` |
| Every other `ureq::Error` or I/O kind | other | `thinkthen: the backend could not be reached; check --url and the network` |

Direct classifier tests construct each structured class. No DNS test reaches an outside resolver. The private engine error stays structured through retrying and reaches the command as the same kind.

When a question set lacks the required `questions` wrapper, that missing wrapper wins over any unknown top-level key. The complete sentence is `thinkthen: the question set is missing its \`questions\` object`. Once the wrapper exists, the existing unknown-key rules apply.

## Scope

Replace the messages listed above and pin their complete lines and exit codes. Report the offending unknown key in a single question file, report a missing `questions` wrapper in a question set, document the question-name rule, render the distribution tolerance as `0.01`, and omit the recording count from a stop line when neither recording option was used. Detect a recording path that is a regular file and give a fixed action. Apply the existing one-document `--jobs` rule during dry-run validation as well as live validation.

Excluded: raw backend bodies, recording refused exchanges, backend profiles, local size limits, retry policy, faster connection failure, cache behavior, request bytes, successful output, result shapes, new exit codes, and paid calls.

## Acceptance

- Status 400 and an exhausted 500 print the complete lines fixed above at exit 4. The 500 sentence remains true with zero retries. Tests use hostile response bodies and prove no body byte appears.
- Timeout, refused-connection, name-lookup, prematurely closed, and unknown transport cases print the complete tool-owned lines in the table at exit 4. Direct classifier tests prove the mapping without an outside lookup. Boundary tests prove that no address, client-library category, operating-system sentence, evidence, or credential appears. This ticket does not promise faster failure.
- A single question file with an unknown key names that safely escaped key. A question set without `questions` prints the complete missing-wrapper sentence even when it also holds an unknown top-level key. A bad question name points to the documented lowercase-letter, digit, and underscore rule. These remain local failures at exit 5.
- Empty or blank evidence says that condition directly at exit 2. A probability-total refusal displays tolerance `0.01` while keeping the full internal comparison. A one-field row says `1 field`.
- A stopped record run names recording counts only when `--record`, `--replay`, or `--cache` made recording relevant. Existing counts and plural forms remain exact.
- A recording directory argument that names a regular file gets a fixed sentence that names the path role and says to choose another path or remove the file. The sentence does not repeat the path or operating-system text.
- The `set -e` warning names both no and not sure. `--dry-run --jobs 1` on one document exits 2 with the same sentence as a live run and prints no plan or request.
- Focused tests fail first for each changed boundary, then pass. The secrecy matrix covers the new backend and transport messages. The four repository rungs and `git diff --check` pass with the key and base address unset and without an outside network request.

## Dependencies

Ticket 0057, the A1 section of `sdlc/planning/build-queue-2026-09-21.md`, `specification/backends.md`, `specification/question-file.md`, `specification/records.md`, and the eight open issue pages named by the build queue's two message rows.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 3
- Proof: 2
- Cost of error: 1
- Total: 10
- Minimum level floor: none
- Final level: 3
- Reasons: the behavior stays at existing error boundaries, but the messages span parsing, records, recordings, transport, HTTP status, help, and secrecy. The transport classification and stop-line context require focused boundary tests.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the work changes a retry, timeout duration, request, successful output, exit code, or dependency.

## Review

The first review rejected four ambiguities: the secrecy promise conflicted with naming a local JSON key, the 500 sentence falsely implied a retry when none was allowed, transport classes lacked an exact structured mapping and sentences, and the missing-wrapper error had no precedence. The repair narrows the secrecy promise, safely escapes the one allowed local key, fixes complete 400 and 500 lines, maps structured `ureq` errors without parsing their display text, fixes every transport line, makes the missing wrapper win, and corrects the complexity score. The same reviewer accepted the repaired design and its Sol Medium route.

Code review rejected the first implementation because `find` hardcoded recording as irrelevant when a bad later record stopped its aggregate preflight. The repair carries the parsed recording-option state into that stop path. Exact compiled tests prove that a named recording includes the zero recording count and the same failure without a recording option omits it. The same reviewer accepted the repair with no remaining findings.

## Implementation note

Implemented in the ticket worktree. The engine now reduces structured `ureq` and I/O errors to five safe transport kinds before the command formats them. The command owns the complete transport and status messages. Local validation now covers the missing question-set wrapper, safely escaped question-file keys, recording paths that are files, blank evidence, the singular table noun, the displayed probability tolerance, recording-aware stop summaries, and one-document dry-run width before a plan prints. Help and the affected specification and how-to pages carry the same contract.

Red evidence: the first status test printed only `the backend answered with status 400` instead of the fixed action; the first full test rung found how-to 12 still asserted the old generic transport sentence; the first named-recording `find` test omitted `0 records from a recording`. Green evidence: 177 library tests, 216 backend tests, 18 question-file tests, 18 decide-edge tests, 14 find-edge tests, and the focused all-how-to runner pass. The install rung passed before the review repair; the focused `find`, lint, full test, and specification rungs pass after it. The specification rung runs 26 command examples, 7 transform examples, replay checks, and all 19 green how-tos. `git diff --check` also passes. No paid call or outside network request ran.

The source ratchet rises from 26,179 to 26,545 non-blank Rust lines. The growth is the structured error boundary plus focused behavior and secrecy coverage across the existing command surfaces, including the two exact `find` stop-summary cases. No dependency or request behavior changed. Independent code re-review accepted the increase.
