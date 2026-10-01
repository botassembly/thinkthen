LOAD './thinkthen.duckdb_extension';

SELECT name.text, name.kind
FROM (
    SELECT unnest(thinkthen_recognize(
        'Maria Chen joined Northwind Freight, ' ||
        'a company in Chicago.',
        ['person', 'organization', 'place']
    )) AS name
);
