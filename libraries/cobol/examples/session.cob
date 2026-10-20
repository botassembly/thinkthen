identification division.
program-id. SessionCaller.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 engine usage pointer.
01 session-owner usage pointer.
01 packet-owner usage pointer.
01 terminal-owner usage pointer value null.
01 packet-view usage pointer.
01 value-owner usage pointer value null.
01 record-address usage pointer.
01 first-value usage pointer based.
01 status-code usage binary-long signed.
01 read-status usage binary-long unsigned.
01 question-text pic x(8192).
01 question-length usage binary-double unsigned.
01 evidence-text pic x(8193).
01 evidence-length usage binary-double unsigned.
01 expected-failure pic x.
01 output-text pic x(8192).
01 written usage binary-double unsigned.
procedure division.
    accept question-text from environment "TT_SESSION_QUESTION_FILE"
    accept evidence-text from environment "TT_SESSION_EVIDENCE"
    accept expected-failure from environment "TT_SESSION_EXPECT_FAILURE"
    move function length(function trim(evidence-text trailing))
       to evidence-length
    call "thinkthen_engine_new" returning engine
    if engine = null move 1 to return-code goback end-if
    allocate tt-n-cobol-RequestCall
    allocate tt-n-cobol-RequestQuestion
    allocate tt-n-cobol-RequestQuestionFile
    allocate tt-n-cobol-RequestQuestionFilePath
    allocate tt-n-cobol-RequestInput
    allocate tt-n-cobol-RequestInputRecords
    allocate tt-n-cobol-RequestInputRecordsItems
    allocate tt-n-cobol-RequestItem
    allocate tt-n-cobol-RequestOriginal
    allocate tt-n-cobol-RequestOriginalText
    allocate tt-n-cobol-RequestOriginalTextText
    move low-values to tt-n-cobol-RequestCall
       tt-n-cobol-RequestQuestionFile tt-n-cobol-RequestInputRecords
       tt-n-cobol-RequestItem tt-n-cobol-RequestOriginalText
    move function length(function trim(question-text trailing)) to question-length
    call "TT_QUESTION_FILE" using
       by reference tt-n-cobol-RequestQuestion tt-n-cobol-RequestQuestionFile
          tt-n-cobol-RequestQuestionFilePath question-text
       by value question-length
       returning status-code
    move tt-n-cobol-requestinputrecords-kind-k
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInputRecords
    call "TT_TEXT_ITEM" using
       by reference tt-n-cobol-RequestItem tt-n-cobol-RequestOriginal
          tt-n-cobol-RequestOriginalText tt-n-cobol-RequestOriginalTextText
          evidence-text by value evidence-length returning status-code
    set record-address to address of tt-n-cobol-RequestItem
    set v-data of tt-n-cobol-RequestInputRecordsItems
       to address of record-address
    move 1 to v-len of tt-n-cobol-RequestInputRecordsItems
    set v-m-items of tt-n-cobol-RequestInputRecords
       to address of tt-n-cobol-RequestInputRecordsItems
    set v-m-input of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestInput
    set v-m-question of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestQuestion
    call "TT_SESSION_DECIDE" using by value engine
       by reference tt-n-cobol-RequestCall session-owner
       returning status-code
    free tt-n-cobol-RequestCall tt-n-cobol-RequestQuestion
       tt-n-cobol-RequestQuestionFile
       tt-n-cobol-RequestQuestionFilePath
       tt-n-cobol-RequestInput tt-n-cobol-RequestInputRecords
       tt-n-cobol-RequestInputRecordsItems
       tt-n-cobol-RequestItem tt-n-cobol-RequestOriginal
       tt-n-cobol-RequestOriginalText
       tt-n-cobol-RequestOriginalTextText
    if status-code not = 0
       call "thinkthen_engine_free" using by value engine returning omitted
       display "ADMISSION " status-code
       move 0 to return-code goback
    end-if
    perform until read-status = tt-n-session-end-k
       call "TT_SESSION_NEXT" using by value session-owner
          by reference read-status packet-owner returning status-code
       if status-code not = 0 move 2 to return-code goback end-if
       if read-status = tt-n-session-result-k
          call "thinkthen_session_result_view" using by value packet-owner
             by reference packet-view returning status-code
          if status-code not = 0 move 3 to return-code goback end-if
          set address of tt-n-complete-session-packet to packet-view
          if v-kind of tt-n-complete-session-packet =
             tt-n-complete-session-packet-terminal-k
             move packet-owner to terminal-owner
          else
             if value-owner = null and
                v-kind of tt-n-complete-session-packet not =
                tt-n-complete-session-packet-observation-k
                move packet-owner to value-owner
             else
             call "thinkthen_session_result_free" using by value packet-owner
                returning omitted
             end-if
          end-if
       end-if
    end-perform
    call "thinkthen_session_cancel" using by value session-owner
       returning omitted
    call "thinkthen_session_free" using by value session-owner returning omitted
    call "thinkthen_engine_free" using by value engine returning omitted
    if terminal-owner = null move 4 to return-code goback end-if
    call "thinkthen_session_result_view" using by value terminal-owner
       by reference packet-view returning status-code
    set address of tt-n-complete-session-packet to packet-view
    set address of tt-n-complete-session-packet-terminal
       to v-data-terminal of tt-n-complete-session-packet
    evaluate v-facts-presence of tt-n-complete-session-packet-terminal
       when tt-n-complete-presence-value-k
          set address of tt-n-complete-facts
             to v-facts-value of tt-n-complete-session-packet-terminal
          display "REQUESTS " v-requests-sent of tt-n-complete-facts
       when tt-n-complete-presence-missing-k
          display "FACTS_ABSENT"
       when tt-n-complete-presence-null-k
          display "FACTS_NULL"
       when other move 5 to return-code goback
    end-evaluate
    if expected-failure = "Y"
       if v-failure-presence of tt-n-complete-session-packet-terminal
          not = tt-n-complete-presence-value-k
          move 6 to return-code goback end-if
       set address of tt-n-complete-call-error
          to v-failure-value of tt-n-complete-session-packet-terminal
       if v-facts-presence of tt-n-complete-call-error =
          tt-n-complete-presence-value-k
          set address of tt-n-complete-facts
             to v-facts-value of tt-n-complete-call-error
          display "FAILURE_REQUESTS " v-requests-sent of tt-n-complete-facts
       else
          display "FAILURE_FACTS_ABSENT"
       end-if
       set address of tt-n-complete-error
          to v-error of tt-n-complete-call-error
       set address of tt-n-complete-failure-kind
          to v-kind of tt-n-complete-error
       display "ERROR_KIND " v-kind of tt-n-complete-failure-kind
       call "TT_SESSION_TEXT" using
          by reference v-message of tt-n-complete-error output-text written
          returning status-code
       if status-code not = 0 or written = 0
          move 7 to return-code goback end-if
       display "FAILURE " function trim(output-text trailing)
    else
       if v-failure-presence of tt-n-complete-session-packet-terminal
          not = tt-n-complete-presence-missing-k
          move 8 to return-code goback end-if
    end-if
    if value-owner not = null
       call "thinkthen_session_result_view" using by value value-owner
          by reference packet-view returning status-code
       set address of tt-n-complete-session-packet to packet-view
       if v-kind of tt-n-complete-session-packet =
          tt-n-complete-session-packet-decide-aggregate-k or
          tt-n-complete-session-packet-decide-row-k
          if v-kind of tt-n-complete-session-packet =
             tt-n-complete-session-packet-decide-row-k
             set address of tt-n-complete-session-packet-decide-row
                to v-data-decide-row of tt-n-complete-session-packet
             set address of tt-n-complete-atomic-decide-value
                to v-value of tt-n-complete-session-packet-decide-row
          else
          set address of tt-n-complete-session-packet-decide-aggregate
             to v-data-decide-aggregate of tt-n-complete-session-packet
          if v-value-len of tt-n-complete-session-packet-decide-aggregate not = 1
             move 13 to return-code goback end-if
          set address of first-value
             to v-value-data of tt-n-complete-session-packet-decide-aggregate
          set address of tt-n-complete-atomic-decide-value to first-value
          end-if
          evaluate v-value-presence of tt-n-complete-atomic-decide-value
             when tt-n-complete-presence-null-k
                display "VALUE_NULL"
             when tt-n-complete-presence-value-k
                set address of tt-n-complete-decide-value
                   to v-value-value of tt-n-complete-atomic-decide-value
                set address of tt-n-complete-json
                   to v-value of tt-n-complete-decide-value
                if v-kind of tt-n-complete-json not =
                   tt-n-complete-json-boolean-k
                   move 10 to return-code goback end-if
                display "VALUE " v-data-boolean of tt-n-complete-json
             when other move 9 to return-code goback
          end-evaluate
       end-if
       if v-kind of tt-n-complete-session-packet =
          tt-n-complete-session-packet-choose-aggregate-k or
          tt-n-complete-session-packet-choose-row-k
          if v-kind of tt-n-complete-session-packet =
             tt-n-complete-session-packet-choose-row-k
             set address of tt-n-complete-session-packet-choose-row
                to v-data-choose-row of tt-n-complete-session-packet
             set address of tt-n-complete-atomic-nullable-string
                to v-value of tt-n-complete-session-packet-choose-row
          else
          set address of tt-n-complete-session-packet-choose-aggregate
             to v-data-choose-aggregate of tt-n-complete-session-packet
          if v-value-len of tt-n-complete-session-packet-choose-aggregate not = 1
             move 13 to return-code goback end-if
          set address of first-value
             to v-value-data of tt-n-complete-session-packet-choose-aggregate
          set address of tt-n-complete-atomic-nullable-string to first-value
          end-if
          if v-value-presence of tt-n-complete-atomic-nullable-string
             not = tt-n-complete-presence-value-k
             move 11 to return-code goback end-if
          call "TT_SESSION_TEXT" using
             by reference v-value-value of tt-n-complete-atomic-nullable-string
             output-text written returning status-code
          if status-code not = 0 move 12 to return-code goback end-if
          display "CHOICE " function trim(output-text trailing)
       end-if
       if v-kind of tt-n-complete-session-packet =
          tt-n-complete-session-packet-score-aggregate-k or
          tt-n-complete-session-packet-score-row-k
          if v-kind of tt-n-complete-session-packet =
             tt-n-complete-session-packet-score-row-k
             set address of tt-n-complete-session-packet-score-row
                to v-data-score-row of tt-n-complete-session-packet
             set address of tt-n-complete-atomic-double
                to v-value of tt-n-complete-session-packet-score-row
          else
          set address of tt-n-complete-session-packet-score-aggregate
             to v-data-score-aggregate of tt-n-complete-session-packet
          if v-value-len of tt-n-complete-session-packet-score-aggregate not = 1
             move 13 to return-code goback end-if
          set address of first-value
             to v-value-data of tt-n-complete-session-packet-score-aggregate
          set address of tt-n-complete-atomic-double to first-value
          end-if
          display "SCORE " v-value of tt-n-complete-atomic-double
       end-if
       call "thinkthen_session_result_free" using by value value-owner
          returning omitted
    end-if
    call "thinkthen_session_result_free" using by value terminal-owner
       returning omitted
    display "SESSION_PASS"
    move 0 to return-code goback.
