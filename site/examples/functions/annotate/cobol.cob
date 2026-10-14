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
    allocate tt-n-cobol-AnswerAnnotateResult
    allocate tt-n-cobol-AnswerStatus
    set ask to address of tt-n-cobol-RequestQuestion
    set evidence to address of tt-n-cobol-RequestInput
    call "TT_ANNOTATE" using by value
       engine ask evidence no-controls
       by reference tt-n-cobol-AnswerAnnotateResult
       tt-n-cobol-AnswerStatus returning status-code
    if status-code not = 0 stop run returning 1 end-if
    set row-at to v-rows of tt-n-cobol-AnswerAnnotateResult
    perform varying row-number from 1 by 1 until
       row-number > v-count of
          tt-n-cobol-AnswerAnnotateResult
       set address of tt-n-cobol-AnswerAnnotationField
          to row-at
       evaluate v-name-text of
          tt-n-cobol-AnswerAnnotationField
       when "steps"
          if v-boolean of
             tt-n-cobol-AnswerAnnotationField = 1
             move "Y" to steps
          end-if
       when "area"
          move v-text-text of
             tt-n-cobol-AnswerAnnotationField to app-area
       when "impact"
          compute impact rounded = v-number of
             tt-n-cobol-AnswerAnnotationField
       end-evaluate
       set row-at up by length of
          tt-n-cobol-AnswerAnnotationField
    end-perform
    call "TT_ANSWER_FREE" using by value
       v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if steps not = "Y" or app-area not = "login"
       or impact not = 1.98
       stop run returning 1
    end-if
    stop run.
