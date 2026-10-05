SET autoinstall_known_extensions = false;
SET autoload_known_extensions = false;
CREATE TEMP TABLE answers AS
SELECT * FROM read_json_auto('files/results.jsonl', format='newline_delimited');
-- Saved rows are observations, not cache lookup events or independent trials.
SELECT count(*) AS saved_rows,
       count(value) AS answered_rows,
       count(*) FILTER (WHERE value IS NULL) AS without_answer,
       sum(meta.requests_sent) AS recorded_requests_sent
FROM answers;
-- Selected-label probability is not confidence or accuracy.
SELECT count(*) AS answered_rows,
       count(*) FILTER (WHERE abs(json_extract_string(to_json(answer.probabilities), '$.' || value)::DOUBLE - 0.75) <= 0.05) AS near_cut
FROM answers WHERE value IS NOT NULL;
