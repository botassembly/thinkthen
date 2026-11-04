.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.');

SELECT id AS msg_id, answer AS names, status
FROM thinkthen_recognize('SELECT msg_id AS id, body AS record FROM messages')
WHERE kinds = '["person", "organization"]';
