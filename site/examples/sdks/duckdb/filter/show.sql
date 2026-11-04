SELECT msg_id, body
FROM thinkthen_filter(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'Does the customer request a refund?',
  record := 'body');
