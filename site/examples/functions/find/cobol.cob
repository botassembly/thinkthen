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
01 which-line pic x(8192).
01 which-line-length usage binary-double unsigned.
01 deadline pic x(8192).
01 deadline-length usage binary-double unsigned.
01 member-name pic x(64).
01 picked pic x(8192).
01 picked-length usage binary-double unsigned.
01 refund-line pic x(8192).
01 refund-line-length usage binary-double unsigned.
01 member-status usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"find": "Which line gives the refund '
        & 'deadline?", "units": ['
        & '"Returns need the original receipt.", '
        & '"Refunds are issued within 30 days '
        & 'of purchase.", '
        & '"Shipping is free on orders over $50.", '
        & '"Gift cards cannot be exchanged for cash."]}'
        to which-line
    move length(trim(which-line trailing))
        to which-line-length
    call "TT-CALL" using engine
        which-line which-line-length tt-deadline-ms
        deadline deadline-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    move "value" to member-name
    call "TT-JSON-MEMBER" using deadline deadline-length
        member-name picked picked-length member-status
    move "unit" to member-name
    call "TT-JSON-MEMBER" using picked picked-length
        member-name refund-line refund-line-length
        member-status
    if refund-line(1:refund-line-length) not =
        '"Refunds are issued within 30 days of purchase."'
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
