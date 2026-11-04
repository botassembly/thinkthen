SELECT id AS msg_id, answer AS topic, status
FROM thinkthen_choose(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 1',
  'What is the message about?')
WHERE choices = '["billing", "shipping", "other"]';
