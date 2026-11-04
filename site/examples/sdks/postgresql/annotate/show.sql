SELECT id AS msg_id,
       (answer ->> 'refund')::boolean AS is_refund,
       answer ->> 'topic' AS topic,
       (answer ->> 'urgency')::float8 AS urgency
FROM thinkthen.annotate(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  toolbox => '{
    "refund": {"function": "decide",
      "question": "Does the customer request a refund?"},
    "topic": {"function": "choose",
      "question": "What is the message about?",
      "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score",
      "question": "How urgent is the request?",
      "levels": ["routine", "urgent"]}
  }');
