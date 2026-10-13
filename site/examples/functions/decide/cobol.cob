identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question pic x(35) value
   "Does the customer ask for a refund?".
01 broken pic x(42) value
   "Please refund my order. It arrived broken.".
01 is-refund pic x value "N".
01 engine usage pointer.
01 session-owner usage pointer.
01 packet-owner usage pointer.
01 packet-view usage pointer.
01 read-status usage binary-long unsigned.
01 status-code usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine
    allocate tt-n-cobol-RequestQuestion initialized
    allocate tt-n-cobol-RequestQuestionText initialized
    allocate tt-n-cobol-RequestQuestionTextText
    set v-data of
       tt-n-cobol-RequestQuestionTextText
       to address of question
    move length of question to v-len of
       tt-n-cobol-RequestQuestionTextText
    set v-m-text of tt-n-cobol-RequestQuestionText
       to address of
       tt-n-cobol-RequestQuestionTextText
    move tt-n-cobol-requestquestiontext-kind-k
       to v-kind of tt-n-cobol-RequestQuestion
    set v-value of tt-n-cobol-RequestQuestion
       to address of tt-n-cobol-RequestQuestionText
    allocate tt-n-cobol-RequestInput initialized
    allocate tt-n-cobol-RequestInputText initialized
    allocate tt-n-cobol-RequestInputTextText
    set v-data of tt-n-cobol-RequestInputTextText
       to address of broken
    move length of broken to v-len of
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
    call "TT_SESSION_DECIDE" using by value engine
       by reference tt-n-cobol-RequestCall
       session-owner returning status-code
    if status-code not = 0 stop run returning 1 end-if
    perform read-packet
       until read-status = tt-n-session-end-v1-k
    call "thinkthen_session_free"
       using by value session-owner returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if is-refund not = "Y" stop run returning 1 end-if
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
       tt-n-complete-session-packet-decide-row-v1-k
       perform read-decision
    end-if
    call "thinkthen_session_result_free"
       using by value packet-owner returning omitted.
read-decision.
    set address of
       tt-n-complete-session-packet-decide-row-v1
       to v-data-decide-row of
       tt-n-complete-session-packet-v1
    set address of tt-n-complete-atomic-decide-value-v1
       to v-value of
       tt-n-complete-session-packet-decide-row-v1
    if v-value-presence of
       tt-n-complete-atomic-decide-value-v1 =
       tt-n-complete-presence-value-v1-k
       set address of tt-n-complete-decide-value-v1
          to v-value-value of
          tt-n-complete-atomic-decide-value-v1
       set address of tt-n-complete-json-v1
          to v-value of tt-n-complete-decide-value-v1
       if v-data-boolean of tt-n-complete-json-v1 = 1
          move "Y" to is-refund
       end-if
    end-if.
