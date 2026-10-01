identification division.
program-id. first-call.
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

    move '{"decide": "Does the customer ask ' &
        'for a refund?", "threshold": "0.2:0.8"}'
        to question
    move "I want to send this back." to evidence
    perform ask-it
    if not outcome-not-sure
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
