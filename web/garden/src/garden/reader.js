import { companionAppearance, gardenAppearance } from './seed.js';
import { attachmentBlob } from './session.js';

// The reader: one memory, opened for reading, through three lenses over the same stored evidence.
//
//   Mother's Eyes  existing reflections an assistant wrote, each attributed, never generated here
//   My Memories    the record's own account, labelled by how it was captured; never shown as the owner's
//                  exact words, since nothing stored says it is
//   The Archive    exact identifiers, provenance, links and revisions, with every gap named as a gap
//
// Switching lenses re-renders the same reading; nothing is fetched again and nothing is written. The only
// browser preferences kept are the last lens and the text size, never anything a record says.

const esc = (s) => String(s ?? '').replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
const OWNER_REVIEW = '@owner-review';
const LENSES = [
  ['mother', "Mother's Eyes"],
  ['owner', 'My Memories'],
  ['archive', 'The Archive'],
];
const KIND = { memory: 'Memory', decision: 'Decision', task: 'Task', checkpoint: 'Checkpoint', artifact: 'Artifact', action: 'Action' };
const RETIRED = { retracted: 'Retracted', superseded: 'Superseded', cancelled: 'Cancelled' };
const BASIS = {
  owner_statement: 'something the owner said, written down by the writer',
  source: 'a source the writer read',
  model_inference: "the writer's own interpretation",
};
const SIZES = [15, 16, 17, 18, 20, 22, 24];

const date = (t) => {
  const d = new Date(t);
  return Number.isNaN(d.getTime()) ? 'an unrecorded date' : d.toLocaleDateString('en-GB', { day: 'numeric', month: 'long', year: 'numeric' });
};
const stamp = (t) => {
  const d = new Date(t);
  return Number.isNaN(d.getTime()) ? 'Not recorded' : `${d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric' })}, ${d.toTimeString().slice(0, 5)}`;
};
const writer = (w) => (w ? esc(w) : 'a writer not recorded here');
const recall = (key, fallback) => {
  try {
    return localStorage.getItem(key) ?? fallback;
  } catch {
    return fallback;
  }
};
const keep = (key, value) => {
  try {
    localStorage.setItem(key, value);
  } catch {
    // storage may be blocked: the reader works the same, it only forgets the preference
  }
};

const reflection = (r) => r.kind === 'memory' && r.basis === 'model_inference';
function formOf(r) {
  if (r.kind !== 'memory') return 'working_record';
  return { owner_statement: 'captured_paraphrase', source: 'source_summary', model_inference: 'interpretation' }[r.basis] ?? 'unlabelled';
}

/**
 * The same reading built from the snapshot alone, for when the full record cannot be fetched: the preview text, the
 * links inside the snapshot and the reflections among its records. Marked partial, so the page says so.
 * @param src a record as the snapshot carries it; @param all every record of the snapshot
 */
export function fromSnapshot(src, all) {
  const known = new Map(all.map((r) => [r.id, r]));
  const brief = (r) => ({ id: r.id, kind: r.kind, title: r.title, status: r.status, updatedAt: r.updatedAt, basis: r.basis });
  const reviewed = src.actor === OWNER_REVIEW;
  const witnessOf = (r, relation) => ({
    recordId: r.id,
    revision: r.revision,
    title: r.title,
    text: r.content,
    truncated: r.truncated,
    relation,
    writer: r.actor === OWNER_REVIEW ? null : r.actor,
    attribution: null,
    writtenAt: r.updatedAt,
    reviewStatus: r.actor === OWNER_REVIEW ? 'reviewed' : 'unknown',
    status: r.status,
    references: r.references ?? [],
    sources: r.sources ?? [],
  });
  const naming = all.filter((r) => r.id !== src.id && (r.references ?? []).includes(src.id)).sort((a, b) => String(b.updatedAt).localeCompare(String(a.updatedAt)));
  const witnesses = [...(reflection(src) ? [witnessOf(src, 'self')] : []), ...naming.filter((r) => reflection(r) && r.status === 'active').map((r) => witnessOf(r, 'references'))];
  return {
    recordId: src.id,
    kind: src.kind,
    status: src.status,
    title: src.title,
    revision: src.revision,
    actor: src.actor,
    basis: src.basis ?? null,
    createdAt: src.createdAt,
    updatedAt: src.updatedAt,
    eventDate: src.eventDate ?? null,
    content: src.content,
    truncated: src.truncated,
    length: null,
    sources: src.sources ?? [],
    account: { form: formOf(src), ownerWords: false, writer: reviewed ? null : src.actor, attribution: null, reviewStatus: reviewed ? 'reviewed' : 'unknown' },
    witnessAccounts: witnesses,
    witnessOmitted: 0,
    references: (src.references ?? []).map((id) => (known.has(id) ? brief(known.get(id)) : { id, available: false })),
    referencedBy: naming.map(brief),
    referencedByOmitted: 0,
    history: src.history ?? [],
    historyOmitted: src.historyOmitted ?? 0,
    defaultLens: witnesses.length ? 'mother' : 'owner',
    ...(src.nextStep ? { nextStep: src.nextStep } : {}),
    partial: true,
  };
}

