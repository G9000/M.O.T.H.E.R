// The memory garden's land: one round hill with a great tree on its crown, and a footpath that leaves the tree and
// winds outward and down the hill in a spiral. The path is the garden's timeline (see ../garden/garden.js): today is
// at the tree, and every turn further out is further back. It ends at a still pool at the hill's foot. Rolling downs
// lie round the hill, high wooded hills close them in, and one side stands open to the far mountains.
//
// Plain functions of (x, z), read once by the field and by the far land. The path's length depends on how many
// records there are, so `configure` must be called (on the page and in the worker) before anything is read.

/** The way you look from the start, a unit vector (x, z): the open side of the country. */
export const HERO = [0.6, 0.8];
const [DX, DZ] = HERO;
const TAU = Math.PI * 2;

/** Where you start, and the great tree just behind you: [x, z]. */
export const START = [0, 0];
export const TREE = fromValley(-9, -6);
/** The pool the path ends at. Placed by `configure`. */
export const TARN = { at: [0, 0], rx: 17, rz: 11, level: 0 };

/** (metres ahead of the start, metres to the right of it) to (x, z). */
export function fromValley(u, v) {
  return [u * DX - v * DZ, u * DZ + v * DX];
}

// ---- the path ---------------------------------------------------------------------------------------------------
const R_IN = 10; // metres from the start to where the path begins
const PITCH = 13; // metres between one turn of the path and the next
const PHI0 = Math.atan2(DZ, DX);
const STEP = 0.02;
/** Arc length at each step of the spiral's angle. */
let lengths = new Float32Array([0]);
const G = { length: 0, thetaMax: 0, radius: R_IN, tarnFloor: 0 };
const spiralR = (theta) => R_IN + (PITCH * theta) / TAU;

/** Makes the land for a path of `length` metres. */
export function configure({ length }) {
  const out = [0];
  let theta = 0;
  let sum = 0;
  while (sum < length) {
    const r = spiralR(theta + STEP / 2);
    sum += Math.hypot(r, PITCH / TAU) * STEP;
    theta += STEP;
    out.push(sum);
  }
  lengths = Float32Array.from(out);
  G.length = sum;
  G.thetaMax = theta;
  G.radius = spiralR(theta);
  // the pool lies just past the path's end, further out
  const end = pathAt(G.length);
  TARN.at = [end.x + end.ox * 24 + end.tx * 10, end.z + end.oz * 24 + end.tz * 10];
  G.tarnFloor = land(TARN.at[0], TARN.at[1]);
  TARN.level = G.tarnFloor - 0.55;
}

/** How long the path is, and how far from the start its last turn lies, metres. */
export const pathLength = () => G.length;
export const gardenRadius = () => G.radius;

/**
 * The path at `s` metres along it from the tree: where it is, the way it runs (tx, tz) and the way out from the
 * hill's middle (ox, oz), both unit vectors.
 */
export function pathAt(s) {
  const at = Math.min(Math.max(s, 0), G.length);
  let lo = 0;
  let hi = lengths.length - 1;
  while (hi - lo > 1) {
    const mid = (lo + hi) >> 1;
    if (lengths[mid] <= at) lo = mid;
    else hi = mid;
  }
  const span = lengths[hi] - lengths[lo] || 1;
  const theta = (lo + (at - lengths[lo]) / span) * STEP;
  const r = spiralR(theta);
  const a = PHI0 + theta;
  const ox = Math.cos(a);
  const oz = Math.sin(a);
  // (the tangent: round the hill, and a little outward)
  const k = PITCH / TAU / r;
  const n = Math.hypot(1, k);
  return { x: r * ox, z: r * oz, ox, oz, tx: (-oz + k * ox) / n, tz: (ox + k * oz) / n };
}

/** How far (x, z) is from the middle of the footpath, metres. */
export function pathDistance(x, z) {
  const r = Math.hypot(x, z);
  let phi = (Math.atan2(z, x) - PHI0) % TAU;
  if (phi < 0) phi += TAU;
  // the turn of the spiral nearest this far out, along this bearing
  const turn = Math.round((r - R_IN) / PITCH - phi / TAU);
  const theta = phi + TAU * turn;
  if (theta >= 0 && theta <= G.thetaMax) return Math.abs(r - spiralR(theta));
  // past either end it peters out
  const end = theta < 0 ? 0 : G.thetaMax;
  const er = spiralR(end);
  return Math.hypot(x - er * Math.cos(PHI0 + end), z - er * Math.sin(PHI0 + end)) + 1.2;
}

/** True where the garden's flowers grow: no tree, bush or stone may stand there. */
export const inGarden = (x, z) => Math.hypot(x, z) < G.radius + 9;

// ---- noise ------------------------------------------------------------------------------------------------------
function hash(ix, iz, seed) {
  let h = Math.imul(ix, 374761393) ^ Math.imul(iz, 668265263) ^ Math.imul(seed, 1274126177);
  h = Math.imul(h ^ (h >>> 13), 1274126177);
  h ^= h >>> 16;
  return (h >>> 0) / 4294967296;
}

function vnoise(x, z, seed) {
  const ix = Math.floor(x);
  const iz = Math.floor(z);
  const fx = x - ix;
  const fz = z - iz;
  const ux = fx * fx * fx * (fx * (fx * 6 - 15) + 10);
  const uz = fz * fz * fz * (fz * (fz * 6 - 15) + 10);
  const a = hash(ix, iz, seed);
  const b = hash(ix + 1, iz, seed);
  const c = hash(ix, iz + 1, seed);
  const d = hash(ix + 1, iz + 1, seed);
  return a + (b - a) * ux + (c - a) * uz + (a - b - c + d) * ux * uz;
}

