LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
  (1, 'I was charged twice and my parcel is late again.');

SELECT msg_id, answer AS topics, status
FROM thinkthen_tag((SELECT msg_id, body FROM messages),
                   'Which topics appear?',
                   labels := ['billing', 'shipping'], record := 'body');
