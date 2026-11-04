LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
    (1, 'Please refund my order.'),
    (2, 'Where is my parcel?'),
    (3, 'I was charged twice for one invoice.');

CREATE TABLE refund_answers AS
SELECT msg_id, answer AS is_refund, status, error_code
FROM thinkthen_decide((SELECT msg_id, body FROM messages),
                      'Does the customer request a refund?', record := 'body');

SELECT msg_id FROM refund_answers WHERE is_refund IS TRUE;
SELECT msg_id FROM refund_answers WHERE status = 'unsure';
SELECT msg_id, error_code FROM refund_answers WHERE status = 'failed';
