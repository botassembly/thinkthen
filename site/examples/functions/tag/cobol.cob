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
01 ask usage pointer.
01 evidence usage pointer.
01 no-controls usage pointer value null.
01 row-at usage pointer.
01 row-number usage binary-long.
01 written usage binary-double unsigned.
01 selected-text pic x(8192).
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
    allocate tt-n-cobol-AnswerTagResult
    allocate tt-n-cobol-AnswerStatus
    set ask to address of tt-n-cobol-RequestQuestion
    set evidence to address of tt-n-cobol-RequestInput
    call "TT_TAG" using by value
       engine ask evidence no-controls
       by reference tt-n-cobol-AnswerTagResult
       tt-n-cobol-AnswerStatus returning status-code
    if status-code not = 0 stop run returning 1 end-if
    set row-at to v-first-rows of tt-n-cobol-AnswerTagResult
    perform varying row-number from 1 by 1 until
       row-number > v-first-count of
          tt-n-cobol-AnswerTagResult
       set address of tt-n-cobol-AnswerText to row-at
       string v-text of tt-n-cobol-AnswerText
          (1:v-length of tt-n-cobol-AnswerText) " "
          into fitting-labels with pointer labels-end
       end-string
       set row-at up by length of tt-n-cobol-AnswerText
    end-perform
    call "TT_ANSWER_FREE" using by value
       v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if fitting-labels not = "praise bug billing"
       stop run returning 1
    end-if
    stop run.
