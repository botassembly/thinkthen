CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

SELECT msg_id, body
FROM messages
WHERE to_jsonb(msg_id) IN (SELECT id FROM thinkthen.filter(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
  'Does the customer request a refund?'))
ORDER BY msg_id;
