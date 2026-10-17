CREATE TABLE entities (id int, name text, kind text);
INSERT INTO entities VALUES
    (1, 'Apple', 'company'),
    (2, 'Amazon', 'company'),
    (3, 'iPhone', 'product'),
    (4, 'Kindle', 'product');

SELECT * FROM thinkthen_relate(
    'SELECT id, name, kind FROM entities',
    ARRAY['makes=company:product']
) AS makes;
