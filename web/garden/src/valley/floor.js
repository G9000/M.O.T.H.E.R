// The valley floor beyond the walk, painted from above: a patchwork of fields, each its own colour and worked in
// its own direction, hedges along most of their edges, and a road winding through to the village. It is drawn once
// into a picture that the far ground wears. The same plan says where the hedgerows, orchards, haystacks and
// roadside poplars stand (farwoods.js), so what is painted flat and what stands up on it agree.
import * as THREE from 'three';
import { fbm, fieldsAt, fromValley, VILLAGE } from './world.js';

export { VILLAGE };

/** The square of valley the picture covers, in metres down the valley (u) and to the right of it (v). */
export const FLOOR = { u0: 250, v0: -1200, size: 2400 };

const CELL = 150;

/** What a field is, and its colour as paint (linear). Near fields are grass of one kind or another. */
export const KINDS = [
  { name: 'pasture', colour: [0.17, 0.46, 0.14] },
  { name: 'pasture', colour: [0.27, 0.57, 0.13] },
  { name: 'hay', colour: [0.4, 0.62, 0.14] },
  { name: 'wheat', colour: [0.66, 0.6, 0.2] },
  { name: 'young', colour: [0.24, 0.58, 0.16] },
  { name: 'clover', colour: [0.11, 0.4, 0.17] },
  { name: 'orchard', colour: [0.2, 0.5, 0.14] },
  { name: 'plough', colour: [0.4, 0.3, 0.17] },
];
const ORCHARD = 6;

/** No road crosses this country's far plain. */
export const ROADS = [];

function rnd(ix, iy, salt) {
  let h = Math.imul(ix, 374761393) ^ Math.imul(iy, 668265263) ^ Math.imul(salt, 1274126177);
  h = Math.imul(h ^ (h >>> 13), 1274126177);
  h ^= h >>> 16;
  return (h >>> 0) / 4294967296;
}

/** A smooth curve through points, as points a `step` apart or so. */
export function along(points, step = 1) {
  const out = [];
  const at = (i) => points[Math.min(points.length - 1, Math.max(0, i))];
  for (let i = 0; i < points.length - 1; i++) {
    const [a, b, c, d] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
    const n = Math.max(2, Math.ceil(Math.hypot(c[0] - b[0], c[1] - b[1]) / step));
    for (let k = 0; k < n; k++) {
      const t = k / n;
      const t2 = t * t;
      const t3 = t2 * t;
      out.push([0, 1].map((j) => 0.5 * (2 * b[j] + (c[j] - a[j]) * t + (2 * a[j] - 5 * b[j] + 4 * c[j] - d[j]) * t2 + (3 * b[j] - a[j] - 3 * c[j] + d[j]) * t3)));
    }
  }
  out.push(points[points.length - 1]);
  return out;
}

/** What fieldAt found: filled in place, so asking a few million times makes no rubbish. */
const F = { kind: 0, border: 0, hedge: false, key: 0, dirx: 1, diry: 0 };

/**
 * The field at (u, v): its kind, how far it is to the field's edge (metres), whether that edge is hedged, a number
 * that names the field, and the way it is worked. Returns one object, reused.
 */
export function fieldAt(u, v) {
  // (edges that bend a little, as fields laid out by people over centuries do)
  const wu = u + 24 * Math.sin(v / 97 + 1.3) + 11 * Math.sin(v / 41);
  const wv = v + 24 * Math.sin(u / 113 + 0.4) + 11 * Math.sin(u / 37 + 2);
  const fx = wu / CELL;
  const fy = wv / CELL;
  const ix = Math.floor(fx);
  const iy = Math.floor(fy);
  let d1 = 1e9;
  let d2 = 1e9;
  let ax = 0;
  let ay = 0;
  let bx = 0;
  let by = 0;
  for (let j = -1; j <= 1; j++) {
    for (let i = -1; i <= 1; i++) {
      const cx = ix + i;
      const cy = iy + j;
      const dx = cx + 0.12 + 0.76 * rnd(cx, cy, 1) - fx;
      const dy = cy + 0.12 + 0.76 * rnd(cx, cy, 2) - fy;
      const d = dx * dx + dy * dy;
      if (d < d1) {
        d2 = d1;
        bx = ax;
        by = ay;
        d1 = d;
        ax = cx;
        ay = cy;
      } else if (d < d2) {
        d2 = d;
        bx = cx;
        by = cy;
      }
    }
  }
  F.border = (Math.sqrt(d2) - Math.sqrt(d1)) * 0.5 * CELL;
  F.key = ax * 4096 + ay;
  // most edges are hedged; some are only a change of crop
  F.hedge = rnd(Math.min(ax, bx) * 64 + Math.max(ax, bx), Math.min(ay, by) * 64 + Math.max(ay, by), 3) > 0.3;
  const pick = rnd(ax, ay, 4);
  let kind = pick < 0.26 ? 0 : pick < 0.46 ? 1 : pick < 0.6 ? 2 : pick < 0.74 ? 3 : pick < 0.83 ? 4 : pick < 0.89 ? 5 : pick < 0.95 ? ORCHARD : 7;
  // near the walk it is all grass, where your own meadow's blades may be standing on it
  if (Math.hypot((ax + 0.5) * CELL, (ay + 0.5) * CELL) < 690 && kind > 2) kind = kind % 3;
  F.kind = kind;
  const turn = rnd(ax, ay, 5) * Math.PI;
  F.dirx = Math.cos(turn);
  F.diry = Math.sin(turn);
  return F;
}

