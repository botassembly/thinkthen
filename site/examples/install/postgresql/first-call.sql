CREATE TABLE tickets (id int, body text);
INSERT INTO tickets VALUES
    (1, 'Please refund my order. It came broken.'),
    (2, 'Thanks for the quick help yesterday!'),
    (3, 'I want to send this back.');

WITH judged AS (
    SELECT
        id,
        body,
        thinkthen_decide('@refund.json', body) AS is_refund
    FROM tickets
)
SELECT id, body FROM judged
WHERE is_refund IS NULL;
