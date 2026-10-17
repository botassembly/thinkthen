identification division.
program-id. BlockingCaller.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 engine usage pointer.
01 session-owner usage pointer.
01 packet-owner usage pointer.
01 terminal-owner usage pointer value null.
01 packet-view usage pointer.
01 value-owner usage pointer value null.
01 record-address.
 02 first-record usage pointer.
 02 second-record usage pointer.
 02 third-record usage pointer.
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
01 decide-question usage pointer.
01 choose-question usage pointer.
01 decide-arm usage pointer.
01 decide-path usage pointer.
01 choose-text pic x(8192).
01 input-rows usage pointer.
01 boolean-answer usage binary-long unsigned.
01 no-controls usage pointer value null.
procedure division.
    accept question-text from environment "TT_SESSION_QUESTION_FILE"
    accept evidence-text from environment "TT_SESSION_EVIDENCE"
    accept expected-failure from environment "TT_SESSION_EXPECT_FAILURE"
    move function length(function trim(evidence-text trailing))
       to evidence-length
    allocate tt-n-cobol-Settings
    move low-values to tt-n-cobol-Settings
    call "TT_ENGINE_NEW" using by reference tt-n-cobol-Settings engine
       returning status-code
    free tt-n-cobol-Settings
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
    set first-record second-record third-record
       to address of tt-n-cobol-RequestItem
    set v-data of tt-n-cobol-RequestInputRecordsItems
       to address of record-address
    move 3 to v-len of tt-n-cobol-RequestInputRecordsItems
    set v-m-items of tt-n-cobol-RequestInputRecords
       to address of tt-n-cobol-RequestInputRecordsItems
    set v-m-input of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestInput
    set v-m-question of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestQuestion
    set decide-question to address of tt-n-cobol-RequestQuestion
    set decide-arm to address of tt-n-cobol-RequestQuestionFile
    set decide-path to address of tt-n-cobol-RequestQuestionFilePath
    set input-rows to address of tt-n-cobol-RequestInput
    allocate tt-n-cobol-RequestQuestion
    allocate tt-n-cobol-RequestQuestionFile
    allocate tt-n-cobol-RequestQuestionFilePath
    accept choose-text from environment "TT_PUBLIC_CHOOSE_FILE"
    move function length(function trim(choose-text trailing)) to question-length
    call "TT_QUESTION_FILE" using
       by reference tt-n-cobol-RequestQuestion tt-n-cobol-RequestQuestionFile
          tt-n-cobol-RequestQuestionFilePath choose-text
       by value question-length returning status-code
    set choose-question to address of tt-n-cobol-RequestQuestion
    allocate tt-n-cobol-AnswerDecideResult
    allocate tt-n-cobol-AnswerChooseResult
    allocate tt-n-cobol-AnswerStatus
    call "TT_DECIDE" using by value engine decide-question input-rows no-controls
       by reference tt-n-cobol-AnswerDecideResult tt-n-cobol-AnswerStatus
       returning status-code
    if status-code = 0 and v-value-count of tt-n-cobol-AnswerDecideResult not = 3
       move 2 to return-code goback
    end-if
    if status-code = 0 and v-value-first-present of tt-n-cobol-AnswerDecideResult = 1
       call "TT_VALUE_BOOLEAN" using
          by value v-value-first-value of tt-n-cobol-AnswerDecideResult
          by reference boolean-answer returning status-code
       if status-code not = 0 move 3 to return-code goback end-if
       display boolean-answer
    end-if
    call "TT_ANSWER_FREE" using
       by value v-owner of tt-n-cobol-AnswerStatus returning omitted

    call "TT_CHOOSE" using by value engine choose-question input-rows no-controls
       by reference tt-n-cobol-AnswerChooseResult tt-n-cobol-AnswerStatus
       returning status-code
    if status-code = 0 and v-value-count of tt-n-cobol-AnswerChooseResult not = 3
       move 2 to return-code goback
    end-if
    if status-code = 0 and v-value-first-present of tt-n-cobol-AnswerChooseResult = 1
       display v-value-first-value-text of tt-n-cobol-AnswerChooseResult
          (1:v-value-first-value-length of tt-n-cobol-AnswerChooseResult)
    end-if
    call "TT_ANSWER_FREE" using
       by value v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free" using by value engine returning omitted
    free tt-n-cobol-AnswerDecideResult tt-n-cobol-AnswerChooseResult
       tt-n-cobol-AnswerStatus
       tt-n-cobol-RequestQuestion tt-n-cobol-RequestQuestionFile
       tt-n-cobol-RequestQuestionFilePath
    set address of tt-n-cobol-RequestQuestion to decide-question
    set address of tt-n-cobol-RequestQuestionFile to decide-arm
    set address of tt-n-cobol-RequestQuestionFilePath to decide-path
    free tt-n-cobol-RequestCall tt-n-cobol-RequestQuestion
       tt-n-cobol-RequestQuestionFile tt-n-cobol-RequestQuestionFilePath
       tt-n-cobol-RequestInput tt-n-cobol-RequestInputRecords
       tt-n-cobol-RequestInputRecordsItems tt-n-cobol-RequestItem
       tt-n-cobol-RequestOriginal tt-n-cobol-RequestOriginalText
       tt-n-cobol-RequestOriginalTextText
    move 0 to return-code goback.
