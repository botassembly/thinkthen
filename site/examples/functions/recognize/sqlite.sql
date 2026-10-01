.load ./thinkthen

CREATE TABLE tickets(id INTEGER, body TEXT);
INSERT INTO tickets VALUES (
    1,
    'Maria Chen joined Northwind Freight, ' ||
    'a company in Chicago.'
);

SELECT t.id, entity.text, entity.kind
FROM
    tickets t,
    thinkthen_recognize(
        t.body,
        'person,organization,place'
    ) entity;

WITH found AS (
    SELECT
        id,
        thinkthen_relations(body, '@names.json')
            AS links_found
    FROM tickets)
SELECT found.id, link.value ->> 'relation' AS relation,
    link.value ->> '$.source.text' AS source,
    link.value ->> '$.target.text' AS target
FROM
    found,
    json_each(found.links_found, '$.relations') AS link;
