SELECT id AS msg_id, answer AS urgency, status
FROM thinkthen_score(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'How urgent is the request?')
WHERE levels = '["routine", "urgent"]';
