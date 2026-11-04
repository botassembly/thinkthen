LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.');

SELECT msg_id, answer AS names, status
FROM thinkthen_recognize((SELECT msg_id, body FROM messages),
                         kinds := ['person', 'organization'], record := 'body');
