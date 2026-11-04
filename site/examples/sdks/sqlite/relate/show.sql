SELECT links.id AS msg_id,
       link.value ->> '$.source.text' AS person,
       link.value ->> '$.relation' AS relation,
       link.value ->> '$.target.text' AS organization
FROM thinkthen_relate(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 5') AS links,
  json_each(links.answer) AS link
WHERE links.rules = '[{"relation": "works_for",
  "source": "person", "target": "organization"}]';
