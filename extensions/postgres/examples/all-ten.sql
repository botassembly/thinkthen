\echo decide
SELECT position, status, answer, p, error FROM thinkthen_decide(
  $$SELECT 'Please refund the duplicate charge.'::text AS record$$, 'Does the customer request a refund?');
\echo choose
SELECT position, status, answer, p, error FROM thinkthen_choose(
  $$SELECT 'Please refund the duplicate charge.'::text AS record$$, 'Which team owns this?',
  '{"options":["billing","shipping","account"]}');
\echo tag
SELECT position, status, answer, p, error FROM thinkthen_tag(
  $$SELECT 'The parcel is late and I want a refund.'::text AS record$$, 'Which topics apply?',
  '{"labels":["refund","delivery","login"]}');
\echo score
SELECT position, status, answer, p, error FROM thinkthen_score(
  $$SELECT 'Production checkout is down. Please help immediately.'::text AS record$$, 'How urgent is this?',
  '{"levels":["low","medium","high"]}');
\echo filter
SELECT position, status, answer, p, error, kept FROM thinkthen_filter(
  $$SELECT body AS record FROM (VALUES ('Please refund the duplicate charge.'::text),('Thank you for the help.')) AS input(body)$$,
  'Does the customer request a refund?');
\echo rank
SELECT position, status, answer, p, error FROM thinkthen_rank(
  $$SELECT body AS record FROM (VALUES ('A cosmetic typo.'::text),('Production checkout is down.')) AS input(body)$$,
  'How urgent is this?', '{"top":1}');
\echo find
SELECT position, status, answer, p, error FROM thinkthen_find(
  $$SELECT E'The office is closed on Sunday.\nThe meeting is on Monday.'::text AS record$$, 'When is the meeting?');
\echo annotate
SELECT position, status, answer, p, error FROM thinkthen_annotate(
  $$SELECT 'Please refund the duplicate charge.'::text AS record$$,
  '{"type":"inline","value":{"questions":{"refund":{"decide":"Does the customer request a refund?"}}}}');
\echo recognize
SELECT position, status, answer, p, error FROM thinkthen_recognize(
  $$SELECT 'Ada works for Acme.'::text AS record$$, '{"type":"inline","value":["person","organization"]}');
\echo relate
SELECT position, status, answer, p, error FROM thinkthen_relate(
  $$SELECT 'Ada works for Acme.'::text AS record$$,
  '{"type":"inline","value":[{"name":"works_for","source":"person","target":"organization"}]}');
