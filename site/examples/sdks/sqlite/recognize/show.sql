SELECT names.position, names.id AS msg_id, names.status,
       found.value ->> '$.text' AS name, found.value ->> '$.kind' AS kind
FROM thinkthen_recognize('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 6') AS names
LEFT JOIN json_each(names.answer) AS found ON true
WHERE names.kinds = '["person", "organization"]'
ORDER BY names.position, found.key;
