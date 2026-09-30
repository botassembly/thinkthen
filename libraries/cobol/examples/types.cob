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
01 member-name pic x(64).
01 member-json pic x(8192).
01 member-length usage binary-double unsigned.
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
    *> ADR 0112 section 4: null is unresolved, {"failed": ...} is a failure
    *> whose unknown extra member reads without error, any other value is
    *> answered, and another object is refused.
    move 'unresolved' to phase-name
    move 'null' to field-json move 4 to bytes-count
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field
    if not field-not-sure perform fail-now end-if
    move 'failed' to phase-name
    move '{"failed":{"kind":"backend","cause":"missing_probability","later":1}}' to field-json
    compute bytes-count = function length(function trim(field-json trailing))
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field
    if not field-failed or tt-field-failure-code not = 2
       or tt-field-cause not = "missing_probability" perform fail-now end-if
    move 'answered' to phase-name
    move '["billing","urgent"]' to field-json
    compute bytes-count = function length(function trim(field-json trailing))
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field
    if not field-resolved perform fail-now end-if
    move '1.2' to field-json move 3 to bytes-count
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field
    if not field-resolved perform fail-now end-if
    move 'other object' to phase-name
    move '{"failed":null}' to field-json
    compute bytes-count = function length(function trim(field-json trailing))
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field
    if tt-field-kind not = 0 perform fail-now end-if
    move '{"team":"billing"}' to field-json
    compute bytes-count = function length(function trim(field-json trailing))
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field
    if tt-field-kind not = 0 perform fail-now end-if
    *> TT-JSON-MEMBER reads named members and ignores the rest.
    move 'member' to phase-name
    move '{"records":1,"requests_sent":1,"cache_answers":0,"seconds":0.1,"later":{"x":[1]}}' to payload
    compute bytes-count = function length(function trim(payload trailing))
    move "records" to member-name
    call "TT-JSON-MEMBER" using payload bytes-count member-name member-json member-length code-value
    if code-value not = 0 or member-json(1:member-length) not = "1" perform fail-now end-if
    move "later" to member-name
    call "TT-JSON-MEMBER" using payload bytes-count member-name member-json member-length code-value
    if code-value not = 0 or member-json(1:member-length) not = '{"x":[1]}' perform fail-now end-if
    move "model" to member-name
    call "TT-JSON-MEMBER" using payload bytes-count member-name member-json member-length code-value
    if code-value not = 1 perform fail-now end-if
    move 'array member' to phase-name
    move '[{"check":null},{"check":{"failed":{"kind":"backend","cause":"missing_answer"}}}]' to payload
    compute bytes-count = function length(function trim(payload trailing))
    move "2" to member-name
    call "TT-JSON-MEMBER" using payload bytes-count member-name member-json member-length code-value
    if code-value not = 0 perform fail-now end-if
    move member-json to payload move member-length to bytes-count
    move "check" to member-name
    call "TT-JSON-MEMBER" using payload bytes-count member-name member-json member-length code-value
    move member-json to field-json move member-length to bytes-count
    call "TT-PARSE-FIELD" using field-json bytes-count tt-field
    if code-value not = 0 or not field-failed or tt-field-cause not = "missing_answer" perform fail-now end-if
    display "COBOL_TYPES_PASS"
    move 0 to return-code
    goback.
fail-now.
    display "COBOL_TYPES_FAIL phase=" phase-name " code=" code-value " field=" tt-field-kind " bad=" offending
    move 1 to return-code
    goback.
