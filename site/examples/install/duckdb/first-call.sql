INSTALL thinkthen FROM './';
LOAD thinkthen;
SET thinkthen_backend = 'typesafe';

CREATE TABLE tickets AS SELECT * FROM (VALUES
    (1, 'My parcel is a week late. Not okay.'),
    (2, 'Thanks for the quick help yesterday!'),
    (3, 'We are locked out of our account.'),
    (4, 'I was billed twice for one order.')
) t(id, body);

WITH judged AS (
    SELECT
        id,
        body,
        thinkthen_decide('Is this a complaint?', body)
            AS is_complaint
    FROM tickets
)
SELECT id, thinkthen_choose('Which team owns this?',
    body, ['billing', 'shipping', 'account']) AS team
FROM judged
WHERE is_complaint;
