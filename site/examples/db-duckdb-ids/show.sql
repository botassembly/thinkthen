LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE inbox AS
SELECT '{"id": "a1", "text": "Please refund my order."}'::JSON AS record
UNION ALL SELECT '{"id": "a2", "text": "Where is my parcel?"}'::JSON
UNION ALL SELECT
  '{"id": "a3", "text": "I was charged twice for one invoice."}'::JSON;

SELECT id->>'$' AS ticket_id, answer AS is_refund, status
FROM thinkthen_decide((SELECT record FROM inbox),
                      'Does the customer request a refund?',
                      field := '/text');
