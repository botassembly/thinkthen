# 0368: COBOL calls never write into the caller's text

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-2. Reported by the site owner.

## Outcome

`TT-DECIDE`, `TT-CALL` and `TT-ENGINE-NEW` copy the caller's text into their own storage before they add the closing NUL byte the C library needs. They never write into a buffer the caller passed. A caller can call `TT-DECIDE` twice with one question buffer and a length of `length(trim(question trailing))`, and both calls succeed. A caller buffer that is exactly as long as its text keeps the bytes that follow it.

## Evidence

- Starts from: the site owner's report. `src/tt_decide.cob` moves `x"00"` into `question-text(question-length + 1:1)`. `question-text` is the caller's storage, passed by reference. The site's COBOL sample sets the length from `length(trim(question trailing))`. On a second call with the same buffer, the trim keeps the NUL from the first call, so the length grows by one and the NUL scan refuses with "interior NUL in C-string question". The site's COBOL page carries a workaround line: "`TT-DECIDE` writes into the question buffer, so set the question again before each call." The linkage item is `pic x(256)` and the length check allows 255, so a caller buffer of `pic x(200)` holding 200 bytes takes the NUL one byte past its end. A scan of `libraries/cobol/src` finds the same write in `TT-CALL` (`request-text`, up to 8191 bytes) and `TT-ENGINE-NEW` (`settings-text`, up to 8191 bytes). `TT-PLAN` already builds its input in its own `plan-input` and writes the NUL there. The evidence argument of `TT-DECIDE` is passed with its length and gets no NUL, so it needs no change. `TT-VALIDATE-LABELS`, `TT-PARSE-FIELD` and `TT-JSON-MEMBER` write only their output arguments.
- Keeps: each entry point's argument list, copybook layout and length limits; the interior-NUL refusals and their messages; the usage failure code 1 for a length out of range; evidence passed by byte count; all existing COBOL checks and their exact arrival counts, with the new calls added.
- Changes: three called programs copy the caller's text, and one check proves it.
  - `src/tt_decide.cob`: a 256-byte `question-copy` in working storage takes the question's bytes and the NUL; the native call reads the copy.
  - `src/tt_call.cob`: an 8192-byte `request-copy` does the same for the request.
  - `src/tt_engine.cob`: an 8192-byte `settings-copy` does the same for the settings.
  - `checks/failure.cob` opens its engine through `TT-ENGINE-NEW` with an exact 2-byte `{}` settings field followed by a guard field. It calls `TT-DECIDE` twice with one `pic x(200)` question buffer that holds a short question, and `TT-CALL` twice with one `pic x(100)` request buffer that holds a short request. Each call takes its length from `length(trim(... trailing))`. It then fills all 200 bytes of the question buffer and calls `TT-DECIDE` once more, the report's short-buffer case. A guard field follows each buffer. Every call must succeed, and every guard must stay whole.
  - `checks/failure.py` expects the five new arrivals. `check.sh` links `tt_engine.cob` and `tt_call.cob` into the failure program.
  - `libraries/cobol/README.md` says the called programs copy text and never write into caller storage.
- Proof: `sh libraries/cobol/check.sh` on Linux x86_64. The new rows in `checks/failure.cob` fail on main's `src/` (the second `TT-DECIDE` and the second `TT-CALL` refuse with their interior-NUL messages, and the settings and full question guards lose their first byte) and pass on the branch. `sdlc/scripts/lint` and `sdlc/scripts/tickets`. No Rust changes, so `policy.py` is not needed.
- Defers: the site's COBOL page line that tells callers to reset the question belongs to the site owner, who removes it after this lands. Thread safety of the working-storage copies: the COBOL package already keeps per-call state in working storage and documents single-threaded use.
