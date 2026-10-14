identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 names-file pic x(10) value "kinds.json".
01 text-in pic x(51) value
   "Sam Patel says his Kindle arrived broken "
   & "in Denver.".
01 row-address usage pointer based.
01 entity-at usage pointer.
01 entity-address usage pointer based.
01 one-text pic x(8192).
01 text-length usage binary-double unsigned.
01 one-kind pic x(8192).
01 kind-length usage binary-double unsigned.
01 names-end usage binary-long value 1.
01 entity-names pic x(80) value spaces.
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
       to address of names-file
    move length of names-file to v-len of
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
       to address of text-in
    move length of text-in to v-len of
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
    allocate tt-n-cobol-AnswerRecognizeResult
    allocate tt-n-cobol-AnswerStatus
    set ask to address of tt-n-cobol-RequestQuestion
    set evidence to address of tt-n-cobol-RequestInput
    call "TT_RECOGNIZE" using by value
       engine ask evidence no-controls
       by reference tt-n-cobol-AnswerRecognizeResult
       tt-n-cobol-AnswerStatus returning status-code
    if status-code not = 0 stop run returning 1 end-if
    set row-at to v-value-rows of
    tt-n-cobol-AnswerRecognizeResult
    perform varying row-number from 1 by 1 until
       row-number > v-value-count of
          tt-n-cobol-AnswerRecognizeResult
       set address of tt-n-cobol-AnswerRecognition
          to row-at
       string function trim(v-text-text of
          tt-n-cobol-AnswerRecognition trailing)
          "/" function trim(v-kind-text of
          tt-n-cobol-AnswerRecognition trailing) " "
          into entity-names with pointer names-end
       end-string
       set row-at up by length of
          tt-n-cobol-AnswerRecognition
    end-perform
    call "TT_ANSWER_FREE" using by value
       v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if entity-names not = "Sam Patel/person "
       & "Kindle/product Denver/place"
       stop run returning 1
    end-if
    stop run.
