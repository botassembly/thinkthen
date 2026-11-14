SELECT answer AS urgency_rank, id AS msg_id, status
FROM thinkthen_rank(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'How urgent is the request?')
WHERE levels = '["routine", "urgent"]'
ORDER BY urgency_rank NULLS LAST;
