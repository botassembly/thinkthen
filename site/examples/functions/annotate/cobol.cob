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
01 bug-report pic x(8192).
01 bug-report-length usage binary-double unsigned.
01 triage-call pic x(8192).
01 triage-call-length usage binary-double unsigned.
01 member-name pic x(64) value "value".
01 triage pic x(8192).
01 triage-length usage binary-double unsigned.
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
        & '"Steps: click Log in. Nobody gets in."]}'
        to bug-report
    move length(trim(bug-report trailing))
        to bug-report-length
    call "TT-CALL" using engine
        bug-report bug-report-length tt-deadline-ms
        triage-call triage-call-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    call "TT-JSON-MEMBER" using triage-call
        triage-call-length member-name
        triage triage-length member-status
    if triage(1:triage-length) not =
        '[{"steps":true,"area":"login","impact":1.98}]'
        stop run returning 1
    end-if

    call free-engine using by value engine
    stop run returning 0.
