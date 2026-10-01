identification division.
program-id. backends.
environment division.
configuration section.
repository. function all intrinsic.
data division.
working-storage section.
copy "thinkthen.cpy".
01 engine usage pointer.
01 question pic x(200).
01 question-length usage binary-double unsigned.
01 evidence pic x(200).
01 evidence-length usage binary-double unsigned.
procedure division.
    call "thinkthen_engine_new" returning engine

    move "Does the customer ask for a refund?"
        to question
    move "Please refund my order. It arrived broken."
        to evidence
    perform ask-it
    if not outcome-yes
        stop run returning 1
    end-if

    move "Does the customer ask for a refund?"
        to question
    move "Thanks for the quick help yesterday!"
        to evidence
    perform ask-it
    if not outcome-no
        stop run returning 1
    end-if
    stop run returning 0.

ask-it.
    move length(trim(question trailing))
        to question-length
    move length(trim(evidence trailing))
        to evidence-length
    call "TT-DECIDE" using engine
        question question-length
        evidence evidence-length
        tt-deadline-ms tt-answer tt-facts tt-failure.
