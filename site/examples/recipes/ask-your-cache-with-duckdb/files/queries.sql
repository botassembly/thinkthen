SET autoinstall_known_extensions = false;
SET autoload_known_extensions = false;
CREATE TEMP TABLE answers AS
SELECT * FROM read_json_auto(
  'files/results.jsonl', format='newline_delimited');
SELECT count(*) AS saved_rows,
       count(value) AS answered,
       count(*) FILTER (WHERE value IS NULL) AS unanswered,
       sum(meta.requests_sent) AS requests_sent
FROM answers;
CREATE TEMP VIEW selected AS
SELECT json_extract_string(
  to_json(answer.probabilities), '$.' || value)::DOUBLE
  AS probability
FROM answers WHERE value IS NOT NULL;
SELECT count(*) AS answered,
       count(*) FILTER (
         WHERE abs(probability - 0.75) <= 0.05) AS near_cut
FROM selected;
