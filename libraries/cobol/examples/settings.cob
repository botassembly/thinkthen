       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-settings.
data division.
working-storage section.
01 engine usage pointer.
01 other-engine usage pointer.
01 null-engine usage pointer.
01 error-kind usage binary-long signed.
01 settings-length usage binary-double unsigned.
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
01 dyn-free pic x(24) value "thinkthen_engine_free".
01 empty-settings pic x(3) value z"{}".
01 valid-settings pic x(32) value z'{"timeout":10}'.
01 unknown-settings pic x(32) value z'{"unknown":1}'.
01 wrong-settings pic x(32) value z'{"timeout":"bad"}'.
01 question-text pic x(16) value z"Is it?".
01 state-text pic x(16) value z"settings".
01 state-length usage binary-double unsigned value 8.
01 result-code usage binary-long signed.
01 default-answer.
   02 default-outcome usage binary-long signed.
   02 default-pad usage binary-long unsigned.
   02 default-probability usage float-long.
01 other-answer.
   02 other-outcome usage binary-long signed.
   02 other-pad usage binary-long unsigned.
   02 other-probability usage float-long.
procedure division.
    call "thinkthen_engine_new" returning engine
    if engine = null perform fail-now end-if
    move 2 to settings-length
    call "TT-ENGINE-NEW" using empty-settings settings-length
        other-engine failure-row
    if other-engine = null or failure-code not = 0
       perform fail-now end-if
    call "thinkthen_decide" using by value engine
       by reference question-text state-text
       by value size is 8 state-length
       by reference default-answer returning result-code
    if result-code not = 0 perform fail-now end-if
    call "thinkthen_decide" using by value other-engine
       by reference question-text state-text
       by value size is 8 state-length
       by reference other-answer returning result-code
    if result-code not = 0 or other-outcome not = default-outcome
       or other-probability not = default-probability
       perform fail-now
    end-if
    call dyn-free using by value other-engine
    move 14 to settings-length
    call "TT-ENGINE-NEW" using valid-settings settings-length
        other-engine failure-row
    if other-engine = null or failure-code not = 0
       perform fail-now end-if
    call "thinkthen_decide" using by value other-engine
       by reference question-text state-text
       by value size is 8 state-length
       by reference other-answer returning result-code
    if result-code not = 0 or other-outcome not = default-outcome
       or other-probability not = default-probability
       perform fail-now
    end-if
    call dyn-free using by value other-engine
    move 13 to settings-length
    call "TT-ENGINE-NEW" using unknown-settings settings-length
        null-engine failure-row
    if null-engine not = null or failure-code not = 1
       perform fail-now end-if
    move 17 to settings-length
    call "TT-ENGINE-NEW" using wrong-settings settings-length
        null-engine failure-row
    if null-engine not = null or failure-code not = 1
       perform fail-now end-if
    call dyn-free using by value engine
    display "COBOL_SETTINGS_PASS"
    move 0 to return-code
    goback.
fail-now.
    display "COBOL_SETTINGS_FAIL"
    move 1 to return-code
    goback.
