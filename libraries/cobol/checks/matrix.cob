       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-matrix.
environment division.
configuration section.
repository. function all intrinsic.
data division.
working-storage section.
01 dyn-free-engine pic x(24) value "thinkthen_engine_free".
01 dyn-cancel pic x(20) value "thinkthen_cancel".
01 dyn-free-token pic x(30) value "thinkthen_cancel_token_free".
01 dyn-free-string pic x(24) value "thinkthen_free_string".
01 dyn-strlen pic x(16) value "strlen".
01 dyn-memcpy pic x(16) value "memcpy".
01 engine usage pointer.
01 second-engine usage pointer.
01 token usage pointer.
01 no-token usage pointer.
01 json-pointer usage pointer.
01 original-engine usage pointer.
01 error-message-ptr usage pointer.
01 saved-json pic x(8192).
01 saved-error pic x(512).
01 question-text pic x(256).
01 q-len usage binary-double unsigned.
01 text-input pic x(256).
01 text-len usage binary-double unsigned.
01 request-text pic x(2048).
01 check-text pic x(2048).
01 check-length usage binary-double unsigned.
01 json-data pic x(8192).
01 json-length usage binary-double unsigned.
01 result-code usage binary-long signed.
01 expected-code usage binary-long signed.
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
01 answer-row.
   02 outcome usage binary-long signed.
   02 alignment-pad usage binary-long unsigned.
   02 probability usage float-long.
01 row-array.
   02 answer-entry occurs 3 times.
      03 row-outcome usage binary-long signed.
      03 row-padding usage binary-long unsigned.
      03 row-probability usage float-long.
01 p-array.
   02 p usage pointer occurs 3 times.
01 len-array.
   02 n usage binary-double unsigned occurs 3 times.
