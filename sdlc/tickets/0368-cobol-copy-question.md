# 0368: COBOL calls never write into the caller's text

Status: COMPLETE.

Opened as: 2026-10-11. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-2. Reported by the site owner.

## Outcome

`TT-DECIDE`, `TT-CALL` and `TT-ENGINE-NEW` copy the caller's text into their own storage before they add the closing NUL byte the C library needs. They never write into a buffer the caller passed. A caller can call `TT-DECIDE` twice with one question buffer and a length of `length(trim(question trailing))`, and both calls succeed. A caller buffer that is exactly as long as its text keeps the bytes that follow it.

## Evidence

- Starts from: the site owner's report. `src/tt_decide.cob` moves `x"00"` into `question-text(question-length + 1:1)`. `question-text` is the caller's storage, passed by reference. The site's COBOL sample sets the length from `length(trim(question trailing))`. On a second call with the same buffer, the trim keeps the NUL from the first call, so the length grows by one and the NUL scan refuses with "interior NUL in C-string question". The site's COBOL page carries a workaround line: "`TT-DECIDE` writes into the question buffer, so set the question again before each call." The linkage item is `pic x(256)` and the length check allows 255, so a caller buffer of `pic x(200)` holding 200 bytes takes the NUL one byte past its end. A scan of `libraries/cobol/src` finds the same write in `TT-CALL` (`request-text`, up to 8191 bytes) and `TT-ENGINE-NEW` (`settings-text`, up to 8191 bytes). `TT-PLAN` already builds its input in its own `plan-input` and writes the NUL there. The evidence argument of `TT-DECIDE` is passed with its length and gets no NUL, so it needs no change. `TT-VALIDATE-LABELS`, `TT-PARSE-FIELD` and `TT-JSON-MEMBER` write only their output arguments.
- Keeps: each entry point's argument list, copybook layout and length limits; the interior-NUL refusals and their messages; the usage failure code 1 for a length out of range; evidence passed by byte count; all existing COBOL checks and their exact arrival counts, with the new calls added.
- Changes: three called programs copy the caller's text, and one check proves it.
  - `src/tt_decide.cob`: a 256-byte `question-copy` in working storage takes the question's bytes and the NUL; the native call reads the copy.
  - `src/tt_call.cob`: an 8192-byte `request-copy` does the same for the request.
  - `src/tt_engine.cob`: an 8192-byte `settings-copy` does the same for the settings.
  - `checks/failure.cob` opens its engine through `TT-ENGINE-NEW` with an exact 15-byte `{"cache":false}` settings field, so every repeat reaches the backend. Each buffer and an 8-byte guard field are `02` items under one `01` group, so the guard sits right after the buffer. Every call takes its lengths from `length(trim(... trailing))`. The existing three calls keep their evidence (`failure-two` twice, then `first`). Then `TT-DECIDE` runs twice with evidence `decide-again` and one `pic x(200)` question buffer holding `Is it?`. `TT-CALL` runs twice with one `pic x(100)` request buffer holding `{"decide":"Is it?","evidence":"call-again"}`. Last, the question buffer is filled to all 200 bytes and `TT-DECIDE` runs once with evidence `decide-full`, the report's short-buffer case. Every new call must succeed, and every guard must stay whole.
  - `checks/failure.py` expects exactly `{"failure-two": 2, "first": 1, "decide-again": 2, "call-again": 2, "decide-full": 1}`. `check.sh` links `tt_engine.cob` and `tt_call.cob` into the failure program.
  - `libraries/cobol/README.md` says the called programs copy text and never write into caller storage.
- Proof: `sh libraries/cobol/check.sh` on Linux x86_64. The new rows in `checks/failure.cob` fail on main's `src/` (the second `TT-DECIDE` and the second `TT-CALL` refuse with their interior-NUL messages, and the settings and full question guards lose their first byte) and pass on the branch. `sdlc/scripts/lint` and `sdlc/scripts/tickets`. No Rust changes, so `policy.py` is not needed.
- Defers: the site's COBOL page line that tells callers to reset the question belongs to the site owner, who removes it after this lands. Thread safety of the working-storage copies: the COBOL package already keeps per-call state in working storage and documents single-threaded use.

## What the build taught us

- Each new row in `checks/failure.cob` failed alone against main's `src/`, built in a scratch folder with the guard checks before it turned off one at a time. On main the settings guard's first byte became NUL, the second `TT-DECIDE` refused with "interior NUL in C-string question", the full 200-byte question's guard lost its first byte, and the second `TT-CALL` refused with "interior NUL in request". All pass on the branch.
- The answer cache would have hidden the repeats from the backend's arrival count. The check's settings turn the cache off, so the counts stay exact.
- `checks/installed.py` compiles the same `failure.cob` from a copied package, so it needed the two new programs and the five new arrivals too.
- `sh libraries/cobol/check.sh`, `sdlc/scripts/smoke libraries/cobol` and `sdlc/scripts/lint` passed. No Rust changed.
- The code review asked for two changes: the README now names the text the programs never write, and the check runs in the ticket's order. It then accepted.
