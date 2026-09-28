-- The slide's install line and fixture: the sample assumes a reviews
-- table; this builds one whose rows the null backend answers both ways.
.load ./thinkthen
-- Preserve the old warm-then-scalar cache identity: one record per request.
SELECT thinkthen_batch(1);
CREATE TABLE reviews(id INTEGER PRIMARY KEY, body TEXT);
INSERT INTO reviews(body) VALUES ('i want a refund now');
INSERT INTO reviews(body) VALUES ('good morning');
INSERT INTO reviews(body) VALUES ('refund, please');
INSERT INTO reviews(body) VALUES ('maybe later');
INSERT INTO reviews(body) VALUES ('see you');
-- The slide, exactly as drawn.
SELECT thinkthen_warm('Is this a complaint?', body)
FROM reviews;
SELECT id, body
FROM reviews
WHERE thinkthen_decide('Is this a complaint?', body);
SELECT count(*)
FROM reviews
WHERE thinkthen_decide('Is this a complaint?', body);
