CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.'),
  (4, 'I was charged twice and my parcel is late again.'),
  (5, 'Morgan works for Example Group.
Please refund my order.'),
  (6, 'Morgan works for Example Group.');

SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen.decide(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
  'Does the customer request a refund?');

SELECT id AS msg_id, answer AS topic, status
FROM thinkthen.choose(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
  'What is the message about?',
  choices => ARRAY['billing', 'shipping', 'other']);

SELECT id AS msg_id, answer AS topics, status
FROM thinkthen.tag(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 4',
  'Which topics appear?',
  labels => ARRAY['billing', 'shipping']);

SELECT id AS msg_id, answer AS urgency, status
FROM thinkthen.score(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
  'How urgent is the request?',
  levels => ARRAY['routine', 'urgent']);

SELECT id AS msg_id
FROM thinkthen.filter(
  'SELECT msg_id AS id, body AS record FROM messages '
  'WHERE msg_id <= 3 ORDER BY msg_id',
  'Does the customer request a refund?');

SELECT answer AS urgency_rank, id AS msg_id, status
FROM thinkthen.rank(
  'SELECT msg_id AS id, body AS record FROM messages '
  'WHERE msg_id <= 3 ORDER BY msg_id',
  'How urgent is the request?',
  levels => ARRAY['routine', 'urgent'])
ORDER BY urgency_rank NULLS LAST;

SELECT id AS msg_id, (answer).text AS requested_action, status
FROM thinkthen.find(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5',
  'Which line states the requested action?');

SELECT id AS msg_id,
       CASE WHEN status = 'answered' AND detail -> 'refund' ->> 'status' = 'answered'
            THEN (answer ->> 'refund')::boolean END AS is_refund,
       CASE WHEN status = 'answered' AND detail -> 'topic' ->> 'status' = 'answered'
            THEN answer ->> 'topic' END AS topic,
       CASE WHEN status = 'answered' AND detail -> 'urgency' ->> 'status' = 'answered'
            THEN (answer ->> 'urgency')::float8 END AS urgency,
       detail -> 'refund' ->> 'status' AS refund_status,
       detail -> 'topic' ->> 'status' AS topic_status,
       detail -> 'urgency' ->> 'status' AS urgency_status, status
FROM thinkthen.annotate(
  'SELECT msg_id AS id, body AS record FROM messages '
  'WHERE msg_id <= 3 ORDER BY msg_id',
  questions => '{
    "refund": {"function": "decide", "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }');

SELECT r.position, r.id AS msg_id, found.text AS name, found.kind, r.status
FROM thinkthen.recognize(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 6',
  kinds => ARRAY['person', 'organization']) AS r
LEFT JOIN LATERAL unnest(r.answer) AS found ON true;

SELECT r.position, r.id AS msg_id, (link.source).text AS person, link.relation,
       (link.target).text AS organization, r.status
FROM thinkthen.relate(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5',
       rules => ARRAY[
         ROW('person', 'organization', 'works_for', NULL, false)::thinkthen.rule
       ]) AS r
LEFT JOIN LATERAL unnest(r.answer) AS link ON true;
