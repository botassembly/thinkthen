SELECT name.text, name.kind
FROM thinkthen_recognize(
    'Maria Chen joined Northwind Freight, ' ||
    'a company in Chicago.',
    ARRAY['person', 'organization', 'place']
) AS name;
