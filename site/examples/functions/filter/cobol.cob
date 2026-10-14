identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question pic x(20) value "Is this a complaint?".
01 reviews.
   02 filler pic x(43) value
      "Arrived a day early. Thank you!".
   02 filler pic x(43) value
      "The zipper broke the first time I used it.".
   02 filler pic x(43) value
      "Does this come in blue?".
   02 filler pic x(43) value
      "The strap snapped on day two.".
01 review-table redefines reviews.
   02 review pic x(43) occurs 4.
01 review-address usage pointer occurs 4.
01 at-review usage binary-long.
01 complaints pic x(4) value "NNNN".
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
    perform add-review varying at-review from 1 by 1
       until at-review > 4
    allocate tt-n-cobol-RequestInput initialized
    allocate tt-n-cobol-RequestInputRecords initialized
    allocate tt-n-cobol-RequestInputRecordsItems
    set v-data of
       tt-n-cobol-RequestInputRecordsItems
       to address of review-address(1)
    move 4 to v-len of
       tt-n-cobol-RequestInputRecordsItems
    set v-m-items of tt-n-cobol-RequestInputRecords
       to address of
       tt-n-cobol-RequestInputRecordsItems
    move tt-n-cobol-requestinputrecords-kind-k
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInputRecords
    allocate tt-n-cobol-RequestCall initialized
    set v-m-question of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestQuestion
    set v-m-input of tt-n-cobol-RequestCall
       to address of tt-n-cobol-RequestInput
    allocate tt-n-cobol-AnswerFilterResult
    allocate tt-n-cobol-AnswerStatus
    set ask to address of tt-n-cobol-RequestQuestion
    set evidence to address of tt-n-cobol-RequestInput
    call "TT_FILTER" using by value
       engine ask evidence no-controls
       by reference tt-n-cobol-AnswerFilterResult
       tt-n-cobol-AnswerStatus returning status-code
    if status-code not = 0 stop run returning 1 end-if
    if v-count of tt-n-cobol-AnswerFilterResult not = 2
       stop run returning 1
    end-if
    set row-at to v-rows of tt-n-cobol-AnswerFilterResult
    perform varying row-number from 1 by 1 until
       row-number > v-count of tt-n-cobol-AnswerFilterResult
       set address of tt-n-cobol-AnswerFilterOriginal
          to row-at
       move spaces to selected-text
       call "TT_VALUE_TEXT" using by value
          v-original of tt-n-cobol-AnswerFilterOriginal
          by reference selected-text written
          returning status-code
       if selected-text = review(2)
          move "Y" to complaints(2:1)
       end-if
       if selected-text = review(4)
          move "Y" to complaints(4:1)
       end-if
       set row-at up by length of
          tt-n-cobol-AnswerFilterOriginal
    end-perform
    call "TT_ANSWER_FREE" using by value
       v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if complaints not = "NYNY" stop run returning 1 end-if
    stop run.
add-review.
    allocate tt-n-cobol-RequestItem initialized
    allocate tt-n-cobol-RequestOriginal
    allocate tt-n-cobol-RequestOriginalText initialized
    allocate tt-n-cobol-RequestOriginalTextText
    set v-data of
       tt-n-cobol-RequestOriginalTextText
       to address of review(at-review)
    move function length(function trim(
       review(at-review) trailing)) to v-len of
       tt-n-cobol-RequestOriginalTextText
    set v-m-text of tt-n-cobol-RequestOriginalText
       to address of
       tt-n-cobol-RequestOriginalTextText
    move tt-n-cobol-requestoriginaltext-kind-k
       to v-kind of tt-n-cobol-RequestOriginal
    set v-value of tt-n-cobol-RequestOriginal
       to address of tt-n-cobol-RequestOriginalText
    set v-m-original of tt-n-cobol-RequestItem
       to address of tt-n-cobol-RequestOriginal
    set review-address(at-review)
       to address of tt-n-cobol-RequestItem.
