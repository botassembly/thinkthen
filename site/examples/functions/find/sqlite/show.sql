.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT id AS msg_id, answer ->> '$.text' AS requested_action, status
FROM thinkthen_find('SELECT msg_id AS id, body AS record FROM messages',
                    'Which line states the requested action?');
