-- The slide's install line and fixture: the sample assumes a reviews
-- table; this builds one whose rows the null backend answers both ways.
.load ./thinkthen
CREATE TABLE reviews(id INTEGER PRIMARY KEY, body TEXT);
INSERT INTO reviews(body) VALUES ('i want a refund now');
INSERT INTO reviews(body) VALUES ('good morning');
INSERT INTO reviews(body) VALUES ('refund, please');
INSERT INTO reviews(body) VALUES ('maybe later');
INSERT INTO reviews(body) VALUES ('see you');
-- One keyed call carries each original id beside its answer.
SELECT r.id, r.body
FROM reviews AS r
JOIN thinkthen_decide_many('Is this a complaint?',
  (SELECT json_group_object(id, body) FROM reviews)) AS d
  ON d.key = CAST(r.id AS TEXT)
WHERE d.value = 1;
SELECT count(*)
FROM reviews AS r
JOIN thinkthen_decide_many('Is this a complaint?',
  (SELECT json_group_object(id, body) FROM reviews)) AS d
  ON d.key = CAST(r.id AS TEXT)
WHERE d.value = 1;
