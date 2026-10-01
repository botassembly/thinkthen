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
01 reviews pic x(8192).
01 reviews-length usage binary-double unsigned.
01 screened pic x(8192).
01 screened-length usage binary-double unsigned.
01 member-name pic x(64) value "value".
01 complaints pic x(8192).
01 complaints-length usage binary-double unsigned.
01 member-status usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"filter": "Is this a complaint?", '
        & '"records": ['
        & '"Arrived a day early. Thank you!", '
        & '"The zipper broke the first time I used it.", '
        & '"Does this come in blue?", '
        & '"The strap snapped on day two."]}'
        to reviews
    move length(trim(reviews trailing))
        to reviews-length
    call "TT-CALL" using engine
        reviews reviews-length tt-deadline-ms
        screened screened-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    call "TT-JSON-MEMBER" using screened screened-length
        member-name complaints complaints-length
        member-status
    if complaints(1:complaints-length) not =
        '["The zipper broke the first time I used it.",'
        & '"The strap snapped on day two."]'
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
