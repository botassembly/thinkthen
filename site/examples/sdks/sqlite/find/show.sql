SELECT id AS msg_id,
       answer ->> '$.text' AS requested_action, status
FROM thinkthen_find(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 5',
  'Which line states the requested action?');
