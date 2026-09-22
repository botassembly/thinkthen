-- The DuckDB calls exactly as the product deck's `recognize-surfaces.md`
-- page draws them (the deck is the source of truth; this copy lets the
-- check run with no private deck on disk). One call per line, in the
-- deck's order: `-- DuckDB:` marks the first two, `-- all three` the
-- third. Update this file from the deck when the deck changes.
SELECT id, unnest(thinkthen_recognize(body, ['person', 'organization'])) AS name FROM tickets;
SELECT * FROM thinkthen_relations(body, '@names.json');
SELECT * FROM thinkthen_relate((SELECT id, body FROM alerts), ['caused_by']);
