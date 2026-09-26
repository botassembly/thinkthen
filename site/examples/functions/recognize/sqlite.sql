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

SELECT *
FROM
    tickets t,
    thinkthen_relations(t.body, '@names.json') AS link;
