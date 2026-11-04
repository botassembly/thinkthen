.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES (1, 'Please refund my order.');

SELECT id AS msg_id, answer AS urgency, status
FROM thinkthen_score('SELECT msg_id AS id, body AS record FROM messages',
                     'How urgent is the request?')
WHERE levels = '["routine", "urgent"]';
