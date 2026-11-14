LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
  (1, 'I was charged twice and my parcel is late again.');

CREATE TABLE message_topics AS
SELECT position, id, msg_id, answer AS topics, status
FROM thinkthen_tag((SELECT msg_id, body FROM messages),
                   'Which topics appear?',
                   labels := ['billing', 'shipping'], record := 'body');

SELECT msg_id, topics, status FROM message_topics;
-- Keep an empty or failed list visible when reading individual topics.
SELECT topics.position, topics.id, topics.msg_id, topics.status, label.topic
FROM message_topics AS topics
LEFT JOIN LATERAL (SELECT unnest(topics.topics) AS topic) AS label ON true;

-- Count only actual labels, leaving a NULL group with zero for an empty list.
SELECT label.topic, count(label.topic) AS messages
FROM message_topics AS topics
LEFT JOIN LATERAL (SELECT unnest(topics.topics) AS topic) AS label ON true
WHERE topics.status = 'answered'
GROUP BY label.topic ORDER BY label.topic;
