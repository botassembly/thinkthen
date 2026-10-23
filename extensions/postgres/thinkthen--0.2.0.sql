CREATE FUNCTION thinkthen_decide(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_decide' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_choose(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_choose' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_tag(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_tag' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_score(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_score' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_filter(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_filter' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_rank(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_rank' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_find(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_find' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_annotate(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_annotate' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_recognize(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_recognize' LANGUAGE C VOLATILE PARALLEL UNSAFE;
CREATE FUNCTION thinkthen_relate(source_sql text, question text, options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json, p double precision, error json, detail json, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_relate' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen_decide(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_choose(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_tag(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_score(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_filter(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_rank(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_find(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_annotate(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_recognize(text, text, json, json) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION thinkthen_relate(text, text, json, json) FROM PUBLIC;
