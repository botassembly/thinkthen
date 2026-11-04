SELECT msg_id, answer AS urgency, status
FROM thinkthen_score(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'How urgent is the request?',
  levels := ['routine', 'urgent'],
  record := 'body');
