LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Please refund my order.');

SELECT msg_id, answer AS is_refund, status
FROM thinkthen_decide((SELECT msg_id, body FROM messages),
                      'Does the customer request a refund?', record := 'body');
