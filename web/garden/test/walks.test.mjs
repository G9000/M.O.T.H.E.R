import assert from 'node:assert/strict';
import test from 'node:test';
import { createWalks, parseWalk } from '../src/garden/walks.js';

const photo = 'media:' + 'a'.repeat(64);
const saved = {format:'mother-walk-v1',workspace:'garden',generation:'one',name:'Winter',stops:['letter',photo,'decision']};

test('walk files preserve the exact named sequence and reject an unrelated Garden or invalid stops', () => {
  assert.deepEqual(parseWalk(JSON.stringify(saved),'garden','one'),{name:'Winter',stops:saved.stops});
  for (const change of [{generation:'two'},{workspace:'elsewhere'},{stops:['same','same']},{name:' '},{stops:['https://example.test/']}])
    assert.throws(() => parseWalk(JSON.stringify({...saved,...change}),'garden','one'));
});

test('the walk editor reorders a mixed walk, reopens it and keeps an unavailable stop in place', async () => {
  const nodes = new Map();
  const node = () => ({
    children:[],dataset:{},value:'',
    replaceChildren(...items) { this.children = items; },
    append(...items) { this.children.push(...items); },
    add(item) { this.children.push(item); },
    focus() {},
  });
  const panel = {querySelector(selector) {
    if (!nodes.has(selector)) nodes.set(selector,node());
    return nodes.get(selector);
  }};
  globalThis.Option = class { constructor(text,value) {Object.assign(this,{text,value});} };
  let download;
  globalThis.document = {createElement(kind) {
    const el = node();
    if (kind === 'a') el.click = () => { download = fetch(el.href).then(r=>r.text()); };
    return el;
  }};
  const fields = name => panel.querySelector('[data-'+name+']');
  const click = action => panel.onclick({target:{closest:()=>({dataset:{[action]:''},disabled:fields(action).disabled})}});
  const records = [{id:'letter',title:'A letter',content:'A synthetic letter'},{id:'decision',title:'A decision',content:'A synthetic decision'}];
  const images = [{sha256:photo.slice(6),mediaType:'image/jpeg',acquisitions:[{originalFilename:'A photo'}]}];
  let opened;
  const editor = createWalks({workspaceId:'garden',generation:'one'},(id)=>{opened=id;});
  editor.update(records, images);
  editor.mount(panel);
  fields('name').oninput({target:{value:'Winter'}});
  for (const id of ['letter','decision',photo]) { fields('pick').value=id; click('add'); }
  click('up');
  assert.deepEqual(fields('order').children.map(o=>o.value),saved.stops);
  click('save');
  assert.deepEqual(JSON.parse(await download),saved);
  click('remove');
  await fields('file').onchange({target:{files:[new Blob([JSON.stringify(saved)])]}});
  assert.deepEqual(fields('order').children.map(o=>o.value),saved.stops);
  editor.update(records,[]);
  click('start');
  fields('stop').children.at(-1).onclick();
  assert.equal(opened,'letter');
  click('next');
  assert.match(fields('stop').children[0].textContent,/2 of 3.*Unavailable/);
  click('next');
  assert.match(fields('stop').children[0].textContent,/3 of 3.*A decision/);
  assert.equal(fields('next').disabled,true);
  editor.unmount();
});
