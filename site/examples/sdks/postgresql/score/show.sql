SELECT id AS msg_id, answer AS urgency, status
FROM thinkthen.score(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'How urgent is the request?',
  levels => ARRAY['routine', 'urgent']);
