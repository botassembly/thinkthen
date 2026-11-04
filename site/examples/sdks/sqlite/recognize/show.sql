SELECT names.id AS msg_id, found.value ->> '$.text' AS name,
       found.value ->> '$.kind' AS kind
FROM thinkthen_recognize(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 6') AS names,
  json_each(names.answer) AS found
WHERE names.kinds = '["person", "organization"]';
