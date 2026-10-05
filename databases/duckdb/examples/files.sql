-- From the repository root, after LOAD of the matching extension.
-- Materialize once and commit before relate's separate read-only connection.
CREATE TABLE documents AS
SELECT * FROM thinkthen_read_files('specification/fixtures/files/documents', '{"unit":"file"}');

CREATE TABLE lines AS
SELECT * FROM thinkthen_read_files('specification/fixtures/files/documents');

-- decide
SELECT *, thinkthen_decide('Does this document contain a support contract?', record) AS value
FROM documents ORDER BY ordinal;
-- choose
SELECT *, thinkthen_choose('Which topic best describes this document?', record, ['policy','contract']) AS value
FROM documents ORDER BY ordinal;
-- tag
SELECT *, thinkthen_tag('Which topics apply to this document?', record, ['refund','support','billing']) AS value
FROM documents ORDER BY ordinal;
-- score
SELECT *, thinkthen_score('How urgent is this document?', record, ['routine','urgent']) AS value
FROM documents ORDER BY ordinal;
-- filter
SELECT * FROM lines
WHERE thinkthen_decide('Does this line describe a refund?', record) ORDER BY ordinal;
-- rank
SELECT d.*, r.rank, r.probability
FROM documents d JOIN thinkthen_rank('Does this document discuss a billing dispute?',
  (SELECT json_group_object(ordinal, record) FROM (SELECT * FROM documents ORDER BY ordinal))) r
ON r.key = CAST(d.ordinal AS VARCHAR) ORDER BY r.rank;
-- find: returned indexes are zero-based positions in the ordered candidate list.
WITH chosen AS (
  SELECT thinkthen_find('Which line gives the refund policy?', list(record ORDER BY ordinal)) AS result
  FROM lines
), mapped AS (SELECT *, row_number() OVER (ORDER BY ordinal) - 1 AS candidate FROM lines)
SELECT d.ordinal, d.record, d.file, d.first_line, d.last_line, c.result.probability
FROM mapped d JOIN chosen c ON d.candidate = c.result.index;
-- annotate
SELECT *, thinkthen_annotate('@specification/fixtures/files/questions.json', record) AS value
FROM documents ORDER BY ordinal;
-- recognize: retain native Unicode offsets and map the physical span through Rust.
WITH names AS (
  SELECT d.*, unnest(thinkthen_recognize(record, ['person','organization'])) AS entity FROM documents d
), located AS (
  SELECT *, thinkthen_span_lines(record, first_line, entity.start, entity."end") AS lines FROM names
)
SELECT ordinal, record, file, entity.text, entity.start, entity."end", entity.kind,
       lines.first_line, lines.last_line FROM located ORDER BY ordinal, entity.start;
-- relate: names are complete document content, never filenames or guessed sentences.
SELECT e.relation, s.ordinal AS source_ordinal, s.record AS source_record,
       s.file AS source_file, s.first_line AS source_first_line, s.last_line AS source_last_line,
       t.ordinal AS target_ordinal, t.record AS target_record,
       t.file AS target_file, t.first_line AS target_first_line, t.last_line AS target_last_line,
       e.probability
FROM thinkthen_relate('SELECT ordinal AS id, record AS name, ''*'' AS kind FROM documents ORDER BY ordinal',
                      ['supports=*:*']) e
JOIN documents s ON e.source = CAST(s.ordinal AS VARCHAR)
JOIN documents t ON e.target = CAST(t.ordinal AS VARCHAR)
ORDER BY source_ordinal, target_ordinal;
