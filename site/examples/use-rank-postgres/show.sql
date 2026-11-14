CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

-- Rank every message from most to least urgent, and keep the top two.
SELECT r.answer AS urgency_rank, m.msg_id, m.body, r.status
FROM thinkthen.rank(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
                    'How urgent is the request?',
                    levels => ARRAY['routine', 'urgent'], top => 2) AS r
JOIN messages AS m ON m.msg_id = (r.id)::integer
ORDER BY urgency_rank NULLS LAST;
