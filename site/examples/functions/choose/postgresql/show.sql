CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES (1, 'Please refund my order.');

SELECT id AS msg_id, answer AS topic, status
FROM thinkthen.choose('SELECT msg_id AS id, body AS record FROM messages',
                      'What is the message about?',
                      choices => ARRAY['billing', 'shipping', 'other']);
