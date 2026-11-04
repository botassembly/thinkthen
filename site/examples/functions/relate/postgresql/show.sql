CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT r.id AS msg_id, (link.source).text AS person, link.relation,
       (link.target).text AS organization, r.status
FROM thinkthen.relate(
  'SELECT msg_id AS id, body AS record FROM messages',
  rules => ARRAY[
    ROW('works_for', 'person', 'organization', NULL, false)::thinkthen.rule
  ]) AS r
LEFT JOIN LATERAL unnest(r.answer) AS link ON true;
