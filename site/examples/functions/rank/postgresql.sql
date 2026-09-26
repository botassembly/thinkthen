SELECT body
FROM (VALUES
    ('Newsletter: our autumn catalog is here. ' ||
     'No reply needed.'),
    ('Our checkout page is down and customers cannot pay'),
    ('Reminder: your invoice is due in 30 days'),
    ('Please send the signed quote by 5 pm today')
) AS t(body)
ORDER BY thinkthen_probability(
    '{"decide": "Is this urgent?"}',
    body
) DESC;
