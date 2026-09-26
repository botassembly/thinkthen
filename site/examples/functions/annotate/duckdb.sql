LOAD './thinkthen.duckdb_extension';

SELECT thinkthen_annotate('@form.json', body) AS triage
FROM (VALUES
    ('CSV export fails. Steps: click Export.'),
    ('The login page spins and nobody can sign in.'),
    ('The Pay button on the billing page is too blue.')
) t(body);
