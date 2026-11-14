CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT relations.position, relations.id AS msg_id, relations.status,
       (link.source).text AS person, link.relation,
       (link.target).text AS organization
FROM thinkthen.relate('SELECT msg_id AS id, body AS record FROM messages',
                   rules => ARRAY[ROW('person', 'organization', 'works_for', NULL, false)::thinkthen.rule]) AS relations
LEFT JOIN LATERAL unnest(relations.answer) WITH ORDINALITY AS link ON true
ORDER BY relations.position, link.ordinality;
