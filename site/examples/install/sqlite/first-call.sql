.load ./thinkthen

CREATE TABLE messages(id INTEGER, body TEXT);
INSERT INTO messages VALUES
    (1, 'Love the new dashboard, but export crashes ' ||
        'the app,' || char(10) ||
        'and I was charged twice.' || char(10));

SELECT id, thinkthen_tag(
    '{"tag": "Which labels fit this message?",
      "labels": ["praise", "bug", "billing"]}',
    body) AS fitting_labels
FROM messages;
