import { afterEach, expect, test } from 'bun:test';
import { mkdtempSync, rmSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { Store } from '../src/store';
import { exportBundle, restoreBundle } from '../src/portability';
const dirs: string[] = []; const stores: Store[] = [];
afterEach(() => { for (const s of stores.splice(0)) s.close(); for (const d of dirs.splice(0)) rmSync(d, { recursive: true, force: true }); });
function setup() {
  const dir = mkdtempSync(join(tmpdir(), 'mother-portable-')); dirs.push(dir);
  const store = Store.initialize(join(dir, 'source.sqlite'), 'A synthetic personal companion.'); stores.push(store);
  const auth = store.grant('source', ['read', 'memory:write', 'artifact:write', 'action:write', 'checkpoint:write']);
  const write = (record: unknown) => store.write(auth, { workspaceId: store.workspace().id, generation: store.workspace().generation, operationId: crypto.randomUUID(), expectedRevision: 0, record });
  const artifact = write({ kind: 'artifact', title: 'Plan', content: '# A portable plan', status: 'active', mediaType: 'text/markdown' });
  const action = write({ kind: 'action', title: 'Booking outcome', content: 'Timed out; destination not reconciled.', status: 'unknown', destination: 'synthetic booking service', references: [artifact.record.id], sources: [{ label: 'Selected source', uri: 'https://example.com/evidence' }] });
  return { dir, store, auth, artifact, action };
}
test('export restores identity, persona, records and uncertain outcomes without credentials', () => {
  const { dir, store, auth, artifact, action } = setup(); const before = store.workspace();
  const bundle = exportBundle(store); const serialized = JSON.stringify(bundle);
  expect(serialized).not.toContain(auth.token); expect(serialized).not.toContain('token_hash');
  expect(bundle.omittedExternalSources).toEqual(['https://example.com/evidence']);
  store.close();
  rmSync(join(dir, 'source.sqlite')); // restoration cannot read the original store
  const restored = restoreBundle(join(dir, 'restored.sqlite'), bundle); stores.push(restored);
  expect(restored.workspace().id).toBe(before.id); expect(restored.workspace().generation).not.toBe(before.generation);
  expect(restored.persona().content).toBe('A synthetic personal companion.');
  expect(() => restored.context(auth, {})).toThrow('UNAUTHORIZED');
  const next = restored.grant('destination', ['read', 'memory:write']);
  expect(restored.get(next, artifact.record.id)).toEqual(artifact.record);
  expect(restored.get(next, action.record.id).status).toBe('unknown');
  expect(exportBundle(restored).operations).toEqual(bundle.operations);
  expect(() => restored.write(next, { workspaceId: before.id, generation: before.generation, operationId: crypto.randomUUID(), expectedRevision: 0, record: { kind: 'memory', status: 'active', title: 'Old write', content: 'Stale', basis: 'source' } })).toThrow('STALE_GENERATION');
});
test('restore validates references, duplicates, persona digest, unknown fields and schema before creating files', () => {
  const { dir, store } = setup(); const bundle = exportBundle(store); const path = join(dir, 'invalid.sqlite');
  for (const invalid of [
    { ...bundle, schemaVersion: 99 },
    { ...bundle, credentials: 'not permitted' },
    { ...bundle, persona: { ...bundle.persona, content: 'tampered' } },
    { ...bundle, records: [...bundle.records, bundle.records[0]] },
    { ...bundle, records: bundle.records.map((r, i) => i === 0 ? { ...r, references: [crypto.randomUUID()] } : r) },
    { ...bundle, operations: [...bundle.operations, bundle.operations[0]] },
  ]) {
    expect(() => restoreBundle(path, invalid)).toThrow(); expect(existsSync(path)).toBe(false);
  }
});
test('restore refuses existing destinations and preserves retracted memories', () => {
  const { dir, store, auth } = setup(); const w = store.workspace();
  const first = store.write(auth, { workspaceId: w.id, generation: w.generation, operationId: crypto.randomUUID(), expectedRevision: 0, record: { kind: 'memory', title: 'Old preference', content: 'No longer current', status: 'retracted', basis: 'owner_statement' } });
  const bundle = exportBundle(store);
  expect(() => restoreBundle(join(dir, 'source.sqlite'), bundle)).toThrow();
  const dest = restoreBundle(join(dir, 'fresh.sqlite'), bundle); stores.push(dest); const next = dest.grant('reader', ['read']);
  expect(dest.get(next, first.record.id).status).toBe('retracted');
  expect(dest.context(next, {}).records.some(r => r.id === first.record.id)).toBe(false);
});
