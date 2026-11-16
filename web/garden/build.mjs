// Builds the page the executable embeds: `npm install --ignore-scripts`, then `npm run build`, then rebuild
// `mother`. Node is needed only for this step, never to run the executable.
import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const out = path.resolve(here, '../../assets/garden-explorer');
execSync(`node "${path.join(here, 'node_modules/vite/bin/vite.js')}" build --config vite.config.mjs`, { cwd: here, stdio: 'inherit' });
for (const name of ['LICENSE', 'LICENCE-NOTES.md']) fs.copyFileSync(path.join(here, name), path.join(out, name === 'LICENSE' ? 'LICENSE.txt' : name));
// the recordings that are here, so the page asks for no others
const audio = path.join(out, 'audio');
const sounds = fs.readdirSync(audio).filter((f) => f.endsWith('.ogg')).map((f) => f.slice(0, -4)).sort();
fs.writeFileSync(path.join(audio, 'manifest.json'), `${JSON.stringify(sounds)}\n`);
const files = [];
const walk = (dir) => {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p);
    else files.push([path.relative(out, p).split(path.sep).join('/'), fs.statSync(p).size]);
  }
};
walk(out);
files.sort();
for (const [name, size] of files) console.log(`${String(size).padStart(9)}  ${name}`);
console.log(`total: ${(files.reduce((sum, f) => sum + f[1], 0) / 1048576).toFixed(1)} MB in ${files.length} files`);
