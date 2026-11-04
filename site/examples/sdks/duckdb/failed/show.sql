SELECT msg_id, answer AS is_refund, status, error_code
FROM thinkthen_decide(
  (SELECT msg_id, body FROM messages WHERE msg_id = 1
   UNION ALL SELECT 7, NULL),
  'Does the customer request a refund?',
  record := 'body');
