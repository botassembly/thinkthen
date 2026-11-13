LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.'),
  (4, 'I was charged twice and my parcel is late again.'),
  (5, 'Morgan works for Example Group.
Please refund my order.'),
  (6, 'Morgan works for Example Group.');

SELECT msg_id, answer AS is_refund, status
FROM thinkthen_decide(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'Does the customer request a refund?',
  record := 'body');
