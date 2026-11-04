SELECT msg_id, link.source.text AS person, link.relation,
       link.target.text AS organization
FROM (
  SELECT msg_id, unnest(answer) AS link
  FROM thinkthen_relate(
    (SELECT msg_id, body FROM messages WHERE msg_id = 5),
    rules := [{'relation': 'works_for',
               'source': 'person',
               'target': 'organization'}],
    record := 'body'));
