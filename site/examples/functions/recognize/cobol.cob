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
01 kinds pic x(8192).
01 kinds-length usage binary-double unsigned.
01 names pic x(8192).
01 names-length usage binary-double unsigned.
01 member-name pic x(64).
01 found-names pic x(8192).
01 found-names-length usage binary-double unsigned.
01 entities pic x(8192).
01 entities-length usage binary-double unsigned.
01 entity pic x(8192).
01 entity-length usage binary-double unsigned.
01 entity-text pic x(8192).
01 entity-text-length usage binary-double unsigned.
01 member-status usage binary-long signed.
01 row pic 9.
01 expected-names.
   02 filler pic x(20) value '"Maria Chen"'.
   02 filler pic x(20) value '"Northwind Freight"'.
   02 filler pic x(20) value '"Chicago"'.
01 expected-table redefines expected-names.
   02 expected-name pic x(20) occurs 3 times.
procedure division.
    call "thinkthen_engine_new" returning engine

    move '{"version": 1, "recognize": {"kinds": {'
        & '"PER": "Part of a person''s name.", '
        & '"ORG": "Part of the name of an organization: '
        & 'a company, band, team, agency, government '
        & 'body, or media outlet.", '
        & '"LOC": "Part of the name of a place: '
        & 'a country, region, city, or geographic '
        & 'feature.", '
        & '"MISC": "Part of another named entity: a '
        & 'nationality, an event, a product, or the '
        & 'name of a creative work."}}, '
        & '"evidence": "Maria Chen joined Northwind '
        & 'Freight in Chicago last spring."}'
        to kinds
    move length(trim(kinds trailing)) to kinds-length
    call "TT-CALL" using engine
        kinds kinds-length tt-deadline-ms
        names names-length tt-failure
    if tt-failure-code not = 0
        stop run returning 1
    end-if

    move "value" to member-name
    call "TT-JSON-MEMBER" using names names-length
        member-name found-names found-names-length
        member-status
    move "entities" to member-name
    call "TT-JSON-MEMBER" using found-names
        found-names-length member-name
        entities entities-length member-status
    perform varying row from 1 by 1 until row > 3
        move row to member-name
        call "TT-JSON-MEMBER" using entities
            entities-length member-name
            entity entity-length member-status
        move "text" to member-name
        call "TT-JSON-MEMBER" using entity entity-length
            member-name entity-text entity-text-length
            member-status
        if entity-text(1:entity-text-length)
            not = trim(expected-name(row))
            stop run returning 1
        end-if
    end-perform

    call free-engine using by value engine
    stop run returning 0.
