.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES (1, 'Please refund my order.');

SELECT id AS msg_id, answer AS topic, status
FROM thinkthen_choose('SELECT msg_id AS id, body AS record FROM messages',
                      'What is the message about?')
WHERE choices = '["billing", "shipping", "other"]';
