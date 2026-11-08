import { z } from 'zod';
export const ScopeSchema = z.enum(['read', 'memory:write', 'decision:write', 'task:write', 'checkpoint:write', 'artifact:write', 'action:write']);
export type Scope = z.infer<typeof ScopeSchema>;
export const ALL_SCOPES = ScopeSchema.options;
export const Id = z.uuid();
export const SourceSchema = z.strictObject({ label: z.string().trim().min(1).max(200), uri: z.url().refine(v => v.startsWith('https://'), 'Sources must use HTTPS') });
const common = { title: z.string().trim().min(1).max(240), content: z.string().trim().min(1).max(12000), references: z.array(Id).max(30).default([]), sources: z.array(SourceSchema).max(20).default([]) };
export const DraftSchema = z.discriminatedUnion('kind', [
  z.strictObject({ ...common, kind: z.literal('memory'), status: z.enum(['active', 'retracted']), basis: z.enum(['owner_statement', 'source', 'model_inference']) }),
  z.strictObject({ ...common, kind: z.literal('decision'), status: z.enum(['proposed', 'confirmed', 'superseded', 'retracted']) }),
  z.strictObject({ ...common, kind: z.literal('task'), status: z.enum(['todo', 'doing', 'blocked', 'done', 'cancelled']) }),
  z.strictObject({ ...common, kind: z.literal('checkpoint'), status: z.enum(['active', 'superseded']), nextStep: z.string().trim().min(1).max(4000), completed: z.array(z.string().max(2000)).max(30).default([]), openQuestions: z.array(z.string().max(2000)).max(30).default([]) }),
  z.strictObject({ ...common, kind: z.literal('artifact'), status: z.enum(['active', 'retracted']), mediaType: z.enum(['text/plain', 'text/markdown']) }),
  z.strictObject({ ...common, kind: z.literal('action'), status: z.enum(['pending', 'succeeded', 'failed', 'unknown']), destination: z.string().trim().min(1).max(500) }),
]);
export type Draft = z.infer<typeof DraftSchema>;
export const MetadataSchema = z.strictObject({ id: Id, revision: z.number().int().positive(), actor: z.string().min(1).max(80), createdAt: z.iso.datetime(), updatedAt: z.iso.datetime() });
export type ContextRecord = Draft & z.infer<typeof MetadataSchema>;
// Validate fields separately so strict draft validation also rejects unknown metadata.
export function parseRecord(value: unknown): ContextRecord {
  const raw = z.record(z.string(), z.unknown()).parse(value);
  const { id, revision, actor, createdAt, updatedAt, ...draft } = raw;
  const meta = MetadataSchema.parse({ id, revision, actor, createdAt, updatedAt });
  if (meta.updatedAt < meta.createdAt) throw new Error('Invalid record timestamps');
  return { ...DraftSchema.parse(draft), ...meta };
}
export const WriteSchema = z.strictObject({ workspaceId: Id, generation: Id, operationId: Id, id: Id.optional(), expectedRevision: z.number().int().nonnegative(), record: DraftSchema }).refine(v => v.id !== undefined || v.expectedRevision === 0, 'New records require revision zero; updates require an ID');
export const ContextQuerySchema = z.strictObject({ query: z.string().max(200).optional(), kind: z.enum(['memory', 'decision', 'task', 'checkpoint', 'artifact', 'action']).optional(), limit: z.number().int().min(1).max(100).default(20) });
export const WorkspaceSchema = z.strictObject({ id: Id, generation: Id, schemaVersion: z.literal(1), createdAt: z.iso.datetime() });
export type Workspace = z.infer<typeof WorkspaceSchema>;
export const PersonaSchema = z.strictObject({ content: z.string().min(1).max(32000), hash: z.string().regex(/^[a-f0-9]{64}$/) });
export type Persona = z.infer<typeof PersonaSchema>;
export interface Auth { clientId: string; token: string }
export interface Grant extends Auth { scopes: Scope[] }
export interface Receipt { operationId: string; generation: string; committedAt: string; record: ContextRecord }
