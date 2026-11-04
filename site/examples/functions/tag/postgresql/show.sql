CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES
  (1, 'I was charged twice and my parcel is late again.');

SELECT id AS msg_id, answer AS topics, status
FROM thinkthen.tag('SELECT msg_id AS id, body AS record FROM messages',
                   'Which topics appear?',
                   labels => ARRAY['billing', 'shipping']);
