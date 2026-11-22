// The Reading Path: every record of the snapshot as text, newest first, to search, choose and read one after another.
// It needs no WebGL, no pointer capture and no walking. Over the garden it is a page of its own that the garden waits
// behind; where the garden can't be drawn, or when asked for with ?view=read, it is the whole page.
//
// Read-only like the rest: it shows the snapshot it was given and opens records in the reader. Nothing it holds is
// kept in browser storage.
import { createReader } from './reader.js';

const esc = (s) => String(s ?? '').replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
const KINDS = ['memory', 'decision', 'task', 'checkpoint', 'artifact', 'action'];
const KIND = { memory: 'Memory', decision: 'Decision', task: 'Task', checkpoint: 'Checkpoint', artifact: 'Artifact', action: 'Action' };
const PLURAL = { memory: 'Memories', decision: 'Decisions', task: 'Tasks', checkpoint: 'Checkpoints', artifact: 'Artifacts', action: 'Actions' };
const RETIRED = { retracted: 'Retracted', superseded: 'Superseded', cancelled: 'Cancelled' };
const CSS = { memory: '#c6abf2', decision: '#86d4c0', task: '#f2a66e', checkpoint: '#8db6f2', artifact: '#e3cf7d', action: '#ef93ae', pinned: '#f0d9a0' };

const day = (t) => {
  const d = new Date(t);
  return Number.isNaN(d.getTime()) ? 'an unrecorded date' : d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric' });
};
const monthOf = (t) => {
  const d = new Date(t);
  return Number.isNaN(d.getTime()) ? 'Undated' : d.toLocaleDateString('en-GB', { month: 'long', year: 'numeric' });
};
/** The opening of a record's text, cut at a word, for the list: the reader has the rest. */
function opening(text, max = 240) {
  const flat = String(text ?? '').replace(/\s+/g, ' ').trim();
  if (flat.length <= max) return flat;
  const cut = flat.slice(0, max);
  return `${cut.slice(0, Math.max(cut.lastIndexOf(' '), max * 0.6)).replace(/[\s.,;:!?]+$/, '')}…`;
}
/** How a record came to be written, in a few words; the reader says it in full. */
function capture(r) {
  if (r.kind !== 'memory') return 'Working record';
  return { owner_statement: 'Captured from a conversation', source: 'From a source', model_inference: "Mother's interpretation" }[r.basis] ?? 'How it was captured is not recorded';
}
const reflection = (r) => r.kind === 'memory' && r.basis === 'model_inference' && r.status === 'active';

/**
 * @param root the element the path is drawn in
 * @param snapshot the garden snapshot
 * @param read (id, opener, ids) opens a record in the reader, stepping through `ids`
 * @param closeable true over the garden, where it has a way back; false where it is the whole page
 * @param banner HTML said above the path (already escaped)
 * @param onOpen, onClose told when the path is shown and put away
 */
