.load ./thinkthen

WITH t(body) AS (VALUES
    ('Please refund my order. It arrived broken.'),
    ('Thanks for the quick help yesterday!'))
SELECT
    thinkthen_decide(
        'Does the customer ask for a refund?',
        body
    ) AS is_refund,
    body
FROM t;
