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
01 is-urgent pic x(8192).
01 is-urgent-length usage binary-double unsigned.
01 most-urgent pic x(8192).
01 most-urgent-length usage binary-double unsigned.
01 member-name pic x(64).
01 ranking pic x(8192).
01 ranking-length usage binary-double unsigned.
01 one-rank pic x(8192).
01 one-rank-length usage binary-double unsigned.
01 rank-index pic x(8192).
01 rank-index-length usage binary-double unsigned.
01 member-status usage binary-long signed.
01 row pic 9.
01 expected-order pic x(4) value "1320".
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"rank": "Is this urgent?", "records": ['
        & '"Newsletter: our autumn catalog is here. '
        & 'No reply needed.", '
        & '"Our checkout page is down and customers '
        & 'cannot pay", '
        & '"Reminder: your invoice is due in 30 days", '
        & '"Please send the signed quote by 5 pm today"]}'
        to is-urgent
    move length(trim(is-urgent trailing))
        to is-urgent-length
    call "TT-CALL" using engine
        is-urgent is-urgent-length tt-deadline-ms
        most-urgent most-urgent-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    move "value" to member-name
    call "TT-JSON-MEMBER" using most-urgent
        most-urgent-length member-name
        ranking ranking-length member-status
    perform varying row from 1 by 1 until row > 4
        move row to member-name
        call "TT-JSON-MEMBER" using ranking ranking-length
            member-name one-rank one-rank-length
            member-status
        move "index" to member-name
        call "TT-JSON-MEMBER" using one-rank
            one-rank-length member-name
            rank-index rank-index-length member-status
        if rank-index(1:rank-index-length)
            not = expected-order(row:1)
            stop run returning 1
        end-if
    end-perform

    call free-engine using by value engine
    stop run returning 0.
