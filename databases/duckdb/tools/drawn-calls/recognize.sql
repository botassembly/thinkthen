-- A copy of the three DuckDB calls the product deck's
-- `recognize-surfaces.md` page draws, vendored again on 2026-09-23 from
-- the page whose sha256 was
-- 643ce4b4baf63d373b760ac1e0de6b7f07bbaf6dfb2d430bbe319693c4917cef.
-- One call per line, whitespace folded, in the page's order. `check.sh`
-- pins the sha256 of these call lines, and `tools/drawn_calls_drift.py`
-- compares them with the page when the page is on the machine (review 5,
-- R5-24). Changing them is a re-vendoring that updates the pin and the
-- recognize acceptance's expectations in the same commit.
SELECT id, unnest(thinkthen_recognize(body, ['person', 'organization', 'place'])) AS name FROM tickets;
SELECT * FROM thinkthen_relations((SELECT body FROM tickets), '@names.json');
SELECT * FROM thinkthen_relate('SELECT id, body FROM rules', ['{"either": ["contradicts"]}']);
