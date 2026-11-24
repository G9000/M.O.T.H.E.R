// Garden appearance v1. The order and random draws here are part of a record's visual identity.
// A decimal seed can be copied from its card and passed back to appearanceFromSeed.
export const FORMS = ['daisy', 'tulip', 'poppy', 'bellflower', 'sunflower', 'rose', 'river stone', 'slate cairn'];
export const COMPANIONS = ['hydrangea', 'camellia', 'winterberry', 'bearded iris', 'foxglove', 'lavender'];
const petals = [
  [0.98, 0.55, 0.65], [0.74, 0.54, 0.94], [0.50, 0.68, 0.98], [0.98, 0.84, 0.55],
  [0.98, 0.65, 0.35], [0.96, 0.84, 0.84], [0.51, 0.84, 0.72], [0.90, 0.39, 0.57],
];
const minerals = [[0.45, 0.54, 0.55], [0.53, 0.42, 0.45], [0.47, 0.54, 0.38], [0.68, 0.52, 0.40]];

export function appearanceFromSeed(seed) {
  const bits = BigInt(seed);
  let state = Number(BigInt.asUintN(32, bits));
  const step = (Number(BigInt.asUintN(32, bits >> 32n)) | 1) >>> 0;
  function draw() {
    state = (state + step) >>> 0;
    let v = Math.imul(state ^ (state >>> 16), 0x21f0aaad);
    v = Math.imul(v ^ (v >>> 15), 0x735a2d97);
    return ((v ^ (v >>> 15)) >>> 0) / 4294967296;
  }
  const form = FORMS[Math.floor(draw() * FORMS.length)];
  const stone = form === 'river stone' || form === 'slate cairn';
  const palette = stone ? minerals : petals;
  const a = palette[Math.floor(draw() * palette.length)], b = palette[Math.floor(draw() * palette.length)];
  const blend = 0.35 * draw();
  return {
    seed: bits.toString(), form, stone, pigment: a.map((v, i) => v + (b[i] - v) * blend),
    width: 0.78 + 0.55 * draw(), depth: 0.8 + 0.4 * draw(), height: 0.84 + 0.36 * draw(),
    turn: draw() * Math.PI * 2, curve: draw() * 1.2 - 0.6, phase: draw() * Math.PI * 2,
    structure: form === 'river stone' ? 0 : Math.floor(draw() * 3) - 1,
  };
}

export function gardenAppearance(id) {
  // FNV-1a over the stable ID bytes. Revisions, content, pins and neighbours can all change.
  let hash = 0xcbf29ce484222325n;
  for (const byte of new TextEncoder().encode(id)) hash = BigInt.asUintN(64, (hash ^ BigInt(byte)) * 0x100000001b3n);
  return appearanceFromSeed(hash);
}

// A separate sequence lets the border gain species without rerolling an established record.
export function companionAppearance(seed) {
  const bits = BigInt(seed) ^ 0x719b3c5a8d02e6f1n;
  const a = appearanceFromSeed(bits);
  const form = COMPANIONS[Number(bits % BigInt(COMPANIONS.length))];
  const tint = {
    hydrangea: [0.48, 0.59, 0.98], camellia: [0.95, 0.28, 0.45], winterberry: [0.88, 0.13, 0.07],
    'bearded iris': [0.53, 0.36, 0.93], foxglove: [0.91, 0.50, 0.73], lavender: [0.51, 0.36, 0.82],
  }[form];
  return { ...a, form, stone: false, companion: true, seed: String(seed),
    structure: Number((bits >> 16n) % 3n) - 1,
    pigment: tint.map((v, i) => 0.72 * v + 0.28 * a.pigment[i]) };
}
