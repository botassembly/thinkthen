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
01 bug-reports pic x(8192).
01 bug-reports-length usage binary-double unsigned.
01 triage pic x(8192).
01 triage-length usage binary-double unsigned.
01 member-name pic x(64) value "value".
01 rows pic x(8192).
01 rows-length usage binary-double unsigned.
01 member-status usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"annotate": {"version": 1, "questions": {'
        & '"steps": {"decide": '
        & '"Does the report give steps to reproduce?"}, '
        & '"area": {"choose": '
        & '"Which part of the app is this?", '
        & '"options": ["export", "login", "billing"]}, '
        & '"impact": {"score": '
        & '"How much does this block the user?", '
        & '"levels": ["None.", "Slows them.", '
        & '"Blocks work."]}}}, '
        & '"records": ['
        & '"Steps: click Export. It is very slow.", '
        & '"Steps: click Log in. Nobody gets in.", '
        & '"The Pay button on billing is too blue."]}'
        to bug-reports
    move length(trim(bug-reports trailing))
        to bug-reports-length
    call "TT-CALL" using engine
        bug-reports bug-reports-length tt-deadline-ms
        triage triage-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    call "TT-JSON-MEMBER" using triage triage-length
        member-name rows rows-length member-status
    if rows(1:rows-length) not =
        '[{"steps":true,"area":"export","impact":1.04},'
        & '{"steps":true,"area":"login","impact":1.98},'
        & '{"steps":false,"area":"billing",'
        & '"impact":0.09}]'
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
