CREATE TABLE tickets (id int, body text);
INSERT INTO tickets VALUES (
    1,
    'Maria Chen joined Northwind Freight, ' ||
    'a company in Chicago.'
);

SELECT t.id, n.text, n.kind
FROM tickets t, LATERAL thinkthen_recognize(
    t.body,
    ARRAY['person', 'organization', 'place']
) n;

SELECT r.*
FROM tickets t, LATERAL thinkthen_relations(
    t.body,
    '@names.json'
) r;
