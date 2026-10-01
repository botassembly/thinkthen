LOAD './thinkthen.duckdb_extension';

SELECT
    trim(body, chr(10)) AS body,
    thinkthen_decide('@refund.json', body) AS is_refund
FROM (VALUES
    ('I would like to return this and get ' ||
     'my money back.' || chr(10)),
    ('I want to send this back.' || chr(10))
) t(body);
