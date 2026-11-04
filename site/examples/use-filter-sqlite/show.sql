.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

CREATE TABLE refund_requests AS
SELECT id AS msg_id
FROM thinkthen_filter(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
                      'Does the customer request a refund?');

-- The kept messages, with their text.
SELECT msg_id, body
FROM messages JOIN refund_requests USING (msg_id)
ORDER BY msg_id;
