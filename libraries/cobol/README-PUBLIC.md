# ThinkThen for COBOL

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Install the matching archive: `tar -xzf thinkthen-cobol-0.2.0-TARGET.tar.gz`. Replace `TARGET` with your platform.
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```cobol
identification division. program-id. RegistryExample.
data division. working-storage section.
copy "tt-native-generated.cpy".
01 engine usage pointer. 01 session-owner usage pointer. 01 packet-owner usage pointer. 01 packet-view usage pointer.
01 status-code binary-long. 01 read-status binary-long unsigned. 01 turn binary-long. 01 request-text pic x(1000).
01 output-text pic x(8192). 01 written binary-double unsigned. 01 answers binary-long value 0. 01 request-length binary-double unsigned.
procedure division. move spaces to request-text call "thinkthen_engine_new" returning engine if engine = null move 1 to return-code goback end-if perform varying turn from 1 by 1 until turn > 2
 if turn = 1 string '{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Does the customer ask for a refund?"},"input":{"kind":"records","items":[{'
    '"original":{"kind":"text","text":"Please refund my order. It arrived broken."}},{"original":{"kind":"text","text":"Thanks for the quick help yesterday!"}},{"original":{"kind":'
    '"text","text":"I want to send this back."}}]}}}' into request-text end-string
 else string '{"schema":"thinkthen.request/1","call":{"function":"choose","question":{"kind":"definition","value":{"choose":"Which team owns this?","options":{"billing":'
    '"Invoices, fees, and refunds.","shipping":"Parcels and delivery.","account":"Logins and passwords."}}},"input":{"kind":"records","items":[{"original":{"kind":"text","text":'
    '"Please refund the extra fee on my invoice."}},{"original":{"kind":"text","text":"My parcel went to the wrong address."}},{"original":{"kind":"text","text":' '"I cannot reset my password."}}]}}}' into request-text end-string end-if
 move function length(function trim(request-text trailing)) to request-length
 call "thinkthen_session_new_with_surface" using by value engine by reference request-text by value request-length by reference "cobol" by value 5 by reference session-owner returning status-code
 if status-code not = 0 move 1 to return-code goback end-if perform until read-status = tt-n-session-end-v1-k call "TT_SESSION_NEXT" using by value session-owner by reference read-status packet-owner returning status-code
  if status-code not = 0 move 1 to return-code goback end-if if read-status = tt-n-session-result-v1-k call "thinkthen_session_result_view" using by value packet-owner by reference packet-view returning status-code
   set address of tt-n-complete-session-packet-v1 to packet-view if v-kind of tt-n-complete-session-packet-v1 = tt-n-complete-session-packet-terminal-v1-k
    set address of tt-n-complete-session-packet-terminal-v1 to v-data-terminal of tt-n-complete-session-packet-v1
    if v-failure-presence of tt-n-complete-session-packet-terminal-v1 = tt-n-complete-presence-value-v1-k move 1 to return-code goback end-if end-if if v-kind of tt-n-complete-session-packet-v1 = tt-n-complete-session-packet-decide-row-v1-k
    set address of tt-n-complete-session-packet-decide-row-v1 to v-data-decide-row of tt-n-complete-session-packet-v1 set address of tt-n-complete-atomic-decide-value-v1 to v-value of tt-n-complete-session-packet-decide-row-v1
    if v-value-presence of tt-n-complete-atomic-decide-value-v1 = tt-n-complete-presence-value-v1-k set address of tt-n-complete-decide-value-v1 to v-value-value of tt-n-complete-atomic-decide-value-v1
     set address of tt-n-complete-json-v1 to v-value of tt-n-complete-decide-value-v1 add 1 to answers display v-data-boolean of tt-n-complete-json-v1 else display "not sure" end-if end-if
   if v-kind of tt-n-complete-session-packet-v1 = tt-n-complete-session-packet-choose-row-v1-k set address of tt-n-complete-session-packet-choose-row-v1 to v-data-choose-row of tt-n-complete-session-packet-v1
    set address of tt-n-complete-atomic-nullable-string-v1 to v-value of tt-n-complete-session-packet-choose-row-v1 if v-value-presence of tt-n-complete-atomic-nullable-string-v1 = tt-n-complete-presence-value-v1-k
     call "TT_SESSION_TEXT" using by reference v-value-value of tt-n-complete-atomic-nullable-string-v1 output-text written returning status-code add 1 to answers display function trim(output-text trailing) else display "not sure" end-if end-if
   call "thinkthen_session_result_free" using by value packet-owner returning omitted end-if end-perform call "thinkthen_session_free" using by value session-owner returning omitted move 0 to read-status move spaces to request-text end-perform
call "thinkthen_engine_free" using by value engine returning omitted if answers = 6 move 0 to return-code else move 1 to return-code end-if goback.
```
The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.
[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
