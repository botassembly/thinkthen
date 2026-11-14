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
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  questions => '{
    "refund": {"function": "decide",
      "question": "Does the customer request a refund?"},
    "topic": {"function": "choose",
      "question": "What is the message about?",
      "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score",
      "question": "How urgent is the request?",
      "levels": ["routine", "urgent"]}
  }');
