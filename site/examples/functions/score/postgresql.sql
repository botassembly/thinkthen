SELECT body, thinkthen_score(
    '{"score": "How urgent is this?",
      "levels": ["Routine.", "Soon.", "Immediate."]}',
    body, NULL) AS urgency
FROM (VALUES
    ('Please update my mailing address when you can.'),
    ('Can you send the signed contract by Friday?'),
    ('Nobody can log in to the site right now.')
) AS t(body);