export function createReadingPath(root, { snapshot, read, closeable = true, banner = '', onOpen = () => {}, onClose = () => {}, signal }) {
  let records = [];
  let reflections = new Map();
  let opener = null;
  let behind = [];
  const want = { words: [], kinds: new Set(KINDS), retired: false, arrange: 'date' };

  function take(next) {
    snapshot = next;
    records = (next.records ?? []).filter((r) => KINDS.includes(r.kind));
    reflections = new Map();
    for (const r of records) if (reflection(r)) for (const id of new Set(r.references ?? [])) if (id !== r.id) reflections.set(id, (reflections.get(id) ?? 0) + 1);
  }
  take(snapshot);

  const time = (r) => Date.parse(r.createdAt) || 0;
  const passes = (r) => {
    if (!want.kinds.has(r.kind)) return false;
    if (want.retired && RETIRED[r.status]) return false;
    if (!want.words.length) return true;
    const text = `${r.title}\n${r.content}\n${r.status}\n${r.kind}`.toLowerCase();
    return want.words.every((w) => text.includes(w));
  };
  /** [heading, records] in reading order: what is kept close first, then by month or by kind, newest first. */
  function groups() {
    const shown = records.filter(passes).sort((a, b) => time(b) - time(a));
    const pinned = shown.filter((r) => r.pinned);
    const rest = shown.filter((r) => !r.pinned);
    const out = pinned.length ? [['Kept close', pinned]] : [];
    if (want.arrange === 'kind') {
      for (const k of KINDS) {
        const of = rest.filter((r) => r.kind === k);
        if (of.length) out.push([PLURAL[k], of]);
      }
    } else {
      for (const r of rest) {
        const name = monthOf(r.createdAt);
        if (out.length && out[out.length - 1][0] === name && out[out.length - 1].month) out[out.length - 1][1].push(r);
        else out.push(Object.assign([name, [r]], { month: true }));
      }
    }
    return out;
  }
  const order = () => groups().flatMap(([, rs]) => rs.map((r) => r.id));

  function entry(r) {
    const retired = RETIRED[r.status];
    const n = reflections.get(r.id) ?? 0;
    const meta = [
      `<span class="dot" style="background:${CSS[r.pinned ? 'pinned' : r.kind]}"></span>${esc(KIND[r.kind])}`,
      `Recorded ${day(r.createdAt)}`,
      esc(capture(r)),
      ...(n ? [`${n} ${n === 1 ? 'reflection' : 'reflections'}`] : []),
      ...(retired ? [`<span class="rpoff">${retired}, kept as history</span>`] : []),
    ];
    const id = esc(r.id);
    return `
      <li>
        <button class="rpentry${retired ? ' retired' : ''}" data-id="${id}" aria-label="Read ${esc(r.title)}" aria-describedby="rpd-${id}">
          <span class="rpmeta">${meta.join('<span class="rpsep" aria-hidden="true">·</span>')}</span>
          <span class="rptitle">${esc(r.title)}</span>
          <span class="rpexcerpt" id="rpd-${id}">${esc(opening(r.content))}</span>
        </button>
      </li>`;
  }

  function list() {
    const gs = groups();
    const count = gs.reduce((n, [, rs]) => n + rs.length, 0);
    root.querySelector('.rpcount').textContent = !records.length
      ? 'No records yet.'
      : count === records.length
        ? `${count} ${count === 1 ? 'record' : 'records'}, newest first.`
        : `${count} of ${records.length} records shown.`;
    root.querySelector('.rpgroups').innerHTML = gs.length
      ? gs
          .map(
            ([name, rs], i) => `
        <section class="rpgroup" aria-labelledby="rpg-${i}">
          <h2 id="rpg-${i}">${esc(name)} <span class="rpn">${rs.length}</span></h2>
          <ol class="rpitems">${rs.map(entry).join('')}</ol>
        </section>`,
          )
          .join('')
      : `<p class="rpnone">${records.length ? 'Nothing matches. Try fewer words, or bring back a kind.' : 'The garden has no records yet.'}</p>`;
  }

  function draw() {
    const shown = snapshot.shown ?? records.length;
    const partial = snapshot.total > shown ? `<p class="rpnote">This garden view holds the ${shown} most recent of ${snapshot.total} records, with pinned ones first.</p>` : '';
    root.innerHTML = `
      <div class="rpwrap">
        <header class="rptop">
          <div>
            <p class="rpkicker">Memory Garden · read-only</p>
            <h1 id="rp-title" tabindex="-1">Reading Path</h1>
          </div>
          ${closeable ? '<button class="rpback" data-back>Back to the garden</button>' : ''}
        </header>
        ${banner}
        ${partial}
        <div class="rpcontrols">
          <label class="rpfind"><span>Search</span><input id="rp-q" type="search" autocomplete="off" spellcheck="false" placeholder="Words in a title or its text" /></label>
          <div class="rprow">
            <div class="rpseg" role="group" aria-label="Arrange the path">
              <button data-arrange="date" aria-pressed="${want.arrange === 'date'}">By month</button>
              <button data-arrange="kind" aria-pressed="${want.arrange === 'kind'}">By kind</button>
            </div>
            <div class="rpkinds" role="group" aria-label="Kinds shown">${KINDS.filter((k) => records.some((r) => r.kind === k))
              .map((k) => `<button data-kind="${k}" aria-pressed="${want.kinds.has(k)}"><span class="dot" style="background:${CSS[k]}"></span>${PLURAL[k]}</button>`)
              .join('')}</div>
            <label class="check"><input type="checkbox" id="rp-retired" ${want.retired ? 'checked' : ''} /> Set aside retired records</label>
          </div>
        </div>
        <p class="rpcount" role="status" aria-live="polite"></p>
        <div class="rpgroups"></div>
      </div>`;
    root.querySelector('#rp-q').value = want.words.join(' ');
    list();
  }

  const entries = () => [...root.querySelectorAll('.rpentry')];
  const on = (event, handler) => root.addEventListener(event, handler, { signal });
  on('input', (e) => {
    if (e.target.id === 'rp-q') {
      want.words = e.target.value.trim().toLowerCase().split(/\s+/).filter(Boolean);
      list();
    } else if (e.target.id === 'rp-retired') {
      want.retired = e.target.checked;
      list();
    }
  });
  on('click', (e) => {
    const t = e.target.closest('button');
    if (!t) return;
    if (t.dataset.back !== undefined) close();
    else if (t.dataset.id) read(t.dataset.id, t, order());
    else if (t.dataset.arrange) {
      want.arrange = t.dataset.arrange;
      root.querySelectorAll('[data-arrange]').forEach((b) => b.setAttribute('aria-pressed', b.dataset.arrange === want.arrange));
      list();
    } else if (t.dataset.kind) {
      const k = t.dataset.kind;
      if (want.kinds.has(k)) want.kinds.delete(k);
      else want.kinds.add(k);
      t.setAttribute('aria-pressed', want.kinds.has(k));
      list();
    }
  });
  on('keydown', (e) => {
    // while the path is up, the garden's own keys (walking, time of day, sound) are not pressed
    if (closeable) e.stopPropagation();
    const inSearch = e.target.id === 'rp-q';
    if (e.key === 'Escape') {
      if (inSearch && e.target.value) return; // the search box clears itself
      if (closeable) {
        e.preventDefault();
        close();
      }
      return;
    }
    if (e.key === '/' && !inSearch && e.target.tagName !== 'INPUT') {
      e.preventDefault();
      root.querySelector('#rp-q').focus();
      return;
    }
    // up and down step from one record to the next; from the search box, down goes to the first
    const all = entries();
    const at = all.indexOf(e.target);
    let to = -1;
    if (e.key === 'ArrowDown' && (at >= 0 || inSearch)) to = at + 1;
    else if (e.key === 'ArrowUp' && at >= 0) to = at - 1;
    else if (e.key === 'Home' && at >= 0) to = 0;
    else if (e.key === 'End' && at >= 0) to = all.length - 1;
    else return;
    e.preventDefault();
    if (to < 0) root.querySelector('#rp-q').focus();
    else all[Math.min(to, all.length - 1)]?.focus();
  });

  function open(from = document.activeElement) {
    if (!root.hidden) return;
    opener = from;
    draw();
    root.hidden = false;
    // over the garden, what is behind can't be reached by Tab or a screen reader; the reader stays reachable
    if (closeable) {
      behind = [...root.parentElement.children].filter((el) => el !== root && el.id !== 'reader' && !el.inert && !['SCRIPT', 'NOSCRIPT'].includes(el.tagName));
      for (const el of behind) el.inert = true;
    }
    onOpen();
    root.querySelector('#rp-title').focus();
  }
  function close() {
    if (root.hidden || !closeable) return;
    root.hidden = true;
    root.innerHTML = '';
    for (const el of behind) el.inert = false;
    behind = [];
    onClose();
    const back = opener;
    opener = null;
    if (back?.isConnected) back.focus();
  }
  /** A newer snapshot: the list is drawn again, keeping the search, the place in it and the record in focus. */
  function update(next) {
    take(next);
    if (root.hidden) return;
    const focused = document.activeElement?.closest?.('.rpentry')?.dataset.id;
    const scroll = root.scrollTop;
    list();
    root.scrollTop = scroll;
    if (focused) root.querySelector(`.rpentry[data-id="${window.CSS.escape(focused)}"]`)?.focus();
  }

  return {
    open,
    close,
    update,
    get isOpen() {
      return !root.hidden;
    },
  };
}

