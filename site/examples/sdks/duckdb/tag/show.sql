SELECT msg_id, answer AS topics, status
FROM thinkthen_tag(
  (SELECT msg_id, body FROM messages WHERE msg_id = 4),
  'Which topics appear?',
  labels := ['billing', 'shipping'],
  record := 'body');
