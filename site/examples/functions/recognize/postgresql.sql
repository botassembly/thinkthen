CREATE TABLE tickets (id int, body text);
INSERT INTO tickets VALUES (
    1,
    'Maria Chen joined Northwind Freight, ' ||
    'a company in Chicago.'
);

SELECT t.id, entity.text, entity.kind
FROM tickets t, LATERAL thinkthen_recognize(
    t.body,
    ARRAY['person', 'organization', 'place']
) entity;

SELECT t.id, link.relation, link.source_text,
    link.target_text
FROM tickets t, LATERAL thinkthen_relations(
    t.body,
    '@names.json'
) link;
