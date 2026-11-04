LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Please refund my order.');

-- Ask once and keep every row: answered, not sure or failed.
CREATE TABLE refund_answers AS
SELECT msg_id, answer AS is_refund, status, probability, error_code
FROM thinkthen_decide((SELECT msg_id, body FROM messages),
                      'Does the customer request a refund?', record := 'body',
                      low := 0.3, high := 0.7);

SELECT msg_id, is_refund, status, probability FROM refund_answers;
SELECT msg_id FROM refund_answers WHERE is_refund IS TRUE;
SELECT msg_id FROM refund_answers WHERE status = 'unsure';
SELECT msg_id, error_code FROM refund_answers WHERE status = 'failed';
