\echo Use "ALTER EXTENSION thinkthen UPDATE" to load this file. \quit

-- 0.1.0 installed its functions in the extension's own schema. 0.2.0 replaces
-- them with the functions in schema thinkthen and reads no key from SQL.
DROP EVENT TRIGGER thinkthen_guard_public;
DROP FUNCTION @extschema@.thinkthen_annotate(text, text, json);
DROP FUNCTION @extschema@.thinkthen_choose(text, text, text[], json, text, text, text, text, bigint);
DROP FUNCTION @extschema@.thinkthen_choose(text, text, text[], text);
DROP FUNCTION @extschema@.thinkthen_choose_many(text, jsonb, json);
DROP FUNCTION @extschema@.thinkthen_decide(text, text, json, text, text, text, text, bigint);
DROP FUNCTION @extschema@.thinkthen_decide(text, text[]);
DROP FUNCTION @extschema@.thinkthen_decide(text, text[], text);
DROP FUNCTION @extschema@.thinkthen_decide(text, text, text);
DROP FUNCTION @extschema@.thinkthen_decide_many(text, jsonb, json);
DROP FUNCTION @extschema@.thinkthen_details(text, text, text);
DROP FUNCTION @extschema@.thinkthen_details(text, text, json, text, text, text, text, bigint);
DROP FUNCTION @extschema@.thinkthen_find(text, text[], json);
DROP FUNCTION @extschema@.thinkthen_find(text, text[], bool);
DROP FUNCTION @extschema@.thinkthen_plan(text, jsonb, json);
DROP FUNCTION @extschema@.thinkthen_probability(text, text);
DROP FUNCTION @extschema@.thinkthen_probability(text, text, text);
DROP FUNCTION @extschema@.thinkthen_rank(text, jsonb, json);
DROP FUNCTION @extschema@.thinkthen_recognize(text, text[]);
DROP FUNCTION @extschema@.thinkthen_recognize(text, text);
DROP FUNCTION @extschema@.thinkthen_relate(text, text[]);
DROP FUNCTION @extschema@.thinkthen_relate(text, text);
DROP FUNCTION @extschema@.thinkthen_relations(text, text);
DROP FUNCTION @extschema@.thinkthen_score(text, text, text[], json, text, text, text, text, bigint);
DROP FUNCTION @extschema@.thinkthen_score(text, text, text[], text);
DROP FUNCTION @extschema@.thinkthen_score_many(text, jsonb, json);
DROP FUNCTION @extschema@.thinkthen_tag(text, text, text[], json, text, text, text, text, bigint);
DROP FUNCTION @extschema@.thinkthen_tag(text, text, text[], text);
DROP FUNCTION @extschema@.thinkthen_tag_many(text, jsonb, json);
DROP FUNCTION @extschema@.thinkthen_try_details(text, text, json, text, text, text, text, bigint);
DROP FUNCTION @extschema@.thinkthen_try_details(text, text, text);
DROP FUNCTION @extschema@.thinkthen_usage();
DROP FUNCTION @extschema@.thinkthen_warm(text, text);
DROP FUNCTION @extschema@.thinkthen_warm(text, text, text);
DROP FUNCTION @extschema@.thinkthen_guard_public();
DO $thinkthen_schema$
BEGIN
    IF to_regnamespace('thinkthen') IS NULL THEN
        CREATE SCHEMA thinkthen;
    END IF;
END
$thinkthen_schema$;

@INSTALL@
