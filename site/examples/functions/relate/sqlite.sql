.load ./thinkthen

CREATE TABLE entities(id INTEGER, name TEXT, kind TEXT);
INSERT INTO entities VALUES
    (1, 'Paul McCartney', 'singer'),
    (2, 'Ringo Starr', 'singer'),
    (3, 'Yesterday', 'song'),
    (4, 'Octopus''s Garden', 'song');

SELECT * FROM thinkthen_relate(
    'SELECT id, name, kind FROM entities',
    'sings=singer:song'
) AS sings;
