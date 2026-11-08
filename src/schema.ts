export const SCHEMA = `
CREATE TABLE metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE clients (id TEXT PRIMARY KEY, token_hash TEXT NOT NULL, scopes TEXT NOT NULL, revoked INTEGER NOT NULL DEFAULT 0);
CREATE TABLE records (id TEXT PRIMARY KEY, kind TEXT NOT NULL, status TEXT NOT NULL, revision INTEGER NOT NULL, updated_at TEXT NOT NULL, title TEXT NOT NULL, content TEXT NOT NULL, payload TEXT NOT NULL);
CREATE TABLE operations (actor TEXT NOT NULL, generation TEXT NOT NULL, operation_id TEXT NOT NULL, request_hash TEXT NOT NULL, receipt TEXT NOT NULL, PRIMARY KEY(actor, generation, operation_id));
CREATE INDEX records_recent ON records(updated_at DESC, id);
PRAGMA user_version = 1;
`;
