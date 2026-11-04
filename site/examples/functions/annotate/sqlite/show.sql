.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode json
CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

SELECT id AS msg_id, answer ->> '$.refund' AS is_refund,
       answer ->> '$.topic' AS topic, answer ->> '$.urgency' AS urgency, status
FROM thinkthen_annotate(
  'SELECT msg_id AS id, body AS record FROM messages ORDER BY msg_id')
WHERE toolbox = '{
  "refund": {"function": "decide",
             "question": "Does the customer request a refund?"},
  "topic": {"function": "choose", "question": "What is the message about?",
            "choices": ["billing", "shipping", "other"]},
  "urgency": {"function": "score", "question": "How urgent is the request?",
              "levels": ["routine", "urgent"]}
}';
