LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
  (1, 'I was charged twice and my parcel is late again.');

CREATE TABLE message_topics AS
SELECT msg_id, answer AS topics, status
FROM thinkthen_tag((SELECT msg_id, body FROM messages),
                   'Which topics appear?',
                   labels := ['billing', 'shipping'], record := 'body');

SELECT msg_id, topics, status FROM message_topics;
-- One row per message and topic, for counting.
SELECT topic, count(*) AS messages
FROM (SELECT unnest(topics) AS topic
      FROM message_topics WHERE status = 'answered')
GROUP BY topic ORDER BY topic;
