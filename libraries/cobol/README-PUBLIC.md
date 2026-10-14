# ThinkThen for COBOL
ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Unpack the matching archive: `tar -xzf thinkthen-cobol-0.2.0-TARGET.tar.gz`. Replace `TARGET` with your platform. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.
```cobol
identification division. program-id. RegistryExample. environment division. input-output section. file-control. select team-file assign to "team.json" organization line sequential. data division. file section. fd team-file. 01 team-line pic x(300).
working-storage section. copy "tt-native-generated.cpy" replacing leading ==tt-n-cobol-== by ==t-==. 01 refund-question pic x(35) value "Does the customer ask for a refund?" . 01 team-path pic x(9) value "team.json" . 01 answer-row usage pointer. 01
engine usage pointer. 01 no-controls usage pointer value null. 01 status-code usage binary-long signed. 01 i usage binary-long. 01 j usage binary-long. 01 batch usage binary-long. 01 evidence-length usage binary-double unsigned. 01 evidence-table. 02
evidence pic x(80) occurs 6 times. 01 owned-rows. 02 owned-row occurs 3 times. 03 item-owner usage pointer. 03 original-owner usage pointer. 03 text-owner usage pointer. 03 scalar-owner usage pointer. 01 record-address. 02 record-pointer usage pointer
occurs 3 times. procedure division. allocate t-Settings. move low-values to t-Settings. call "TT_ENGINE_NEW" using by reference t-Settings engine returning status-code. free t-Settings. if status-code not = 0 display "Engine failed: " status-code move 1
to return-code goback end-if. allocate t-RequestQuestion. allocate t-RequestQuestionText. allocate t-RequestQuestionTextText. allocate t-RequestQuestionFile. allocate t-RequestQuestionFilePath. allocate t-RequestInput. allocate t-RequestInputRecords.
allocate t-RequestInputRecordsItems. allocate t-AnswerDecideResult. allocate t-AnswerChooseResult. allocate t-AnswerStatus. move low-values to t-RequestInputRecords. move t-requestinputrecords-kind-k to v-kind of t-RequestInput. set v-value of
t-RequestInput to address of t-RequestInputRecords. set v-data of t-RequestInputRecordsItems to address of record-address. move 3 to v-len of t-RequestInputRecordsItems. set v-m-items of t-RequestInputRecords to address of t-RequestInputRecordsItems.
move "Please refund my order. It arrived broken." to evidence(1). move "Thanks for the quick help yesterday!" to evidence(2). move "I want to send this back." to evidence(3). move "Please refund the extra fee on my invoice." to evidence(4). move
"My parcel went to the wrong address." to evidence(5). move "I cannot reset my password." to evidence(6). open output team-file. write team-line from
'{"choose":"Which team owns this?","options":{"billing":"Invoices, fees, and refunds.","shipping":"Parcels and delivery.","account":"Logins and passwords."}}' . close team-file. perform varying batch from 0 by 1 until batch > 1 perform varying j from 1
by 1 until j > 3 compute i = batch * 3 + j allocate t-RequestItem allocate t-RequestOriginal allocate t-RequestOriginalText allocate t-RequestOriginalTextText set item-owner(j) record-pointer(j) to address of t-RequestItem set original-owner(j) to
address of t-RequestOriginal set text-owner(j) to address of t-RequestOriginalText set scalar-owner(j) to address of t-RequestOriginalTextText move function length(function trim(evidence(i) trailing)) to evidence-length call "TT_TEXT_ITEM" using by
reference t-RequestItem t-RequestOriginal t-RequestOriginalText t-RequestOriginalTextText evidence(i) by value evidence-length returning status-code end-perform if batch = 0 call "TT_QUESTION_TEXT" using by reference t-RequestQuestion
t-RequestQuestionText t-RequestQuestionTextText by reference refund-question by value 35 returning status-code call "TT_DECIDE" using by value engine by reference t-RequestQuestion t-RequestInput by value no-controls by reference t-AnswerDecideResult
t-AnswerStatus returning status-code if status-code = 0 set answer-row to v-rows of t-AnswerDecideResult perform varying j from 1 by 1 until j > 3 set address of t-AnswerDecision to answer-row if v-present of t-AnswerDecision = 0 display "not sure" else
if v-value of t-AnswerDecision = 1 display "true" else display "false" end-if end-if set answer-row up by length of t-AnswerDecision end-perform end-if else call "TT_QUESTION_FILE" using by reference t-RequestQuestion t-RequestQuestionFile
t-RequestQuestionFilePath by reference team-path by value 9 returning status-code call "TT_CHOOSE" using by value engine by reference t-RequestQuestion t-RequestInput by value no-controls by reference t-AnswerChooseResult t-AnswerStatus returning
status-code if status-code = 0 set answer-row to v-rows of t-AnswerChooseResult perform varying j from 1 by 1 until j > 3 set address of t-AnswerChoice to answer-row if v-present of t-AnswerChoice = 1 display v-value-text of
t-AnswerChoice(1:v-value-length of t-AnswerChoice) else display "not sure" end-if set answer-row up by length of t-AnswerChoice end-perform end-if end-if call "TT_ANSWER_FREE" using by value v-owner of t-AnswerStatus returning omitted if status-code not
= 0 display v-message-text of t-AnswerStatus(1:v-message-length of t-AnswerStatus) move 1 to return-code goback end-if perform varying j from 1 by 1 until j > 3 free item-owner(j) original-owner(j) text-owner(j) scalar-owner(j) end-perform end-perform.
call "thinkthen_engine_free" using by value engine returning omitted. free t-RequestQuestion t-RequestQuestionText t-RequestQuestionTextText t-RequestQuestionFile t-RequestQuestionFilePath t-RequestInput t-RequestInputRecords t-RequestInputRecordsItems
t-AnswerDecideResult t-AnswerChooseResult t-AnswerStatus. move 0 to return-code. goback.
```
```text
true
false
true
billing
shipping
account
```
The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.
[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
