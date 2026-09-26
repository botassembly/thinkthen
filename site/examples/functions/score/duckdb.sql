LOAD './thinkthen.duckdb_extension';

SELECT
    body,
    thinkthen_score(
        'How urgent is this?',
        body,
        ['Routine.', 'Soon.', 'Immediate.']
    ) AS urgency
FROM (VALUES
    ('Please update my mailing address when you can.'),
    ('Can you send the signed contract by Friday?'),
    ('Nobody can log in to the site right now.')
) t(body);