const toSrgb = (c) => Math.round(255 * Math.min(1, Math.max(0, c <= 0.0031308 ? c * 12.92 : 1.055 * c ** (1 / 2.4) - 0.055)));

/** Paints the picture: sRGB bytes, four a texel. Plain sums, so a worker can do it. @param size texels along its side */
export function paintFloor(size = 1536) {
  const data = new Uint8Array(size * size * 4);
  const px = FLOOR.size / size;
  // the roads first, as a stencil
  const road = new Float32Array(size * size);
  for (const line of ROADS) {
    for (const [u, v] of along(line, 0.7)) {
      const ci = (u - FLOOR.u0) / px;
      const cj = (v - FLOOR.v0) / px;
      const reach = 3.4 / px;
      for (let j = Math.max(0, Math.floor(cj - reach)); j <= Math.min(size - 1, Math.ceil(cj + reach)); j++) {
        for (let i = Math.max(0, Math.floor(ci - reach)); i <= Math.min(size - 1, Math.ceil(ci + reach)); i++) {
          const d = Math.hypot(i + 0.5 - ci, j + 0.5 - cj) * px;
          const k = 1 - Math.min(1, Math.max(0, (d - 1.9) / 1.3));
          if (k > road[i + j * size]) road[i + j * size] = k;
        }
      }
    }
  }
  let p = 0;
  for (let j = 0; j < size; j++) {
    const v = FLOOR.v0 + (j + 0.5) * px;
    for (let i = 0; i < size; i++, p += 4) {
      const u = FLOOR.u0 + (i + 0.5) * px;
      const [x, z] = fromValley(u, v);
      // faded out at the picture's own edge, wherever the farmland may reach
      const edge = Math.min(i, j, size - 1 - i, size - 1 - j) / 24;
      const farmed = fieldsAt(x, z) * Math.min(1, edge);
      if (farmed <= 0) {
        data[p] = 70;
        data[p + 1] = 130;
        data[p + 2] = 60;
        continue;
      }
      const f = fieldAt(u, v);
      const c = KINDS[f.kind].colour;
      // the rows it is worked in, and a brush's unevenness
      const rows = f.kind === 0 || f.kind === 1 || f.kind === 5 ? 0 : Math.sin(((u * f.dirx + v * f.diry) * Math.PI * 2) / 7) * 0.11;
      const wash = 0.9 + 0.2 * fbm(u / 55 + 3, v / 55 + 7, 2, 211) + rows;
      let r = c[0] * wash;
      let g = c[1] * wash;
      let b = c[2] * wash;
      // a hedge, or a strip of rough grass, where one field ends and the next begins
      const line = 1 - Math.min(1, Math.max(0, (f.border - (f.hedge ? 4.2 : 1.4)) / 1.2));
      if (line > 0) {
        const k = line * (f.hedge ? 1 : 0.5);
        r += (0.035 - r) * k;
        g += (0.16 - g) * k;
        b += (0.085 - b) * k;
      }
      // the village stands on plain trodden grass, not in a field
      const yard = 1 - Math.min(1, Math.max(0, (Math.hypot((u - VILLAGE[0]) / 80, (v - VILLAGE[1]) / 96) - 0.9) / 0.2));
      if (yard > 0) {
        r += (0.3 - r) * yard;
        g += (0.52 - g) * yard;
        b += (0.15 - b) * yard;
      }
      // the road: pale earth, with a grass crown where wheels do not run
      const way = road[i + j * size];
      if (way > 0) {
        r += (0.5 - r) * way;
        g += (0.45 - g) * way;
        b += (0.32 - b) * way;
      }
      data[p] = toSrgb(r);
      data[p + 1] = toSrgb(g);
      data[p + 2] = toSrgb(b);
      data[p + 3] = Math.round(255 * farmed);
    }
  }
  return data;
}

/** The picture as a texture. @param data what paintFloor(size) made, if it has been made already */
export function createFloor(renderer, size = 1536, data = paintFloor(size)) {
  const texture = new THREE.DataTexture(data, size, size, THREE.RGBAFormat, THREE.UnsignedByteType);
  texture.colorSpace = THREE.SRGBColorSpace;
  texture.generateMipmaps = true;
  texture.minFilter = THREE.LinearMipmapLinearFilter;
  texture.magFilter = THREE.LinearFilter;
  // (it is always seen at a low angle: without this it would be a smear)
  texture.anisotropy = renderer.capabilities.getMaxAnisotropy();
  texture.wrapS = texture.wrapT = THREE.ClampToEdgeWrapping;
  texture.needsUpdate = true;
  return texture;
}
