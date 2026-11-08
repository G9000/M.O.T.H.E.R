import { afterEach, describe, expect, test } from 'bun:test';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { Store } from '../src/store';

const dirs: string[] = [];
const stores: Store[] = [];
afterEach(() => { for (const s of stores.splice(0)) s.close(); for (const d of dirs.splice(0)) rmSync(d, { recursive: true, force: true }); });
function setup() {
  const dir = mkdtempSync(join(tmpdir(), 'mother-test-')); dirs.push(dir);
  const path = join(dir, 'context.sqlite');
  const store = Store.initialize(path); stores.push(store);
  const auth = store.grant('client-a', ['read', 'memory:write', 'task:write', 'checkpoint:write', 'action:write']);
  return { store, auth, path };
}
const memory = { kind: 'memory', title: 'Tea', content: 'Prefers green tea', status: 'active', basis: 'owner_statement', references: [], sources: [] };
function input(store: Store, extra: Record<string, unknown> = {}) {
  const w = store.workspace();
  return { workspaceId: w.id, generation: w.generation, operationId: crypto.randomUUID(), expectedRevision: 0, record: memory, ...extra };
}
describe('canonical personal context', () => {
  test('two connections see one stable identity and attributed records after reopen', () => {
    const { store, auth, path } = setup();
    const saved = store.write(auth, input(store));
    const other = Store.open(path); stores.push(other);
    expect(other.workspace()).toEqual(store.workspace());
    expect(other.get(auth, saved.record.id).content).toBe('Prefers green tea');
    expect(saved.record.actor).toBe('client-a');
  });
  test('correction rejects a stale writer and preserves current content', () => {
    const { store, auth } = setup(); const first = store.write(auth, input(store));
    const corrected = store.write(auth, input(store, { id: first.record.id, expectedRevision: 1, record: { ...memory, content: 'Prefers black tea' } }));
    expect(corrected.record.revision).toBe(2);
    expect(() => store.write(auth, input(store, { id: first.record.id, expectedRevision: 1 }))).toThrow('CONFLICT');
    expect(store.get(auth, first.record.id).content).toBe('Prefers black tea');
  });
  test('same operation survives retry and changed payload is rejected', () => {
    const { store, auth, path } = setup(); const request = input(store); const receipt = store.write(auth, request);
    const other = Store.open(path); stores.push(other);
    expect(other.write(auth, request)).toEqual(receipt);
    expect(() => other.write(auth, { ...request, record: { ...memory, content: 'Changed' } })).toThrow('OPERATION_REUSED');
    expect(store.context(auth, {}).records).toHaveLength(1);
  });
  test('read-only and revoked clients cannot write, revoked clients cannot read', () => {
    const { store, auth } = setup(); const readonly = store.grant('reader', ['read']);
    expect(() => store.write(readonly, input(store))).toThrow('FORBIDDEN');
    store.revoke(auth.clientId);
    expect(() => store.context(auth, {})).toThrow('UNAUTHORIZED');
    expect(() => store.write(auth, input(store))).toThrow('UNAUTHORIZED');
  });
  test('wrong token, workspace, restore generation and actor spoofing are rejected', () => {
    const { store, auth } = setup();
    expect(() => store.context({ ...auth, token: 'incorrect' }, {})).toThrow('UNAUTHORIZED');
    expect(() => store.write(auth, input(store, { workspaceId: crypto.randomUUID() }))).toThrow('WORKSPACE_MISMATCH');
    expect(() => store.write(auth, input(store, { generation: crypto.randomUUID() }))).toThrow('STALE_GENERATION');
    expect(() => store.write(auth, { ...input(store), actor: 'owner' })).toThrow();
  });
  test('retractions disappear from context but remain explicitly retrievable', () => {
    const { store, auth } = setup(); const saved = store.write(auth, input(store));
    store.write(auth, input(store, { id: saved.record.id, expectedRevision: 1, record: { ...memory, status: 'retracted' } }));
    expect(store.context(auth, {}).records).toHaveLength(0);
    expect(store.get(auth, saved.record.id).status).toBe('retracted');
  });
  test('missing references fail atomically without consuming operation ID', () => {
    const { store, auth } = setup(); const missing = crypto.randomUUID(); const op = crypto.randomUUID();
    expect(() => store.write(auth, input(store, { operationId: op, record: { ...memory, references: [missing] } }))).toThrow('REFERENCE_NOT_FOUND');
    expect(store.write(auth, input(store, { operationId: op })).record.revision).toBe(1);
  });
  test('bounded context reports omissions and literal search does not treat wildcards specially', () => {
    const { store, auth } = setup(); store.write(auth, input(store)); store.write(auth, input(store));
    const result = store.context(auth, { limit: 1 });
    expect(result.records).toHaveLength(1); expect(result.omitted).toBe(1);
    expect(store.context(auth, { query: '%' }).records).toHaveLength(0);
    expect(store.context(auth, { query: 'green tea' }).records).toHaveLength(2);
  });
  test('record kind cannot change and create/update revisions are unambiguous', () => {
    const { store, auth } = setup(); const saved = store.write(auth, input(store));
    expect(() => store.write(auth, input(store, { expectedRevision: 1 }))).toThrow();
    expect(() => store.write(auth, input(store, { id: saved.record.id, expectedRevision: 0 }))).toThrow('CONFLICT');
    expect(() => store.write(auth, input(store, { id: saved.record.id, expectedRevision: 1, record: { kind: 'task', title: 'Tea', content: 'Buy tea', status: 'todo', references: [], sources: [] } }))).toThrow('KIND_MISMATCH');
  });
  test('initialization never overwrites an existing database', () => {
    const { path, store } = setup(); const id = store.workspace().id;
    expect(() => Store.initialize(path)).toThrow(); expect(store.workspace().id).toBe(id);
  });
});
