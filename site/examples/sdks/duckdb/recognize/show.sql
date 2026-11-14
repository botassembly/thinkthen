SELECT names.position, names.id, names.msg_id, names.status,
       found.name.text AS name, found.name.kind AS kind
FROM thinkthen_recognize((SELECT msg_id, body FROM messages WHERE msg_id = 6),
                   kinds := ['person', 'organization'], record := 'body') AS names
LEFT JOIN LATERAL (SELECT unnest(names.answer) AS name,
                          generate_subscripts(names.answer, 1) AS answer_position) AS found ON true
ORDER BY names.position, found.answer_position;
