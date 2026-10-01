SELECT thinkthen_annotate('@form.json', body) AS triage
FROM (VALUES
    ('Steps: click Export. It is very slow.'),
    ('Steps: click Log in. Nobody gets in.'),
    ('The Pay button on billing is too blue.')
) AS t(body);
