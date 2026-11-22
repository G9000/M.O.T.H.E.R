// Invented records for working on the page with `npm run dev` only. The production build never includes this file:
// valley/main.js imports it behind import.meta.env.DEV, which the build replaces with false.
function seeded(seed = 1) {
  let s = seed >>> 0 || 1;
  return () => {
    s ^= s << 13;
    s ^= s >>> 17;
    s ^= s << 5;
    return (s >>> 0) / 4294967296;
  };
}

const WORDS = 'river lantern quiet harbor orchard letter morning thread compass meadow ember window garden bridge lark tide willow ladder cedar ribbon hearth kettle clover map'.split(' ');
const HARNESSES = ['claude-code', 'codex', 'chatgpt', 'owner', 'cursor'];

export function demoSnapshot(count = Number(new URLSearchParams(location.search).get('records')) || 420) {
  const rand = seeded(42);
  const pick = (a) => a[Math.floor(rand() * a.length)];
  const now = Date.UTC(2026, 10, 21);
  const day = 86400000;
  const records = [];
  for (let n = 0; n < count; n++) {
    const kind = pick(['memory', 'memory', 'memory', 'decision', 'task', 'task', 'checkpoint', 'artifact', 'action']);
    const status = {
      memory: n % 23 === 2 ? 'retracted' : 'active',
      decision: pick(['proposed', 'confirmed', 'confirmed', 'superseded']),
      task: pick(['todo', 'doing', 'blocked', 'done', 'done', 'cancelled']),
      checkpoint: 'active',
      artifact: 'active',
      action: pick(['pending', 'succeeded', 'succeeded', 'failed', 'unknown']),
    }[kind];
    const created = now - Math.pow(rand(), 1.4) * 300 * day;
    const revision = 1 + Math.floor(Math.pow(rand(), 3) * 9);
    const actor = pick(HARNESSES);
    const title = `${pick(WORDS)[0].toUpperCase()}${pick(WORDS).slice(1)} ${pick(WORDS)} ${pick(['notes', 'plan', 'choice', 'idea', 'check', 'draft', 'promise', 'question'])}`;
    const history = [];
    for (let v = 1; v <= revision; v++) history.push({ revision: v, actor: v === 1 ? actor : pick(HARNESSES), at: new Date(created + (v - 1) * day * 2).toISOString(), changed: v === 1 ? null : ['content'], excerpt: `Invented example text, revision ${v}.` });
    records.push({
      id: `demo-${n.toString(16).padStart(6, '0')}-${Math.floor(rand() * 1e9).toString(16)}`,
      kind,
      status,
      title,
      content: `Invented example text number ${n}, written only to grow a demonstration garden. ${pick(WORDS)} and ${pick(WORDS)} by the ${pick(WORDS)}.`,
      truncated: false,
      revision,
      actor: history[history.length - 1].actor,
      createdAt: new Date(created).toISOString(),
      updatedAt: history[history.length - 1].at,
      references: [],
      history,
      historyOmitted: 0,
      ...(kind === 'checkpoint' ? { nextStep: 'Invented next step.' } : {}),
      ...(n % 41 === 0 ? { pinned: { actor: 'owner', pinnedAt: new Date(created).toISOString() } } : {}),
      ...(kind === 'task' && status === 'doing' && n % 3 === 0 ? { assignment: { assignee: 'codex', assignedBy: 'claude-code', assignedAt: new Date(created).toISOString() } } : {}),
    });
  }
  for (const r of records) if (rand() < 0.28) for (let i = 0; i < 1 + Math.floor(rand() * 3); i++) r.references.push(pick(records).id);
  return { workspaceId: 'demo', generatedAt: new Date(now).toISOString(), total: count + 12, shown: count, records };
}

/** Invented photos for the grove: little painted scenes, made here, with the fields the gallery listing carries. */
export async function demoMedia(count = 14) {
  const rand = seeded(7);
  const skies = [
    ['#7fb2e5', '#f8d9b0'],
    ['#f1a77f', '#ffe3b5'],
    ['#2f3d73', '#d17d8f'],
    ['#9ad0e8', '#e8f3ea'],
    ['#6f5aa8', '#f2b6c4'],
  ];
  const words = ['harbour at dusk', 'the old orchard', 'lanterns by the river', 'morning over the hill', 'a letter on the desk', 'snow on the ridge', 'the kitchen window'];
  const out = [];
  for (let i = 0; i < count; i++) {
    const wide = rand() < 0.6;
    const c = document.createElement('canvas');
    c.width = wide ? 960 : 640;
    c.height = wide ? 640 : 860;
    const g = c.getContext('2d');
    const [top, low] = skies[i % skies.length];
    const sky = g.createLinearGradient(0, 0, 0, c.height);
    sky.addColorStop(0, top);
    sky.addColorStop(0.7, low);
    g.fillStyle = sky;
    g.fillRect(0, 0, c.width, c.height);
    g.fillStyle = 'rgba(255, 245, 220, 0.9)';
    g.beginPath();
    g.arc(c.width * (0.2 + rand() * 0.6), c.height * (0.25 + rand() * 0.2), 30 + rand() * 40, 0, Math.PI * 2);
    g.fill();
    for (let h = 0; h < 3; h++) {
      g.fillStyle = `hsl(${100 + rand() * 60}, ${30 + h * 10}%, ${45 - h * 10}%)`;
      g.beginPath();
      g.moveTo(0, c.height);
      for (let x = 0; x <= c.width; x += 40) g.lineTo(x, c.height * (0.55 + h * 0.12) + Math.sin(x * 0.01 + h * 2 + i) * 30);
      g.lineTo(c.width, c.height);
      g.fill();
    }
    const blob = await new Promise((done) => c.toBlob(done, 'image/png'));
    const title = words[i % words.length];
    const at = new Date(Date.UTC(2026, 10, 20) - rand() * 300 * 86400000).toISOString();
    out.push({
      blob,
      item: {
        sha256: `demo${i.toString(16).padStart(60, '0')}`,
        variant: i % 3 ? 'derivative' : 'original',
        mediaType: 'image/png',
        width: c.width,
        height: c.height,
        uses: 1 + (i % 3),
        acquisitions: [{ originalFilename: `IMG_${2040 + i}.png`, acquiredAt: at, eventTime: { at, precision: 'day' }, origin: i % 4 ? 'captured' : 'screenshot' }],
        interpretations: i % 2 ? [{ predicate: 'mother.media:caption', text: `Invented caption: ${title}`, attribution: { producer: { name: 'demo-model' } } }] : [],
      },
    });
  }
  return out;
}
