-- A FROZEN copy of the three DuckDB calls the product deck's
-- `recognize-surfaces.md` page drew when this check was written: the page
-- whose sha256 was 218c8e28a91777b9ab97c005d7e0deb44f74bcc22197dfbf1c6748e85a3cf294
-- (2026-09-21). One call per line, in the page's order. The deck has
-- changed since (2026-09-23: its relations call reads a subquery and its
-- relate call reads a query string), so this file is a fixture, not a
-- mirror: `check.sh` pins the sha256 of these call lines, and changing
-- them is a deliberate re-vendoring that updates that pin and the
-- recognize acceptance's expectations in the same commit (review 5).
SELECT id, unnest(thinkthen_recognize(body, ['person', 'organization'])) AS name FROM tickets;
SELECT * FROM thinkthen_relations(body, '@names.json');
SELECT * FROM thinkthen_relate((SELECT id, body FROM alerts), ['caused_by']);
