\echo Use "CREATE EXTENSION thinkthen" to load this file. \quit

CREATE TYPE thinkthen.status AS ENUM ('answered', 'unsure', 'failed');
COMMENT ON TYPE thinkthen.status IS 'How a record ended: answered, unsure (answer is NULL) or failed (answer is NULL and error_code says why).';

CREATE TYPE thinkthen.error_code AS ENUM ('usage.threshold', 'usage.question', 'usage.name', 'usage.settings', 'usage.backend', 'usage.url', 'usage.key-missing', 'usage.batch', 'usage.pointer', 'usage.sql-policy', 'usage.sql-selection', 'usage.abi', 'usage.closed', 'usage.function', 'usage.record-kind', 'usage.offsets', 'usage.argument', 'usage.option', 'usage.column', 'usage.sheet', 'usage.output', 'usage.table', 'record.json', 'record.missing', 'record.too-large', 'record.invalid', 'record.empty', 'backend.transport', 'backend.status', 'backend.reply', 'local.cancelled', 'local.io', 'local.toolbox', 'local.accounting', 'local.request-limit', 'internal.bug');
COMMENT ON TYPE thinkthen.error_code IS 'The engine''s error codes. A failed row carries one in error_code; a raised error carries one in DETAIL.';

CREATE TYPE thinkthen.source AS (question_identity text, name text, backend text, model text);
COMMENT ON TYPE thinkthen.source IS 'Which question, name, backend and model produced a row.';

CREATE TYPE thinkthen.rule AS (relation text, source text, target text, reads text, either boolean);
COMMENT ON TYPE thinkthen.rule IS 'A relation for relate to look for: its relation name, the source and target kinds, optional reading text, and whether either direction counts.';

CREATE TYPE thinkthen.span AS (text text, start_offset bigint, end_offset bigint);
COMMENT ON TYPE thinkthen.span IS 'Matched text and where it starts and ends, in characters from 0, end exclusive.';

CREATE TYPE thinkthen.recognized_name AS (text text, kind text, start_offset bigint, end_offset bigint, probability float8);
COMMENT ON TYPE thinkthen.recognized_name IS 'A name recognize found: its text, kind, character offsets (end exclusive) and probability.';

CREATE TYPE thinkthen.endpoint AS (text text, kind text, start_offset bigint, end_offset bigint);
COMMENT ON TYPE thinkthen.endpoint IS 'One end of a relation: its text, kind and character offsets (end exclusive).';

CREATE TYPE thinkthen.relation AS (source thinkthen.endpoint, target thinkthen.endpoint, relation text, probability float8, either boolean);
COMMENT ON TYPE thinkthen.relation IS 'A relation relate found between two names.';

