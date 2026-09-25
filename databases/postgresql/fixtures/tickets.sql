DROP TABLE IF EXISTS tickets;
CREATE TABLE tickets (id int, body text);
INSERT INTO tickets VALUES
  (1, 'I renewed once this morning, but my card shows two charges. Please refund the duplicate.'),
  (2, 'maybe this is on our side, but the charge looks wrong. Can you check?'),
  (3, 'Is the jug dishwasher safe? The manual does not say.');
