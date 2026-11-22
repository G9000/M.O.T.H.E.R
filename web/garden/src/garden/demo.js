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
