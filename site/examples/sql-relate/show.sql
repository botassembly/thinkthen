LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT msg_id, answer AS works_for, status
FROM thinkthen_relate((SELECT msg_id, body FROM messages),
                      rules := [{'relation': 'works_for',
                                 'source': 'person',
                                 'target': 'organization'}],
                      record := 'body');
