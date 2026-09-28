LOAD './thinkthen.duckdb_extension';

SELECT thinkthen_tag(
    'Which labels fit this message?',
    'Love the new dashboard, ' ||
    'but export crashes the app, ' ||
    'and I was charged twice.',
    ['praise', 'bug', 'billing']
) AS fitting_labels;
