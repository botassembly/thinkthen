PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = FULL;
CREATE TABLE IF NOT EXISTS questions(id TEXT PRIMARY KEY, body TEXT NOT NULL, request TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS wordings(id TEXT PRIMARY KEY, question TEXT NOT NULL REFERENCES questions(id), body TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS evidence(id TEXT PRIMARY KEY, body TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS gold(
    id INTEGER PRIMARY KEY,
    question TEXT NOT NULL REFERENCES questions(id),
    evidence TEXT NOT NULL REFERENCES evidence(id),
    answer TEXT NOT NULL,
    attribution TEXT NOT NULL,
    recorded_at INTEGER NOT NULL,
    supersedes INTEGER REFERENCES gold(id)
);
CREATE INDEX IF NOT EXISTS gold_history ON gold(question, evidence, id);
CREATE TABLE IF NOT EXISTS audit(gold INTEGER PRIMARY KEY REFERENCES gold(id));
CREATE TABLE IF NOT EXISTS current_gold(
    question TEXT NOT NULL REFERENCES questions(id),
    evidence TEXT NOT NULL REFERENCES evidence(id),
    gold INTEGER NOT NULL REFERENCES gold(id),
    PRIMARY KEY(question, evidence)
);
