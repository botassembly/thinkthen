SELECT id AS msg_id, answer AS is_refund, status, error_code
FROM thinkthen.decide(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id = 1
   UNION ALL SELECT 7, NULL',
  'Does the customer request a refund?');
