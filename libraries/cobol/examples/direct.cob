       >>SOURCE FORMAT FREE
identification division.
program-id. direct-cobol.
environment division.
configuration section.
repository. function all intrinsic.
data division.
working-storage section.
01 dyn-free-engine pic x(24) value "thinkthen_engine_free".
01 engine usage pointer.
01 question-text pic x(80) value z"Is this a decision?".
01 evidence pic x(8) value z"café".
01 evidence-length usage binary-double unsigned value 5.
01 result-code usage binary-long signed.
01 answer-row.
   02 outcome usage binary-long signed.
   02 alignment-pad usage binary-long unsigned.
   02 probability usage float-long.
01 error-code usage binary-long signed.
01 error-retryable usage binary-long signed.
01 error-message usage pointer.
procedure division.
    call "thinkthen_engine_new" returning engine
    if engine = null
       display "engine_new failed"
       move 1 to return-code
       goback
    end-if
    call "thinkthen_decide" using by value engine
         by reference question-text evidence
         by value size is 8 evidence-length
         by reference answer-row
         returning result-code
    if result-code not = 0
       call "thinkthen_error_code" using by value engine returning error-code
       call "thinkthen_error_retryable" using by value engine returning error-retryable
       call "thinkthen_error_message" using by value engine returning error-message
       display "direct-error=" error-code " retry=" error-retryable
       call dyn-free-engine using by value engine
       move 1 to return-code
       goback
    end-if
    display "outcome=" outcome " probability=" probability
    call dyn-free-engine using by value engine
    move 0 to return-code
    goback.
