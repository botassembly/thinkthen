CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.');

SELECT r.id AS msg_id, found.text AS name, found.kind, r.status
FROM thinkthen.recognize('SELECT msg_id AS id, body AS record FROM messages',
                         kinds => ARRAY['person', 'organization']) AS r
LEFT JOIN LATERAL unnest(r.answer) AS found ON true;
