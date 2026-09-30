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
*> One annotate answer member: 1 unresolved null, 2 failure with its kind
*> code and cause, 3 answered, 0 another object or invalid JSON.
data division.
working-storage section.
01 native-code usage binary-long signed.
linkage section.
01 field-json pic x(8192).
01 field-length usage binary-double unsigned.
01 field-row.
   02 field-kind usage binary-long signed.
   02 field-failure-code usage binary-long signed.
   02 field-cause pic x(64).
procedure division using field-json field-length field-row.
    move 0 to field-kind field-failure-code
    move spaces to field-cause
    if field-length > 8192 goback end-if
    call "tt_cobol_field" using by reference field-json
       by value size is 8 field-length by reference field-failure-code
       by reference field-cause by value size is 8 64 returning native-code
    move native-code to field-kind
    inspect field-cause replacing all x"00" by space
    goback.
end program TT-PARSE-FIELD.
       >>SOURCE FORMAT FREE
identification division.
program-id. TT-JSON-MEMBER.
*> Copy one member's JSON text: an object member by name, or an array
*> element by its 1-based index. Status 0 found, 1 missing, 2 invalid or
*> too long. A member the caller does not name is never read.
data division.
working-storage section.
01 native-code usage binary-long signed.
01 name-length usage binary-long unsigned.
linkage section.
01 json-text pic x(8192).
01 json-length usage binary-double unsigned.
01 member-name pic x(64).
01 member-text pic x(8192).
01 member-length usage binary-double unsigned.
01 status-code usage binary-long signed.
procedure division using json-text json-length member-name member-text
   member-length status-code.
    move spaces to member-text
    move 0 to member-length
    move 2 to status-code
    if json-length > 8192 goback end-if
    move function length(function trim(member-name trailing)) to name-length
    call "tt_cobol_member" using by reference json-text
       by value size is 8 json-length by reference member-name
       by value size is 8 name-length by reference member-text
       by value size is 8 8192 by reference member-length returning native-code
    move native-code to status-code
    goback.
end program TT-JSON-MEMBER.
