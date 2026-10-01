       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-failure.
*> Copied failure facts across repeated calls, and ticket 0368: the
*> called programs never write into the caller's text. A guard field
*> follows each buffer, and each buffer is reused with a trimmed length.
data division.
working-storage section.
copy "thinkthen.cpy".
01 engine usage pointer.
01 dyn-free pic x(24) value "thinkthen_engine_free".
01 settings-row.
   02 settings-text pic x(15) value '{"cache":false}'.
   02 settings-guard pic x(8) value "GUARDOK!".
01 settings-length usage binary-double unsigned value 15.
01 question-row.
   02 question-text pic x(200) value "Is it?".
   02 question-guard pic x(8) value "GUARDOK!".
01 question-length usage binary-double unsigned.
01 request-row.
   02 request-text pic x(100).
   02 request-guard pic x(8) value "GUARDOK!".
01 request-length usage binary-double unsigned.
01 result-text pic x(8192).
01 result-length usage binary-double unsigned.
01 evidence-text pic x(256).
01 evidence-length usage binary-double unsigned.
01 saved-facts pic x(8192).
01 saved-length usage binary-double unsigned.
01 saved-message pic x(512).
procedure division.
    call "TT-ENGINE-NEW" using settings-text settings-length engine tt-failure
    if engine = null or settings-guard not = "GUARDOK!" perform fail-now end-if
    move "failure-two" to evidence-text
    perform decide-it
    if not failure-backend or tt-failure-facts-length = 0
       or tt-failure-message = spaces perform fail-now end-if
    move tt-failure-facts-json to saved-facts
    move tt-failure-facts-length to saved-length
    move tt-failure-message to saved-message
    perform decide-it
    if not failure-backend or tt-failure-facts-length = 0
       or saved-facts(1:saved-length) = spaces
       or saved-message = spaces perform fail-now end-if
    move "first" to evidence-text
    perform decide-it
    if tt-failure-code not = 0 or tt-failure-facts-length not = 0
       or not outcome-yes or tt-probability not = 0.9
       or tt-facts-length = 0 or tt-facts-json = spaces
       perform fail-now end-if
    display "FACTS " saved-facts(1:saved-length)
    display "SUCCESS_FACTS " tt-facts-json(1:tt-facts-length)
    move "decide-again" to evidence-text
    perform decide-ok 2 times
    move all "x" to question-text
    move "Is it?" to question-text(1:6)
    move "decide-full" to evidence-text
    perform decide-ok
    move '{"decide":"Is it?","evidence":"call-again"}' to request-text
    perform call-ok 2 times
    display "COBOL_FAILURE_OWNERSHIP_PASS"
    call dyn-free using by value engine
    move 0 to return-code
    goback.
decide-it.
    compute question-length = function length(function trim(question-text trailing))
    compute evidence-length = function length(function trim(evidence-text trailing))
    call "TT-DECIDE" using engine question-text question-length
       evidence-text evidence-length tt-deadline-ms tt-answer tt-facts tt-failure.
decide-ok.
    perform decide-it
    if tt-failure-code not = 0 or not outcome-yes
       or question-guard not = "GUARDOK!" perform fail-now end-if.
call-ok.
    compute request-length = function length(function trim(request-text trailing))
    call "TT-CALL" using engine request-text request-length tt-deadline-ms
       result-text result-length tt-failure
    if tt-failure-code not = 0 or result-length = 0
       or request-guard not = "GUARDOK!" perform fail-now end-if.
fail-now.
    display "COBOL_FAILURE_OWNERSHIP_FAIL " tt-failure-code " "
       function trim(tt-failure-message)
    if engine not = null call dyn-free using by value engine end-if
    move 1 to return-code
    goback.
