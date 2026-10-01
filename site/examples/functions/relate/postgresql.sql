CREATE TABLE entities (id int, name text, kind text);
INSERT INTO entities VALUES
    (1, 'Paul McCartney', 'singer'),
    (2, 'Ringo Starr', 'singer'),
    (3, 'Yesterday', 'song'),
    (4, 'Octopus''s Garden', 'song');

SELECT * FROM thinkthen_relate(
    'SELECT id, name, kind FROM entities',
    ARRAY['sings=singer:song']
) AS sings;
