SELECT id AS msg_id, status
FROM thinkthen_filter(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'Does the customer request a refund?');
