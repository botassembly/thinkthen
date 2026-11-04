SELECT answer AS urgency_rank, msg_id, body
FROM thinkthen_rank(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'How urgent is the request?',
  levels := ['routine', 'urgent'],
  record := 'body')
ORDER BY urgency_rank;
