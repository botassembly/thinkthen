.load ./thinkthen

WITH t(body) AS (VALUES
    ('Please refund my order. It arrived broken.'),
    ('Thanks for the quick help yesterday!'),
    ('I want to send this back.'))
SELECT
    thinkthen_decide('@refund.json', body) AS is_refund,
    body
FROM t;
