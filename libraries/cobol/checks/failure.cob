       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-failure.
data division.
working-storage section.
copy "thinkthen.cpy".
01 engine usage pointer.
01 dyn-free pic x(24) value "thinkthen_engine_free".
01 question-text pic x(256) value z"Is it?".
01 question-length usage binary-double unsigned value 6.
01 evidence-text pic x(256).
01 evidence-length usage binary-double unsigned.
01 saved-facts pic x(8192).
01 saved-length usage binary-double unsigned.
01 saved-message pic x(512).
procedure division.
    call "thinkthen_engine_new" returning engine
    if engine = null perform fail-now end-if
    move "failure-two" to evidence-text
    move 11 to evidence-length
    call "TT-DECIDE" using engine question-text question-length
       evidence-text evidence-length tt-answer tt-facts tt-failure
    if not failure-backend or tt-failure-facts-length = 0
       or tt-failure-message = spaces perform fail-now end-if
    move tt-failure-facts-json to saved-facts
    move tt-failure-facts-length to saved-length
    move tt-failure-message to saved-message
    call "TT-DECIDE" using engine question-text question-length
       evidence-text evidence-length tt-answer tt-facts tt-failure
    if not failure-backend or tt-failure-facts-length = 0
       or saved-facts(1:saved-length) = spaces
       or saved-message = spaces perform fail-now end-if
    move "first" to evidence-text
    move 5 to evidence-length
    call "TT-DECIDE" using engine question-text question-length
       evidence-text evidence-length tt-answer tt-facts tt-failure
    if tt-failure-code not = 0 or tt-failure-facts-length not = 0
       or not outcome-yes or tt-probability not = 0.9
       or tt-facts-length = 0 or tt-facts-json = spaces
       perform fail-now end-if
    display "FACTS " saved-facts(1:saved-length)
    display "SUCCESS_FACTS " tt-facts-json(1:tt-facts-length)
    display "COBOL_FAILURE_OWNERSHIP_PASS"
    call dyn-free using by value engine
    move 0 to return-code
    goback.
fail-now.
    display "COBOL_FAILURE_OWNERSHIP_FAIL"
    if engine not = null call dyn-free using by value engine end-if
    move 1 to return-code
    goback.
