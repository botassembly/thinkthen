.load ./thinkthen
SELECT thinkthen_configure('{"backend":"typesafe"}');

CREATE TABLE messages(id INTEGER, body TEXT);
INSERT INTO messages VALUES
    (1, 'Love the new dashboard, but export crashes ' ||
        'the app,' || char(10) ||
        'and I was charged twice.' || char(10));

WITH question(wording) AS (VALUES
    ('{"tag": "Which labels fit this message?",
      "labels": ["praise", "bug", "billing"]}'))
SELECT id, labels.value AS fitting_labels
FROM messages, question JOIN thinkthen_tag_details(
    wording, body) AS labels;

WITH question(wording) AS (VALUES
    ('Does the customer ask for a refund?')),
refunds(body) AS (VALUES
    ('Please refund my order. It arrived broken.'),
    ('Thanks for the quick help yesterday!'),
    ('I want to send this back.'))
SELECT refund.ordinal, refund.value AS is_refund,
    refund.probability AS p
FROM question JOIN thinkthen_decide_details
    AS refund
WHERE request_question = wording
    AND request_records = (
        SELECT json_group_array(json_object('text', body))
        FROM refunds);
