.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES
  (1, 'I was charged twice and my parcel is late again.');

SELECT id AS msg_id, answer AS topics, status
FROM thinkthen_tag('SELECT msg_id AS id, body AS record FROM messages',
                   'Which topics appear?')
WHERE labels = '["billing", "shipping"]';