/**
 * The page as the Reading Path alone: where the garden can't be drawn, or when asked for with ?view=read. Keeps up
 * with a live owner the way the garden does, every five seconds while the page is seen and nothing is being read.
 * @param reason 'chosen' or 'no-webgl'
 */
export function startReadingPage({ snapshot, fetchSnapshot, fetchReading, reason }) {
  document.body.classList.add('reading-only');
  document.getElementById('loading')?.remove();
  const banner =
    reason === 'no-webgl'
      ? '<p class="rpnote">This browser does not offer WebGL 2, so the 3D garden can’t be drawn here. Every record is on this page to read.</p>'
      : '<p class="rpintro">The records as text, without the 3D garden. <a href="./">Open the 3D garden</a></p>';
  const all = () => (snapshot.records ?? []).filter((r) => KINDS.includes(r.kind));
  const reader = createReader(document.getElementById('reader'), {
    fetchReading,
    snapshotOf: (id) => all().find((r) => r.id === id),
    all,
    order: () => all().map((r) => r.id),
  });
  const path = createReadingPath(document.getElementById('path'), {
    snapshot,
    read: (id, from, ids) => reader.open(id, from, ids),
    closeable: false,
    banner,
  });
  path.open();
  document.title = 'Reading Path · Memory Garden';

  async function refresh() {
    try {
      if (document.hidden || reader.isOpen) return;
      const next = await fetchSnapshot(snapshot.liveRevision);
      if (!next || next.unchanged || reader.isOpen) return;
      snapshot = next;
      path.update(next);
    } catch {
      // keep what is shown if the owner or the browser is briefly unavailable
    } finally {
      setTimeout(refresh, 5000);
    }
  }
  if (snapshot.liveRevision) setTimeout(refresh, 5000);
  return { path, reader };
}
