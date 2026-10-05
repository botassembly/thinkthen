identification division.
program-id. backends.
environment division.
configuration section.
repository. function all intrinsic.
data division.
working-storage section.
copy "thinkthen.cpy".
01 engine usage pointer.
01 settings-table.
   02 filler pic x(100) value
       '{"backend":"typesafe"}'.
   02 filler pic x(100) value
       '{"backend":"liquid"}'.
   02 filler pic x(100) value
       '{"backend":"ollama","base_url":"http://localhost:11535/v1"}'.
01 settings-rows redefines settings-table.
   02 settings-text pic x(100) occurs 3 times.
01 settings-index usage binary-long.
01 settings-length usage binary-double unsigned.
01 free-engine pic x(24) value "thinkthen_engine_free".
01 refund-question pic x(200)
    value "Does the customer ask for a refund?".
01 question pic x(200).
01 question-length usage binary-double unsigned.
01 evidence pic x(200).
01 evidence-length usage binary-double unsigned.
procedure division.
    perform varying settings-index from 1 by 1
        until settings-index > 3
        move length(trim(settings-text(settings-index) trailing))
            to settings-length
        call "TT-ENGINE-NEW" using settings-text(settings-index)
            settings-length engine tt-failure
        if engine = null or tt-failure-code not = 0
            stop run returning 1
        end-if

        move "Please refund my order. It arrived broken."
            to evidence
        perform ask-it
        if not outcome-yes
            stop run returning 1
        end-if

        move "Thanks for the quick help yesterday!"
            to evidence
        perform ask-it
        if not outcome-no
            stop run returning 1
        end-if
        call free-engine using by value engine
    end-perform
    stop run returning 0.

ask-it.
    move refund-question to question
    move length(trim(question trailing))
        to question-length
    move length(trim(evidence trailing))
        to evidence-length
    call "TT-DECIDE" using engine
        question question-length
        evidence evidence-length
        tt-deadline-ms tt-answer tt-facts tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if.
