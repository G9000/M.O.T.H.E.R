import { afterEach, expect, test } from 'bun:test';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { tmpdir } from 'node:os';
import { Client } from '@modelcontextprotocol/sdk/client/index.js';
import { StdioClientTransport } from '@modelcontextprotocol/sdk/client/stdio.js';
import { Store } from '../src/store';
import type { Grant, Receipt } from '../src/contracts';
const dirs: string[] = []; const stores: Store[] = []; const clients: Client[] = [];
afterEach(async () => { for (const client of clients.splice(0)) await client.close(); for (const s of stores.splice(0)) s.close(); for (const d of dirs.splice(0)) rmSync(d, { recursive: true, force: true }); });
async function connect(dir: string, databasePath: string, grant: Grant, suffix = '') {
  const config = join(dir, grant.clientId + suffix + '.private.json');
  writeFileSync(config, JSON.stringify({ version: 1, databasePath, clientId: grant.clientId, token: grant.token }));
  const client = new Client({ name: 'synthetic-' + grant.clientId, version: '1.0.0' }); clients.push(client);
  await client.connect(new StdioClientTransport({ command: process.execPath, args: [resolve('src/main.ts'), '--config', config], stderr: 'pipe' }));
  return client;
}
async function call<T>(client: Client, name: string, args: Record<string, unknown> = {}): Promise<T> {
  const result = await client.callTool({ name, arguments: args });
  if (result.isError) throw new Error(JSON.stringify(result.structuredContent ?? result.content));
  return result.structuredContent as T;
}
async function setup() {
  const dir = mkdtempSync(join(tmpdir(), 'mother-mcp-')); dirs.push(dir); const path = join(dir, 'state.sqlite');
  const store = Store.initialize(path); stores.push(store);
  const grantA = store.grant('harness-a', ['read', 'memory:write', 'task:write', 'checkpoint:write']);
  const grantB = store.grant('harness-b', ['read', 'memory:write', 'task:write', 'checkpoint:write']);
  const a = await connect(dir, path, grantA); const b = await connect(dir, path, grantB);
  const w = store.workspace();
  const request = (record: unknown, extra: Record<string, unknown> = {}) => ({ workspaceId: w.id, generation: w.generation, operationId: crypto.randomUUID(), expectedRevision: 0, record, ...extra });
  return { dir, path, store, grantA, grantB, a, b, request };
}
const memory = { kind: 'memory', title: 'Tea', content: 'Green tea', basis: 'owner_statement', status: 'active' };
test('real stdio round trip: handoff, correction, task progress, restart and connected revocation', async () => {
  const { dir, path, store, grantA, grantB, a, b, request } = await setup();
  const first = await call<Receipt>(a, 'mother_write_record', request(memory));
  const task = await call<Receipt>(a, 'mother_write_record', request({ kind: 'task', title: 'Plan a weekend', content: 'Choose a destination', status: 'todo' }));
  const checkpoint = await call<Receipt>(a, 'mother_checkpoint', request({ kind: 'checkpoint', title: 'Weekend handoff', content: 'Destination still open', status: 'active', nextStep: 'Choose a destination with the owner', references: [first.record.id, task.record.id] }));
  const context = await call<ReturnType<Store['context']>>(b, 'mother_get_context');
  expect(context.workspace.id).toBe(store.workspace().id); expect(context.records.some(r => r.id === checkpoint.record.id)).toBe(true);
  const corrected = await call<Receipt>(b, 'mother_write_record', request({ ...memory, content: 'Black tea' }, { id: first.record.id, expectedRevision: 1 }));
  await call<Receipt>(b, 'mother_write_record', request({ kind: 'task', title: 'Plan a weekend', content: 'Destination selected', status: 'done' }, { id: task.record.id, expectedRevision: 1 }));
  expect(corrected.record.actor).toBe('harness-b');
  await a.close(); const resumed = await connect(dir, path, grantA, '-resumed');
  expect(await call(resumed, 'mother_get_record', { id: first.record.id })).toEqual(corrected.record);
  expect((await call<{ status: string }>(resumed, 'mother_get_record', { id: task.record.id })).status).toBe('done');
  const stale = await b.callTool({ name: 'mother_write_record', arguments: request(memory, { id: first.record.id, expectedRevision: 1 }) });
  expect(stale.isError).toBe(true); expect(stale.structuredContent).toMatchObject({ error: { code: 'CONFLICT' } });
  store.revoke(grantB.clientId);
  const revoked = await b.callTool({ name: 'mother_get_context', arguments: {} });
  expect(revoked.isError).toBe(true); expect(revoked.structuredContent).toMatchObject({ error: { code: 'UNAUTHORIZED' } });
}, 15000);
test('two processes race one revision: exactly one commit survives and both see it after restart', async () => {
  const { dir, path, store, grantA, grantB, a, b, request } = await setup();
  const first = await call<Receipt>(a, 'mother_write_record', request(memory));
  const results = await Promise.all([a, b].map((client, i) => client.callTool({ name: 'mother_write_record', arguments: request({ ...memory, content: 'Winner ' + i }, { id: first.record.id, expectedRevision: 1 }) })));
  expect(results.filter(r => !r.isError)).toHaveLength(1); expect(results.filter(r => r.isError)).toHaveLength(1);
  expect(results.find(r => r.isError)!.structuredContent).toMatchObject({ error: { code: 'CONFLICT' } });
  const winner = results.find(r => !r.isError)!.structuredContent as unknown as Receipt;
  expect(winner.record.revision).toBe(2);
  await a.close(); await b.close();
  const a2 = await connect(dir, path, grantA, '-restart'); const b2 = await connect(dir, path, grantB, '-restart');
  expect(await call(a2, 'mother_get_record', { id: first.record.id })).toEqual(winner.record);
  expect(await call(b2, 'mother_get_record', { id: first.record.id })).toEqual(winner.record);
  expect(store.receipt(winner.record.actor === grantA.clientId ? grantA : grantB, winner.operationId)).toEqual(winner);
}, 15000);
test('duplicate-operation race creates one record and read-only clients cannot mutate', async () => {
  const { dir, path, store, grantA, a, request } = await setup(); const twin = await connect(dir, path, grantA, '-twin');
  const mutation = request(memory);
  const [one, two] = await Promise.all([call<Receipt>(a, 'mother_write_record', mutation), call<Receipt>(twin, 'mother_write_record', mutation)]);
  expect(one).toEqual(two); expect(store.context(grantA, {}).records).toHaveLength(1);
  expect(await call(a, 'mother_get_receipt', { operationId: mutation.operationId })).toEqual(one);
  const reader = await connect(dir, path, store.grant('reader', ['read']));
  const denied = await reader.callTool({ name: 'mother_write_record', arguments: request(memory) });
  expect(denied.isError).toBe(true); expect(denied.structuredContent).toMatchObject({ error: { code: 'FORBIDDEN' } });
}, 15000);
