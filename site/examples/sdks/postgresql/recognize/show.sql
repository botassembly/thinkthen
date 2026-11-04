SELECT names.id AS msg_id, found.text AS name, found.kind
FROM thinkthen.recognize(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 6',
  kinds => ARRAY['person', 'organization']) AS names
LEFT JOIN LATERAL unnest(names.answer) AS found ON true;
