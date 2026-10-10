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
    allocate tt-n-cobol-RequestCall-decide
    allocate tt-n-cobol-RequestQuestion
    allocate tt-n-cobol-RequestQuestion-file
    allocate tt-n-cobol-RequestQuestion-file-member-path
    allocate tt-n-cobol-RequestInput
    allocate tt-n-cobol-RequestInput-records
    allocate tt-n-cobol-RequestInput-records-member-items
    allocate tt-n-cobol-RequestItem
    allocate tt-n-cobol-RequestOriginal
    allocate tt-n-cobol-RequestOriginal-text
    allocate tt-n-cobol-RequestOriginal-text-member-text
    move low-values to tt-n-cobol-RequestCall-decide
       tt-n-cobol-RequestQuestion-file tt-n-cobol-RequestInput-records
       tt-n-cobol-RequestItem tt-n-cobol-RequestOriginal-text
    move tt-n-cobol-requestquestion-file-kind-constant
       to v-kind of tt-n-cobol-RequestQuestion
    set v-value of tt-n-cobol-RequestQuestion
       to address of tt-n-cobol-RequestQuestion-file
    set v-data of tt-n-cobol-RequestQuestion-file-member-path
       to address of question-text
    move function length(function trim(question-text trailing))
       to v-len of tt-n-cobol-RequestQuestion-file-member-path
    set v-m-path of tt-n-cobol-RequestQuestion-file
       to address of tt-n-cobol-RequestQuestion-file-member-path
    move tt-n-cobol-requestinput-records-kind-constant
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInput-records
    set v-data of tt-n-cobol-RequestOriginal-text-member-text
       to address of evidence-text
    move evidence-length to v-len of tt-n-cobol-RequestOriginal-text-member-text
    set v-m-text of tt-n-cobol-RequestOriginal-text
       to address of tt-n-cobol-RequestOriginal-text-member-text
    move tt-n-cobol-requestoriginal-text-kind-constant
       to v-kind of tt-n-cobol-RequestOriginal
    set v-value of tt-n-cobol-RequestOriginal
       to address of tt-n-cobol-RequestOriginal-text
    set v-m-original of tt-n-cobol-RequestItem
       to address of tt-n-cobol-RequestOriginal
    set record-address to address of tt-n-cobol-RequestItem
    set v-data of tt-n-cobol-RequestInput-records-member-items
       to address of record-address
    move 1 to v-len of tt-n-cobol-RequestInput-records-member-items
    set v-m-items of tt-n-cobol-RequestInput-records
       to address of tt-n-cobol-RequestInput-records-member-items
    set v-m-input of tt-n-cobol-RequestCall-decide
       to address of tt-n-cobol-RequestInput
    set v-m-question of tt-n-cobol-RequestCall-decide
       to address of tt-n-cobol-RequestQuestion
    call "TT_SESSION_DECIDE" using by value engine
       by reference tt-n-cobol-RequestCall-decide session-owner
       returning status-code
    free tt-n-cobol-RequestCall-decide tt-n-cobol-RequestQuestion
       tt-n-cobol-RequestQuestion-file
       tt-n-cobol-RequestQuestion-file-member-path
       tt-n-cobol-RequestInput tt-n-cobol-RequestInput-records
       tt-n-cobol-RequestInput-records-member-items
       tt-n-cobol-RequestItem tt-n-cobol-RequestOriginal
       tt-n-cobol-RequestOriginal-text
       tt-n-cobol-RequestOriginal-text-member-text
    if status-code not = 0
       call "thinkthen_engine_free" using by value engine returning omitted
       display "ADMISSION " status-code
       move 0 to return-code goback
    end-if
    perform until read-status = tt-n-session-end-v1-constant
       call "TT_SESSION_NEXT" using by value session-owner
          by reference read-status packet-owner returning status-code
       if status-code not = 0 move 2 to return-code goback end-if
       if read-status = tt-n-session-result-v1-constant
          call "thinkthen_session_result_view" using by value packet-owner
             by reference packet-view returning status-code
          if status-code not = 0 move 3 to return-code goback end-if
          set address of tt-n-complete-session-packet-v1 to packet-view
          if v-kind of tt-n-complete-session-packet-v1 =
             tt-n-complete-session-packet-terminal-v1-constant
             move packet-owner to terminal-owner
          else
             if value-owner = null and
                v-kind of tt-n-complete-session-packet-v1 not =
                tt-n-complete-session-packet-observation-v1-constant
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
    set address of tt-n-complete-session-packet-v1 to packet-view
    set address of tt-n-complete-session-packet-terminal-v1
       to v-data-terminal of tt-n-complete-session-packet-v1
    evaluate v-facts-presence of tt-n-complete-session-packet-terminal-v1
       when tt-n-complete-presence-value-v1-constant
          set address of tt-n-complete-facts-v1
             to v-facts-value of tt-n-complete-session-packet-terminal-v1
          display "REQUESTS " v-requests-sent of tt-n-complete-facts-v1
       when tt-n-complete-presence-missing-v1-constant
          display "FACTS_ABSENT"
       when tt-n-complete-presence-null-v1-constant
          display "FACTS_NULL"
       when other move 5 to return-code goback
    end-evaluate
    if expected-failure = "Y"
       if v-failure-presence of tt-n-complete-session-packet-terminal-v1
          not = tt-n-complete-presence-value-v1-constant
          move 6 to return-code goback end-if
       set address of tt-n-complete-call-error-v1
          to v-failure-value of tt-n-complete-session-packet-terminal-v1
       if v-facts-presence of tt-n-complete-call-error-v1 =
          tt-n-complete-presence-value-v1-constant
          set address of tt-n-complete-facts-v1
             to v-facts-value of tt-n-complete-call-error-v1
          display "FAILURE_REQUESTS " v-requests-sent of tt-n-complete-facts-v1
       else
          display "FAILURE_FACTS_ABSENT"
       end-if
       set address of tt-n-complete-error-v1
          to v-error of tt-n-complete-call-error-v1
       set address of tt-n-complete-failure-kind-v1
          to v-kind of tt-n-complete-error-v1
       display "ERROR_KIND " v-kind of tt-n-complete-failure-kind-v1
       call "TT_SESSION_TEXT" using
          by reference v-message of tt-n-complete-error-v1 output-text written
          returning status-code
       if status-code not = 0 or written = 0
          move 7 to return-code goback end-if
       display "FAILURE " function trim(output-text trailing)
    else
       if v-failure-presence of tt-n-complete-session-packet-terminal-v1
          not = tt-n-complete-presence-missing-v1-constant
          move 8 to return-code goback end-if
    end-if
    if value-owner not = null
       call "thinkthen_session_result_view" using by value value-owner
          by reference packet-view returning status-code
       set address of tt-n-complete-session-packet-v1 to packet-view
       if v-kind of tt-n-complete-session-packet-v1 =
          tt-n-complete-session-packet-decide-aggregate-v1-constant or
          tt-n-complete-session-packet-decide-row-v1-constant
          if v-kind of tt-n-complete-session-packet-v1 =
             tt-n-complete-session-packet-decide-row-v1-constant
             set address of tt-n-complete-session-packet-decide-row-v1
                to v-data-decide-row of tt-n-complete-session-packet-v1
             set address of tt-n-complete-atomic-decide-value-v1
                to v-value of tt-n-complete-session-packet-decide-row-v1
          else
          set address of tt-n-complete-session-packet-decide-aggregate-v1
             to v-data-decide-aggregate of tt-n-complete-session-packet-v1
          if v-value-len of tt-n-complete-session-packet-decide-aggregate-v1 not = 1
             move 13 to return-code goback end-if
          set address of first-value
             to v-value-data of tt-n-complete-session-packet-decide-aggregate-v1
          set address of tt-n-complete-atomic-decide-value-v1 to first-value
          end-if
          evaluate v-value-presence of tt-n-complete-atomic-decide-value-v1
             when tt-n-complete-presence-null-v1-constant
                display "VALUE_NULL"
             when tt-n-complete-presence-value-v1-constant
                set address of tt-n-complete-decide-value-v1
                   to v-value-value of tt-n-complete-atomic-decide-value-v1
                set address of tt-n-complete-json-v1
                   to v-value of tt-n-complete-decide-value-v1
                if v-kind of tt-n-complete-json-v1 not =
                   tt-n-complete-json-boolean-v1-constant
                   move 10 to return-code goback end-if
                display "VALUE " v-data-boolean of tt-n-complete-json-v1
             when other move 9 to return-code goback
          end-evaluate
       end-if
       if v-kind of tt-n-complete-session-packet-v1 =
          tt-n-complete-session-packet-choose-aggregate-v1-constant or
          tt-n-complete-session-packet-choose-row-v1-constant
          if v-kind of tt-n-complete-session-packet-v1 =
             tt-n-complete-session-packet-choose-row-v1-constant
             set address of tt-n-complete-session-packet-choose-row-v1
                to v-data-choose-row of tt-n-complete-session-packet-v1
             set address of tt-n-complete-atomic-nullable-string-v1
                to v-value of tt-n-complete-session-packet-choose-row-v1
          else
          set address of tt-n-complete-session-packet-choose-aggregate-v1
             to v-data-choose-aggregate of tt-n-complete-session-packet-v1
          if v-value-len of tt-n-complete-session-packet-choose-aggregate-v1 not = 1
             move 13 to return-code goback end-if
          set address of first-value
             to v-value-data of tt-n-complete-session-packet-choose-aggregate-v1
          set address of tt-n-complete-atomic-nullable-string-v1 to first-value
          end-if
          if v-value-presence of tt-n-complete-atomic-nullable-string-v1
             not = tt-n-complete-presence-value-v1-constant
             move 11 to return-code goback end-if
          call "TT_SESSION_TEXT" using
             by reference v-value-value of tt-n-complete-atomic-nullable-string-v1
             output-text written returning status-code
          if status-code not = 0 move 12 to return-code goback end-if
          display "CHOICE " function trim(output-text trailing)
       end-if
       if v-kind of tt-n-complete-session-packet-v1 =
          tt-n-complete-session-packet-score-aggregate-v1-constant or
          tt-n-complete-session-packet-score-row-v1-constant
          if v-kind of tt-n-complete-session-packet-v1 =
             tt-n-complete-session-packet-score-row-v1-constant
             set address of tt-n-complete-session-packet-score-row-v1
                to v-data-score-row of tt-n-complete-session-packet-v1
             set address of tt-n-complete-atomic-double-v1
                to v-value of tt-n-complete-session-packet-score-row-v1
          else
          set address of tt-n-complete-session-packet-score-aggregate-v1
             to v-data-score-aggregate of tt-n-complete-session-packet-v1
          if v-value-len of tt-n-complete-session-packet-score-aggregate-v1 not = 1
             move 13 to return-code goback end-if
          set address of first-value
             to v-value-data of tt-n-complete-session-packet-score-aggregate-v1
          set address of tt-n-complete-atomic-double-v1 to first-value
          end-if
          display "SCORE " v-value of tt-n-complete-atomic-double-v1
       end-if
       call "thinkthen_session_result_free" using by value value-owner
          returning omitted
    end-if
    call "thinkthen_session_result_free" using by value terminal-owner
       returning omitted
    display "SESSION_PASS"
    move 0 to return-code goback.
