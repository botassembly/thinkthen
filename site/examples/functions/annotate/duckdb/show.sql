LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
    (1, 'Please refund my order.'),
    (2, 'Where is my parcel?'),
    (3, 'I was charged twice for one invoice.');

SELECT msg_id, answer.refund AS is_refund, answer.topic AS topic,
       answer.urgency AS urgency, status
FROM thinkthen_annotate((SELECT msg_id, body FROM messages), record := 'body',
  toolbox := '{
    "refund": {"function": "decide",
               "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }'::JSON);
