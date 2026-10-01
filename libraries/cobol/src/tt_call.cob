       >>SOURCE FORMAT FREE
identification division.
program-id. TT-CALL.
data division.
working-storage section.
01 dyn-strlen pic x(16) value "strlen".
01 dyn-memcpy pic x(16) value "memcpy".
01 dyn-free-string pic x(24) value "thinkthen_free_string".
01 result-ptr usage pointer.
01 native-length usage binary-double unsigned.
01 idx usage binary-double unsigned.
01 no-token usage pointer.
*> The caller's text is copied here before the NUL; caller storage is never written.
01 request-copy pic x(8192).
linkage section.
01 engine usage pointer.
01 request-text pic x(8192).
01 request-length usage binary-double unsigned.
01 deadline-ms usage binary-double signed.
01 result-text pic x(8192).
01 result-length usage binary-double unsigned.
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
procedure division using engine request-text request-length deadline-ms
   result-text result-length failure-row.
    move spaces to result-text failure-message failure-facts-json
    move 0 to result-length failure-code failure-retryable
       failure-facts-length
    if request-length = 0 or request-length >= 8192
       move 1 to failure-code
       move "request length out of range" to failure-message
       goback
    end-if
    perform varying idx from 1 by 1 until idx > request-length
       if request-text(idx:1) = x"00"
          move 1 to failure-code
          move "interior NUL in request" to failure-message
          goback
       end-if
    end-perform
    move request-text(1:request-length) to request-copy(1:request-length)
    move x"00" to request-copy(request-length + 1:1)
    set no-token to null
    call "thinkthen_call_opts" using by value engine
       by reference request-copy by value size is 8 deadline-ms no-token
       returning result-ptr
    if result-ptr = null
       call "TT-CAPTURE-ERROR" using engine failure-row
       goback
    end-if
    call dyn-strlen using by value result-ptr returning native-length
    if native-length > 8192
       move 6 to failure-code
       move "native result exceeds 8192 bytes" to failure-message
    else
       move native-length to result-length
       if native-length > 0
          call dyn-memcpy using by reference result-text
             by value result-ptr native-length
       end-if
    end-if
    call dyn-free-string using by value result-ptr
    goback.
