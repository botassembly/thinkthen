LOAD './thinkthen.duckdb_extension';

SELECT name.text, name.kind
FROM (
    SELECT unnest(thinkthen_recognize(
        'Sam Patel says his Kindle arrived broken ' ||
        'in Denver.',
        ['person', 'product', 'place']
    )) AS name
);
