import { afterEach, expect, test } from 'bun:test';
import { mkdtempSync, rmSync, readFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { Store } from '../src/store';
const dirs: string[] = [];
afterEach(() => { for (const d of dirs.splice(0)) rmSync(d, { recursive: true, force: true }); });
function fixture() { const dir = mkdtempSync(join(tmpdir(), 'mother-cli-')); dirs.push(dir); return dir; }
function run(dir: string, ...args: string[]) {
  const result = Bun.spawnSync([process.execPath, resolve('src/cli.ts'), ...args], { env: { ...process.env, MOTHER_HOME: dir }, stdout: 'pipe', stderr: 'pipe' });
  return { code: result.exitCode, out: result.stdout.toString(), err: result.stderr.toString() };
}
test('owner can initialize, grant, inspect, export, restore and revoke without leaking credentials', () => {
  const dir = fixture(); expect(run(dir, 'init').code).toBe(0);
  expect(run(dir, 'init').code).not.toBe(0);
  const grant = run(dir, 'grant', 'codex', '--scopes', 'read,memory:write,task:write,checkpoint:write');
  expect(grant.code).toBe(0);
  const configPath = join(dir, 'clients', 'codex.private.json');
  const config = JSON.parse(readFileSync(configPath, 'utf8'));
  expect(grant.out).not.toContain(config.token);
  expect(run(dir, 'context', '--config', configPath).code).toBe(0);
  const connection = run(dir, 'connection', '--config', configPath);
  expect(connection.code).toBe(0); expect(connection.out).not.toContain(config.token);
  expect(JSON.parse(connection.out).args).toEqual([resolve('src/main.ts'), '--config', configPath]);
  expect(run(dir, 'grant', 'codex').code).not.toBe(0);
  const backup = join(dir, 'backup.private.json');
  expect(run(dir, 'export', '--out', backup).code).toBe(0);
  expect(run(dir, 'export', '--out', backup).code).not.toBe(0);
  expect(readFileSync(backup, 'utf8')).not.toContain(config.token);
  const destination = join(dir, 'restored.sqlite');
  expect(run(dir, 'restore', '--from', backup, '--db', destination).code).toBe(0);
  const restored = Store.open(destination);
  expect(() => restored.context(config, {})).toThrow('UNAUTHORIZED'); restored.close();
  expect(run(dir, 'revoke', 'codex').code).toBe(0);
  expect(run(dir, 'context', '--config', configPath).code).not.toBe(0);
});
test('invalid CLI flags and unsafe client names fail before mutation', () => {
  const dir = fixture();
  expect(run(dir, 'init', '--typo').code).not.toBe(0); expect(existsSync(join(dir, 'context.sqlite'))).toBe(false);
  expect(run(dir, 'init').code).toBe(0);
  expect(run(dir, 'grant', '../escape').code).not.toBe(0);
  expect(run(dir, 'grant', 'writer', '--scopes', 'read,admin').code).not.toBe(0);
  expect(run(dir, 'help').code).toBe(0);
});
