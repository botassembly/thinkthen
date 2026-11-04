SELECT id AS msg_id, answer AS is_refund, probability
FROM thinkthen.decide(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'Does the customer request a refund?');
