SELECT names.position, names.id AS msg_id, names.status,
       found.text AS name, found.kind
FROM thinkthen.recognize('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 6',
                   kinds => ARRAY['person', 'organization']) AS names
LEFT JOIN LATERAL unnest(names.answer) WITH ORDINALITY AS found ON true
ORDER BY names.position, found.ordinality;
