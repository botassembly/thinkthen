.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, NULL),
  (3, 'I was charged twice for one invoice.');

-- Keep every outcome when failed records must be inspected separately.
SELECT id AS msg_id, status, error_code
FROM thinkthen_decide(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
                      'Does the customer request a refund?');
