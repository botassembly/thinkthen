LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
    (1, 'Please refund my order.'),
    (2, NULL),
    (3, 'I was charged twice for one invoice.');

-- Keep every outcome when failed records must be inspected separately.
SELECT msg_id, body, status, error_code
FROM thinkthen_decide((SELECT msg_id, body FROM messages),
                      'Does the customer request a refund?', record := 'body');
