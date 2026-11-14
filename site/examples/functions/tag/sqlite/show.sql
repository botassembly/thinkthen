.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES
  (1, 'I was charged twice and my parcel is late again.');

SELECT topics.position, topics.id AS msg_id, topics.status,
       label.value AS topic
FROM thinkthen_tag('SELECT msg_id AS id, body AS record FROM messages',
                   'Which topics appear?') AS topics
LEFT JOIN json_each(topics.answer) AS label ON true
WHERE topics.labels = '["billing", "shipping"]'
ORDER BY topics.position, label.key;
