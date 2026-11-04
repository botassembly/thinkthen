CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT id AS msg_id, (answer).text AS requested_action, status
FROM thinkthen.find('SELECT msg_id AS id, body AS record FROM messages',
                    'Which line states the requested action?');
