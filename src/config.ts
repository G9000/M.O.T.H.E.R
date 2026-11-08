import { readFileSync, statSync } from 'node:fs';
import { homedir } from 'node:os';
import { isAbsolute, join, resolve } from 'node:path';
import { z } from 'zod';
export const ClientConfigSchema = z.strictObject({ version: z.literal(1), databasePath: z.string().refine(isAbsolute, 'Database path must be absolute'), clientId: z.string().regex(/^[a-zA-Z0-9][a-zA-Z0-9._-]{0,79}$/), token: z.string().regex(/^[a-f0-9]{64}$/) });
export type ClientConfig = z.infer<typeof ClientConfigSchema>;
export function readJson(path: string, maxBytes = 64 * 1024 * 1024): unknown {
  if (statSync(path).size > maxBytes) throw new Error('File exceeds the supported size limit.');
  return JSON.parse(readFileSync(path, 'utf8').replace(/^\uFEFF/, ''));
}
export function loadConfig(path: string): ClientConfig { return ClientConfigSchema.parse(readJson(resolve(path), 16384)); }
export function dataDirectory(): string {
  if (process.env.MOTHER_HOME) {
    if (!isAbsolute(process.env.MOTHER_HOME)) throw new Error('MOTHER_HOME must be absolute.');
    return process.env.MOTHER_HOME;
  }
  return process.platform === 'win32' && process.env.LOCALAPPDATA ? join(process.env.LOCALAPPDATA, 'MOTHER') : join(homedir(), '.mother');
}
