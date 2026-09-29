       >>SOURCE FORMAT FREE
identification division.
program-id. TT-DECIDE.
data division.
working-storage section.
01 dyn-free-engine pic x(24) value "thinkthen_engine_free".
01 dyn-cancel pic x(20) value "thinkthen_cancel".
01 dyn-free-token pic x(30) value "thinkthen_cancel_token_free".
01 dyn-free-string pic x(24) value "thinkthen_free_string".
01 native-code usage binary-long signed.
01 idx usage binary-long unsigned.
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
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
procedure division using engine question-text question-length
   evidence-text evidence-length answer-row failure-row.
    move spaces to failure-message
    move spaces to failure-facts-json
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
    call "thinkthen_decide" using by value engine
       by reference question-text evidence-text by value size is 8 evidence-length
       by reference answer-row returning native-code
    if native-code not = 0
       call "TT-CAPTURE-ERROR" using engine failure-row
    end-if
    goback.
