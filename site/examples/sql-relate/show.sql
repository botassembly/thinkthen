LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT relations.position, relations.id, relations.msg_id, relations.status,
       link.relation.source.text AS person, link.relation.relation AS relation,
       link.relation.target.text AS organization
FROM thinkthen_relate((SELECT msg_id, body FROM messages),
                   rules := [{'source': 'person', 'target': 'organization', 'relation': 'works_for'}], record := 'body') AS relations
LEFT JOIN LATERAL (SELECT unnest(relations.answer) AS relation,
                          generate_subscripts(relations.answer, 1) AS answer_position) AS link ON true
ORDER BY relations.position, link.answer_position;
