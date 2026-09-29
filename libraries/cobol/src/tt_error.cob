       >>SOURCE FORMAT FREE
identification division.
program-id. TT-CAPTURE-ERROR.
data division.
working-storage section.
01 dyn-strlen pic x(16) value "strlen".
01 dyn-memcpy pic x(16) value "memcpy".
01 message-ptr usage pointer.
01 facts-ptr usage pointer.
01 message-length usage binary-double unsigned.
01 facts-length usage binary-double unsigned.
linkage section.
01 engine usage pointer.
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
procedure division using engine failure-row.
    move spaces to failure-message failure-facts-json
    move 0 to failure-facts-length
    call "thinkthen_error_code" using by value engine
       returning failure-code
    call "thinkthen_error_retryable" using by value engine
       returning failure-retryable
    call "thinkthen_error_message" using by value engine
       returning message-ptr
    if message-ptr not = null
       call dyn-strlen using by value message-ptr
          returning message-length
       if message-length > 512 move 512 to message-length end-if
       if message-length > 0
          call dyn-memcpy using by reference failure-message
             by value message-ptr message-length
       end-if
    end-if
    call "thinkthen_error_facts_json" using by value engine
       returning facts-ptr
    if facts-ptr not = null
       call dyn-strlen using by value facts-ptr
          returning facts-length
       if facts-length > 8192
          move 6 to failure-code
          move "failure facts exceed 8192 bytes" to failure-message
       else
          move facts-length to failure-facts-length
          if facts-length > 0
             call dyn-memcpy using by reference failure-facts-json
                by value facts-ptr facts-length
          end-if
       end-if
    end-if
    goback.
