.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.');

SELECT names.position, names.id AS msg_id, names.status,
       found.value ->> '$.text' AS name, found.value ->> '$.kind' AS kind
FROM thinkthen_recognize('SELECT msg_id AS id, body AS record FROM messages') AS names
LEFT JOIN json_each(names.answer) AS found ON true
WHERE names.kinds = '["person", "organization"]'
ORDER BY names.position, found.key;
