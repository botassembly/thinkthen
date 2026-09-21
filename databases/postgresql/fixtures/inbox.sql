DROP TABLE IF EXISTS inbox;
CREATE TABLE inbox (id int, body text);
INSERT INTO inbox VALUES
  (1, 'Maria Chen joined Northwind Freight in Chicago last spring.'),
  (2, 'Amara Okafor founded Kestrel Labs in 2015.'),
  (3, 'The Millbrook Athletics won the county final on Saturday.');

DROP TABLE IF EXISTS accounts;
CREATE TABLE accounts (name text, owner text);
INSERT INTO accounts VALUES
  ('Northwind Freight', 'dana'),
  ('Kestrel Labs', 'amara'),
  ('Millbrook Athletics', 'lee');
