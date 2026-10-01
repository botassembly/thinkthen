SELECT thinkthen_tag(
    '{"tag": "Which labels fit this message?",
      "labels": ["praise", "bug", "billing"]}',
    'Love the new dashboard, but export crashes ' ||
    'the app,' || chr(10) ||
    'and I was charged twice.' || chr(10),
    NULL
) AS fitting_labels;
