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
01 which-team pic x(8192).
01 which-team-length usage binary-double unsigned.
01 owners pic x(8192).
01 owners-length usage binary-double unsigned.
01 member-name pic x(64) value "value".
01 team pic x(8192).
01 team-length usage binary-double unsigned.
01 member-status usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"choose": "Which team owns this?", '
        & '"options": {'
        & '"billing": "Invoices, fees, and refunds.", '
        & '"shipping": "Parcels and delivery.", '
        & '"account": "Logins and passwords."}, '
        & '"evidence": '
        & '"My parcel went to the wrong address."}'
        to which-team
    move length(trim(which-team trailing))
        to which-team-length
    call "TT-CALL" using engine
        which-team which-team-length tt-deadline-ms
        owners owners-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    call "TT-JSON-MEMBER" using owners owners-length
        member-name team team-length member-status
    if team(1:team-length) not = '"shipping"'
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
