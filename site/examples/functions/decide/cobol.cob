identification division.
program-id. sample.
environment division.
configuration section.
repository. function all intrinsic.
data division.
working-storage section.
copy "thinkthen.cpy".
01 engine usage pointer.
01 free-engine pic x(24) value "thinkthen_engine_free".
01 question pic x(200)
    value "Does the customer ask for a refund?".
01 question-length usage binary-double unsigned.
01 evidence pic x(200)
    value "Please refund my order. It arrived broken.".
01 evidence-length usage binary-double unsigned.
01 is-refund.
   02 refund-outcome usage binary-long signed.
      88 is-refund-yes value 1.
   02 filler usage binary-long unsigned.
   02 filler usage float-long.
procedure division.
    call "thinkthen_engine_new" returning engine

    move length(trim(question trailing))
        to question-length
    move length(trim(evidence trailing))
        to evidence-length
    call "TT-DECIDE" using engine
        question question-length
        evidence evidence-length
        tt-deadline-ms is-refund tt-facts tt-failure
    if not is-refund-yes
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
