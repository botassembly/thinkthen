SELECT msg_id, answer.text AS requested_action, status
FROM thinkthen_find(
  (SELECT msg_id, body FROM messages WHERE msg_id = 5),
  'Which line states the requested action?',
  record := 'body');
