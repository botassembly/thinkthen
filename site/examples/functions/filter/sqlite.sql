.load ./thinkthen

WITH t(body) AS (VALUES
    ('Arrived a day early. Thank you!'),
    ('The zipper broke the first time I used it.'),
    ('Does this come in blue?'),
    ('The strap snapped on day two.'))
SELECT body FROM t
WHERE thinkthen_decide('Is this a complaint?', body);
