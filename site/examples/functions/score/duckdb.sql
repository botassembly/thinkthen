LOAD './thinkthen.duckdb_extension';

SELECT thinkthen_score(
    'How urgent is this?',
    'Our checkout page is down and customers ' ||
    'cannot pay.' || chr(10),
    ['Routine.', 'Soon.', 'Immediate.']
) AS urgency;
