CREATE TABLE tickets (id int, body text);
INSERT INTO tickets VALUES
    (1, 'Please refund my order. It came broken.'),
    (2, 'Thanks for the quick help yesterday!'),
    (3, 'I want to send this back.');

SELECT id, body FROM tickets
WHERE thinkthen_decide('@refund.json', body) IS NULL;
