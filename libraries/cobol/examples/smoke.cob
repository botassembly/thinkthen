       >>SOURCE FORMAT FREE
*> The replay smoke (ticket 0335): one decide through TT-DECIDE on the
*> environment-reading engine, with the question and text sdlc/scripts/smoke names.
identification division.
program-id. cobol-smoke.
environment division.
configuration section.
repository. function all intrinsic.
data division.
working-storage section.
copy "thinkthen.cpy".
01 engine usage pointer.
01 question-text pic x(256).
01 question-length usage binary-double unsigned.
01 evidence-text pic x(256).
01 evidence-length usage binary-double unsigned.
procedure division.
    accept question-text from environment "THINKTHEN_SMOKE_QUESTION"
    accept evidence-text from environment "THINKTHEN_SMOKE_TEXT"
    move length(trim(question-text trailing)) to question-length
    move length(trim(evidence-text trailing)) to evidence-length
    call "thinkthen_engine_new" returning engine
    if engine = null
       move 1 to return-code
       goback
    end-if
    call "TT-DECIDE" using engine question-text question-length
       evidence-text evidence-length tt-deadline-ms tt-answer tt-facts tt-failure
    if tt-failure-code not = 0
       display "smoke decide failed: " trim(tt-failure-message) upon syserr
       move 1 to return-code
       goback
    end-if
    evaluate true
       when outcome-yes display "smoke: true"
       when outcome-no display "smoke: false"
       when other display "smoke: null"
    end-evaluate
    move 0 to return-code
    goback.
