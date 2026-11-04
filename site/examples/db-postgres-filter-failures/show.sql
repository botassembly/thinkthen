CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, NULL),
  (3, 'I was charged twice for one invoice.');

SELECT id AS msg_id, status, error_code
FROM thinkthen.filter(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
                      'Does the customer request a refund?', failures => 'row');
