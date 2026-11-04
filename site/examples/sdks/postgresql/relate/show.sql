SELECT links.id AS msg_id, (link.source).text AS person,
       link.relation, (link.target).text AS organization
FROM thinkthen.relate(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 5',
  rules => ARRAY[ROW('works_for', 'person', 'organization',
                     NULL, false)::thinkthen.rule]) AS links
LEFT JOIN LATERAL unnest(links.answer) AS link ON true;
