PRAGMA application_id = 1414812243;
PRAGMA user_version = 0;
CREATE TABLE store(format INTEGER NOT NULL, access INTEGER NOT NULL);
INSERT INTO store VALUES(0, 0);
CREATE TABLE exchanges(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  recorded INTEGER NOT NULL CHECK(recorded IN (0,1)),
  received INTEGER NOT NULL,
  metadata BLOB NOT NULL,
  request BLOB NOT NULL,
  response BLOB NOT NULL,
  digest BLOB NOT NULL,
  size INTEGER NOT NULL CHECK(size >= 0)
);
CREATE TABLE answers(
  exchange INTEGER NOT NULL REFERENCES exchanges(id) ON DELETE CASCADE,
  slot INTEGER NOT NULL,
  key BLOB NOT NULL,
  answer BLOB NOT NULL,
  digest BLOB NOT NULL,
  size INTEGER NOT NULL CHECK(size >= 0),
  PRIMARY KEY(exchange, slot)
);
CREATE INDEX answer_lookup ON answers(key, exchange);
CREATE TABLE eligible(
  key BLOB PRIMARY KEY,
  exchange INTEGER NOT NULL,
  slot INTEGER NOT NULL,
  touched INTEGER NOT NULL,
  FOREIGN KEY(exchange,slot) REFERENCES answers(exchange,slot) ON DELETE CASCADE
);
