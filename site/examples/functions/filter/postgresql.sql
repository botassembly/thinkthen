SELECT body
FROM (
    SELECT body, thinkthen_decide(
        '{"decide": "Is this a complaint?"}',
        body
    ) AS is_complaint
    FROM (VALUES
        ('Arrived a day early. Thank you!'),
        ('The zipper broke the first time I used it.'),
        ('Does this come in blue?'),
        ('The strap snapped on day two.')
    ) AS t(body)
) AS reviews
WHERE is_complaint;
