CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.');

SELECT names.position, names.id AS msg_id, names.status,
       found.text AS name, found.kind
FROM thinkthen.recognize('SELECT msg_id AS id, body AS record FROM messages',
                   kinds => ARRAY['person', 'organization']) AS names
LEFT JOIN LATERAL unnest(names.answer) WITH ORDINALITY AS found ON true
ORDER BY names.position, found.ordinality;
