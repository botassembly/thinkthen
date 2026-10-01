.load ./thinkthen

WITH t(body) AS (VALUES
    ('I would like to return this and get ' ||
     'my money back.' || char(10)),
    ('I want to send this back.' || char(10)))
SELECT
    thinkthen_decide('@refund.json', body) AS is_refund,
    trim(body, char(10)) AS body
FROM t;
