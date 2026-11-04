.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

SELECT answer AS urgency_rank, id AS msg_id, status
FROM thinkthen_rank(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
  'How urgent is the request?')
WHERE levels = '["routine", "urgent"]'
ORDER BY urgency_rank NULLS LAST;
