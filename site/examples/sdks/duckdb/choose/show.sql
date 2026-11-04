SELECT msg_id, answer AS topic, status
FROM thinkthen_choose(
  (SELECT msg_id, body FROM messages WHERE msg_id = 1),
  'What is the message about?',
  choices := ['billing', 'shipping', 'other'],
  record := 'body');
