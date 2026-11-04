SELECT msg_id, answer AS is_refund, status
FROM thinkthen_decide(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'Does the customer request a refund?',
  record := 'body', low := 0.2, high := 0.8);
