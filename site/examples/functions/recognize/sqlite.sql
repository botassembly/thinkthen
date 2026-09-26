CREATE TABLE tickets(id INTEGER, body TEXT);
INSERT INTO tickets VALUES (
    1,
    'Maria Chen joined Northwind Freight, ' ||
    'a company in Chicago.'
);

SELECT t.id, n.text, n.kind
FROM
    tickets t,
    thinkthen_recognize(
        t.body,
        'person,organization,place'
    ) n;

SELECT *
FROM
    tickets t,
    thinkthen_relations(t.body, '@names.json');
