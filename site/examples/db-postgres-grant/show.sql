CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
CREATE TABLE messages (msg_id integer PRIMARY KEY, body text NOT NULL);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.');

-- Let the role app call decide, with any systemone model.
CREATE ROLE app LOGIN;
GRANT USAGE ON SCHEMA thinkthen TO app;
GRANT EXECUTE ON FUNCTION thinkthen.decide TO app;
GRANT SELECT ON messages TO app;
ALTER ROLE app SET thinkthen.allowed_backends = 'systemone:*';

-- A new session as app: decide runs, and filter is refused.
\connect - app
SELECT id AS msg_id, answer AS is_refund
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages',
                      'Does the customer request a refund?');
SELECT id AS msg_id
FROM thinkthen.filter('SELECT msg_id AS id, body AS record FROM messages',
                      'Does the customer request a refund?');
