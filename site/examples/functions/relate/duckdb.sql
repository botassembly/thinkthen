LOAD './thinkthen.duckdb_extension';

CREATE TABLE entities AS FROM (VALUES
    (1, 'Paul McCartney', 'singer'),
    (2, 'Ringo Starr', 'singer'),
    (3, 'Yesterday', 'song'),
    (4, 'Octopus''s Garden', 'song')
) t(id, name, kind);

SELECT * FROM thinkthen_relate(
    'SELECT id, name, kind FROM entities',
    ['sings=singer:song']
) AS sings;
