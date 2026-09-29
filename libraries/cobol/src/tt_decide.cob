       >>SOURCE FORMAT FREE
identification division.
program-id. TT-DECIDE.
data division.
working-storage section.
01 dyn-free-engine pic x(24) value "thinkthen_engine_free".
01 dyn-cancel pic x(20) value "thinkthen_cancel".
01 dyn-free-token pic x(30) value "thinkthen_cancel_token_free".
01 dyn-free-string pic x(24) value "thinkthen_free_string".
01 dyn-memcpy pic x(16) value "memcpy".
01 native-code usage binary-long signed.
01 shape-code usage binary-long signed.
01 idx usage binary-long unsigned.
01 no-deadline usage binary-double signed value -1.
01 no-token usage pointer.
01 facts-ptr usage pointer.
01 native-facts-length usage binary-double unsigned.
01 native-answer.
   02 native-outcome usage binary-long signed.
   02 native-pad usage binary-long unsigned.
   02 native-probability usage float-long.
linkage section.
01 engine usage pointer.
01 question-text pic x(256).
01 question-length usage binary-double unsigned.
01 evidence-text pic x(256).
01 evidence-length usage binary-double unsigned.
01 answer-row.
   02 outcome usage binary-long signed.
   02 alignment-pad usage binary-long unsigned.
   02 probability usage float-long.
01 facts-row.
   02 facts-length usage binary-double unsigned.
   02 facts-json pic x(8192).
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
procedure division using engine question-text question-length
   evidence-text evidence-length answer-row facts-row failure-row.
    move spaces to failure-message
    move spaces to failure-facts-json
    move spaces to facts-json
    move 0 to facts-length
    move 0 to failure-facts-length
    move 0 to failure-code failure-retryable
    if question-length > 255 or evidence-length > 256
       move 1 to failure-code
       move "length out of range" to failure-message
       goback
    end-if
    perform varying idx from 1 by 1 until idx > question-length
       if question-text(idx:1) = x"00"
          move 1 to failure-code
          move "interior NUL in C-string question" to failure-message
          goback
       end-if
    end-perform
    move x"00" to question-text(question-length + 1:1)
    set facts-ptr to null
    move 0 to native-facts-length
    call "thinkthen_decide_with_facts_opts" using by value engine
       by reference question-text evidence-text
       by value size is 8 evidence-length no-deadline no-token
       by reference native-answer facts-ptr native-facts-length
       returning native-code
    if native-code not = 0
       call "TT-CAPTURE-ERROR" using engine failure-row
       if facts-ptr not = null
          call dyn-free-string using by value facts-ptr
       end-if
       goback
    end-if
    if facts-ptr = null or native-facts-length = 0
       or native-facts-length > 8192
       move 6 to failure-code
       move "native facts exceed or miss 8192-byte storage"
          to failure-message
    else
       call "tt_cobol_facts" using by value facts-ptr
          native-facts-length returning shape-code
       if shape-code not = 0
          move 6 to failure-code
          move "native facts broke the type contract" to failure-message
       else
          move native-answer to answer-row
          move native-facts-length to facts-length
          call dyn-memcpy using by reference facts-json
             by value facts-ptr native-facts-length
       end-if
    end-if
    if facts-ptr not = null
       call dyn-free-string using by value facts-ptr
    end-if
    goback.
