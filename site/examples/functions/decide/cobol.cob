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
01 ask usage pointer.
01 evidence usage pointer.
01 no-controls usage pointer value null.
01 row-at usage pointer.
01 row-number usage binary-long.
01 written usage binary-double unsigned.
01 selected-text pic x(8192).
01 boolean-answer usage binary-long unsigned.
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
    allocate tt-n-cobol-AnswerDecideResult
    allocate tt-n-cobol-AnswerStatus
    set ask to address of tt-n-cobol-RequestQuestion
    set evidence to address of tt-n-cobol-RequestInput
    call "TT_DECIDE" using by value
       engine ask evidence no-controls
       by reference tt-n-cobol-AnswerDecideResult
       tt-n-cobol-AnswerStatus returning status-code
    if status-code not = 0 stop run returning 1 end-if
    if v-value-first-present of
    tt-n-cobol-AnswerDecideResult = 1
       call "TT_VALUE_BOOLEAN" using
          by value v-value-first-value of
          tt-n-cobol-AnswerDecideResult
          by reference boolean-answer returning status-code
       if status-code not = 0 stop run returning 3 end-if
       if boolean-answer = 1 move "Y" to is-refund end-if
    end-if
    call "TT_ANSWER_FREE" using by value
       v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if is-refund not = "Y" stop run returning 1 end-if
    stop run.
