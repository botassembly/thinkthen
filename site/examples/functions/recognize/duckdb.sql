CREATE TABLE tickets AS FROM (VALUES
    (1, 'Maria Chen joined Northwind Freight, ' ||
        'a company in Chicago.')
) t(id, body);

SELECT
    id,
    unnest(thinkthen_recognize(
        body,
        ['person', 'organization', 'place']
    )) AS name
FROM tickets;

SELECT * FROM thinkthen_relations(
    (SELECT body FROM tickets),
    '@names.json'
);