CREATE FUNCTION thinkthen.decide(source_sql text, question text, threshold float8 DEFAULT NULL, low float8 DEFAULT NULL, high float8 DEFAULT NULL, field text DEFAULT NULL, yes_means text DEFAULT NULL, no_means text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer boolean, probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_decide' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.decide(text, text, float8, float8, float8, text, text, text, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.decide(text, text, float8, float8, float8, text, text, text, text, jsonb, name, name) IS 'Answer a yes-or-no question about each record of source_sql. answer is true or false, and NULL when unsure or failed, so test it with IS TRUE or IS FALSE. Example: SELECT id, answer AS is_refund FROM thinkthen.decide($$SELECT id, body AS record FROM tickets$$, ''Does the customer ask for a refund?'')';

CREATE FUNCTION thinkthen.choose(source_sql text, question text, choices text[], threshold float8 DEFAULT NULL, field text DEFAULT NULL, descriptions jsonb DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer text, probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_choose' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.choose(text, text, text[], float8, text, jsonb, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.choose(text, text, text[], float8, text, jsonb, text, jsonb, name, name) IS 'Pick one of choices for each record of source_sql. answer is the choice, NULL when unsure or failed. Example: SELECT id, answer AS team FROM thinkthen.choose($$SELECT id, body AS record FROM tickets$$, ''Which team owns this?'', choices => ARRAY[''billing'',''shipping''])';

CREATE FUNCTION thinkthen.tag(source_sql text, question text, labels text[], threshold float8 DEFAULT NULL, field text DEFAULT NULL, descriptions jsonb DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer text[], probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_tag' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.tag(text, text, text[], float8, text, jsonb, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.tag(text, text, text[], float8, text, jsonb, text, jsonb, name, name) IS 'Pick every label that applies to each record of source_sql. answer is the array of labels, NULL when unsure or failed. Example: SELECT id, answer AS topics FROM thinkthen.tag($$SELECT id, body AS record FROM tickets$$, ''Which topics apply?'', labels => ARRAY[''refund'',''delivery''])';

CREATE FUNCTION thinkthen.score(source_sql text, question text, levels text[], field text DEFAULT NULL, descriptions jsonb DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer float8, probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_score' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.score(text, text, text[], text, jsonb, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.score(text, text, text[], text, jsonb, text, jsonb, name, name) IS 'Score each record of source_sql from 0 (the first level) to 1 (the last level). answer is the score, NULL when failed. Example: SELECT id, answer AS urgency FROM thinkthen.score($$SELECT id, body AS record FROM tickets$$, ''How urgent is this?'', levels => ARRAY[''low'',''high''])';

CREATE FUNCTION thinkthen.filter(source_sql text, question text, threshold float8 DEFAULT NULL, low float8 DEFAULT NULL, high float8 DEFAULT NULL, field text DEFAULT NULL, yes_means text DEFAULT NULL, no_means text DEFAULT NULL, failures text DEFAULT 'raise', name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer boolean, probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_filter' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.filter(text, text, float8, float8, float8, text, text, text, text, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.filter(text, text, float8, float8, float8, text, text, text, text, text, jsonb, name, name) IS 'Return only the records of source_sql for which the answer is yes. Unsure and no records are left out. If any record fails, the statement raises one error after every record is judged; failures => ''row'' returns the failed rows too, with status failed. Example: SELECT id FROM thinkthen.filter($$SELECT id, body AS record FROM tickets$$, ''Does the customer ask for a refund?'')';

CREATE FUNCTION thinkthen.rank(source_sql text, question text, levels text[], top bigint DEFAULT NULL, field text DEFAULT NULL, descriptions jsonb DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer bigint, probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_rank' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.rank(text, text, text[], bigint, text, jsonb, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.rank(text, text, text[], bigint, text, jsonb, text, jsonb, name, name) IS 'Rank the records of source_sql from the last level down; answer 1 is the highest. top keeps only that many answered rows. Failed rows follow in input order. Example: SELECT id, answer AS urgency_rank FROM thinkthen.rank($$SELECT id, body AS record FROM tickets$$, ''How urgent is this?'', levels => ARRAY[''low'',''high''], top => 3)';

CREATE FUNCTION thinkthen.find(source_sql text, question text, threshold float8 DEFAULT NULL, field text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer thinkthen.span, probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_find' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.find(text, text, float8, text, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.find(text, text, float8, text, text, jsonb, name, name) IS 'Find the line of each record of source_sql that answers the question. answer is the matched span, NULL when no line answers (status answered), when unsure, or when failed. Example: SELECT id, (answer).text AS meeting_line FROM thinkthen.find($$SELECT id, body AS record FROM notes$$, ''When is the meeting?'')';

CREATE FUNCTION thinkthen.recognize(source_sql text, kinds text[], threshold float8 DEFAULT NULL, field text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer thinkthen.recognized_name[], probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_recognize' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.recognize(text, text[], float8, text, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.recognize(text, text[], float8, text, text, jsonb, name, name) IS 'Recognize names of the given kinds in each record of source_sql. answer is the array of names found. Example: SELECT id, n.text AS person FROM thinkthen.recognize($$SELECT id, body AS record FROM notes$$, kinds => ARRAY[''person'']) r, unnest(r.answer) n';

CREATE FUNCTION thinkthen.relate(source_sql text, rules thinkthen.rule[], threshold float8 DEFAULT NULL, field text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer thinkthen.relation[], probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_relate' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.relate(text, thinkthen.rule[], float8, text, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.relate(text, thinkthen.rule[], float8, text, text, jsonb, name, name) IS 'Find the relations named by rules in each record of source_sql. answer is the array of relations found. Example: SELECT id, answer AS employment FROM thinkthen.relate($$SELECT id, body AS record FROM notes$$, rules => ARRAY[ROW(''works_for'',''person'',''organization'',NULL,false)::thinkthen.rule])';

CREATE FUNCTION thinkthen.annotate(source_sql text, toolbox jsonb, field text DEFAULT NULL, name text DEFAULT NULL, params jsonb DEFAULT '[]'::jsonb, record_column name DEFAULT 'record', id_column name DEFAULT NULL)
RETURNS TABLE("position" bigint, id jsonb, answer jsonb, probability float8, status thinkthen.status, error_code thinkthen.error_code, error_message text, error_http_status integer, source thinkthen.source, detail jsonb)
AS 'MODULE_PATHNAME', 'thinkthen_pg_annotate' LANGUAGE C VOLATILE PARALLEL UNSAFE;
REVOKE EXECUTE ON FUNCTION thinkthen.annotate(text, jsonb, text, text, jsonb, name, name) FROM PUBLIC;
COMMENT ON FUNCTION thinkthen.annotate(text, jsonb, text, text, jsonb, name, name) IS 'Ask every question in toolbox about each record of source_sql. answer maps each question''s name to its bare answer, JSON null when that question is unsure or failed; detail holds each question''s full outcome. Example: SELECT id, (answer->''refund'')::boolean AS is_refund FROM thinkthen.annotate($$SELECT id, body AS record FROM tickets$$, toolbox => ''{"refund": {"function": "decide", "question": "Does the customer ask for a refund?"}}'')';
