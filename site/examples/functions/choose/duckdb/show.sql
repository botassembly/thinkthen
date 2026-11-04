LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Please refund my order.');

SELECT msg_id, answer AS topic, status
FROM thinkthen_choose((SELECT msg_id, body FROM messages),
                      'What is the message about?',
                      choices := ['billing', 'shipping', 'other'],
                      record := 'body');
