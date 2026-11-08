import { afterEach, expect, setSystemTime, test } from 'bun:test';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { tmpdir } from 'node:os';
import { Store } from '../src/store';
import { exportBundle } from '../src/portability';
const dirs: string[] = []; const stores: Store[] = [];
afterEach(() => { for (const s of stores.splice(0)) s.close(); for (const d of dirs.splice(0)) rmSync(d, { recursive: true, force: true }); });
function setup() {
  const dir = mkdtempSync(join(tmpdir(), 'mother-boundary-')); dirs.push(dir); const path = join(dir, 'state.sqlite');
  const store = Store.initialize(path); stores.push(store); const auth = store.grant('owner-client', ['read', 'memory:write']);
  const w = store.workspace(); const record = { kind: 'memory', title: 'Preference', content: 'Tea', status: 'active', basis: 'owner_statement' };
  const request = { workspaceId: w.id, generation: w.generation, operationId: crypto.randomUUID(), expectedRevision: 0, record };
  return { dir, store, auth, request };
}
test('clock rollback cannot commit an unreadable or unexportable correction', () => {
  const { store, auth, request } = setup(); const first = store.write(auth, request);
  setSystemTime(new Date(0));
  try {
    const updated = store.write(auth, { ...request, operationId: crypto.randomUUID(), id: first.record.id, expectedRevision: 1 });
    expect(store.get(auth, updated.record.id)).toEqual(updated.record);
    expect(updated.record.updatedAt >= first.record.updatedAt).toBe(true);
    expect(exportBundle(store).records).toHaveLength(1);
  } finally { setSystemTime(); }
});
test('record admission bound keeps the existing workspace exportable and does not consume failed operations', () => {
  const { store, auth, request } = setup(); const first = store.write(auth, request);
  store.db.transaction(() => {
    for (let i = 1; i < 10000; i++) store.insertRecord({ ...first.record, id: crypto.randomUUID() });
    store.rebuildUsage();
  }).immediate();
  expect(() => store.write(auth, { ...request, operationId: crypto.randomUUID() })).toThrow('RESOURCE_LIMIT');
  expect(exportBundle(store).records).toHaveLength(10000);
  expect(store.context(auth, {}).omitted).toBe(9980);
  expect(store.write(auth, { ...request, operationId: crypto.randomUUID(), id: first.record.id, expectedRevision: 1 }).record.revision).toBe(2);
}, 15000);
test('killing restore during record import cannot publish a valid empty workspace', () => {
  const { dir, store, auth, request } = setup(); store.write(auth, request);
  const bundlePath = join(dir, 'bundle.private.json'); writeFileSync(bundlePath, JSON.stringify(exportBundle(store)));
  const destination = join(dir, 'interrupted.sqlite');
  const code = `import { Store } from ${JSON.stringify(resolve('src/store.ts'))}; import { restoreBundle } from ${JSON.stringify(resolve('src/portability.ts'))}; Store.prototype.insertRecord = function() { process.exit(73); }; restoreBundle(${JSON.stringify(destination)}, JSON.parse(await Bun.file(${JSON.stringify(bundlePath)}).text()));`;
  const killed = Bun.spawnSync([process.execPath, '-e', code]);
  expect(killed.exitCode).toBe(73);
  expect(() => { const opened = Store.open(destination); stores.push(opened); }).toThrow('SCHEMA_MISMATCH');
});
