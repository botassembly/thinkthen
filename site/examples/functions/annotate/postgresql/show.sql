CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

SELECT id AS msg_id, (answer ->> 'refund')::boolean AS is_refund,
       answer ->> 'topic' AS topic, (answer ->> 'urgency')::float8 AS urgency,
       status
FROM thinkthen.annotate(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
  toolbox => '{
    "refund": {"function": "decide",
               "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }');
