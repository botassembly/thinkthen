.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

CREATE TABLE message_topics AS
SELECT id AS msg_id, answer AS topic, status, error_code
FROM thinkthen_choose(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
                      'What is the message about?')
WHERE choices = '["billing", "shipping", "other"]';

SELECT msg_id, topic, status FROM message_topics ORDER BY msg_id;
SELECT topic, count(*) AS messages
FROM message_topics WHERE status = 'answered' GROUP BY topic ORDER BY topic;
