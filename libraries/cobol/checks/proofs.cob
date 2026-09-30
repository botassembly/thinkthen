       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-proofs.
*> "plan VERB QUESTION SETTINGS TEXT..." prints TT-PLAN's object.
*> "fields REQUEST NAME..." reads each named annotate member through
*> TT-JSON-MEMBER and TT-PARSE-FIELD. "limits" checks ticket 0291's zero
*> budgets and zero cap. Each prints one JSON line.
data division.
working-storage section.
copy "thinkthen.cpy".
01 arg-count usage binary-long signed.
01 arg-number usage binary-long signed.
01 mode-name pic x(16).
01 engine usage pointer.
01 dyn-free pic x(24) value "thinkthen_engine_free".
01 settings-text pic x(8192) value "{}".
01 settings-length usage binary-double unsigned value 2.
01 verb-name pic x(16).
01 question-text pic x(8192).
01 question-length usage binary-double unsigned.
01 request-text pic x(8192).
01 request-length usage binary-double unsigned.
01 result-text pic x(8192).
01 result-length usage binary-double unsigned.
01 rows-text pic x(8192).
01 rows-length usage binary-double unsigned.
01 row-text pic x(8192).
01 row-length usage binary-double unsigned.
01 member-name pic x(64).
01 member-json pic x(8192).
01 member-length usage binary-double unsigned.
01 status-code usage binary-long signed.
01 row-index usage binary-long unsigned.
01 row-label pic z(4)9.
01 name-index usage binary-long signed.
01 separator pic x value space.
01 state-text pic x(96).
01 evidence-text pic x(256).
01 evidence-length usage binary-double unsigned.
01 question-short pic x(256) value z"Is it?".
01 question-short-length usage binary-double unsigned value 6.
procedure division.
    accept arg-count from argument-number
    move 1 to arg-number display arg-number upon argument-number
    accept mode-name from argument-value
    evaluate mode-name
       when "plan" perform plan-mode
       when "fields" perform fields-mode
       when "limits" perform limits-mode
       when other move 2 to return-code goback
    end-evaluate
    move 0 to return-code
    goback.
open-engine.
    call "TT-ENGINE-NEW" using settings-text settings-length engine tt-failure
    if engine = null display "engine failed" upon syserr move 1 to return-code goback end-if.
argument-at.
    display arg-number upon argument-number
    move spaces to request-text
    accept request-text from argument-value
    compute request-length = function length(function trim(request-text trailing)).
plan-mode.
    perform open-engine
    move 2 to arg-number perform argument-at move request-text to verb-name
    move 3 to arg-number perform argument-at
    move request-text to question-text move request-length to question-length
    move 4 to arg-number perform argument-at
    move request-text to settings-text move request-length to settings-length
    move 0 to tt-text-count
    perform varying arg-number from 5 by 1 until arg-number > arg-count
       perform argument-at
       add 1 to tt-text-count
       move request-text to tt-text(tt-text-count)
       move request-length to tt-text-length(tt-text-count)
    end-perform
    call "TT-PLAN" using engine verb-name question-text question-length
       tt-text-count tt-texts settings-text settings-length
       result-text result-length tt-failure
    if tt-failure-code = 0 display result-text(1:result-length)
    else perform print-failed end-if
    call dyn-free using by value engine.
fields-mode.
    perform open-engine
    move 2 to arg-number perform argument-at
    call "TT-CALL" using engine request-text request-length tt-deadline-ms
       result-text result-length tt-failure
    if tt-failure-code not = 0 perform print-failed goback end-if
    move "value" to member-name
    call "TT-JSON-MEMBER" using result-text result-length member-name
       rows-text rows-length status-code
    display "[" with no advancing
    perform varying row-index from 1 by 1 until status-code not = 0
       move row-index to row-label
       move function trim(row-label) to member-name
       call "TT-JSON-MEMBER" using rows-text rows-length member-name
          row-text row-length status-code
       if status-code = 0
          if row-index > 1 display "," with no advancing end-if
          display "{" with no advancing
          move space to separator
          perform varying name-index from 3 by 1 until name-index > arg-count
             perform field-state
          end-perform
          display "}" with no advancing
       end-if
    end-perform
    display "]"
    call dyn-free using by value engine.
field-state.
    display name-index upon argument-number
    move spaces to member-name
    accept member-name from argument-value
    call "TT-JSON-MEMBER" using row-text row-length member-name
       member-json member-length status-code
    call "TT-PARSE-FIELD" using member-json member-length tt-field
    evaluate true
       when field-not-sure move "unresolved" to state-text
       when field-resolved move "answered" to state-text
       when field-failed and tt-field-failure-code = 2
          string "failed backend " function trim(tt-field-cause)
             delimited by size into state-text
       when other move "invalid" to state-text
    end-evaluate
    if separator = "," display "," with no advancing end-if
    move "," to separator
    display '"' function trim(member-name) '":"' function trim(state-text) '"'
       with no advancing
    move 0 to status-code.
*> Ticket 0291: a zero cap and a zero budget each refuse before sending.
limits-mode.
    move '{"max_requests_total":0,"cache":false}' to settings-text
    move 38 to settings-length
    perform open-engine
    move "capped" to evidence-text move 6 to evidence-length
    call "TT-DECIDE" using engine question-short question-short-length
       evidence-text evidence-length tt-deadline-ms tt-answer tt-facts tt-failure
    if not failure-usage or tt-failure-code not = 1 perform limits-fail end-if
    move 0 to status-code
    inspect tt-failure-message tallying status-code for all "process send budget"
    if status-code not = 1 perform limits-fail end-if
    call dyn-free using by value engine
    move "{}" to settings-text move 2 to settings-length
    perform open-engine
    move 0 to tt-deadline-ms
    move "zero-decide" to evidence-text move 11 to evidence-length
    call "TT-DECIDE" using engine question-short question-short-length
       evidence-text evidence-length tt-deadline-ms tt-answer tt-facts tt-failure
    if not failure-deadline perform limits-fail end-if
    move '{"decide":"Is it?","evidence":"zero-call"}' to request-text
    compute request-length = function length(function trim(request-text trailing))
    call "TT-CALL" using engine request-text request-length tt-deadline-ms
       result-text result-length tt-failure
    if not failure-deadline or tt-failure-code not = 3 perform limits-fail end-if
    call dyn-free using by value engine
    display '{"limits":"pass"}'.
limits-fail.
    display "limits failed: " tt-failure-code " " function trim(tt-failure-message) upon syserr
    move 1 to return-code
    goback.
print-failed.
    evaluate true
       when failure-usage display '{"failed":{"kind":"usage","code":1}}'
       when failure-deadline display '{"failed":{"kind":"deadline","code":3}}'
       when other display '{"failed":{"kind":"other","code":' tt-failure-code '}}'
    end-evaluate.
