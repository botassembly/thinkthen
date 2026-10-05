       >>SOURCE FORMAT FREE
identification division.
program-id. TT-FILES.
*> Any of the ten existing JSON question grammars through the native reader.
*> source-text is {"paths":[...],"unit":"line|window|file","window":N}.
data division.
working-storage section.
01 request-text pic x(8192).
01 request-length usage binary-double unsigned.
01 result-code usage binary-long signed.
linkage section.
01 engine usage pointer.
01 question-text pic x(8192).
01 question-length usage binary-double unsigned.
01 source-text pic x(8192).
01 source-length usage binary-double unsigned.
01 deadline-ms usage binary-double signed.
01 result-text pic x(8192).
01 result-length usage binary-double unsigned.
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
procedure division using engine question-text question-length source-text
   source-length deadline-ms result-text result-length failure-row.
    move 0 to result-length failure-code failure-retryable failure-facts-length
    move spaces to failure-message failure-facts-json result-text
    if question-length > 8192 or source-length > 8192
       move 1 to failure-code
       move "files input length out of range" to failure-message
       goback
    end-if
    call "tt_cobol_files_input" using by reference question-text
       by value size is 8 question-length by reference source-text
       by value size is 8 source-length by reference request-text
       by value size is 8 8191 by reference request-length
       returning result-code
    if result-code not = 0
       move 1 to failure-code
       move "files requires question and source objects within 8192 bytes"
          to failure-message
       goback
    end-if
    call "TT-CALL" using engine request-text request-length deadline-ms
       result-text result-length failure-row
    goback.
