LOAD './thinkthen.duckdb_extension';

SELECT thinkthen_tag(
    'Which labels fit this message?',
    'Love the new dashboard, but export crashes ' ||
    'the app,' || chr(10) ||
    'and I was charged twice.' || chr(10),
    ['praise', 'bug', 'billing']
) AS fitting_labels;
