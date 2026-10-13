identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question-file pic x(11) value "labels.json".
01 message-text.
   02 filler pic x(51) value
      "Love the new dashboard, but export crashes the app,".
   02 filler pic x value x"0A".
   02 filler pic x(24) value "and I was charged twice.".
   02 filler pic x value x"0A".
01 label-at usage pointer.
01 one-label pic x(8192).
01 label-length usage binary-double unsigned.
01 labels-end usage binary-long value 1.
01 fitting-labels pic x(60) value spaces.
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
       to address of question-file
    move length of question-file to v-len of
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
       to address of message-text
    move length of message-text to v-len of
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
    call "TT_SESSION_TAG" using by value engine
       by reference tt-n-cobol-RequestCall
       session-owner returning status-code
    if status-code not = 0 stop run returning 1 end-if
    perform read-packet
       until read-status = tt-n-session-end-v1-k
    call "thinkthen_session_free"
       using by value session-owner returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if fitting-labels not = "praise bug billing"
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
       tt-n-complete-session-packet-tag-row-v1-k
       perform read-labels
    end-if
    call "thinkthen_session_result_free"
       using by value packet-owner returning omitted.
read-labels.
    set address of
       tt-n-complete-session-packet-tag-row-v1
       to v-data-tag-row of
       tt-n-complete-session-packet-v1
    set address of tt-n-complete-atomic-array-of-string-v1
       to v-value of tt-n-complete-session-packet-tag-row-v1
    set label-at to v-value-data of
       tt-n-complete-atomic-array-of-string-v1
    perform v-value-len of
       tt-n-complete-atomic-array-of-string-v1 times
       set address of tt-n-complete-utf8-v1 to label-at
       call "TT_SESSION_TEXT" using by reference
          tt-n-complete-utf8-v1 one-label label-length
          returning status-code
       string one-label(1:label-length) " "
          delimited by size into fitting-labels
          with pointer labels-end
       set label-at up by length of tt-n-complete-utf8-v1
    end-perform.
