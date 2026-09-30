       >>SOURCE FORMAT FREE
identification division.
program-id. cobol-door.
data division.
working-storage section.
01 arg-count usage binary-long signed.
01 arg-number usage binary-long signed.
01 engine usage pointer.
01 settings-text pic x(8192) value "{}".
01 settings-length usage binary-double unsigned value 2.
01 request-text pic x(8192).
01 request-length usage binary-double unsigned.
01 result-text pic x(8192).
01 result-length usage binary-double unsigned.
01 dyn-free pic x(24) value "thinkthen_engine_free".
copy "thinkthen.cpy".
procedure division.
    accept arg-count from argument-number
    if arg-count < 1 or arg-count > 2
       move 2 to return-code
       goback
    end-if
    move 1 to arg-number
    display arg-number upon argument-number
    accept request-text from argument-value
    compute request-length =
       function length(function trim(request-text trailing))
    if arg-count = 2
       move 2 to arg-number
       display arg-number upon argument-number
       accept settings-text from argument-value
       compute settings-length =
          function length(function trim(settings-text trailing))
    end-if
    call "TT-ENGINE-NEW" using settings-text settings-length
       engine tt-failure
    if engine = null
       perform print-error
       move 0 to return-code
       goback
    end-if
    call "TT-CALL" using engine request-text request-length tt-deadline-ms
       result-text result-length tt-failure
    if tt-failure-code = 0
       display result-text(1:result-length)
    else
       perform print-error
    end-if
    call dyn-free using by value engine
    move 0 to return-code
    goback.
print-error.
    evaluate true
       when failure-usage display '{"failed":{"kind":"usage","code":1}}'
       when failure-backend display '{"failed":{"kind":"backend","code":2}}'
       when failure-deadline display '{"failed":{"kind":"deadline","code":3}}'
       when failure-local display '{"failed":{"kind":"local","code":4}}'
       when failure-cancelled display '{"failed":{"kind":"cancelled","code":5}}'
       when failure-defect display '{"failed":{"kind":"defect","code":6}}'
       when other display '{"failed":{"kind":"unknown","code":0}}'
    end-evaluate.
