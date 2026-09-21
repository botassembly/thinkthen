DROP TABLE IF EXISTS alerts;
CREATE TABLE alerts (id int, body text);
INSERT INTO alerts VALUES
  (1, 'Checkout returns 500 at the payment step.'),
  (2, 'Card charges are failing for every customer.'),
  (3, 'The nightly export ran two hours late.'),
  (4, 'The payments database ran out of disk space.');
