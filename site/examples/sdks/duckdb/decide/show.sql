SELECT msg_id, answer AS is_refund, probability
FROM thinkthen_decide(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'Does the customer request a refund?',
  record := 'body');
