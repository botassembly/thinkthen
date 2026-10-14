.load ./thinkthen

CREATE TABLE entities(id INTEGER, name TEXT, kind TEXT);
INSERT INTO entities VALUES
    (1, 'Apple', 'company'),
    (2, 'Amazon', 'company'),
    (3, 'iPhone', 'product'),
    (4, 'Kindle', 'product');

SELECT relation, source, target, probability
FROM thinkthen_relate(
    'SELECT id, name, kind FROM entities',
    'makes=company:product'
) AS makes;
