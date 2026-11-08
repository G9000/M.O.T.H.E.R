import { Database } from 'bun:sqlite';
import { closeSync, openSync, existsSync, readFileSync, rmSync } from 'node:fs';
import { createHash, randomBytes, timingSafeEqual } from 'node:crypto';
import { z } from 'zod';
import { fileURLToPath } from 'node:url';
import { ContextQuerySchema, PersonaSchema, ScopeSchema, WorkspaceSchema, WriteSchema, parseRecord, type Auth, type ContextRecord, type Grant, type Persona, type Receipt, type Scope, type Workspace } from './contracts';
import { SCHEMA } from './schema';
import { checkUsage, type Usage } from './limits';
import { fail } from './errors';
export const hash = (text: string) => createHash('sha256').update(text).digest('hex');
export function canonical(value: unknown): string {
  if (Array.isArray(value)) return '[' + value.map(canonical).join(',') + ']';
  if (value && typeof value === 'object') return '{' + Object.entries(value).sort(([a], [b]) => a.localeCompare(b)).map(([k, v]) => JSON.stringify(k) + ':' + canonical(v)).join(',') + '}';
  return JSON.stringify(value);
}
export class Store {
  private closed = false;
  private constructor(readonly db: Database) {}
  static initialize(path: string, personaContent?: string, populate?: (store: Store) => void): Store {
    const content = personaContent ?? readFileSync(fileURLToPath(new URL('../SOUL.md', import.meta.url)), 'utf8');
    const persona = PersonaSchema.parse({ content, hash: hash(content) });
    closeSync(openSync(path, 'wx', 0o600));
    const db = new Database(path, { strict: true });
    try {
      db.exec('PRAGMA busy_timeout = 5000; PRAGMA journal_mode = WAL; PRAGMA synchronous = FULL;');
      db.transaction(() => {
        db.exec(SCHEMA);
        const workspace: Workspace = { id: crypto.randomUUID(), generation: crypto.randomUUID(), schemaVersion: 1, createdAt: new Date().toISOString() };
        db.query('INSERT INTO metadata VALUES (?, ?)').run('workspace', JSON.stringify(workspace));
        db.query('INSERT INTO metadata VALUES (?, ?)').run('persona', JSON.stringify(persona));
        db.query('INSERT INTO metadata VALUES (?, ?)').run('usage', JSON.stringify({ records: 0, operations: 0, payloadBytes: 0 }));
        populate?.(new Store(db));
      }).immediate();
      return new Store(db);
    } catch (e) {
      db.close();
      for (const suffix of ['', '-wal', '-shm']) rmSync(path + suffix, { force: true });
      throw e;
    }
  }
  static open(path: string): Store {
    if (!existsSync(path)) fail('NOT_INITIALIZED', 'Initialize this workspace before connecting.');
    const db = new Database(path, { strict: true, readwrite: true });
    try {
      db.exec('PRAGMA busy_timeout = 5000; PRAGMA synchronous = FULL;');
      const version = db.query('PRAGMA user_version').get() as { user_version: number };
      if (version.user_version !== 1) fail('SCHEMA_MISMATCH', 'Unsupported database version.');
      const store = new Store(db); store.workspace(); store.persona(); return store;
    } catch (e) { db.close(); throw e; }
  }
  close() { if (!this.closed) { this.db.close(); this.closed = true; } }
  metadata(key: string): unknown {
    const row = this.db.query('SELECT value FROM metadata WHERE key = ?').get(key) as { value: string } | null;
    if (!row) fail('CORRUPT_STORE', 'Missing workspace metadata.');
    return JSON.parse(row.value);
  }
  workspace(): Workspace { return WorkspaceSchema.parse(this.metadata('workspace')); }
  persona(): Persona {
    const persona = PersonaSchema.parse(this.metadata('persona'));
    if (hash(persona.content) !== persona.hash) fail('CORRUPT_STORE', 'Persona digest mismatch.');
    return persona;
  }
  grant(clientId: string, scopes: Scope[]): Grant {
    z.string().regex(/^[a-zA-Z0-9][a-zA-Z0-9._-]{0,79}$/).parse(clientId);
    const validated = z.array(ScopeSchema).min(1).parse(scopes);
    const token = randomBytes(32).toString('hex');
    this.db.query('INSERT INTO clients(id, token_hash, scopes) VALUES (?, ?, ?)').run(clientId, hash(token), JSON.stringify([...new Set(validated)]));
    return { clientId, token, scopes: validated };
  }
  revoke(clientId: string) {
    const result = this.db.query('UPDATE clients SET revoked = 1 WHERE id = ?').run(clientId);
    if (!result.changes) fail('NOT_FOUND', 'Unknown client.');
  }
  authorize(auth: Auth, scope: Scope) {
    const row = this.db.query('SELECT token_hash, scopes, revoked FROM clients WHERE id = ?').get(auth.clientId) as { token_hash: string; scopes: string; revoked: number } | null;
    if (!row || row.revoked || !timingSafeEqual(Buffer.from(row.token_hash), Buffer.from(hash(auth.token)))) fail('UNAUTHORIZED', 'Client is unknown, revoked, or has an invalid token.');
    const scopes = z.array(ScopeSchema).parse(JSON.parse(row.scopes));
    if (!scopes.includes(scope)) fail('FORBIDDEN', `This client needs ${scope}.`);
  }
  private readRecord(id: string): ContextRecord {
    const row = this.db.query('SELECT payload FROM records WHERE id = ?').get(id) as { payload: string } | null;
    if (!row) fail('NOT_FOUND', 'Record does not exist.');
    return parseRecord(JSON.parse(row.payload));
  }
  get(auth: Auth, id: string): ContextRecord {
    return this.db.transaction(() => { this.authorize(auth, 'read'); return this.readRecord(z.uuid().parse(id)); })();
  }
  context(auth: Auth, raw: unknown) {
    const query = ContextQuerySchema.parse(raw);
    return this.db.transaction(() => {
      this.authorize(auth, 'read');
      const where = "status NOT IN ('retracted', 'superseded', 'cancelled') AND (? IS NULL OR kind = ?) AND (? IS NULL OR instr(lower(title || ' ' || content), lower(?)) > 0)";
      const args = [query.kind ?? null, query.kind ?? null, query.query ?? null, query.query ?? null] as const;
      const rows = this.db.query(`SELECT payload FROM records WHERE ${where} ORDER BY updated_at DESC, id LIMIT ?`).all(...args, query.limit) as { payload: string }[];
      const total = (this.db.query(`SELECT COUNT(*) AS count FROM records WHERE ${where}`).get(...args) as { count: number }).count;
      const records = rows.map(row => {
        const record = parseRecord(JSON.parse(row.payload));
        return { id: record.id, kind: record.kind, title: record.title, status: record.status, revision: record.revision, actor: record.actor, updatedAt: record.updatedAt, excerpt: record.content.slice(0, 400), truncated: record.content.length > 400 };
      });
      const work = this.db.query("SELECT id, kind, status FROM records WHERE (kind = 'task' AND status IN ('doing','blocked')) OR (kind = 'action' AND status IN ('pending','unknown')) ORDER BY updated_at DESC, id LIMIT 21").all() as { id: string; kind: string; status: string }[];
      return { workspace: this.workspace(), persona: this.persona(), capturedAt: new Date().toISOString(), records, omitted: total - records.length, unsettledWork: work.slice(0, 20), unsettledWorkTruncated: work.length > 20, note: 'Excerpts are navigation, not complete records. Resolve IDs before editing or acting. A handoff does not pause running jobs.' };
    })();
  }
  receipt(auth: Auth, operationId: string): Receipt {
    return this.db.transaction(() => {
      this.authorize(auth, 'read');
      const row = this.db.query('SELECT receipt FROM operations WHERE actor = ? AND generation = ? AND operation_id = ?').get(auth.clientId, this.workspace().generation, z.uuid().parse(operationId)) as { receipt: string } | null;
      if (!row) fail('NOT_FOUND', 'No committed receipt for this client and generation.');
      return JSON.parse(row.receipt);
    })();
  }
  write(auth: Auth, raw: unknown): Receipt {
    const input = WriteSchema.parse(raw);
    return this.db.transaction(() => {
      this.authorize(auth, `${input.record.kind}:write` as Scope);
      const workspace = this.workspace();
      if (input.workspaceId !== workspace.id) fail('WORKSPACE_MISMATCH', 'Fetch the current workspace.');
      if (input.generation !== workspace.generation) fail('STALE_GENERATION', 'Workspace was restored; reconnect and reconcile current records.');
      const requestHash = hash(canonical(input));
      const prior = this.db.query('SELECT request_hash, receipt FROM operations WHERE actor = ? AND generation = ? AND operation_id = ?').get(auth.clientId, input.generation, input.operationId) as { request_hash: string; receipt: string } | null;
      if (prior) {
        if (prior.request_hash !== requestHash) fail('OPERATION_REUSED', 'An operation ID cannot be reused for a different request.');
        return JSON.parse(prior.receipt) as Receipt;
      }
      const previous = input.id ? this.readRecord(input.id) : undefined;
      if (previous && previous.revision !== input.expectedRevision) fail('CONFLICT', 'Record changed; retrieve it before proposing a new edit.');
      if (previous && previous.kind !== input.record.kind) fail('KIND_MISMATCH', 'Record kind cannot change.');
      for (const ref of input.record.references) {
        const found = this.db.query('SELECT id FROM records WHERE id = ?').get(ref);
        if (!found) fail('REFERENCE_NOT_FOUND', 'Referenced record does not exist in this workspace.');
      }
      const now = new Date(Math.max(Date.now(), previous ? Date.parse(previous.updatedAt) : 0)).toISOString();
      const record: ContextRecord = { ...input.record, id: previous?.id ?? crypto.randomUUID(), revision: (previous?.revision ?? 0) + 1, actor: auth.clientId, createdAt: previous?.createdAt ?? now, updatedAt: now };
      const receipt: Receipt = { operationId: input.operationId, generation: input.generation, committedAt: now, record };
      const usage = this.metadata('usage') as Usage;
      const nextUsage = { records: usage.records + (previous ? 0 : 1), operations: usage.operations + 1,
        payloadBytes: usage.payloadBytes + Buffer.byteLength(JSON.stringify(record)) + Buffer.byteLength(JSON.stringify(receipt)) - (previous ? Buffer.byteLength(JSON.stringify(previous)) : 0) };
      checkUsage(nextUsage);
      this.insertRecord(record);
      this.db.query("UPDATE metadata SET value = ? WHERE key = 'usage'").run(JSON.stringify(nextUsage));
      this.db.query('INSERT INTO operations VALUES (?, ?, ?, ?, ?)').run(auth.clientId, input.generation, input.operationId, requestHash, JSON.stringify(receipt));
      return receipt;
    }).immediate();
  }
  // Trusted initialization/restore hook only; never expose through an MCP tool.
  rebuildUsage() {
    const recordStats = this.db.query('SELECT COUNT(*) AS count, COALESCE(SUM(length(CAST(payload AS BLOB))), 0) AS bytes FROM records').get() as { count: number; bytes: number };
    const operationStats = this.db.query('SELECT COUNT(*) AS count, COALESCE(SUM(length(CAST(receipt AS BLOB))), 0) AS bytes FROM operations').get() as { count: number; bytes: number };
    const usage: Usage = { records: recordStats.count, operations: operationStats.count, payloadBytes: recordStats.bytes + operationStats.bytes };
    checkUsage(usage);
    this.db.query("UPDATE metadata SET value = ? WHERE key = 'usage'").run(JSON.stringify(usage));
  }
  // Trusted transaction primitive used by write and fresh restore.
  insertRecord(record: ContextRecord) {
    this.db.query('INSERT INTO records VALUES (?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET kind=excluded.kind, status=excluded.status, revision=excluded.revision, updated_at=excluded.updated_at, title=excluded.title, content=excluded.content, payload=excluded.payload').run(record.id, record.kind, record.status, record.revision, record.updatedAt, record.title, record.content, JSON.stringify(record));
  }
}
