WITH t(id, body) AS (VALUES
    ('1', 'Newsletter: our autumn catalog is here. ' ||
          'No reply needed.'),
    ('2', 'Our checkout page is down and ' ||
          'customers cannot pay'),
    ('3', 'Reminder: your invoice is due in 30 days'),
    ('4', 'Please send the signed quote by 5 pm today')
)
SELECT body
FROM (
    SELECT t.body, d.probability AS urgency
    FROM t
    JOIN thinkthen_decide_many(
        '{"decide": "Is this urgent?"}',
        (SELECT jsonb_object_agg(id, body) FROM t)
    ) AS d ON d.key = t.id
) AS ranked
ORDER BY urgency DESC;