/**
 * @param root the dialog element
 * @param fetchReading (id) => Promise of the full reading, or null when it cannot be had
 * @param snapshotOf (id) => the snapshot's record, or undefined
 * @param all () => every snapshot record, for the fallback
 * @param order () => record IDs in reading order, newest first; one open may bring its own order instead
 * @param walk (id) => stands the walker beside that record's flower; left out where there is no garden to walk
 */
export function createReader(root, { fetchReading, snapshotOf, all, order, walk, signal }) {
  let current = null;
  let lens = recall('garden-reader-lens', '');
  let size = Number(recall('garden-reader-size', '17')) || 17;
  let opener = null;
  let sequence = null;
  let asked = 0;
  // the lens on screen, and the record it was picked for by hand (a pick is kept while that record loads)
  let shown = 'owner';
  let pickedFor = null;
  root.setAttribute('role', 'dialog');
  root.setAttribute('aria-modal', 'true');
  root.setAttribute('aria-labelledby', 'reader-title');

  const sheet = () => root.querySelector('.sheet');

  function lensFor(reading) {
    // on opening, the last lens chosen is used again, except Mother's Eyes where there is nothing of hers to
    // read; a lens picked by hand is always shown, empty or not
    if (lens === 'archive' || lens === 'owner') return lens;
    if (lens === 'mother' && reading.witnessAccounts.length) return 'mother';
    return reading.defaultLens;
  }

  function header(r) {
    const retired = RETIRED[r.status];
    return `
      <header class="rhead">
        <p class="rkind">${esc(KIND[r.kind] ?? r.kind)}${retired ? ` · <span class="rretired">${retired}, kept as history</span>` : ''}</p>
        <h2 id="reader-title" tabindex="-1">${esc(r.title)}</h2>
        <p class="rdate">${r.eventDate?.value ? `${esc(r.eventDate.value)}` : `Recorded ${date(r.createdAt)}`}${r.updatedAt && r.updatedAt !== r.createdAt ? ` · last revised ${date(r.updatedAt)}` : ''}</p>
      </header>`;
  }

  function tabs(active) {
    return `<div class="rtabs" role="tablist" aria-label="Ways of reading this memory">${LENSES.map(
      ([key, name]) => `<button role="tab" id="lens-${key}" aria-controls="reader-panel" aria-selected="${key === active}" tabindex="${key === active ? 0 : -1}" data-lens="${key}">${name}</button>`,
    ).join('')}</div>`;
  }

  const prose = (text) => `<div class="rtext">${esc(text)}</div>`;
  function completeness(r) {
    if (r.partial && r.truncated) return '<p class="rnote">Only the beginning of this record is in the garden right now. Its full text could not be read.</p>';
    if (r.truncated) return `<p class="rnote">This record is very long: the first ${(24000).toLocaleString('en-GB')} characters are shown.</p>`;
    return '';
  }
  const cite = (ids) => {
    const found = ids.map((id) => [id, snapshotOf(id)]);
    if (!found.length) return '';
    return `<div class="rcites"><span>Cites</span>${found.map(([id, s]) => (s ? `<button class="chip" data-read="${esc(id)}">${esc(s.title)}</button>` : '<span class="chip off">a record not in this garden view</span>')).join('')}</div>`;
  };

  function mother(r) {
    if (!r.witnessAccounts.length)
      return `
        <div class="rempty">
          <p><b>No witness reflection has been recorded for this memory yet.</b></p>
          <p>Mother doesn't write one on her own when you open a memory. If you ask for a Witness Journal letter about it and approve it in your review inbox, it will appear here.</p>
        </div>`;
    return (
      r.witnessAccounts
        .map(
          (w) => `
        <article class="rwitness">
          <p class="rlabel">Mother's interpretation</p>
          ${w.relation === 'self' ? '' : `<h3>${esc(w.title)}</h3>`}
          ${prose(w.text)}
          ${w.truncated ? '<p class="rnote">Only the beginning of this reflection is shown.</p>' : ''}
          <p class="rmeta">Written by ${writer(w.writer)} · ${date(w.writtenAt)} · ${w.reviewStatus === 'reviewed' ? '<span class="rok">Owner-reviewed</span>' : 'Review status unavailable'}</p>
          ${w.attribution ? `<p class="rmeta">Source given: ${esc(w.attribution)}</p>` : ''}
          <p class="rmeta">${w.relation === 'self' ? 'This record is itself a reflection.' : 'Shown because this reflection names this memory among its references. It was not chosen as the memory’s story.'}</p>
          ${cite(w.references.filter((id) => id !== r.recordId))}
        </article>`,
        )
        .join('') + (r.witnessOmitted ? `<p class="rnote">${r.witnessOmitted} more ${r.witnessOmitted === 1 ? 'reflection names' : 'reflections name'} this memory. They are listed in the Archive.</p>` : '')
    );
  }

  function owner(r) {
    const a = r.account;
    const approved = a.reviewStatus === 'reviewed' ? ' You approved it in your review inbox.' : '';
    const lines = {
      captured_paraphrase: ['Account captured from a conversation', `Written down by ${writer(a.writer)} from something you said. The wording is theirs, not a quotation of you.${approved}`],
      source_summary: ['Summary recorded from a source', `Written by ${writer(a.writer)} from a source, not in your words.${approved}`],
      unlabelled: ['How this account was captured is not recorded', `Written by ${writer(a.writer)}.${approved}`],
      working_record: ['A working record, not a recollection', `Written by ${writer(a.writer)}.${approved}`],
    };
    if (a.form === 'interpretation')
      return `
        <div class="rempty">
          <p><b>No account in your own words is recorded for this.</b></p>
          <p>This record is an interpretation written by ${writer(a.writer)}. Read it under Mother's Eyes, or see what it cites in the Archive.</p>
        </div>`;
    const [label, detail] = lines[a.form];
    const facts = [];
    if (r.nextStep) facts.push(['Next step', esc(r.nextStep)]);
    if (r.completed?.length) facts.push(['Done', r.completed.map(esc).join('<br>')]);
    if (r.openQuestions?.length) facts.push(['Open questions', r.openQuestions.map(esc).join('<br>')]);
    if (r.kind !== 'memory') facts.push(['Status', esc(r.status)]);
    return `
      <p class="rlabel">${label}</p>
      <p class="rmeta">${detail}</p>
      ${prose(r.content)}
      ${completeness(r)}
      ${facts.length ? `<dl class="rfacts">${facts.map(([k, v]) => `<dt>${k}</dt><dd>${v}</dd>`).join('')}</dl>` : ''}`;
  }

  function archive(r) {
    const appearance = gardenAppearance(r.recordId);
    const row = (k, v) => `<dt>${k}</dt><dd>${v}</dd>`;
    const copy = (value, what) => `<button class="rcopy" data-copy="${esc(value)}" aria-label="Copy ${what}">Copy</button>`;
    const link = (b) =>
      b.available === false
        ? `<li><code>${esc(b.id)}</code> ${copy(b.id, 'record ID')}<br><span class="rgap">Not available here. This is a gap in what can be shown, not proof that it never existed.</span></li>`
        : `<li><button class="chip" data-read="${esc(b.id)}">${esc(b.title)}</button> <span class="rmeta">${esc(b.kind)}${b.basis ? ` · ${esc(b.basis)}` : ''} · ${esc(b.status)}</span></li>`;
    const length = r.length == null ? (r.truncated ? 'Only the first 4,000 characters are in this snapshot' : 'As in the snapshot') : `${r.length.toLocaleString('en-GB')} characters${r.truncated ? ', cut short for reading' : ', complete'}`;
    return `
      <h3 id="provenance" tabindex="-1">This record</h3>
      <dl class="rfacts">
        ${row('Record ID', `<code>${esc(r.recordId)}</code> ${copy(r.recordId, 'record ID')}`)}
        ${row('Kind', esc(r.kind))}
        ${row('Garden appearance', `${esc(appearance.form)} with ${esc(companionAppearance(appearance.seed).form)} · Seed <code>${appearance.seed}</code> ${copy(appearance.seed, 'garden seed')}`)}
        ${row('Status', esc(r.status))}
        ${row('Revision', esc(r.revision))}
        ${row('Last written by', esc(r.actor ?? 'Not recorded'))}
        ${row('Basis', r.basis ? `${esc(r.basis)} <span class="rmeta">(${esc(BASIS[r.basis] ?? 'meaning not known')})</span>` : 'Not recorded for this kind of record')}
        ${row('Owner review', r.account.reviewStatus === 'reviewed' ? 'Approved in the review inbox' : 'No review is recorded for this revision')}
        ${r.account.attribution ? row('Source given', esc(r.account.attribution)) : ''}
        ${row('Event date', r.eventDate?.value ? esc(r.eventDate.value) : 'Not recorded in this record')}
        ${row('First recorded', stamp(r.createdAt))}
        ${row('Last revised', stamp(r.updatedAt))}
        ${row('Text', length)}
      </dl>
      <h3>Sources</h3>
      ${r.sources.length ? `<ul class="rlist">${r.sources.map((s) => `<li>${esc(s.label)}<br><code>${esc(s.uri)}</code> ${copy(s.uri, 'source address')}</li>`).join('')}</ul><p class="rmeta">Source addresses are stored, not opened: the garden has not checked what they lead to.</p>` : '<p class="rmeta">No source links are stored with this record.</p>'}
      <h3>Records this one names</h3>
      ${r.references.length ? `<ul class="rlist">${r.references.map(link).join('')}</ul>` : '<p class="rmeta">None.</p>'}
      <h3>Records that name this one</h3>
      ${r.referencedBy.length ? `<ul class="rlist">${r.referencedBy.map(link).join('')}</ul>` : '<p class="rmeta">None.</p>'}
      ${r.referencedByOmitted ? `<p class="rmeta">And ${r.referencedByOmitted} more.</p>` : ''}
      <h3>Revisions</h3>
      ${r.historyOmitted ? `<p class="rmeta">${r.historyOmitted} earlier ${r.historyOmitted === 1 ? 'revision is' : 'revisions are'} not shown.</p>` : ''}
      <ol class="rhist">${[...r.history]
        .reverse()
        .map((e) => `<li><span class="rmeta">rev ${esc(e.revision)} · ${esc(e.actor)} · ${stamp(e.at)}${e.changed ? ` · changed ${e.changed.map(esc).join(', ')}` : ''}</span>${e.excerpt ? `<span class="rx">“${esc(e.excerpt)}”</span>` : ''}</li>`)
        .join('')}</ol>`;
  }

  function related(r) {
    const seen = new Set();
    const links = [...r.references, ...r.referencedBy].filter((b) => b.available !== false && b.id !== r.recordId && !seen.has(b.id) && seen.add(b.id));
    if (!links.length) return '';
    return `
      <section class="rrelated" aria-labelledby="related-title">
        <h3 id="related-title">Linked records</h3>
        <p class="rmeta">Exact links stored with these records. Nothing here is a suggestion.</p>
        <div class="chips">${links.map((b) => `<button class="chip" data-read="${esc(b.id)}">${esc(b.title)}${RETIRED[b.status] ? ` <span class="rgap">· ${RETIRED[b.status].toLowerCase()}</span>` : ''}</button>`).join('')}</div>
      </section>`;
  }

  const objectUrls = new Set();
  function clearMedia() {
    for (const url of objectUrls) URL.revokeObjectURL(url);
    objectUrls.clear();
  }
  function attachments(r) {
    if (!r.attachments?.length && !r.mediaWithheld) return '';
    const media = (r.attachments ?? []).map((a, i) => {
      const name = esc(a.acquisition?.originalFilename || a.mediaType);
      const details = shown === 'archive'
        ? `<dl class="rfacts"><dt>Original SHA-256</dt><dd><code>${esc(a.originalSha256)}</code></dd><dt>Evidence</dt><dd>${esc(a.link.evidenceId)}</dd><dt>Source</dt><dd>${esc(a.acquisition?.source || 'Owner-selected file')}</dd><dt>Approved</dt><dd>${esc(a.link.approvedAt)}</dd><dt>Event date</dt><dd>${esc(a.acquisition?.eventTime?.at || 'Unknown')}</dd></dl>` : '';
      return `<figure><button data-attachment="${i}">Open ${name}</button><figcaption>${esc(a.variant)} · Owner-approved</figcaption>${details}</figure>`;
    }).join('');
    return `<section aria-label="Moments captured"><h3>Moments captured</h3>${media}${r.mediaWithheld ? '<p>Media not available in this view.</p>' : ''}</section>`;
  }
  async function openAttachment(button) {
    const a = current?.attachments?.[Number(button.dataset.attachment)];
    if (!a) return;
    button.disabled = true;
    try {
      const blob = await attachmentBlob(a.sha256);
      if (!button.isConnected) return;
      if (!blob) throw Error();
      const kind = a.mediaType.split('/')[0];
      const media = document.createElement(kind === 'image' ? 'img' : kind === 'audio' ? 'audio' : 'video');
      const url = URL.createObjectURL(blob);
      objectUrls.add(url);
      media.src = url;
      media.style.maxWidth = '100%';
      media.style.maxHeight = '70vh';
      if (kind === 'image') media.alt = a.acquisition?.originalFilename || 'Attached image';
      else { media.controls = true; media.preload = 'metadata'; }
      button.after(media);
      button.hidden = true;
      media.tabIndex = 0;
      media.focus();
    } catch {
      if (button.isConnected) { button.disabled = false; button.textContent = 'Media unavailable. Retry'; }
    }
  }

  function render(focusTab = false) {
    clearMedia();
    const r = current;
    const active = shown;
    const ids = sequence ?? order();
    const at = ids.indexOf(r.recordId);
    const body = { mother, owner, archive }[active](r);
    root.innerHTML = `
      <div class="sheet" style="--rsize:${size}px">
        <div class="rbar">
          <div class="rnav">
            <button data-step="-1" ${at <= 0 ? 'disabled' : ''} aria-label="Later record">‹ Later</button>
            <button data-step="1" ${at < 0 || at >= ids.length - 1 ? 'disabled' : ''} aria-label="Earlier record">Earlier ›</button>
          </div>
          <div class="rnav">
            <button data-size="-1" aria-label="Smaller text" ${size <= SIZES[0] ? 'disabled' : ''}>A−</button>
            <button data-size="1" aria-label="Larger text" ${size >= SIZES[SIZES.length - 1] ? 'disabled' : ''}>A+</button>
            ${walk ? `<button data-walk ${snapshotOf(r.recordId) ? '' : 'disabled'}>Walk there</button>` : ''}
            <button class="close" data-close aria-label="Close the reader">×</button>
          </div>
        </div>
        ${header(r)}
        ${r.loading ? '<p class="rnote" role="status">Reading the full record…</p>' : r.partial ? '<p class="rnote" role="status">Showing what this garden snapshot holds. The full record could not be read just now.</p>' : ''}
        ${tabs(active)}
        <div class="rpanel" id="reader-panel" role="tabpanel" aria-labelledby="lens-${active}" tabindex="0">${body}${attachments(r)}</div>
        <footer class="rfoot">
          ${active === 'archive' ? '' : '<button class="rhow" data-how>How do we know? <span aria-hidden="true">→</span> The Archive</button>'}
          ${related(r)}
        </footer>
      </div>`;
    if (focusTab) root.querySelector(`#lens-${active}`)?.focus();
  }

  function pick(key) {
    lens = shown = key;
    pickedFor = current?.recordId ?? null;
    keep('garden-reader-lens', lens);
  }

  function show(reading) {
    if (pickedFor !== reading.recordId) {
      shown = lensFor(reading);
      pickedFor = null;
    }
    current = reading;
    render();
    sheet().scrollTop = 0;
    root.querySelector('#reader-title')?.focus();
  }

  async function load(id) {
    const ask = ++asked;
    const src = snapshotOf(id);
    if (src) show({ ...fromSnapshot(src, all()), loading: true });
    const full = await fetchReading(id).catch(() => null);
    if (ask !== asked || root.hidden) return;
    if (full) show(full);
    else if (src) show(fromSnapshot(src, all()));
    else {
      root.innerHTML = `<div class="sheet"><div class="rbar"><span></span><button class="close" data-close aria-label="Close the reader">×</button></div><p class="rempty">This record could not be read.</p></div>`;
      root.querySelector('[data-close]').focus();
    }
  }

  /** @param ids the order Later and Earlier step through for this reading, when not the garden's own */
  function open(id, from = document.activeElement, ids = null) {
    if (root.hidden) {
      opener = from;
      sequence = ids;
    }
    document.pointerLockElement && document.exitPointerLock();
    root.hidden = false;
    load(id);
  }
  function close() {
    if (root.hidden) return;
    asked++;
    clearMedia();
    root.hidden = true;
    root.innerHTML = '';
    current = null;
    sequence = null;
    const back = opener;
    opener = null;
    if (back?.isConnected) back.focus();
  }

  const on = (event, handler) => root.addEventListener(event, handler, { signal });
  on('click', (e) => {
    if (e.target === root) return close();
    const t = e.target.closest('button');
    if (!t || t.disabled) return;
    if (t.dataset.attachment !== undefined) void openAttachment(t);
    else if (t.dataset.close !== undefined) close();
    else if (t.dataset.lens) {
      pick(t.dataset.lens);
      render(true);
    } else if (t.dataset.how !== undefined) {
      pick('archive');
      render();
      root.querySelector('#provenance')?.focus();
    } else if (t.dataset.read) load(t.dataset.read);
    else if (t.dataset.step) {
      const ids = sequence ?? order();
      const next = ids[ids.indexOf(current.recordId) + Number(t.dataset.step)];
      if (next) load(next);
    } else if (t.dataset.size) {
      size = SIZES[Math.min(SIZES.length - 1, Math.max(0, SIZES.indexOf(size) + Number(t.dataset.size)))] ?? 17;
      keep('garden-reader-size', String(size));
      sheet().style.setProperty('--rsize', `${size}px`);
      render();
      root.querySelector(`[data-size="${t.dataset.size}"]`)?.focus();
    } else if (t.dataset.walk !== undefined) {
      const id = current.recordId;
      opener = null;
      close();
      walk(id);
    } else if (t.dataset.copy) {
      const done = (text) => {
        t.textContent = text;
        setTimeout(() => t.isConnected && (t.textContent = 'Copy'), 1500);
      };
      navigator.clipboard?.writeText(t.dataset.copy).then(() => done('Copied'), () => done('Copy failed')) ?? done('Copy failed');
    }
  });
  on('keydown', (e) => {
    // while reading, the garden's own keys (walking, time of day, sound) are not pressed
    e.stopPropagation();
    if (e.key === 'Escape') {
      e.preventDefault();
      return close();
    }
    const tab = e.target.closest?.('[role="tab"]');
    if (tab && ['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(e.key)) {
      e.preventDefault();
      const at = LENSES.findIndex(([k]) => k === tab.dataset.lens);
      const to = e.key === 'Home' ? 0 : e.key === 'End' ? LENSES.length - 1 : (at + (e.key === 'ArrowRight' ? 1 : -1) + LENSES.length) % LENSES.length;
      pick(LENSES[to][0]);
      render(true);
      return;
    }
    if (e.key === 'Tab') {
      // focus stays in the reader while it is open
      const stops = [...root.querySelectorAll('button:not([disabled]), [tabindex="0"], [href]')].filter((el) => el.offsetParent !== null);
      if (!stops.length) return;
      const first = stops[0];
      const last = stops[stops.length - 1];
      if (e.shiftKey && (document.activeElement === first || !root.contains(document.activeElement))) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  });

  return {
    open,
    close,
    get isOpen() {
      return !root.hidden;
    },
    /** The record on screen, while open. */
    get recordId() {
      return current?.recordId ?? null;
    },
  };
}
