LOAD './thinkthen.duckdb_extension';

SELECT
    body,
    thinkthen_decide('@refund.json', body) AS is_refund
FROM (VALUES
    ('Please refund my order. It arrived broken.'),
    ('Thanks for the quick help yesterday!'),
    ('I want to send this back.')
) t(body);
