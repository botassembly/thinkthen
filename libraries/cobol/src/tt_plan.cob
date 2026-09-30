       >>SOURCE FORMAT FREE
identification division.
program-id. TT-PLAN.
*> Preview a decide, choose, score or tag call without sending it. The
*> question is bare text, or one question object when it starts with "{".
*> settings-length 0 means no settings. Returns the result schema's plan
*> object as JSON text. The preview needs no key and sends nothing.
data division.
working-storage section.
01 dyn-memcpy pic x(16) value "memcpy".
01 dyn-free-string pic x(24) value "thinkthen_free_string".
01 native-code usage binary-long signed.
01 verb-length usage binary-long unsigned.
01 plan-input pic x(8192).
01 plan-input-length usage binary-double unsigned.
01 result-ptr usage pointer.
01 native-length usage binary-double unsigned.
linkage section.
01 engine usage pointer.
01 verb-name pic x(16).
01 question-text pic x(8192).
01 question-length usage binary-double unsigned.
01 text-count usage binary-long unsigned.
01 text-rows pic x(16896).
01 settings-text pic x(8192).
01 settings-length usage binary-double unsigned.
01 result-text pic x(8192).
01 result-length usage binary-double unsigned.
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
procedure division using engine verb-name question-text question-length
   text-count text-rows settings-text settings-length
   result-text result-length failure-row.
    move spaces to result-text failure-message failure-facts-json
    move 0 to result-length failure-code failure-retryable failure-facts-length
    move function length(function trim(verb-name trailing)) to verb-length
    if question-length > 8192 or settings-length > 8192 or text-count > 64
       move 1 to failure-code
       move "plan input out of range" to failure-message
       goback
    end-if
    call "tt_cobol_plan_input" using by reference verb-name
       by value size is 8 verb-length by reference question-text
       by value size is 8 question-length by reference text-rows
       by value size is 4 text-count by reference settings-text
       by value size is 8 settings-length by reference plan-input
       by value size is 8 8191 by reference plan-input-length
       returning native-code
    if native-code = 1
       move 1 to failure-code
       move "plan question object or settings is not a JSON object"
          to failure-message
       goback
    end-if
    if native-code not = 0
       move 1 to failure-code
       move "plan input exceeds 8192 bytes" to failure-message
       goback
    end-if
    move x"00" to plan-input(plan-input-length + 1:1)
    set result-ptr to null
    call "thinkthen_plan_json" using by value engine
       by reference plan-input result-ptr native-length
       returning native-code
    if native-code not = 0
       call "TT-CAPTURE-ERROR" using engine failure-row
       goback
    end-if
    if native-length > 8192
       move 6 to failure-code
       move "native plan exceeds 8192 bytes" to failure-message
    else
       move native-length to result-length
       call dyn-memcpy using by reference result-text
          by value result-ptr native-length
    end-if
    call dyn-free-string using by value result-ptr
    goback.
