SELECT id AS msg_id, answer AS topics, status
FROM thinkthen_tag(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 4',
  'Which topics appear?')
WHERE labels = '["billing", "shipping"]';
