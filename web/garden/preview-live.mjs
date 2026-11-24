// Preview the built artwork against an already running owner. All memory reads
// still use the native Explorer's browser authentication and read-only API.
import { execFileSync, spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import http from 'node:http';
import { Readable } from 'node:stream';
import { pipeline } from 'node:stream/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const assetRoot = new URL('../../assets/garden-explorer/', import.meta.url);
const home = process.env.MOTHER_HOME || (process.env.LOCALAPPDATA && join(process.env.LOCALAPPDATA, 'MOTHER'));
const endpoint = process.argv[2] || (home && join(home, 'native-runtime', 'service.json'));
const executable = process.platform === 'win32' ? 'mother.exe' : 'mother';
const binaries = ['garden-build/release', 'release'].map(dir =>
  fileURLToPath(new URL('../../target/' + dir + '/' + executable, import.meta.url)));
if (home) binaries.push(join(home, 'bin', executable));
const binary = process.argv[3] || binaries.find(path => existsSync(path)) || executable;
const mime = { html: 'text/html', js: 'text/javascript', jpg: 'image/jpeg', glb: 'model/gltf-binary', ogg: 'audio/ogg', json: 'application/json', md: 'text/plain', txt: 'text/plain' };
const asset = /^\/(?:assets\/(?:index|bake\.worker)\.js|glasshouse-pool\.jpg|cherry-bough\.glb|textures\/Ground(?:048|103)_(?:Color|NormalGL)\.jpg|audio\/(?:manifest\.json|footstepOnGrass[1-5]Medium\.ogg)|(?:LICENSE|LICENSES\/CC0-1\.0)\.txt|(?:LICENCE-NOTES|CREDITS|THIRD-PARTY-NOTICES)\.md)$/;
const readRoute = /^\/api\/(?:garden(?:\?since=[a-f0-9]{64}|\/records\/[a-zA-Z0-9_%.-]+)?|media(?:\/[a-f0-9]{64})?)$/;

async function launch() {
  if (!endpoint) throw Error('Supply the owner endpoint path and mother executable as arguments.');
  const descriptor = JSON.parse(execFileSync(binary, ['garden', 'explore', '--endpoint', endpoint], { encoding: 'utf8' }));
  const owner = new URL(descriptor.url);
  if (!descriptor.live || !descriptor.readOnly || !/^http:\/\/127\.0\.0\.1:\d+\/#[a-f0-9]{64}$/.test(owner.href)) {
    throw Error('The owner did not return a local, read-only Garden link.');
  }
  const probe = await fetch(owner.origin, { redirect: 'error', signal: AbortSignal.timeout(5000) });
  await probe.body?.cancel();
  const policy = probe.headers.get('content-security-policy');
  if (!probe.ok || !policy) throw Error('The owner did not serve the Garden page.');
  await readFile(new URL('index.html', assetRoot));
  let local;
  const server = http.createServer(async (req, res) => {
    const refuse = (code) => { res.writeHead(code); res.end(); };
    try {
      res.setHeader('Content-Security-Policy', policy);
      res.setHeader('Cache-Control', 'no-store');
      res.setHeader('X-Content-Type-Options', 'nosniff');
      res.setHeader('Referrer-Policy', 'no-referrer');
      res.setHeader('Cross-Origin-Resource-Policy', 'same-origin');
      if (req.headers.host !== local.host || (req.headers.origin && req.headers.origin !== local.origin)) return refuse(403);
      const path = req.url.split('?')[0];
      if (req.method === 'GET' && (path === '/' || asset.test(path))) {
        const name = path === '/' ? 'index.html' : path.slice(1);
        const bytes = await readFile(new URL(name, assetRoot));
        res.writeHead(200, { 'Content-Type': mime[name.split('.').pop()] });
        return res.end(bytes);
      }
      const exchange = req.method === 'POST' && req.url === '/api/session' && req.headers.origin === local.origin;
      if (!exchange && !(req.method === 'GET' && readRoute.test(req.url))) return refuse(404);
      if (!/^Bearer [a-f0-9]{64}$/.test(req.headers.authorization || '')) return refuse(401);
      const upstream = await fetch(owner.origin + req.url, {
        method: req.method, redirect: 'error', signal: AbortSignal.timeout(60000),
        headers: { Authorization: req.headers.authorization, Origin: owner.origin },
      });
      res.writeHead(upstream.status, { 'Content-Type': upstream.headers.get('content-type') || 'application/octet-stream' });
      if (upstream.body) await pipeline(Readable.fromWeb(upstream.body), res);
      else res.end();
    } catch {
      if (!res.headersSent) refuse(502);
      else res.destroy();
    }
  });
  server.maxConnections = 32;
  server.on('error', error => {
    console.error(error.code === 'EADDRINUSE' ? 'Port 49568 is in use. Close the previous Garden preview and reopen this launcher.' : error.message);
    process.exitCode = 1;
  });
  server.listen(49568, '127.0.0.1', () => {
    local = new URL('http://127.0.0.1:' + server.address().port);
    const link = local.origin + '/' + owner.hash;
    const cmd = process.platform === 'win32' ? 'rundll32' : process.platform === 'darwin' ? 'open' : 'xdg-open';
    const args = process.platform === 'win32' ? ['url.dll,FileProtocolHandler', link] : [link];
    spawn(cmd, args, { stdio: 'ignore', windowsHide: true }).on('error', () => {
      console.error('Could not open the browser for the local Garden preview.');
    });
    console.log('Opened the updated live Garden. Leave this terminal open while walking. Press Ctrl+C to close.');
  });
}

launch().catch((error) => {
  console.error('Could not start the live Garden preview:', error.cause?.message || error.message);
  process.exitCode = 1;
});
