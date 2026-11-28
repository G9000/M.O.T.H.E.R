import { attachmentBlob } from './session.js';
import { describe } from './media.js';

const MAX_STOPS = 100;
const validId = /^(?:[a-zA-Z0-9-]{1,80}|media:[0-9a-f]{64})$/;

export function parseWalk(text, workspace, generation) {
  const walk = JSON.parse(text);
  if (walk?.format !== 'mother-walk-v1' || walk.workspace !== workspace || walk.generation !== generation)
    throw Error('Choose a walk saved from this Garden generation.');
  if (typeof walk.name !== 'string' || !walk.name.trim() || walk.name.length > 80 ||
      !Array.isArray(walk.stops) || !walk.stops.length || walk.stops.length > MAX_STOPS ||
      !walk.stops.every(id => typeof id === 'string' && validId.test(id)) || new Set(walk.stops).size !== walk.stops.length)
    throw Error('This walk has an invalid name or list of stops.');
  return { name: walk.name.trim(), stops: walk.stops };
}

// Walk files contain only the chosen name and ordered references. Resolve each stop from
// the currently open owner view, including standalone photos that have no v1 record.
export function createWalks(snapshot, read) {
  const workspace = snapshot.workspaceId;
  const generation = snapshot.generation ?? null;
  let walk = {name: '', stops: []};
  let at = 0;
  let panel;
  let catalog = new Map();
  let photoUrl;
  let viewing = false;
  let renderId = 0;

  function release() {
    renderId++;
    if (photoUrl) URL.revokeObjectURL(photoUrl);
    photoUrl = null;
  }
  function status(message) { panel.querySelector('[role=status]').textContent = message; }
  function title(id) { return catalog.get(id)?.title ?? 'Unavailable in this Garden view'; }
  const field = name => panel.querySelector('[data-' + name + ']');

  function catalogOptions() {
    const picker = field('pick');
    const prior = picker.value;
    picker.replaceChildren(new Option('Choose a memory, letter or photo', ''));
    for (const [id, item] of catalog) {
      if (!walk.stops.includes(id)) picker.add(new Option(item.title, id));
    }
    if (catalog.has(prior)) picker.value = prior;
  }
  async function display() {
    release();
    const version = renderId;
    const id = walk.stops[at];
    const item = catalog.get(id);
    const body = field('stop');
    body.replaceChildren();
    if (!viewing || !id) return;
    const heading = document.createElement('h3');
    heading.textContent = (at + 1) + ' of ' + walk.stops.length + ' · ' + title(id);
    body.append(heading);
    if (!item) return;
    if (!item.photo) {
      const excerpt = document.createElement('p');
      excerpt.textContent = item.content;
      const open = document.createElement('button');
      open.type = 'button';
      open.textContent = 'Open full record';
      open.onclick = () => read(id, open, [id]);
      body.append(excerpt, open);
      return;
    }
    try {
      const blob = await attachmentBlob(item.sha);
      if (version !== renderId) return;
      if (!blob) throw Error();
      const img = document.createElement('img');
      photoUrl = URL.createObjectURL(blob);
      img.src = photoUrl;
      img.alt = item.title;
      body.append(img);
    } catch {
      if (version === renderId) status('This photo could not be opened.');
    }
  }
  function render() {
    if (!panel) return;
    catalogOptions();
    const order = field('order');
    order.replaceChildren(...walk.stops.map((id, i) => new Option((i+1) + '. ' + title(id), id)));
    order.selectedIndex = at;
    for (const action of ['up','prev']) field(action).disabled = at <= 0;
    for (const action of ['down','next']) field(action).disabled = at >= walk.stops.length-1;
    for (const action of ['remove','start','save']) field(action).disabled = !walk.stops.length;
    field('editor').hidden = viewing;
    field('player').hidden = !viewing;
    void display();
  }
  function update(records, photos) {
    catalog = new Map(records.map(r => [r.id, r]));
    for (const photo of photos.filter(p => p.mediaType?.startsWith('image/'))) {
      const item = describe(photo);
      catalog.set(item.id, item);
    }
    render();
  }
  function mount(root) {
    panel = root;
    panel.innerHTML = `<summary>Curated walks</summary>
      <div data-editor>
        <label>Walk name<input data-name maxlength="80" placeholder="Name your walk"></label>
        <label>Add a stop<select data-pick></select></label><button data-add>Add stop</button>
        <label>Walking order<select data-order size="4"></select></label>
        <div><button data-up>Move up</button> <button data-down>Move down</button> <button data-remove>Remove stop</button></div>
        <div><button data-start>Start walk</button> <button data-save>Save walk file</button> <button data-load>Open walk file</button></div>
        <input data-file hidden type="file" accept="application/json,.json">
      </div><div data-player hidden><div data-stop></div>
        <button data-prev>Previous stop</button> <button data-next>Next stop</button> <button data-edit>Edit walk</button>
      </div><p role="status" aria-live="polite"></p>`;
    field('name').value = walk.name;
    field('name').oninput = e => { walk.name = e.target.value; };
    field('order').onchange = e => { at = Math.max(0,e.target.selectedIndex); render(); };
    field('file').onchange = async e => {
      const file = e.target.files[0];
      if (!file) return;
      try {
        if (file.size > 16384) throw Error('Choose a walk file smaller than 16 KiB.');
        const loaded = parseWalk(await file.text(),workspace,generation);
        if (panel !== root) return;
        walk = loaded;
        field('name').value = walk.name;
        at = 0;
        render();
        status('Opened ' + walk.name + '.');
      } catch (err) { if (panel === root) status(err instanceof SyntaxError ? 'Invalid walk file.' : err.message); }
      e.target.value = '';
    };
    panel.onclick = e => {
      const b = e.target.closest('button');
      if (!b || b.disabled) return;
      const action = Object.keys(b.dataset)[0];
      if (!action) return;
      status('');
      switch (action) {
        case 'add': {
          const id = field('pick').value;
          if (!id) return status('Choose a stop first.');
          if (walk.stops.length >= MAX_STOPS) return status('A walk holds up to 100 stops.');
          walk.stops.push(id); at = walk.stops.length - 1; break;
        }
        case 'remove': walk.stops.splice(at,1); at = Math.max(0,Math.min(at,walk.stops.length-1)); break;
        case 'up': case 'down': {
          const next = at + (action === 'up' ? -1 : 1);
          [walk.stops[next],walk.stops[at]] = [walk.stops[at],walk.stops[next]];
          at = next; break;
        }
        case 'prev': at--; break;
        case 'next': at++; break;
        case 'start': viewing = true; at = 0; break;
        case 'edit': viewing = false; break;
        case 'load': return field('file').click();
        case 'save': {
          const name = walk.name.trim();
          if (!name) { field('name').focus(); return status('Give this walk a name.'); }
          const url = URL.createObjectURL(new Blob([JSON.stringify({format:'mother-walk-v1',workspace,generation,name,stops:walk.stops})],{type:'application/json'}));
          const link = document.createElement('a');
          link.href = url;
          link.download = (name.replace(/[^\p{L}\p{N} -]/gu,'').trim() || 'garden') + '.walk.json';
          link.click();
          setTimeout(() => URL.revokeObjectURL(url),1000);
          return status('Walk file ready to download.');
        }
        default: return;
      }
      render();
      if (action === 'start') field('edit').focus();
      if (action === 'edit') field('start').focus();
    };
    render();
  }
  return { mount, update, unmount() { release(); panel = null; } };
}
