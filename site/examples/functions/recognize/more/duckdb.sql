LOAD './thinkthen.duckdb_extension';

CREATE TABLE tickets AS FROM (VALUES
    (1, 'Maria Chen joined Northwind Freight, ' ||
        'a company in Chicago.')
) t(id, body);

SELECT id, entity.text, entity.kind
FROM (
    SELECT
        id,
        unnest(thinkthen_recognize(
            body,
            ['person', 'organization', 'place']
        )) AS entity
    FROM tickets
);

SELECT id, link.relation, link.source, link.target
FROM (
    SELECT id, unnest(thinkthen_relations(
        body, '@names.json'
    )) AS link
    FROM tickets
);
