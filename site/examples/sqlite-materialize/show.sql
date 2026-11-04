.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(
  msg_id TEXT PRIMARY KEY, customer TEXT NOT NULL, body TEXT NOT NULL);
INSERT INTO messages VALUES
  ('a1', 'Ana', 'Please refund my order.'),
  ('a2', 'Ben', 'Where is my parcel?'),
  ('a3', 'Ana', 'I was charged twice for one invoice.');

-- Ask once, and keep the answers.
CREATE TEMP TABLE refund_answers AS
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide(
  'SELECT msg_id AS id, body AS record FROM messages',
                      'Does the customer request a refund?');

-- Join the saved answers as often as you like. No question is sent again.
SELECT m.customer, m.msg_id, r.is_refund, r.status
FROM messages AS m JOIN refund_answers AS r USING (msg_id)
ORDER BY m.msg_id;
