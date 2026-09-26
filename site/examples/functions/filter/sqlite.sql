.load ./thinkthen

WITH t(body) AS (VALUES
    ('Arrived a day early. Thank you!'),
    ('The zipper broke the first time I used it.'),
    ('Does this come in blue?'),
    ('The strap snapped on day two.')),
reviews AS (
    SELECT
        body,
        thinkthen_decide('Is this a complaint?', body)
            AS is_complaint
    FROM t)
SELECT body FROM reviews
WHERE is_complaint;