01 text-a pic x(8) value z"first".
01 text-b pic x(8) value z"second".
01 text-c pic x(8) value z"third".
01 count-value usage binary-double unsigned.
01 deadline-ms usage binary-double signed.
01 no-deadline usage binary-double signed value -1.
01 one usage binary-long unsigned.
01 i usage binary-long unsigned.
01 operation-name pic x(60).
01 spec-json pic x(512).
01 record-one pic x(48) value z'{"name":"Third","kind":"alert"}'.
01 record-two pic x(48) value z'{"name":"Fourth","kind":"alert"}'.
01 barrier-folder pic x(1024).
01 marker pic x(1200).
01 marker-fd usage binary-long signed.
01 flag usage binary-long signed.
procedure division.
    move 0 to return-code
    call "thinkthen_engine_new" returning engine
    if engine = null
       display "engine_new failed" move 1 to return-code goback
    end-if
    move z"Is it?" to question-text
    move 6 to q-len
    move "yes" to text-input move 3 to text-len
    move "typed-yes" to operation-name perform typed-case
    if outcome not = 1 or probability not = 0.9 perform fail-now end-if
    move "no" to text-input move 2 to text-len
    move "typed-no" to operation-name perform typed-case
    if outcome not = 0 or probability not = 0.1 perform fail-now end-if
    move z'{"decide":"Is it?","threshold":"0.4:0.8"}' to question-text
    compute q-len = function length(function trim(question-text trailing)) - 1
    move "unsure" to text-input move 6 to text-len
    move "typed-unsure" to operation-name perform typed-case
    if outcome not = 2 or probability not = 0.5 perform fail-now end-if
    move z"Is it?" to question-text move 6 to q-len
    move z"café" to text-input move 5 to text-len
    move "typed-utf8" to operation-name perform typed-case
    if outcome not = 1 perform fail-now end-if
    move spaces to text-input
    move "a" to text-input(1:1)
    move x"00" to text-input(2:1)
    move "b" to text-input(3:1)
    move 3 to text-len
    move "typed-counted-nul" to operation-name perform typed-case
    if outcome not = 1 perform fail-now end-if
    move "x" to text-input move x"ff" to text-input(2:1)
    move "y" to text-input(3:1) move 3 to text-len
    move "invalid-utf8" to operation-name
    perform expect-usage-typed
    move "Is" to question-text move x"00" to question-text(3:1)
    move "bad" to question-text(4:3) move 6 to q-len
    move 111 to outcome
    move "interior-nul-guard" to operation-name
    call "TT-DECIDE" using engine question-text q-len text-input text-len
       answer-row failure-row
    if failure-code not = 1 or outcome not = 111
       perform fail-now
    end-if
    move z"Is it?" to question-text move 6 to q-len
    set p(1) to address of text-a
    set p(2) to address of text-b
    set p(3) to address of text-c
    move 5 to n(1) move 6 to n(2) move 5 to n(3)
    move question-text to check-text
    move q-len to check-length
    move "typed-bulk" to operation-name
    perform guard-cstring
    move 3 to count-value
    call "thinkthen_decide_many" using by value engine
      by reference question-text p-array len-array by value size is 8 count-value
      by reference row-array returning result-code
    perform require-ok
    if row-probability(1) not = 0.9 or row-probability(2) not = 0.1
       or row-probability(3) not = 0.6 perform fail-now end-if
    move 0 to count-value
    move "empty-bulk" to operation-name
    call "thinkthen_decide_many_opts" using by value engine
      by reference question-text p-array len-array by value size is 8 count-value
      by value size is 8 no-deadline no-token by reference row-array returning result-code
    perform require-ok
    move 3 to count-value
    move 0 to deadline-ms
    move "zero-deadline-bulk" to operation-name
    call "thinkthen_decide_many_opts" using by value engine
      by reference question-text p-array len-array by value size is 8 count-value
      by value size is 8 deadline-ms no-token by reference row-array returning result-code
    move 3 to expected-code perform require-error
    move 0 to deadline-ms move 111 to outcome
    move z"spent-scalar" to text-input move 12 to text-len
    move "zero-deadline-scalar" to operation-name
    call "thinkthen_decide_opts" using by value engine
      by reference question-text text-input by value size is 8 text-len deadline-ms no-token
      by reference answer-row returning result-code
    move 3 to expected-code perform require-error
    if outcome not = 111 perform fail-now end-if
    call "thinkthen_cancel_token_new" returning token
    if token = null move "new-token" to operation-name perform fail-now end-if
    call dyn-cancel using by value token
    call dyn-cancel using by value token
    move "pre-fired" to operation-name
    call "thinkthen_decide_opts" using by value engine
      by reference question-text text-input by value size is 8 text-len -1 token
      by reference answer-row returning result-code
    move 5 to expected-code perform require-error
    if outcome not = 111 perform fail-now end-if
    call dyn-free-token using by value token
    move "recovery" to text-input move 8 to text-len
    move "fresh-token-recovery" to operation-name
    call "thinkthen_cancel_token_new" returning token
    call "thinkthen_decide_opts" using by value engine
      by reference question-text text-input by value size is 8 text-len -1 token
      by reference answer-row returning result-code
    perform require-ok
    if outcome not = 1 perform fail-now end-if
    call dyn-free-token using by value token
    move "hold-deadline" to text-input move 13 to text-len
    move 25 to deadline-ms move 111 to outcome
    move "held-deadline" to operation-name
    call "thinkthen_decide_opts" using by value engine
      by reference question-text text-input by value size is 8 text-len deadline-ms no-token
      by reference answer-row returning result-code
    move 3 to expected-code perform require-error
    if outcome not = 111 perform fail-now end-if
    move z'{"decide":"Is it?","evidence":"json-decide","details":true}' to request-text
    move "json-decide" to operation-name perform json-case
    if function substitute(json-data, '"schema"', ' ') = json-data
       perform fail-now end-if
    move z'{"choose":"Which team?","options":["first","second"],"evidence":"choose"}' to request-text
    move "choose" to operation-name perform json-case
    if json-data(1:25) not = '{"value":"first","facts":'
       perform fail-now end-if
    move z'{"choose":"Which team?","options":{"first":{"what":"First team","not_for":"Other teams","examples":["one"]},"second":"Second team"},"evidence":"choose-map"}' to request-text
    move "choose-map" to operation-name perform json-case
    if json-data(1:25) not = '{"value":"first","facts":'
       perform fail-now end-if
    move z'{"tag":"Which labels?","labels":["first","second"],"evidence":"tag"}' to request-text
    move "tag" to operation-name perform json-case
    move z'{"score":"What level?","levels":["Low.","High."],"evidence":"score"}' to request-text
    move "score" to operation-name perform json-case
    move z'{"filter":"Is it?","records":["filter-one","filter-two"]}' to request-text
    move "filter" to operation-name perform json-case
    move z'{"rank":"Is it?","records":["rank-one","rank-two"]}' to request-text
    move "rank" to operation-name perform json-case
    move z'{"find":"Which line?","units":["find-one","find-two"]}' to request-text
    move "find" to operation-name perform json-case
    move z'{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?"}}},"records":["annotate-one"]}' to request-text
    move "annotate" to operation-name perform json-case
    move z'{"recognize":{"kinds":{"person":"A person name."}},"version":1,"evidence":"Maria Chen"}' to request-text
    move "recognize" to operation-name perform json-case
    if function substitute(json-data, '"entities"', ' ') = json-data
       perform fail-now end-if
    move z'{"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]},"version":1,"records":[{"name":"First","kind":"alert"},{"name":"Second","kind":"alert"}]}' to request-text
    move "relate" to operation-name perform json-case
    move z'{"usage":true}' to request-text
    move "usage" to operation-name perform json-case
    if function substitute(json-data, '"requests_sent"', ' ') = json-data
       perform fail-now end-if
    move z'{"version":1,"recognize":{"kinds":{"person":"A person name."}}}' to spec-json
    move z"John Smith" to text-input move 10 to text-len
    move "typed-recognize" to operation-name
    move spec-json to check-text
    compute check-length = function length(function trim(spec-json trailing)) - 1
    perform guard-cstring
    call "thinkthen_recognize" using by value engine
       by reference spec-json text-input by value size is 8 text-len
       by reference json-pointer json-length returning result-code
    perform require-ok perform copy-free-json
    if function substitute(json-data, '"entities"', ' ') = json-data
       perform fail-now end-if
    move "typed-recognize-opts-spent" to operation-name
    move 0 to deadline-ms
    call "thinkthen_recognize_opts" using by value engine
       by reference spec-json text-input by value size is 8 text-len deadline-ms no-token
       by reference json-pointer json-length returning result-code
    move 3 to expected-code perform require-error
    move z'{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}' to spec-json
    set p(1) to address of record-one
    set p(2) to address of record-two
    compute n(1) = function length(function trim(record-one trailing)) - 1
    compute n(2) = function length(function trim(record-two trailing)) - 1
    move 2 to count-value
    move "typed-relate" to operation-name
    move spec-json to check-text
    compute check-length = function length(function trim(spec-json trailing)) - 1
    perform guard-cstring
    call "thinkthen_relate" using by value engine
      by reference spec-json p-array len-array by value size is 8 count-value
      by reference json-pointer json-length returning result-code
    perform require-ok perform copy-free-json
    if function substitute(json-data, '"edges"', ' ') = json-data
       perform fail-now end-if
    move "typed-relate-opts-spent" to operation-name
    move 0 to deadline-ms
    call "thinkthen_relate_opts" using by value engine
      by reference spec-json p-array len-array by value size is 8 count-value deadline-ms no-token
      by reference json-pointer json-length returning result-code
    move 3 to expected-code perform require-error
    move z'{"usage":true}' to request-text
    move "json-call-opts" to operation-name
    move request-text to check-text
    compute check-length = function length(function trim(request-text trailing)) - 1
    perform guard-cstring
    call "thinkthen_call_opts" using by value engine
       by reference request-text by value size is 8 no-deadline no-token returning json-pointer
    if json-pointer = null perform fail-now end-if
    perform copy-free-json
    move z'{"decide":"Is it?","evidence":"spent-json"}' to request-text
    move "json-opts-spent" to operation-name
    move request-text to check-text
    compute check-length = function length(function trim(request-text trailing)) - 1
    perform guard-cstring
    move 0 to deadline-ms
    call "thinkthen_call_opts" using by value engine
       by reference request-text by value size is 8 deadline-ms no-token returning json-pointer
    if json-pointer not = null perform fail-now end-if
    move 3 to expected-code perform require-error-metadata
    move z'{"decide":"Is it?","evidence":"status-401"}' to request-text
    move "backend-failure" to operation-name
    call "thinkthen_call" using by value engine
       by reference request-text returning json-pointer
    if json-pointer not = null perform fail-now end-if
    move 2 to expected-code perform require-error-metadata
    move failure-message to saved-error
    call "thinkthen_engine_new" returning second-engine
    if second-engine = null move "second-engine" to operation-name perform fail-now end-if
    move z'{"decide":"Is it?","evidence":"failure-two"}' to request-text
    move "second-engine-failure" to operation-name
    call "thinkthen_call" using by value second-engine
       by reference request-text returning json-pointer
    if json-pointer not = null perform fail-now end-if
    move engine to original-engine
    move second-engine to engine
    move 2 to expected-code perform require-error-metadata
    move original-engine to engine
    call dyn-free-engine using by value second-engine
    move 2 to expected-code perform require-error-metadata
    if saved-error not = failure-message
       move "error-ownership" to operation-name perform fail-now end-if
    display "PASS error-ownership"
    set second-engine to null
    set original-engine to null

    move "all-cobol-cases-pass" to operation-name
    display function trim(operation-name)
    call dyn-free-engine using by value engine
    move 0 to return-code
    goback.

