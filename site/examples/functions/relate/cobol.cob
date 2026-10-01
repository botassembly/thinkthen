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
01 singer-song pic x(8192).
01 singer-song-length usage binary-double unsigned.
01 sings pic x(8192).
01 sings-length usage binary-double unsigned.
01 member-name pic x(64).
01 found-edges pic x(8192).
01 found-edges-length usage binary-double unsigned.
01 edges pic x(8192).
01 edges-length usage binary-double unsigned.
01 edge pic x(8192).
01 edge-length usage binary-double unsigned.
01 singer pic x(8192).
01 singer-length usage binary-double unsigned.
01 singer-name pic x(8192).
01 singer-name-length usage binary-double unsigned.
01 member-status usage binary-long signed.
01 row pic 9.
01 expected-singers.
   02 filler pic x(20) value '"Paul McCartney"'.
   02 filler pic x(20) value '"Ringo Starr"'.
01 expected-table redefines expected-singers.
   02 expected-singer pic x(20) occurs 2 times.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"version": 1, "relate": {"relations": [{'
        & '"name": "sings", "source": "singer", '
        & '"target": "song"}]}, "records": ['
        & '{"name": "Paul McCartney", "kind": "singer"}, '
        & '{"name": "Ringo Starr", "kind": "singer"}, '
        & '{"name": "Yesterday", "kind": "song"}, '
        & '{"name": "Octopus''s Garden", "kind": "song"}]}'
        to singer-song
    move length(trim(singer-song trailing))
        to singer-song-length
    call "TT-CALL" using engine
        singer-song singer-song-length tt-deadline-ms
        sings sings-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    move "value" to member-name
    call "TT-JSON-MEMBER" using sings sings-length
        member-name found-edges found-edges-length
        member-status
    move "edges" to member-name
    call "TT-JSON-MEMBER" using found-edges
        found-edges-length member-name
        edges edges-length member-status
    perform varying row from 1 by 1 until row > 2
        move row to member-name
        call "TT-JSON-MEMBER" using edges edges-length
            member-name edge edge-length member-status
        move "source" to member-name
        call "TT-JSON-MEMBER" using edge edge-length
            member-name singer singer-length member-status
        move "name" to member-name
        call "TT-JSON-MEMBER" using singer singer-length
            member-name singer-name singer-name-length
            member-status
        if singer-name(1:singer-name-length)
            not = trim(expected-singer(row))
            stop run returning 1
        end-if
    end-perform

    call free-engine using by value engine
    stop run returning 0.
