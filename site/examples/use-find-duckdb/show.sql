LOAD '/absolute/path/to/thinkthen.duckdb_extension';
CREATE TABLE notes (note_id INTEGER, body VARCHAR);
INSERT INTO notes VALUES (1, 'Morgan works for Example Group.
Please refund my order.');

SELECT note_id, answer.text AS requested_action,
       answer.start_offset, answer.end_offset, status
FROM thinkthen_find((SELECT note_id, body FROM notes),
                    'Which line states the requested action?',
                    record := 'body');
