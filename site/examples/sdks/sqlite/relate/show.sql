SELECT relations.position, relations.id AS msg_id, relations.status,
       link.value ->> '$.source.text' AS person, link.value ->> '$.relation' AS relation,
       link.value ->> '$.target.text' AS organization
FROM thinkthen_relate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5') AS relations
LEFT JOIN json_each(relations.answer) AS link ON true
WHERE relations.rules = '[{"source": "person", "target": "organization", "relation": "works_for"}]'
ORDER BY relations.position, link.key;
