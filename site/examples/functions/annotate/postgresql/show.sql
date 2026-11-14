CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

SELECT id AS msg_id,
       CASE WHEN status = 'answered' AND detail -> 'refund' ->> 'status' = 'answered'
            THEN (answer ->> 'refund')::boolean END AS is_refund,
       CASE WHEN status = 'answered' AND detail -> 'topic' ->> 'status' = 'answered'
            THEN answer ->> 'topic' END AS topic,
       CASE WHEN status = 'answered' AND detail -> 'urgency' ->> 'status' = 'answered'
            THEN (answer ->> 'urgency')::float8 END AS urgency,
       detail -> 'refund' ->> 'status' AS refund_status,
       detail -> 'topic' ->> 'status' AS topic_status,
       detail -> 'urgency' ->> 'status' AS urgency_status, status
FROM thinkthen.annotate(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id',
  questions => '{
    "refund": {"function": "decide",
               "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }');
