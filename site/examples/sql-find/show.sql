LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT msg_id, answer.text AS requested_action, status
FROM thinkthen_find((SELECT msg_id, body FROM messages),
                    'Which line states the requested action?',
                    record := 'body');
