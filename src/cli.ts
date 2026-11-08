import { closeSync, mkdirSync, openSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { z } from 'zod';
import { ClientConfigSchema, dataDirectory, loadConfig, readJson } from './config';
import { ScopeSchema, type Scope } from './contracts';
import { Store } from './store';
import { exportBundle, restoreBundle } from './portability';
import { MotherError } from './errors';
import { MAX_EXPORT_BYTES } from './limits';
const help = `M.O.T.H.E.R — your independent personal workspace

bun run mother init [--db PATH]
bun run mother grant CLIENT [--db PATH] [--scopes read,memory:write,...] [--out CLIENT.private.json]
bun run mother revoke CLIENT [--db PATH]
bun run mother context --config CLIENT.private.json
bun run mother connection --config CLIENT.private.json [--format json|codex]
bun run mother export --out BACKUP.private.json [--db PATH]
bun run mother restore --from BACKUP.private.json --db NEW_PATH

MOTHER_HOME selects an absolute private data directory. Defaults to LocalAppData/MOTHER
on Windows or ~/.mother elsewhere. Grants default to read-only. Exports omit grants.
Each harness needs its own grant/config. Connection prints configuration without tokens.
`;
function parse(args: string[]) {
  const command = args.shift() ?? 'help';
  const flags: Record<string, string> = {}; const positionals: string[] = [];
  while (args.length) {
    const arg = args.shift()!;
    if (arg.startsWith('--')) {
      const value = args.shift();
      if (!value || value.startsWith('--') || flags[arg] !== undefined) throw new Error('Flags require one value and may not repeat.');
      flags[arg] = value;
    } else positionals.push(arg);
  }
  const allowed: Record<string, string[]> = { help: [], init: ['--db'], grant: ['--db', '--scopes', '--out'], revoke: ['--db'], context: ['--config'], connection: ['--config', '--format'], export: ['--db', '--out'], restore: ['--db', '--from'] };
  if (!allowed[command] || Object.keys(flags).some(f => !allowed[command]!.includes(f))) throw new Error('Unknown command or flag. Run: bun run mother help');
  if (positionals.length !== (['grant', 'revoke'].includes(command) ? 1 : 0)) throw new Error('Unexpected or missing positional argument.');
  return { command, flags, positionals };
}
function required(flags: Record<string, string>, key: string) { const value = flags[key]; if (!value) throw new Error(`Missing ${key}.`); return resolve(value); }
function exclusiveJson(path: string, value: unknown) {
  const text = JSON.stringify(value) + '\n';
  if (Buffer.byteLength(text) > MAX_EXPORT_BYTES) throw new Error('Export exceeds the 64 MiB portable file limit.');
  writeFileSync(path, text, { flag: 'wx', mode: 0o600 });
}
export function main(args: string[]) {
  const { command, flags, positionals } = parse(args);
  if (command === 'help') { console.log(help); return; }
  if (command === 'connection' || command === 'context') {
    const configPath = required(flags, '--config'); const config = loadConfig(configPath);
    const store = Store.open(config.databasePath);
    try {
      store.authorize(config, 'read');
      if (command === 'context') { console.log(JSON.stringify(store.context(config, {}), null, 2)); return; }
      const connection = { command: process.execPath, args: [fileURLToPath(new URL('./main.ts', import.meta.url)), '--config', configPath] };
      const format = flags['--format'] ?? 'json';
      if (format === 'json') console.log(JSON.stringify(connection, null, 2));
      else if (format === 'codex') console.log('[mcp_servers.mother]\ncommand = ' + JSON.stringify(connection.command) + '\nargs = ' + JSON.stringify(connection.args));
      else throw new Error('Format must be json or codex.');
    } finally { store.close(); }
    return;
  }
  const dbPath = flags['--db'] ? resolve(flags['--db']) : join(dataDirectory(), 'context.sqlite');
  if (command === 'init') {
    mkdirSync(dirname(dbPath), { recursive: true, mode: 0o700 });
    const store = Store.initialize(dbPath);
    try { console.log(JSON.stringify({ databasePath: dbPath, workspace: store.workspace() }, null, 2)); } finally { store.close(); }
    return;
  }
  if (command === 'restore') {
    const destination = required(flags, '--db'); const raw = readJson(required(flags, '--from'));
    // The parent must exist; restoration never overwrites a destination.
    const store = restoreBundle(destination, raw);
    try { console.log(JSON.stringify({ databasePath: destination, workspace: store.workspace(), next: 'Create new client grants for this restored database.' }, null, 2)); } finally { store.close(); }
    return;
  }
  if (command === 'export') {
    const out = required(flags, '--out'); const store = Store.open(dbPath);
    try { exclusiveJson(out, exportBundle(store)); console.log(JSON.stringify({ exportedTo: out, credentialsIncluded: false })); } finally { store.close(); }
    return;
  }
  const clientId = z.string().regex(/^[a-zA-Z0-9][a-zA-Z0-9._-]{0,79}$/).parse(positionals[0]);
  if (command === 'grant') {
    const scopes: Scope[] = z.array(ScopeSchema).min(1).parse((flags['--scopes'] ?? 'read').split(','));
    if (!scopes.includes('read')) throw new Error('MCP clients require the read scope.');
    const out = flags['--out'] ? resolve(flags['--out']) : join(dirname(dbPath), 'clients', clientId + '.private.json');
    const store = Store.open(dbPath);
    try {
      mkdirSync(dirname(out), { recursive: true, mode: 0o700 });
      const fd = openSync(out, 'wx', 0o600);
      try {
        store.db.transaction(() => {
          const grant = store.grant(clientId, scopes);
          const config = ClientConfigSchema.parse({ version: 1, databasePath: dbPath, clientId: grant.clientId, token: grant.token });
          writeFileSync(fd, JSON.stringify(config, null, 2) + '\n');
        }).immediate();
      } catch (e) { closeSync(fd); rmSync(out); throw e; }
      closeSync(fd);
      console.log(JSON.stringify({ clientId, scopes, configPath: out }, null, 2));
    } finally { store.close(); }
    return;
  }
  const store = Store.open(dbPath);
  try { store.revoke(clientId); console.log(JSON.stringify({ clientId, revoked: true })); } finally { store.close(); }
}
if (import.meta.main) {
  try { main(process.argv.slice(2)); }
  catch (error) {
    console.error(error instanceof MotherError ? error.message : error instanceof z.ZodError ? 'Invalid input. Check the documented names, scopes and file formats.' : error instanceof Error ? error.message : 'Command failed.');
    process.exitCode = 1;
  }
}
