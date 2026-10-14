identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question pic x(15) value "Is this urgent?".
01 inbox.
   02 filler pic x(56) value
      "Newsletter: our autumn catalog is here. "
      & "No reply needed.".
   02 filler pic x(56) value
      "Our checkout page is down and customers cannot pay".
   02 filler pic x(56) value
      "Reminder: your invoice is due in 30 days".
   02 filler pic x(56) value
      "Please send the signed quote by 5 pm today".
01 inbox-table redefines inbox.
   02 inbox-message pic x(56) occurs 4.
01 message-address usage pointer occurs 4.
01 at-message usage binary-long.
01 place-at usage pointer.
01 place-address usage pointer based.
01 place usage binary-long.
01 message-number pic 9.
01 by-urgency pic x(4) value spaces.
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
    perform add-message varying at-message from 1 by 1
       until at-message > 4
    allocate tt-n-cobol-RequestInput initialized
    allocate tt-n-cobol-RequestInputRecords initialized
    allocate tt-n-cobol-RequestInputRecordsItems
    set v-data of
       tt-n-cobol-RequestInputRecordsItems
       to address of message-address(1)
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
    allocate tt-n-cobol-AnswerRankResult
    allocate tt-n-cobol-AnswerStatus
    set ask to address of tt-n-cobol-RequestQuestion
    set evidence to address of tt-n-cobol-RequestInput
    call "TT_RANK" using by value
       engine ask evidence no-controls
       by reference tt-n-cobol-AnswerRankResult
       tt-n-cobol-AnswerStatus returning status-code
    if status-code not = 0 stop run returning 1 end-if
    set row-at to v-rows of tt-n-cobol-AnswerRankResult
    perform varying row-number from 1 by 1 until
       row-number > v-count of tt-n-cobol-AnswerRankResult
       set address of tt-n-cobol-AnswerRankIndex to row-at
       compute message-number = 1 +
          v-value of tt-n-cobol-AnswerRankIndex
       move message-number to by-urgency(row-number:1)
       set row-at up by length of tt-n-cobol-AnswerRankIndex
    end-perform
    call "TT_ANSWER_FREE" using by value
       v-owner of tt-n-cobol-AnswerStatus returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if by-urgency not = "2431" stop run returning 1 end-if
    stop run.
add-message.
    allocate tt-n-cobol-RequestItem initialized
    allocate tt-n-cobol-RequestOriginal
    allocate tt-n-cobol-RequestOriginalText initialized
    allocate tt-n-cobol-RequestOriginalTextText
    set v-data of
       tt-n-cobol-RequestOriginalTextText
       to address of inbox-message(at-message)
    move function length(function trim(
       inbox-message(at-message) trailing)) to v-len of
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
    set message-address(at-message)
       to address of tt-n-cobol-RequestItem.
