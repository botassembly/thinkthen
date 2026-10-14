LOAD './thinkthen.duckdb_extension';

CREATE TABLE tickets AS FROM (VALUES
    (1, 'My parcel is a week late. Not okay.'),
    (2, 'Thanks for the quick help yesterday!'),
    (3, 'We are locked out of our account.'),
    (4, 'I was billed twice for one order.')
) t(id, body);
SELECT d.ordinal + 1 AS id, d.value AS is_complaint,
       d.probability
FROM (SELECT unnest(thinkthen_decide_details(
    'Is this a complaint?',
    (SELECT list(thinkthen_record(text := body)
                 ORDER BY id) FROM tickets))) AS d);
SELECT d.ordinal + 1 AS id, d.value AS team, d.probability
FROM (SELECT unnest(thinkthen_choose_details(
    {'choose': 'Which team owns this?',
     'options': ['billing', 'shipping', 'account']},
    (SELECT list(thinkthen_record(text := body)
                 ORDER BY id) FROM tickets))) AS d);
