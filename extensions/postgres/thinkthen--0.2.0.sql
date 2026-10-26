CREATE TYPE thinkthen.status AS ENUM (@STATUSES@);

CREATE TYPE thinkthen.error_code AS ENUM (@CODES@);

CREATE TYPE thinkthen.source AS (question_identity text, name text, backend text, model text);

CREATE TYPE thinkthen.rule AS (name text, source text, target text, reads text, either boolean);

CREATE TYPE thinkthen.endpoint AS (text text, start bigint, "end" bigint, kind text);

CREATE TYPE thinkthen.name AS (text text, start bigint, "end" bigint, kind text, probability float8);

CREATE TYPE thinkthen.relation AS (source thinkthen.endpoint, target thinkthen.endpoint, relation text, probability float8, either boolean);

CREATE FUNCTION thinkthen.decide(source_sql text, question text, threshold float8 DEFAULT NULL, low float8 DEFAULT NULL, high float8 DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer boolean)
AS 'MODULE_PATHNAME', 'thinkthen_decide' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.decide(text, text, float8, float8, float8, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.choose(source_sql text, question text, choices text[], threshold float8 DEFAULT NULL, low float8 DEFAULT NULL, high float8 DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer text)
AS 'MODULE_PATHNAME', 'thinkthen_choose' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.choose(text, text, text[], float8, float8, float8, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.tag(source_sql text, question text, labels text[], threshold float8 DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer text[])
AS 'MODULE_PATHNAME', 'thinkthen_tag' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.tag(text, text, text[], float8, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.score(source_sql text, question text, levels text[], evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer float8)
AS 'MODULE_PATHNAME', 'thinkthen_score' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.score(text, text, text[], text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.filter(source_sql text, question text, threshold float8 DEFAULT NULL, low float8 DEFAULT NULL, high float8 DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer boolean, kept boolean)
AS 'MODULE_PATHNAME', 'thinkthen_filter' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.filter(text, text, float8, float8, float8, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.rank(source_sql text, question text, levels text[], top bigint DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, rank bigint)
AS 'MODULE_PATHNAME', 'thinkthen_rank' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.rank(text, text, text[], bigint, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.find(source_sql text, question text, threshold float8 DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, found_text text, found_start bigint, found_end bigint)
AS 'MODULE_PATHNAME', 'thinkthen_find' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.find(text, text, float8, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.recognize(source_sql text, kinds text[], threshold float8 DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer thinkthen.name[])
AS 'MODULE_PATHNAME', 'thinkthen_recognize' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.recognize(text, text[], float8, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.relate(source_sql text, rules thinkthen.rule[], threshold float8 DEFAULT NULL, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer thinkthen.relation[])
AS 'MODULE_PATHNAME', 'thinkthen_relate' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.relate(text, thinkthen.rule[], float8, text, text, jsonb, name, name) FROM PUBLIC;

CREATE FUNCTION thinkthen.annotate(source_sql text, toolbox jsonb, evidence text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, status thinkthen.status, probability float8, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb, answer jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_annotate' LANGUAGE C VOLATILE PARALLEL UNSAFE;

REVOKE EXECUTE ON FUNCTION thinkthen.annotate(text, jsonb, text, text, jsonb, name, name) FROM PUBLIC;
