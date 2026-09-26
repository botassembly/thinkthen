SELECT thinkthen_tag(
    '{"tag": "Which labels fit this message?",
      "labels": ["praise", "bug", "billing"]}',
    'Love the new dashboard, ' ||
    'but export crashes the app, ' ||
    'and I was charged twice.',
    NULL
) AS fitting_labels;
