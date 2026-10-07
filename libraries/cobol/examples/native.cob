identification division.
program-id. NativeConsumer.
data division.
working-storage section.
copy "thinkthen-typed.cpy".
01 settings-text pic x(8192).
01 engine-ptr usage pointer.
01 question-ptr usage pointer.
01 source-ptr usage pointer.
01 result-ptr usage pointer.
01 question-text pic x(7) value "Refund?".
01 evidence-text pic x(9000) value all "x".
01 written usage binary-double unsigned.
01 count-value usage binary-double unsigned value 9000.
01 records-count usage binary-double unsigned value 1.
01 zero-index usage binary-double unsigned value 0.
01 code-value usage binary-long signed.
procedure division.
    accept settings-text from environment "TT_NATIVE_SETTINGS"
    call "thinkthen_engine_new_with" using
       by reference function concatenate(function trim(settings-text), x"00")
       returning engine-ptr
    if engine-ptr = null move 1 to return-code goback end-if
    allocate tt-question-spec-v1
    move low-values to tt-question-spec-v1
    move tt-c-function-decide-v1 to v-kind of tt-question-spec-v1
    set address of tt-content-v1 to address of v-text of tt-question-spec-v1
    move tt-c-content-text-v1 to v-kind of tt-content-v1
    set address of tt-string-v1 to address of v-data of tt-content-v1
    set v-data of tt-string-v1 to address of question-text
    move 7 to v-len of tt-string-v1
    call "TT_QUESTION_NEW" using by value engine-ptr
       by reference tt-question-spec-v1 question-ptr returning code-value
    if code-value not = 0 move 2 to return-code goback end-if
    allocate tt-record-v1
    move low-values to tt-record-v1
    set address of tt-optional-content-v1 to address of v-original of tt-record-v1
    move 1 to v-present of tt-optional-content-v1
    set address of tt-content-v1 to address of v-value of tt-optional-content-v1
    move tt-c-content-text-v1 to v-kind of tt-content-v1
    set address of tt-string-v1 to address of v-data of tt-content-v1
    set v-data of tt-string-v1 to address of evidence-text
    move 9000 to v-len of tt-string-v1
    call "TT_SOURCE_RECORDS" using by value engine-ptr
       by reference tt-record-v1 by value size is 8 records-count
       by reference source-ptr returning code-value
    if code-value not = 0 move 3 to return-code goback end-if
    move all "z" to evidence-text question-text
    free tt-question-spec-v1 tt-record-v1
    allocate tt-controls-v1
    move low-values to tt-controls-v1
    move -1 to v-deadline-ms of tt-controls-v1
    call "TT_DECIDE" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    if code-value not = 0 move 4 to return-code goback end-if
    free tt-controls-v1
    call "thinkthen_source_free" using by value source-ptr returning omitted
    call "thinkthen_question_free" using by value question-ptr returning omitted
    call "thinkthen_engine_free" using by value engine-ptr returning omitted
    allocate tt-decide-view-v1
    call "thinkthen_result_decide" using by value result-ptr
       by value size is 8 zero-index by reference tt-decide-view-v1
       returning code-value
    if code-value not = 0 move 5 to return-code goback end-if
    set address of tt-decide-value-v1 to address of v-value of tt-decide-view-v1
    if v-kind of tt-decide-value-v1 not = tt-c-decide-boolean-v1
       move 6 to return-code goback end-if
    if v-boolean of tt-decide-value-v1 not = 1
       move 7 to return-code goback end-if
    free tt-decide-view-v1
    call "thinkthen_result_free" using by value result-ptr returning omitted
    display "COBOL_NATIVE_PASS"
    move 0 to return-code
    goback.
