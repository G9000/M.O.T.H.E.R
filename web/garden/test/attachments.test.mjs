import test from 'node:test';
import assert from 'node:assert/strict';
import { attachmentBlob, fetchReading, loadSnapshot } from '../src/garden/session.js';
import { createReader, fromSnapshot } from '../src/garden/reader.js';

test('approved video is visible in each lens and fetched only on demand', async () => {
  const src = { id: 'memory', kind: 'memory', status: 'active', basis: 'owner_statement', content: 'Test account', title: 'Screening', revision: 1 };
  const reading = fromSnapshot(src, [src]);
  const digest = 'ab'.repeat(32);
  const requests = [];
  globalThis.location = {hash: ''};
  globalThis.sessionStorage = {getItem: () => 'synthetic-session'};
  const element = () => ({focus() {}});
  globalThis.document = {createElement: type => ({...element(), type, style: {}})};
  globalThis.fetch = async (url, options) => {
    requests.push(url);
    assert.equal(options.headers.Authorization, 'Bearer synthetic-session');
    if (url.endsWith('/garden')) return Response.json({});
    if (url.includes('/records/')) return Response.json(reading);
    if (url.endsWith('/media')) return Response.json({images: [{
      sha256: digest, originalSha256: digest, mediaType: 'video/mp4', variant: 'original',
      acquisitions: [{id: 'acq', originalFilename: 'screening.mp4'}],
      memoryLinks: [{memoryId: 'memory', acquisitionId: 'acq', evidenceId: 'evidence'},
        {memoryId: 'different', acquisitionId: 'acq', evidenceId: 'other'}],
    }]});
    assert.equal(url, './api/media/' + digest);
    return new Response(new Uint8Array([1]), {headers: {'Content-Type':'video/mp4'}});
  };
  await loadSnapshot();
  assert.equal((await fetchReading(src.id)).attachments.length, 1);
  const handlers = {};
  const root = {hidden: true, setAttribute() {}, querySelector: element,
    addEventListener: (name, callback) => {handlers[name] = callback;}};
  const reader = createReader(root, {fetchReading, snapshotOf: () => undefined, all: () => [], order: () => [src.id]});
  const click = button => handlers.click({target:{closest: () => button}});
  reader.open(src.id, null);
  await new Promise(resolve => setImmediate(resolve));
  for (const lens of ['mother', 'owner', 'archive']) {
    click({dataset: {lens}});
    assert.match(root.innerHTML, /Moments captured/);
    assert.match(root.innerHTML, /Open screening\.mp4/);
  }
  assert.match(root.innerHTML, /Original SHA-256/);
  assert.ok(!requests.includes('./api/media/' + digest));
  let video;
  click({dataset:{attachment:'0'}, isConnected:true, after(node) {video = node;}});
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(video.type, 'video');
  assert.equal(video.controls, true);
  assert.ok(!video.autoplay);
  reader.close();
  assert.equal(await attachmentBlob('invalid'), null);
});
