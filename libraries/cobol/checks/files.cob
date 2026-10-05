       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-files.
data division.
working-storage section.
01 engine usage pointer.
01 arg-number usage binary-long signed.
01 settings-text pic x(8192).
01 settings-length usage binary-double unsigned.
01 question-text pic x(8192).
01 question-length usage binary-double unsigned.
01 source-text pic x(8192).
01 source-length usage binary-double unsigned.
01 result-text pic x(8192).
01 result-length usage binary-double unsigned.
01 dyn-free pic x(24) value "thinkthen_engine_free".
copy "thinkthen.cpy".
procedure division.
    move 1 to arg-number
    display arg-number upon argument-number
    accept question-text from argument-value
    compute question-length = function length(function trim(question-text trailing))
    move 2 to arg-number
    display arg-number upon argument-number
    accept source-text from argument-value
    compute source-length = function length(function trim(source-text trailing))
    move 3 to arg-number
    display arg-number upon argument-number
    accept settings-text from argument-value
    compute settings-length = function length(function trim(settings-text trailing))
    call "TT-ENGINE-NEW" using settings-text settings-length engine tt-failure
    if engine = null move 1 to return-code goback end-if
    call "TT-FILES" using engine question-text question-length source-text
       source-length tt-deadline-ms result-text result-length tt-failure
    if tt-failure-code = 0
       display result-text(1:result-length)
    else
       display tt-failure-code
       move 1 to return-code
    end-if
    call dyn-free using by value engine
    if tt-failure-code = 0 move 0 to return-code
    else move 1 to return-code end-if
    goback.
