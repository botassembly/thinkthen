identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question-file pic x(10) value "teams.json".
01 parcel pic x(36) value
   "My parcel went to the wrong address.".
01 team pic x(8192) value spaces.
01 team-length usage binary-double unsigned.
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
       to address of parcel
    move length of parcel to v-len of
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
    allocate tt-n-cobol-AnswerChooseResult
    allocate tt-n-cobol-AnswerStatus
    set ask to address of tt-n-cobol-RequestQuestion
    set evidence to address of tt-n-cobol-RequestInput
    call "TT_CHOOSE" using by value
       engine ask evidence no-controls
       by reference tt-n-cobol-AnswerChooseResult
       tt-n-cobol-AnswerStatus returning status-code
    if status-code not = 0 stop run returning 1 end-if
    if v-value-first-present of tt-n-cobol-AnswerChooseResult = 1
       move v-value-first-value-text of
          tt-n-cobol-AnswerChooseResult
          to team
    end-if
    call "TT_ANSWER_FREE" using by value
       v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if team not = "shipping" stop run returning 1 end-if
    stop run.
