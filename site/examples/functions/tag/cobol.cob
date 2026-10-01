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
01 which-labels pic x(8192).
01 which-labels-length usage binary-double unsigned.
01 fitting pic x(8192).
01 fitting-length usage binary-double unsigned.
01 member-name pic x(64) value "value".
01 labels pic x(8192).
01 labels-length usage binary-double unsigned.
01 member-status usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"tag": "Which labels fit this message?", '
        & '"labels": ["praise", "bug", "billing"], '
        & '"evidence": "Love the new dashboard, but '
        & 'export crashes the app,\nand I was charged '
        & 'twice.\n"}'
        to which-labels
    move length(trim(which-labels trailing))
        to which-labels-length
    call "TT-CALL" using engine
        which-labels which-labels-length tt-deadline-ms
        fitting fitting-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    call "TT-JSON-MEMBER" using fitting fitting-length
        member-name labels labels-length member-status
    if labels(1:labels-length) not =
        '["praise","bug","billing"]'
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
