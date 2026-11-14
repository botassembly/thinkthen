LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
    (1, 'I was charged twice and my parcel is late again.');

SELECT topics.position, topics.id, topics.msg_id, topics.status,
       label.topic
FROM thinkthen_tag((SELECT msg_id, body FROM messages),
                   'Which topics appear?',
                   labels := ['billing', 'shipping'], record := 'body') AS topics
LEFT JOIN LATERAL (SELECT unnest(topics.answer) AS topic,
                          generate_subscripts(topics.answer, 1) AS answer_position) AS label ON true
ORDER BY topics.position, label.answer_position;
