LOAD './thinkthen.duckdb_extension';

CREATE TABLE tickets AS SELECT * FROM (VALUES
    (1, 'My parcel is a week late. Not okay.'),
    (2, 'Thanks for the quick help yesterday!'),
    (3, 'We are locked out of our account.'),
    (4, 'I was billed twice for one order.')
) t(id, body);

SELECT id, thinkthen_choose('Which team owns this?',
    body, ['billing', 'shipping', 'account']) AS team
FROM tickets
WHERE thinkthen_decide('Is this a complaint?', body);