typed-case.
    call "TT-DECIDE" using engine question-text q-len text-input text-len
       answer-row failure-row
    if failure-code not = 0 perform fail-now end-if.
expect-usage-typed.
    call "TT-DECIDE" using engine question-text q-len text-input text-len
       answer-row failure-row
    if failure-code not = 1 or failure-retryable not = 0
       perform fail-now end-if.
json-case.
    move request-text to check-text
    compute check-length = function length(function trim(request-text trailing)) - 1
    perform guard-cstring
    call "thinkthen_call" using by value engine
       by reference request-text returning json-pointer
    if json-pointer = null
       move 0 to result-code
       move 2 to expected-code perform require-error-metadata
       perform fail-now
    end-if
    perform copy-free-json.
guard-cstring.
    if check-length > 2047 perform fail-now end-if
    perform varying i from 1 by 1 until i > check-length
       if check-text(i:1) = x"00" perform fail-now end-if
    end-perform.
copy-free-json.
    if json-pointer = null perform fail-now end-if
    move spaces to json-data
    call dyn-strlen using by value json-pointer returning json-length
    if json-length > 8192 or json-length = 0 perform fail-now end-if
    call dyn-memcpy using by reference json-data
       by value json-pointer json-length
    call dyn-free-string using by value json-pointer
    set json-pointer to null
    display "PASS " function trim(operation-name) " bytes=" json-length
    display "JSON " function trim(operation-name) " " json-data(1:json-length).
require-ok.
    if result-code not = 0 perform fail-now end-if
    display "PASS " function trim(operation-name).
require-error.
    if result-code not = expected-code perform fail-now end-if
    perform require-error-metadata.
require-error-metadata.
    call "thinkthen_error_code" using by value engine returning failure-code
    call "thinkthen_error_retryable" using by value engine returning failure-retryable
    call "thinkthen_error_message" using by value engine returning error-message-ptr
    move spaces to failure-message
    if error-message-ptr = null perform fail-now end-if
    call dyn-strlen using by value error-message-ptr returning json-length
    if json-length = 0 or json-length > 512 perform fail-now end-if
    call dyn-memcpy using by reference failure-message
       by value error-message-ptr json-length
    if failure-code not = expected-code perform fail-now end-if
    display "PASS " function trim(operation-name) " error=" failure-code.
fail-now.
    display "FAIL " function trim(operation-name)
       " native=" result-code " error=" failure-code
       " message=" function trim(failure-message)
    move 1 to return-code
    call dyn-free-engine using by value engine
    stop run.
