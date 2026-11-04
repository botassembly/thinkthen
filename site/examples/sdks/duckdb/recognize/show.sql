SELECT msg_id, found.text AS name, found.kind
FROM (
  SELECT msg_id, unnest(answer) AS found
  FROM thinkthen_recognize(
    (SELECT msg_id, body FROM messages WHERE msg_id = 6),
    kinds := ['person', 'organization'],
    record := 'body'));
