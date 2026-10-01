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
01 how-urgent pic x(8192).
01 how-urgent-length usage binary-double unsigned.
01 urgency pic x(8192).
01 urgency-length usage binary-double unsigned.
01 member-name pic x(64) value "value".
01 level pic x(8192).
01 level-length usage binary-double unsigned.
01 member-status usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"score": "How urgent is this?", '
        & '"levels": ["Routine.", "Soon.", "Immediate."], '
        & '"evidence": "Our checkout page is down '
        & 'and customers cannot pay.\n"}'
        to how-urgent
    move length(trim(how-urgent trailing))
        to how-urgent-length
    call "TT-CALL" using engine
        how-urgent how-urgent-length tt-deadline-ms
        urgency urgency-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    call "TT-JSON-MEMBER" using urgency urgency-length
        member-name level level-length member-status
    if level(1:level-length) not =
        '2.0'
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
