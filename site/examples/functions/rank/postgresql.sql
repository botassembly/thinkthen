WITH t(id, body) AS (VALUES
    ('1', 'Newsletter: our autumn catalog is here. ' ||
          'No reply needed.'),
    ('2', 'Our checkout page is down and ' ||
          'customers cannot pay'),
    ('3', 'Reminder: your invoice is due in 30 days'),
    ('4', 'Please send the signed quote by 5 pm today'))
SELECT t.body
FROM t
JOIN thinkthen_rank(
    'Is this urgent?',
    (SELECT jsonb_object_agg(id, body) FROM t)
) AS urgency ON urgency.key = t.id
ORDER BY urgency.rank;
