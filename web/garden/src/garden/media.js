// The owner's attached photos, recordings and videos, as the garden shows them: what is said about each one, and
// the viewer that opens it large. Read-only: the bytes come from the local server that served the page.

const ORIGIN = { captured: 'Taken', uploaded: 'Uploaded', imported: 'Imported', screenshot: 'Screenshot', ai_generated: 'Made by AI' };
const READING = { caption: 'Caption', description: 'Description', ocr_text: 'Text in the image', transcript: 'Transcript' };
const DAY = { day: 'numeric', month: 'long', year: 'numeric' };
const esc = (s) => String(s ?? '').replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);

/** One media item as the page uses it: its kind, a title, when it is from, its readings and its search text. */
export function describe(item, blob) {
  const kind = String(item.mediaType ?? '').split('/')[0] || 'image';
  const first = item.acquisitions?.[0] ?? {};
  const readings = (item.interpretations ?? []).map((r) => ({
    label: READING[String(r.predicate ?? '').replace('mother.media:', '')] ?? 'Note',
    text: String(r.text ?? ''),
    by: r.attribution?.producer?.name ?? r.contributor ?? '',
  }));
  const caption = readings.find((r) => r.label === 'Caption')?.text;
  const at = Date.parse(first.eventTime?.at ?? first.acquiredAt ?? '') || 0;
  const title = caption || first.originalFilename || (kind === 'audio' ? 'A recording' : kind === 'video' ? 'A video' : 'A photo');
  return {
    photo: true,
    id: `media:${item.sha256}`,
    sha: item.sha256,
    kind,
    mediaType: item.mediaType,
    blob,
    url: blob ? URL.createObjectURL(blob) : '',
    title,
    file: first.originalFilename ?? '',
    origin: ORIGIN[first.origin] ?? '',
    source: first.source ?? '',
    at,
    eventPrecision: first.eventTime?.precision ?? '',
    readings,
    variant: item.variant,
    uses: item.uses ?? 0,
    width: item.width,
    height: item.height,
    text: [title, first.originalFilename, first.source, ORIGIN[first.origin], kind, kind === 'image' ? 'photo' : '', ...readings.map((r) => r.text)].join('\n').toLowerCase(),
  };
}

/** The viewer: one item large over a dimmed garden, with what is known about it beside it. */
export function createViewer(root) {
  let list = [];
  let at = -1;
  let onClose = () => {};
  root.innerHTML = `
    <div class="v-shade" data-close></div>
    <figure class="v-frame">
      <div class="v-media"></div>
      <figcaption class="v-text"></figcaption>
    </figure>
    <button class="v-nav v-prev" aria-label="Previous" data-step="-1"><svg viewBox="0 0 24 24" class="ico"><path d="m14.5 5.5-6.5 6.5 6.5 6.5"/></svg></button>
    <button class="v-nav v-next" aria-label="Next" data-step="1"><svg viewBox="0 0 24 24" class="ico"><path d="m9.5 5.5 6.5 6.5-6.5 6.5"/></svg></button>
    <button class="v-close" aria-label="Close" data-close>×</button>`;
  const media = root.querySelector('.v-media');
  const text = root.querySelector('.v-text');

  function show(i) {
    at = (i + list.length) % list.length;
    const p = list[at];
    media.innerHTML = '';
    if (p.url) {
      const el = document.createElement(p.kind === 'image' ? 'img' : p.kind);
      el.src = p.url;
      if (p.kind === 'image') el.alt = p.title;
      else {
        el.controls = true;
        el.autoplay = true;
      }
      media.append(el);
    } else media.innerHTML = '<p class="v-missing">This item could not be shown.</p>';
    const when = p.at ? new Date(p.at).toLocaleDateString(undefined, DAY) : '';
    text.innerHTML = `
      <span class="v-kind">${esc(p.kind === 'image' ? 'Photo' : p.kind === 'audio' ? 'Recording' : 'Video')}${p.origin ? ` · ${esc(p.origin)}` : ''}</span>
      <h2>${esc(p.title)}</h2>
      ${when ? `<p class="v-when">${esc(when)}</p>` : ''}
      ${p.readings.map((r) => `<div class="v-read"><b>${esc(r.label)}</b><p>${esc(r.text)}</p>${r.by ? `<span>by ${esc(r.by)}</span>` : ''}</div>`).join('')}
      <dl>
        ${p.file ? `<dt>File</dt><dd>${esc(p.file)}</dd>` : ''}
        ${p.source ? `<dt>Source</dt><dd>${esc(p.source)}</dd>` : ''}
        <dt>Shown</dt><dd>${p.variant === 'derivative' ? 'A copy with location and device data removed' : 'The original'}</dd>
        <dt>Cited by</dt><dd>${p.uses === 1 ? '1 memory' : `${esc(p.uses)} memories`}</dd>
      </dl>
      ${list.length > 1 ? `<p class="v-count">${at + 1} of ${list.length}</p>` : ''}`;
    root.querySelector('.v-prev').hidden = root.querySelector('.v-next').hidden = list.length < 2;
  }
  function close() {
    if (root.hidden) return;
    for (const el of media.querySelectorAll('audio, video')) el.pause();
    root.hidden = true;
    onClose();
  }
  root.addEventListener('click', (e) => {
    const step = e.target.closest('[data-step]');
    if (step) show(at + Number(step.dataset.step));
    else if (e.target.closest('[data-close]')) close();
  });
  document.addEventListener('keydown', (e) => {
    if (root.hidden) return;
    if (e.key === 'Escape') close();
    else if (e.key === 'ArrowRight') show(at + 1);
    else if (e.key === 'ArrowLeft') show(at - 1);
    else return;
    e.preventDefault();
    e.stopImmediatePropagation();
  }, true);
  return {
    open(items, index, after = () => {}) {
      list = items;
      onClose = after;
      root.hidden = false;
      show(index);
    },
    close,
    get isOpen() {
      return !root.hidden;
    },
  };
}

/** A small painted stand-in for a recording or a video before its first frame is known. */
export function placeholder(p, size = 512) {
  const c = document.createElement('canvas');
  c.width = c.height = size;
  const g = c.getContext('2d');
  const grad = g.createLinearGradient(0, 0, size, size);
  grad.addColorStop(0, p.kind === 'audio' ? '#6d5aa8' : '#3f6c8f');
  grad.addColorStop(1, p.kind === 'audio' ? '#d99ab4' : '#9fc7c0');
  g.fillStyle = grad;
  g.fillRect(0, 0, size, size);
  g.fillStyle = 'rgba(255, 255, 255, 0.92)';
  g.font = `${size * 0.3}px system-ui, sans-serif`;
  g.textAlign = 'center';
  g.textBaseline = 'middle';
  g.fillText(p.kind === 'audio' ? '♪' : '▶', size / 2, size * 0.44);
  g.font = `${size * 0.055}px system-ui, sans-serif`;
  g.fillText(p.title.slice(0, 28), size / 2, size * 0.78);
  return c;
}
