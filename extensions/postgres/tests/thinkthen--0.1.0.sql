-- Test-only stand-in for the published 0.1.0 install script: the same objects
-- with SQL bodies, so the upgrade test needs no 0.1.0 library.
CREATE FUNCTION "thinkthen_annotate"("set" TEXT, "input" TEXT, "settings" json DEFAULT NULL) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_choose"("question" TEXT, "input" TEXT, "members" TEXT[] DEFAULT NULL, "settings" json DEFAULT NULL, "threshold" TEXT DEFAULT NULL, "context" TEXT DEFAULT NULL, "model" TEXT DEFAULT NULL, "batch" TEXT DEFAULT NULL, "deadline_ms" bigint DEFAULT NULL) RETURNS TEXT LANGUAGE sql AS $$SELECT NULL::TEXT$$;
CREATE FUNCTION "thinkthen_choose"("_question" TEXT, "_input" TEXT, "_members" TEXT[], "_context" TEXT) RETURNS TEXT LANGUAGE sql AS $$SELECT NULL::TEXT$$;
CREATE FUNCTION "thinkthen_choose_many"("question" TEXT, "input" jsonb, "settings" json DEFAULT NULL) RETURNS TABLE ( "key" TEXT, "value" TEXT, "probability" double precision ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::TEXT, NULL::double precision WHERE false$$;
CREATE FUNCTION "thinkthen_decide"("question" TEXT, "input" TEXT, "settings" json DEFAULT NULL, "threshold" TEXT DEFAULT NULL, "context" TEXT DEFAULT NULL, "model" TEXT DEFAULT NULL, "batch" TEXT DEFAULT NULL, "deadline_ms" bigint DEFAULT NULL) RETURNS bool LANGUAGE sql AS $$SELECT NULL::bool$$;
CREATE FUNCTION "thinkthen_decide"("_question" TEXT, "_inputs" TEXT[]) RETURNS bigint LANGUAGE sql AS $$SELECT NULL::bigint$$;
CREATE FUNCTION "thinkthen_decide"("_question" TEXT, "_inputs" TEXT[], "_context" TEXT) RETURNS bigint LANGUAGE sql AS $$SELECT NULL::bigint$$;
CREATE FUNCTION "thinkthen_decide"("_question" TEXT, "_input" TEXT, "_context" TEXT) RETURNS bool LANGUAGE sql AS $$SELECT NULL::bool$$;
CREATE FUNCTION "thinkthen_decide_many"("question" TEXT, "input" jsonb, "settings" json DEFAULT NULL) RETURNS TABLE ( "key" TEXT, "value" bool, "probability" double precision ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::bool, NULL::double precision WHERE false$$;
CREATE FUNCTION "thinkthen_details"("_question" TEXT, "_input" TEXT, "_context" TEXT) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_details"("question" TEXT, "input" TEXT, "settings" json DEFAULT NULL, "threshold" TEXT DEFAULT NULL, "context" TEXT DEFAULT NULL, "model" TEXT DEFAULT NULL, "batch" TEXT DEFAULT NULL, "deadline_ms" bigint DEFAULT NULL) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_find"("question" TEXT, "units" TEXT[], "settings" json DEFAULT NULL) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_find"("question" TEXT, "units" TEXT[], "_none" bool) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_plan"("question" TEXT, "input" jsonb, "settings" json DEFAULT NULL) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_probability"("_question" TEXT, "_input" TEXT) RETURNS double precision LANGUAGE sql AS $$SELECT NULL::double precision$$;
CREATE FUNCTION "thinkthen_probability"("_question" TEXT, "_input" TEXT, "_context" TEXT) RETURNS double precision LANGUAGE sql AS $$SELECT NULL::double precision$$;
CREATE FUNCTION "thinkthen_rank"("question" TEXT, "input" jsonb, "settings" json DEFAULT NULL) RETURNS TABLE ( "key" TEXT, "rank" bigint, "probability" double precision ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::bigint, NULL::double precision WHERE false$$;
CREATE FUNCTION "thinkthen_recognize"("body" TEXT, "kinds" TEXT[]) RETURNS TABLE ( "text" TEXT, "start" INT, "end" INT, "length" INT, "kind" TEXT, "strength" double precision ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::INT, NULL::INT, NULL::INT, NULL::TEXT, NULL::double precision WHERE false$$;
CREATE FUNCTION "thinkthen_recognize"("body" TEXT, "spec" TEXT) RETURNS TABLE ( "text" TEXT, "start" INT, "end" INT, "length" INT, "kind" TEXT, "strength" double precision ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::INT, NULL::INT, NULL::INT, NULL::TEXT, NULL::double precision WHERE false$$;
CREATE FUNCTION "thinkthen_relate"("query" TEXT, "rules" TEXT[]) RETURNS TABLE ( "relation" TEXT, "source" bigint, "target" bigint, "probability" double precision, "either" bool ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::bigint, NULL::bigint, NULL::double precision, NULL::bool WHERE false$$;
CREATE FUNCTION "thinkthen_relate"("query" TEXT, "rules" TEXT) RETURNS TABLE ( "relation" TEXT, "source" bigint, "target" bigint, "probability" double precision, "either" bool ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::bigint, NULL::bigint, NULL::double precision, NULL::bool WHERE false$$;
CREATE FUNCTION "thinkthen_relations"("body" TEXT, "spec" TEXT) RETURNS TABLE ( "relation" TEXT, "source_text" TEXT, "source_kind" TEXT, "target_text" TEXT, "target_kind" TEXT, "probability" double precision, "either" bool ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::TEXT, NULL::TEXT, NULL::TEXT, NULL::TEXT, NULL::double precision, NULL::bool WHERE false$$;
CREATE FUNCTION "thinkthen_score"("question" TEXT, "input" TEXT, "members" TEXT[] DEFAULT NULL, "settings" json DEFAULT NULL, "threshold" TEXT DEFAULT NULL, "context" TEXT DEFAULT NULL, "model" TEXT DEFAULT NULL, "batch" TEXT DEFAULT NULL, "deadline_ms" bigint DEFAULT NULL) RETURNS double precision LANGUAGE sql AS $$SELECT NULL::double precision$$;
CREATE FUNCTION "thinkthen_score"("_question" TEXT, "_input" TEXT, "_members" TEXT[], "_context" TEXT) RETURNS double precision LANGUAGE sql AS $$SELECT NULL::double precision$$;
CREATE FUNCTION "thinkthen_score_many"("question" TEXT, "input" jsonb, "settings" json DEFAULT NULL) RETURNS TABLE ( "key" TEXT, "value" double precision ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::double precision WHERE false$$;
CREATE FUNCTION "thinkthen_tag"("question" TEXT, "input" TEXT, "members" TEXT[] DEFAULT NULL, "settings" json DEFAULT NULL, "threshold" TEXT DEFAULT NULL, "context" TEXT DEFAULT NULL, "model" TEXT DEFAULT NULL, "batch" TEXT DEFAULT NULL, "deadline_ms" bigint DEFAULT NULL) RETURNS TEXT[] LANGUAGE sql AS $$SELECT NULL::TEXT[]$$;
CREATE FUNCTION "thinkthen_tag"("_question" TEXT, "_input" TEXT, "_members" TEXT[], "_context" TEXT) RETURNS TEXT[] LANGUAGE sql AS $$SELECT NULL::TEXT[]$$;
CREATE FUNCTION "thinkthen_tag_many"("question" TEXT, "input" jsonb, "settings" json DEFAULT NULL) RETURNS TABLE ( "key" TEXT, "value" TEXT[] ) LANGUAGE sql AS $$SELECT NULL::TEXT, NULL::TEXT[] WHERE false$$;
CREATE FUNCTION "thinkthen_try_details"("question" TEXT, "input" TEXT, "settings" json DEFAULT NULL, "threshold" TEXT DEFAULT NULL, "context" TEXT DEFAULT NULL, "model" TEXT DEFAULT NULL, "batch" TEXT DEFAULT NULL, "deadline_ms" bigint DEFAULT NULL) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_try_details"("_question" TEXT, "_input" TEXT, "_context" TEXT) RETURNS jsonb LANGUAGE sql AS $$SELECT NULL::jsonb$$;
CREATE FUNCTION "thinkthen_usage"() RETURNS TABLE ( "requests_sent" bigint, "cache_answers" bigint, "input_tokens" bigint, "output_tokens" bigint ) LANGUAGE sql AS $$SELECT NULL::bigint, NULL::bigint, NULL::bigint, NULL::bigint WHERE false$$;
CREATE FUNCTION "thinkthen_warm"("_question" TEXT, "_input" TEXT) RETURNS bigint LANGUAGE sql AS $$SELECT NULL::bigint$$;
CREATE FUNCTION "thinkthen_warm"("_question" TEXT, "_input" TEXT, "_context" TEXT) RETURNS bigint LANGUAGE sql AS $$SELECT NULL::bigint$$;
CREATE FUNCTION thinkthen_guard_public() RETURNS event_trigger LANGUAGE plpgsql AS $$BEGIN END$$;
CREATE EVENT TRIGGER thinkthen_guard_public ON ddl_command_end
    WHEN TAG IN ('CREATE FUNCTION', 'CREATE PROCEDURE', 'CREATE AGGREGATE')
    EXECUTE FUNCTION thinkthen_guard_public();
