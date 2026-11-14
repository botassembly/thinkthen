LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
    (1, 'Please refund my order.'),
    (2, 'Where is my parcel?'),
    (3, 'I was charged twice for one invoice.');

SELECT answer AS urgency_rank, msg_id, body, status
FROM thinkthen_rank((SELECT msg_id, body FROM messages),
                    'How urgent is the request?',
                    levels := ['routine', 'urgent'], record := 'body')
ORDER BY urgency_rank NULLS LAST;
