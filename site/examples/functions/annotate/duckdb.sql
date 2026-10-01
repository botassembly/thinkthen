LOAD './thinkthen.duckdb_extension';

SELECT thinkthen_annotate(
    '@form.json',
    'Steps: click Log in. Nobody gets in.'
) AS triage;
