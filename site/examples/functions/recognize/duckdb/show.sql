LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.');

SELECT names.position, names.id, names.msg_id, names.status,
       found.name.text AS name, found.name.kind AS kind
FROM thinkthen_recognize((SELECT msg_id, body FROM messages),
                   kinds := ['person', 'organization'], record := 'body') AS names
LEFT JOIN LATERAL (SELECT unnest(names.answer) AS name,
                          generate_subscripts(names.answer, 1) AS answer_position) AS found ON true
ORDER BY names.position, found.answer_position;
