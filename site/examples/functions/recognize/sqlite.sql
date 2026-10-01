.load ./thinkthen

SELECT name.text, name.kind
FROM thinkthen_recognize(
    'Maria Chen joined Northwind Freight, ' ||
    'a company in Chicago.',
    'person,organization,place'
) AS name;
