       >>SOURCE FORMAT FREE
identification division.
program-id. TT-VALIDATE-LABELS.
data division.
working-storage section.
01 native-code usage binary-long signed.
linkage section.
01 label-json pic x(8192).
01 label-length usage binary-double unsigned.
01 offending-label pic x(256).
01 status-code usage binary-long signed.
procedure division using label-json label-length offending-label status-code.
    move spaces to offending-label
    if label-length > 8192
       move 1 to status-code
       move "label JSON exceeds 8192 bytes" to offending-label
       goback
    end-if
    call "tt_cobol_labels" using by reference label-json
       by value size is 8 label-length by reference offending-label
       by value size is 8 256 returning native-code
    move native-code to status-code
    goback.
end program TT-VALIDATE-LABELS.
       >>SOURCE FORMAT FREE
identification division.
program-id. TT-PARSE-FIELD.
data division.
working-storage section.
01 native-code usage binary-long signed.
linkage section.
01 field-json pic x(8192).
01 field-length usage binary-double unsigned.
01 field-kind usage binary-long signed.
procedure division using field-json field-length field-kind.
    move 0 to field-kind
    if field-length > 8192 goback end-if
    call "tt_cobol_field" using by reference field-json
       by value size is 8 field-length returning native-code
    move native-code to field-kind
    goback.
end program TT-PARSE-FIELD.
       >>SOURCE FORMAT FREE
identification division.
program-id. TT-VALIDATE-RESULT.
data division.
working-storage section.
01 native-code usage binary-long signed.
01 terminated-verb pic x(17).
01 verb-length usage binary-long unsigned.
linkage section.
01 result-json pic x(8192).
01 result-length usage binary-double unsigned.
01 verb-name pic x(16).
01 status-code usage binary-long signed.
procedure division using result-json result-length verb-name status-code.
    move 0 to status-code
    if result-length > 8192 goback end-if
    move low-values to terminated-verb
    move function length(function trim(verb-name trailing)) to verb-length
    if verb-length = 0 or verb-length > 16 goback end-if
    move verb-name(1:verb-length) to terminated-verb(1:verb-length)
    call "tt_cobol_shape" using by reference result-json
       by value size is 8 result-length by reference terminated-verb
       returning native-code
    move native-code to status-code
    goback.
end program TT-VALIDATE-RESULT.
