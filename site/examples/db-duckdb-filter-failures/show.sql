LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
    (1, 'Please refund my order.'),
    (2, 'Where is my parcel?'),
    (3, 'I was charged twice for one invoice.');

SELECT msg_id, body, status, error_code
FROM thinkthen_filter((SELECT msg_id, body FROM messages),
                      'Does the customer request a refund?', record := 'body',
                      failures := 'row');
