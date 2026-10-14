# ThinkThen for COBOL

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Unpack the matching archive: `tar -xzf thinkthen-cobol-0.2.0-TARGET.tar.gz`. Replace `TARGET` with your platform. Compile this free-format example with `cobc -free -ftext-column=4096`. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```cobol
identification division. program-id. RegistryExample.
environment division. input-output section. file-control. select team-file assign to "team.json" organization line sequential.
data division. file section. fd team-file. 01 team-line pic x(300).
working-storage section. copy "tt-native-generated.cpy".
01 engine usage pointer. 01 no-controls usage pointer value null. 01 status-code usage binary-long signed. 01 i usage binary-long. 01 evidence-length usage binary-double unsigned.
01 evidence-table. 02 evidence pic x(80) occurs 6 times. 01 record-address. 02 first-record usage pointer.
procedure division. allocate tt-n-cobol-Settings. move low-values to tt-n-cobol-Settings. call "TT_ENGINE_NEW" using by reference tt-n-cobol-Settings engine returning status-code. free tt-n-cobol-Settings. if status-code not = 0 move 1 to return-code goback end-if.
allocate tt-n-cobol-RequestQuestion tt-n-cobol-RequestQuestionText tt-n-cobol-RequestQuestionTextText tt-n-cobol-RequestQuestionFile tt-n-cobol-RequestQuestionFilePath.
allocate tt-n-cobol-RequestInput tt-n-cobol-RequestInputRecords tt-n-cobol-RequestInputRecordsItems tt-n-cobol-RequestItem tt-n-cobol-RequestOriginal tt-n-cobol-RequestOriginalText tt-n-cobol-RequestOriginalTextText.
allocate tt-n-cobol-AnswerDecideResult tt-n-cobol-AnswerChooseResult tt-n-cobol-AnswerStatus. move low-values to tt-n-cobol-RequestInputRecords tt-n-cobol-RequestItem tt-n-cobol-RequestOriginalText.
move tt-n-cobol-requestinputrecords-kind-k to v-kind of tt-n-cobol-RequestInput. set v-value of tt-n-cobol-RequestInput to address of tt-n-cobol-RequestInputRecords.
set first-record to address of tt-n-cobol-RequestItem. set v-data of tt-n-cobol-RequestInputRecordsItems to address of record-address. move 1 to v-len of tt-n-cobol-RequestInputRecordsItems. set v-m-items of tt-n-cobol-RequestInputRecords to address of tt-n-cobol-RequestInputRecordsItems.
move "Please refund my order. It arrived broken." to evidence(1). move "Thanks for the quick help yesterday!" to evidence(2). move "I want to send this back." to evidence(3). move "Please refund the extra fee on my invoice." to evidence(4). move "My parcel went to the wrong address." to evidence(5). move "I cannot reset my password." to evidence(6).
open output team-file. write team-line from '{"version":1,"choose":"Which team owns this?","options":{"billing":"Invoices, fees, and refunds.","shipping":"Parcels and delivery.","account":"Logins and passwords."}}'. close team-file.
perform varying i from 1 by 1 until i > 6
    move function length(function trim(evidence(i) trailing)) to evidence-length call "TT_TEXT_ITEM" using by reference tt-n-cobol-RequestItem tt-n-cobol-RequestOriginal tt-n-cobol-RequestOriginalText tt-n-cobol-RequestOriginalTextText evidence(i) by value evidence-length returning status-code
    if i <= 3
        call "TT_QUESTION_TEXT" using by reference tt-n-cobol-RequestQuestion tt-n-cobol-RequestQuestionText tt-n-cobol-RequestQuestionTextText by reference "Does the customer ask for a refund?" by value 35 returning status-code
        call "TT_DECIDE" using by value engine by reference tt-n-cobol-RequestQuestion tt-n-cobol-RequestInput by value no-controls by reference tt-n-cobol-AnswerDecideResult tt-n-cobol-AnswerStatus returning status-code
        if status-code = 0 and v-first-present of tt-n-cobol-AnswerDecideResult = 1 display v-first-value of tt-n-cobol-AnswerDecideResult end-if
    else
        call "TT_QUESTION_FILE" using by reference tt-n-cobol-RequestQuestion tt-n-cobol-RequestQuestionFile tt-n-cobol-RequestQuestionFilePath by reference "team.json" by value 9 returning status-code
        call "TT_CHOOSE" using by value engine by reference tt-n-cobol-RequestQuestion tt-n-cobol-RequestInput by value no-controls by reference tt-n-cobol-AnswerChooseResult tt-n-cobol-AnswerStatus returning status-code
        if status-code = 0 and v-first-present of tt-n-cobol-AnswerChooseResult = 1 display v-first-value-text of tt-n-cobol-AnswerChooseResult(1:v-first-value-length of tt-n-cobol-AnswerChooseResult) end-if
    end-if
    call "TT_ANSWER_FREE" using by value v-owner of tt-n-cobol-AnswerStatus returning omitted
    if status-code not = 0 move 1 to return-code goback end-if
end-perform. call "thinkthen_engine_free" using by value engine returning omitted. free tt-n-cobol-RequestQuestion tt-n-cobol-RequestQuestionText tt-n-cobol-RequestQuestionTextText tt-n-cobol-RequestQuestionFile tt-n-cobol-RequestQuestionFilePath tt-n-cobol-RequestInput tt-n-cobol-RequestInputRecords tt-n-cobol-RequestInputRecordsItems tt-n-cobol-RequestItem tt-n-cobol-RequestOriginal tt-n-cobol-RequestOriginalText tt-n-cobol-RequestOriginalTextText tt-n-cobol-AnswerDecideResult tt-n-cobol-AnswerChooseResult tt-n-cobol-AnswerStatus. move 0 to return-code. goback.
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.
[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
