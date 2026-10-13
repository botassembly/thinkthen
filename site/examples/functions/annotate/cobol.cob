identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 form pic x(9) value "form.json".
01 bug-report pic x(36) value
   "Steps: click Log in. Nobody gets in.".
01 field-at usage pointer.
01 field-name pic x(8192).
01 name-length usage binary-double unsigned.
01 area-length usage binary-double unsigned.
01 steps pic x value "N".
01 app-area pic x(8192) value spaces.
01 impact pic 9v99 value 0.
01 engine usage pointer.
01 session-owner usage pointer.
01 packet-owner usage pointer.
01 packet-view usage pointer.
01 read-status usage binary-long unsigned.
01 status-code usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine
    allocate tt-n-cobol-RequestQuestion initialized
    allocate tt-n-cobol-RequestQuestionFile initialized
    allocate tt-n-cobol-RequestQuestionFilePath
    set v-data of
       tt-n-cobol-RequestQuestionFilePath
       to address of form
    move length of form to v-len of
       tt-n-cobol-RequestQuestionFilePath
    set v-m-path of tt-n-cobol-RequestQuestionFile
       to address of
       tt-n-cobol-RequestQuestionFilePath
    move tt-n-cobol-requestquestionfile-kind-k
       to v-kind of tt-n-cobol-RequestQuestion
    set v-value of tt-n-cobol-RequestQuestion
       to address of tt-n-cobol-RequestQuestionFile
    allocate tt-n-cobol-RequestInput initialized
    allocate tt-n-cobol-RequestInputText initialized
    allocate tt-n-cobol-RequestInputTextText
    set v-data of tt-n-cobol-RequestInputTextText
       to address of bug-report
    move length of bug-report to v-len of
       tt-n-cobol-RequestInputTextText
    set v-m-text of tt-n-cobol-RequestInputText
       to address of
       tt-n-cobol-RequestInputTextText
    move tt-n-cobol-requestinputtext-kind-k
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInputText
    allocate tt-n-cobol-RequestCall initialized
    set v-m-question of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestQuestion
    set v-m-input of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestInput
    call "TT_SESSION_ANNOTATE" using by value engine
       by reference tt-n-cobol-RequestCall
       session-owner returning status-code
    if status-code not = 0 stop run returning 1 end-if
    perform read-packet
       until read-status = tt-n-session-end-v1-k
    call "thinkthen_session_free"
       using by value session-owner returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if steps not = "Y" or app-area not = "login"
       or impact not = 1.98
       stop run returning 1
    end-if
    stop run.
read-packet.
    call "TT_SESSION_NEXT" using by value session-owner
       by reference read-status packet-owner
       returning status-code
    if read-status not = tt-n-session-result-v1-k
       exit paragraph
    end-if
    call "thinkthen_session_result_view"
       using by value packet-owner by reference packet-view
       returning status-code
    set address of tt-n-complete-session-packet-v1
       to packet-view
    if v-kind of tt-n-complete-session-packet-v1 =
       tt-n-complete-session-packet-annotate-row-v1-k
       perform read-triage
    end-if
    call "thinkthen_session_result_free"
       using by value packet-owner returning omitted.
read-triage.
    set address of
       tt-n-complete-session-packet-annotate-row-v1
       to v-data-annotate-row of
       tt-n-complete-session-packet-v1
    set address of tt-n-complete-annotation-v1
       to v-value of
       tt-n-complete-session-packet-annotate-row-v1
    set address of tt-n-complete-annotated-row-v1
       to v-value of tt-n-complete-annotation-v1
    set field-at to v-value-data of
       tt-n-complete-annotated-row-v1
    perform read-field v-value-len of
       tt-n-complete-annotated-row-v1 times.
read-field.
    set address of
       tt-n-complete-annotated-field-map-entry-v1
       to field-at
    call "TT_SESSION_TEXT" using by reference
       v-name of tt-n-complete-annotated-field-map-entry-v1
       field-name name-length returning status-code
    set address of tt-n-complete-annotated-field-v1
       to v-value of
       tt-n-complete-annotated-field-map-entry-v1
    evaluate field-name
       when "steps"
          if v-data-boolean of
             tt-n-complete-annotated-field-v1 = 1
             move "Y" to steps
          end-if
       when "area"
          call "TT_SESSION_TEXT" using by reference
             v-data-string of
             tt-n-complete-annotated-field-v1
             app-area area-length returning status-code
       when "impact"
          compute impact rounded = v-data-number of
             tt-n-complete-annotated-field-v1
    end-evaluate
    set field-at up by length of
       tt-n-complete-annotated-field-map-entry-v1.
