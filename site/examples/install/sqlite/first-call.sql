CREATE TABLE messages(id INTEGER, body TEXT);
INSERT INTO messages VALUES
    (1, 'Love the app, but export crashes.'),
    (2, 'How do I change my billing address?');

SELECT id, thinkthen_tag(
    '{"tag": "Which labels fit this message?",
      "labels": ["praise", "bug", "billing"]}',
    body) AS fitting_labels
FROM messages;
