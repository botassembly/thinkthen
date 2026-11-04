LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Please refund my order.');

SELECT msg_id, answer AS urgency, status
FROM thinkthen_score((SELECT msg_id, body FROM messages),
                     'How urgent is the request?',
                     levels := ['routine', 'urgent'], record := 'body');
