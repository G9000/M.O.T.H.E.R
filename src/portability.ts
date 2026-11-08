import { z } from 'zod';
import { PersonaSchema, WorkspaceSchema, parseRecord, type ContextRecord, type Receipt } from './contracts';
import { Store, hash, canonical } from './store';
import { fail } from './errors';
import { checkUsage } from './limits';
const OperationSchema = z.strictObject({ actor: z.string().min(1).max(80), generation: z.uuid(), operationId: z.uuid(), requestHash: z.string().regex(/^[a-f0-9]{64}$/), receipt: z.strictObject({ operationId: z.uuid(), generation: z.uuid(), committedAt: z.iso.datetime(), record: z.unknown() }) });
const BundleSchema = z.strictObject({ schemaVersion: z.literal(1), exportedAt: z.iso.datetime(), workspace: WorkspaceSchema, persona: PersonaSchema, records: z.array(z.unknown()).max(10000), operations: z.array(OperationSchema).max(100000), omittedExternalSources: z.array(z.url()) });
export interface ExportOperation { actor: string; generation: string; operationId: string; requestHash: string; receipt: Receipt }
export interface Bundle { schemaVersion: 1; exportedAt: string; workspace: ReturnType<Store['workspace']>; persona: ReturnType<Store['persona']>; records: ContextRecord[]; operations: ExportOperation[]; omittedExternalSources: string[] }
function externalSources(records: ContextRecord[], operations: ExportOperation[]) {
  return [...new Set([...records, ...operations.map(op => op.receipt.record)].flatMap(r => r.sources.map(s => s.uri)))].sort();
}
export function validateBundle(raw: unknown): Bundle {
  const parsed = BundleSchema.parse(raw);
  const records = parsed.records.map(parseRecord);
  const operations: ExportOperation[] = parsed.operations.map(op => ({ ...op, receipt: { ...op.receipt, record: parseRecord(op.receipt.record) } }));
  if (hash(parsed.persona.content) !== parsed.persona.hash) fail('INVALID_EXPORT', 'Persona digest mismatch.');
  const byId = new Map(records.map(r => [r.id, r]));
  if (byId.size !== records.length) fail('INVALID_EXPORT', 'Duplicate record IDs.');
  for (const r of [...records, ...operations.map(o => o.receipt.record)]) {
    for (const ref of r.references) if (!byId.has(ref)) fail('INVALID_EXPORT', 'Unresolved record reference.');
  }
  const opKeys = new Set<string>();
  for (const op of operations) {
    const key = `${op.actor}/${op.generation}/${op.operationId}`;
    if (opKeys.has(key)) fail('INVALID_EXPORT', 'Duplicate operation IDs.');
    opKeys.add(key);
    const receipt = op.receipt; const latest = byId.get(receipt.record.id);
    if (op.actor !== receipt.record.actor || op.generation !== receipt.generation || op.operationId !== receipt.operationId || receipt.committedAt !== receipt.record.updatedAt || !latest || receipt.record.revision > latest.revision) fail('INVALID_EXPORT', 'Invalid operation receipt.');
    if (receipt.record.revision === latest.revision && canonical(receipt.record) !== canonical(latest)) fail('INVALID_EXPORT', 'Receipt disagrees with current record.');
  }
  checkUsage({ records: records.length, operations: operations.length,
    payloadBytes: records.reduce((n, r) => n + Buffer.byteLength(JSON.stringify(r)), 0) + operations.reduce((n, op) => n + Buffer.byteLength(JSON.stringify(op.receipt)), 0) });
  const omitted = externalSources(records, operations);
  if (canonical(omitted) !== canonical([...new Set(parsed.omittedExternalSources)].sort())) fail('INVALID_EXPORT', 'External content omissions are incomplete.');
  return { ...parsed, records, operations, omittedExternalSources: omitted };
}
export function exportBundle(store: Store): Bundle {
  return store.db.transaction(() => {
    const records = (store.db.query('SELECT payload FROM records ORDER BY id').all() as { payload: string }[]).map(r => parseRecord(JSON.parse(r.payload)));
    const operations = (store.db.query('SELECT * FROM operations ORDER BY actor, generation, operation_id').all() as { actor: string; generation: string; operation_id: string; request_hash: string; receipt: string }[]).map(op => ({ actor: op.actor, generation: op.generation, operationId: op.operation_id, requestHash: op.request_hash, receipt: JSON.parse(op.receipt) as Receipt }));
    return validateBundle({ schemaVersion: 1, exportedAt: new Date().toISOString(), workspace: store.workspace(), persona: store.persona(), records, operations, omittedExternalSources: externalSources(records, operations) });
  })();
}
export function restoreBundle(path: string, raw: unknown): Store {
  const bundle = validateBundle(raw);
  // Schema, identity, records and receipts become visible in one commit.
  return Store.initialize(path, bundle.persona.content, store => {
    store.db.query("UPDATE metadata SET value = ? WHERE key = 'workspace'").run(JSON.stringify({ ...bundle.workspace, generation: crypto.randomUUID() }));
    for (const record of bundle.records) store.insertRecord(record);
    for (const op of bundle.operations) store.db.query('INSERT INTO operations VALUES (?, ?, ?, ?, ?)').run(op.actor, op.generation, op.operationId, op.requestHash, JSON.stringify(op.receipt));
    store.rebuildUsage();
  });
}