/** 0 to 1, mostly near 0.5. */
export function fbm(x, z, octaves, seed) {
  let sum = 0;
  let amp = 0.5;
  let norm = 0;
  for (let o = 0; o < octaves; o++) {
    sum += amp * vnoise(x, z, seed + o * 17);
    norm += amp;
    amp *= 0.5;
    x = x * 2.03 + 11.7;
    z = z * 2.03 - 5.3;
  }
  return sum / norm;
}

export const smooth = (a, b, x) => {
  const t = Math.min(1, Math.max(0, (x - a) / (b - a)));
  return t * t * (3 - 2 * t);
};

// ---- the land ---------------------------------------------------------------------------------------------------
/** 0 in the middle of the pool, 1 at its shore, more outside. */
export function tarnDistance(x, z) {
  const dx = x - TARN.at[0];
  const dz = z - TARN.at[1];
  const u = dx * DX + dz * DZ;
  const v = -dx * DZ + dz * DX;
  const wobble = 1 + 0.16 * Math.sin(Math.atan2(u, v) * 3 + 0.7) + 0.09 * Math.sin(Math.atan2(u, v) * 7 + 2.1);
  return Math.hypot(v / TARN.rx, u / TARN.rz) / wobble;
}

/** There is no river and no farmland in this country: the modules that paint the far land ask, and are told so. */
export const riverDistance = () => 1e6;
export const fieldsAt = () => 0;
export const VILLAGE = [1e6, 1e6];

/** 0 to 1: forest on the far land at (x, z). None near the walk. */
export function forestAt(x, z) {
  const r = Math.hypot(x, z);
  let f = smooth(0.44, 0.52, fbm(x / 1700 + 4.1, z / 1700 + 8.6, 4, 191) * 0.72 + fbm(x / 430 + 6.2, z / 430 + 1.3, 3, 197) * 0.28);
  f = Math.max(f, smooth(55, 120, land(x, z)) * smooth(0.36, 0.5, fbm(x / 900 + 7.7, z / 900 + 2.4, 3, 201)));
  return f * smooth(820, 1500, r);
}

function land(x, z) {
  const u = x * DX + z * DZ;
  const v = -x * DZ + z * DX;
  const r = Math.hypot(x, z);
  const edge = G.radius;
  // the hill: level on its crown where the tree stands, falling away all round, so each turn of the path lies
  // below the one before and you look down from the present over what came earlier
  const crown = Math.min(46, 7 + edge * 0.17);
  let h = crown * (1 - smooth(5, edge * 1.25 + 34, r));
  // the downs round it: long swells, calm near the garden so its flowers stand on clean ground
  h += (fbm(x / 240 + 5.3, z / 240 - 2.9, 4, 101) - 0.5) * 24 * smooth(edge + 20, edge + 320, r);
  h += (fbm(x / 64 + 3.9, z / 64 + 7.1, 3, 105) - 0.5) * 3.2 * smooth(edge - 10, edge + 70, r);
  h += (fbm(x / 110 + 8.7, z / 110 + 1.3, 2, 113) - 0.5) * 16 * smooth(edge + 40, edge + 160, r) * (1 - smooth(900, 1800, r));
  // high wooded hills close the country in, except ahead of the start, where the land falls away to the plain
  const bearing = Math.atan2(v, u);
  const ahead = Math.exp(-((bearing / 0.8) ** 2));
  const rim = smooth(edge + 240, edge + 1150, r) * (0.6 + 0.8 * fbm(x / 800 + 6, z / 800 + 3, 3, 137));
  h += 125 * rim * (1 - 0.9 * ahead);
  h -= 22 * smooth(edge + 50, edge + 520, r) * ahead;
  return h;
}

export function heightAt(x, z) {
  let h = land(x, z);
  const t = tarnDistance(x, z);
  if (t < 2.2) {
    // level ground round the water, a bank, and a bed that deepens to the middle
    const flat = 1 - smooth(1.25, 2.2, t);
    h += (G.tarnFloor - h) * flat;
    h -= 2.4 * (1 - smooth(0.25, 1.06, t));
  }
  return h;
}

export function paint(x, z) {
  // lush nearly everywhere: summer. A little straw on the driest shoulders.
  const dry = 0.08 + 0.3 * smooth(0.54, 0.8, fbm(x / 90 + 7.2, z / 90 + 3.1, 3, 121));
  const t = tarnDistance(x, z);
  const wet = Math.max(smooth(0.68, 0.76, fbm(x / 47 + 1.5, z / 47 + 6.9, 2, 133)) * 0.6, 1 - smooth(1.0, 1.7, t));
  let growth = 1;
  // the footpath worn to earth, its edges ragged
  const along = Math.max(0, Math.min(R_IN, x * DX + z * DZ));
  const approach = Math.hypot(x - along * DX, z - along * DZ);
  const path = Math.min(approach, pathDistance(x, z));
  growth = Math.min(growth, smooth(0.42, 1.1, path + (fbm(x / 2.7, z / 2.7, 2, 161) - 0.5) * 0.7));
  // nothing grows under the water
  growth = Math.min(growth, smooth(0.8, 0.86, t + (fbm(x / 3.3, z / 3.3, 2, 167) - 0.5) * 0.04));
  // wild flowers in drifts on the downs; few on the hill itself, where the records' own flowers are to be seen
  const drifts = smooth(0.4, 0.68, fbm(x / 38 - 2, z / 38 + 9, 2, 155));
  const flowers = Math.min(1, drifts * (0.25 + 0.65 * smooth(G.radius, G.radius + 40, Math.hypot(x, z))));
  return { dry: dry * (1 - wet), growth, wet, flowers };
}
