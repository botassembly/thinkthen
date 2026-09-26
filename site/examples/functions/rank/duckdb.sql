LOAD './thinkthen.duckdb_extension';

SELECT body
FROM (VALUES
    ('Newsletter: our autumn catalog is here. ' ||
     'No reply needed.'),
    ('Our checkout page is down and customers cannot pay'),
    ('Reminder: your invoice is due in 30 days'),
    ('Please send the signed quote by 5 pm today')
) t(body)
ORDER BY
    thinkthen_probability('Is this urgent?', body) DESC;
