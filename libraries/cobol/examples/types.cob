       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-types.
data division.
working-storage section.
copy "thinkthen.cpy".
01 labels pic x(8192).
01 field-json pic x(8192).
01 payload pic x(8192).
01 bytes-count usage binary-double unsigned.
01 offending pic x(256).
01 code-value usage binary-long signed.
01 name-of-verb pic x(16).
01 phase-name pic x(32).
procedure division.
    move 'bare labels' to phase-name
    move '["first","second"]' to labels
    compute bytes-count = function length(function trim(labels trailing))
    call "TT-VALIDATE-LABELS" using labels bytes-count offending code-value
    if code-value not = 0 perform fail-now end-if
    move 'described labels' to phase-name
    move '{"first":{"what":"First","not_for":"Other","examples":["one"]},"second":"Second"}' to labels
    compute bytes-count = function length(function trim(labels trailing))
    call "TT-VALIDATE-LABELS" using labels bytes-count offending code-value
    if code-value not = 0 perform fail-now end-if
    move 'mixed labels' to phase-name
    move '{"first":"First","second":null}' to labels
    compute bytes-count = function length(function trim(labels trailing))
    call "TT-VALIDATE-LABELS" using labels bytes-count offending code-value
    if code-value not = 1 or offending(1:6) not = "second"
       perform fail-now
    end-if
    move 'mixed list labels' to phase-name
    move '["first",{"second":"Second"}]' to labels
    compute bytes-count = function length(function trim(labels trailing))
    call "TT-VALIDATE-LABELS" using labels bytes-count offending code-value
    if code-value not = 1 or offending(1:6) not = "second"
       perform fail-now
    end-if
    move 'bad structured description' to phase-name
    move '{"first":{"what":"First","not_for":"Other","examples":[1]}}' to labels
    compute bytes-count = function length(function trim(labels trailing))
    call "TT-VALIDATE-LABELS" using labels bytes-count offending code-value
    if code-value not = 1 or offending(1:5) not = "first"
       perform fail-now
    end-if
    move 'unresolved' to phase-name
    move 'null' to field-json move 4 to bytes-count
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field-kind
    if not field-not-sure perform fail-now end-if
    move 'failed' to phase-name
    move '{"failed":{"kind":"backend","cause":"missing_answer"}}' to field-json
    compute bytes-count = function length(function trim(field-json trailing))
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field-kind
    if not field-failed perform fail-now end-if
    move 'bad failure' to phase-name
    move '{"failed":{"kind":"backend","cause":"wrong_cause"}}' to field-json
    compute bytes-count = function length(function trim(field-json trailing))
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field-kind
    if tt-field-kind not = 0 perform fail-now end-if
    move 'annotate' to phase-name
    move '[{"check":null},{"check":{"failed":{"kind":"backend","cause":"missing_answer"}}}]' to payload
    compute bytes-count = function length(function trim(payload trailing))
    move "annotate" to name-of-verb
    call "TT-VALIDATE-RESULT" using payload bytes-count name-of-verb code-value
    if code-value not = 1 perform fail-now end-if
    move 'recognize escaped NUL' to phase-name
    move '{"entities":[{"text":"a\u0000b","start":0,"end":3,"length":3,"kind":"person","strength":0.9}]}' to payload
    compute bytes-count = function length(function trim(payload trailing))
    move "recognize" to name-of-verb
    call "TT-VALIDATE-RESULT" using payload bytes-count name-of-verb code-value
    if code-value not = 1 perform fail-now end-if
    move 'recognize' to phase-name
    move '{"entities":[{"text":"🧬","start":0,"end":1,"length":1,"kind":"gene","strength":0.8}]}' to payload
    compute bytes-count = function length(function trim(payload trailing))
    move "recognize" to name-of-verb
    call "TT-VALIDATE-RESULT" using payload bytes-count name-of-verb code-value
    if code-value not = 1 perform fail-now end-if
    move 'bad offset' to phase-name
    move '{"entities":[{"text":"x","start":3,"end":1,"length":1,"kind":"person","strength":0.8}]}' to payload
    compute bytes-count = function length(function trim(payload trailing))
    move "recognize" to name-of-verb
    call "TT-VALIDATE-RESULT" using payload bytes-count name-of-verb code-value
    if code-value not = 0 perform fail-now end-if
    move 'typed edge' to phase-name
    move '{"edges":[{"relation":"works","source":{"name":"a","kind":"person"},"target":{"name":"b","kind":"person"},"probability":0.9}]}' to payload
    compute bytes-count = function length(function trim(payload trailing))
    move "relate" to name-of-verb
    call "TT-VALIDATE-RESULT" using payload bytes-count name-of-verb code-value
    if code-value not = 1 perform fail-now end-if
    move 'bad edge endpoint' to phase-name
    move '{"edges":[{"relation":"works","source":"a","target":"b","probability":0.9}]}' to payload
    compute bytes-count = function length(function trim(payload trailing))
    call "TT-VALIDATE-RESULT" using payload bytes-count name-of-verb code-value
    if code-value not = 0 perform fail-now end-if
    display "COBOL_TYPES_PASS"
    move 0 to return-code
    goback.
fail-now.
    display "COBOL_TYPES_FAIL phase=" phase-name " code=" code-value " field=" tt-field-kind " bad=" offending
    move 1 to return-code
    goback.
