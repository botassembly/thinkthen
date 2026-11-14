SELECT answer AS urgency_rank, id AS msg_id, status
FROM thinkthen.rank(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'How urgent is the request?',
  levels => ARRAY['routine', 'urgent'])
ORDER BY urgency_rank NULLS LAST;
