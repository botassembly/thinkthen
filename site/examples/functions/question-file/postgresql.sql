SELECT
    id,
    thinkthen_decide('@refund.json', body) AS is_refund
FROM (VALUES
    (1, 'I would like to return this and get ' ||
        'my money back.' || chr(10)),
    (2, 'I want to send this back.' || chr(10))
) AS t(id, body);
