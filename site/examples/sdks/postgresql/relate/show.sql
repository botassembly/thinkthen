SELECT relations.position, relations.id AS msg_id, relations.status,
       (link.source).text AS person, link.relation,
       (link.target).text AS organization
FROM thinkthen.relate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5',
                   rules => ARRAY[ROW('person', 'organization', 'works_for', NULL, false)::thinkthen.rule]) AS relations
LEFT JOIN LATERAL unnest(relations.answer) WITH ORDINALITY AS link ON true
ORDER BY relations.position, link.ordinality;
