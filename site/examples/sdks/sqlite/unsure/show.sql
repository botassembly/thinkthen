SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'Does the customer request a refund?')
WHERE low = 0.2 AND high = 0.8;
