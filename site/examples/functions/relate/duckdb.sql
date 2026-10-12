LOAD './thinkthen.duckdb_extension';

CREATE TABLE entities AS FROM (VALUES
    (1, 'Apple', 'company'),
    (2, 'Amazon', 'company'),
    (3, 'iPhone', 'product'),
    (4, 'Kindle', 'product')
) t(id, name, kind);

SELECT * FROM thinkthen_relate(
    'SELECT id, name, kind FROM entities',
    ['makes=company:product']
) AS makes;
