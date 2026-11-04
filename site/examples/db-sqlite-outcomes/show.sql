.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

CREATE TABLE refund_answers AS
SELECT id AS msg_id, answer AS is_refund, status, error_code
FROM thinkthen_decide('SELECT msg_id AS id, body AS record FROM messages',
                      'Does the customer request a refund?');

SELECT msg_id FROM refund_answers WHERE is_refund IS TRUE;
SELECT msg_id FROM refund_answers WHERE status = 'unsure';
SELECT msg_id, error_code FROM refund_answers WHERE status = 'failed';
