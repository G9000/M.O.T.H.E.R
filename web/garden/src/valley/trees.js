// The trees: one great old tree by the start of the walk and groves of smaller ones about the valley. A tree is a
// trunk and boughs grown by a few rules, and a crown painted the way a background artist paints one: in clumps,
// each clump a ball of leaf sprays lit as a ball, light on the side the sun is and deep blue-green on the other.
// Their shade is baked into a map the grass reads, with moving spots of light let through it.
import * as THREE from 'three';
import { COMMON } from '../lib/index.js';
import { fromValley, gardenRadius, heightAt, inGarden, pathDistance, riverDistance, TARN, tarnDistance, TREE } from './world.js';

const TAU = Math.PI * 2;
const V3 = THREE.Vector3;

function mulberry(seed) {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** A spray of leaves, white on black: the shape every card of a crown is cut to. */
function leafTexture() {
  const size = 256;
  const canvas = document.createElement('canvas');
  canvas.width = canvas.height = size;
  const g = canvas.getContext('2d');
  g.fillStyle = '#000';
  g.fillRect(0, 0, size, size);
  g.fillStyle = '#fff';
  const random = mulberry(77);
  const leaf = (x, y, angle, len) => {
    const wid = len * (0.34 + random() * 0.16);
    // (red is how light this leaf is painted; green is where there is leaf at all)
    g.fillStyle = `rgb(${130 + Math.floor(random() * 125)},255,0)`;
    g.save();
    g.translate(x, y);
    g.rotate(angle);
    g.beginPath();
    g.moveTo(-len / 2, 0);
    g.quadraticCurveTo(-len * 0.05, -wid, len / 2, 0);
    g.quadraticCurveTo(-len * 0.05, wid, -len / 2, 0);
    g.fill();
    g.restore();
  };
  for (let i = 0; i < 30; i++) {
    const a = (i / 30) * TAU * 2 + random() * 0.6;
    const d = 22 + random() * 78;
    leaf(128 + Math.cos(a) * d, 128 + Math.sin(a) * d, a + (random() - 0.5) * 1.5, 26 + random() * 22);
  }
  for (let i = 0; i < 8; i++) leaf(128 + (random() - 0.5) * 60, 128 + (random() - 0.5) * 60, random() * TAU, 28 + random() * 18);
  const texture = new THREE.CanvasTexture(canvas);
  texture.generateMipmaps = true;
  texture.minFilter = THREE.LinearMipmapLinearFilter;
  texture.magFilter = THREE.LinearFilter;
  texture.anisotropy = 4;
  texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
  return texture;
}

/**
 * Grows one tree, standing at the origin. Returns its wood and its leaves as two geometries, and how big its
 * crown is, for its shadow.
 * @param o.trunk height of the bole, o.girth its radius, o.crown the crown's radius, o.rise the crown's height,
 *   o.limbs how many, o.clump the radius of a clump of leaves, o.fill clumps added to round the crown out,
 *   o.cards leaf sprays to a clump, o.spray the size of one
 */
function growTree(seed, o) {
  const random = mulberry(seed);
  const tubes = [];
  const clumps = [];
  // an old tree leans, and its crown is fuller on one side
  const lean = random() * TAU;
  const top = new V3(Math.cos(lean) * o.trunk * 0.1, o.trunk, Math.sin(lean) * o.trunk * 0.1);
  tubes.push({ pts: [new V3(0, -0.8, 0), new V3(Math.cos(lean + 1) * o.girth * 0.25, o.trunk * 0.45, Math.sin(lean + 1) * o.girth * 0.25), top], r0: o.girth, r1: o.girth * 0.74, sides: 14, root: true });
  const crownY = o.trunk + o.rise * 0.42;
  const full = lean + 2 + random() * 2;
  const reach = (a) => o.crown * (0.82 + 0.26 * Math.cos(a - full) + 0.1 * Math.sin(a * 3 + seed));
  for (let i = 0; i < o.limbs; i++) {
    const a = ((i + 0.15 + random() * 0.7) / o.limbs) * TAU;
    const elev = 0.38 + random() * 0.62;
    const len = reach(a) * (0.62 + 0.26 * random());
    const dir = new V3(Math.cos(a) * Math.cos(elev), Math.sin(elev), Math.sin(a) * Math.cos(elev));
    const side = new V3(-Math.sin(a), 0, Math.cos(a));
    const bend = (random() - 0.5) * len * 0.3;
    const mid = top.clone().addScaledVector(dir, len * 0.5).addScaledVector(side, bend).add(new V3(0, -len * 0.07, 0));
    const end = top.clone().addScaledVector(dir, len).addScaledVector(side, bend * 0.4).add(new V3(0, len * 0.14, 0));
    const q1 = top.clone().lerp(mid, 0.5).add(new V3((random() - 0.5) * len * 0.12, len * 0.05, (random() - 0.5) * len * 0.12));
    const q3 = mid.clone().lerp(end, 0.5).add(new V3((random() - 0.5) * len * 0.14, -len * 0.04, (random() - 0.5) * len * 0.14));
    const limb = new THREE.CatmullRomCurve3([top.clone().add(new V3(0, -o.girth * 0.5, 0)), q1, mid, q3, end]);
    tubes.push({ curve: limb, r0: o.girth * (0.36 + 0.12 * random()), r1: o.girth * 0.1, sides: 9 });
    clumps.push({ c: end, r: o.clump * (0.85 + 0.4 * random()) });
    const boughs = 2 + (random() < 0.5 ? 1 : 0);
    for (let j = 0; j < boughs; j++) {
      const from = limb.getPoint(0.4 + 0.45 * random());
      const a2 = a + (random() - 0.5) * 2.2;
      const e2 = 0.25 + random() * 0.85;
      const l2 = len * (0.34 + 0.26 * random());
      const d2 = new V3(Math.cos(a2) * Math.cos(e2), Math.sin(e2), Math.sin(a2) * Math.cos(e2));
      const end2 = from.clone().addScaledVector(d2, l2);
      const mid2 = from.clone().addScaledVector(d2, l2 * 0.5).add(new V3((random() - 0.5) * l2 * 0.2, l2 * 0.06, (random() - 0.5) * l2 * 0.2));
      tubes.push({ pts: [from, mid2, end2], r0: o.girth * 0.13, r1: o.girth * 0.035, sides: 6 });
      clumps.push({ c: end2, r: o.clump * (0.75 + 0.45 * random()) });
    }
  }
  // clumps on the skin of the crown, so its outline is a full, uneven dome and not a ring of branch ends
  for (let i = 0; i < o.fill; i++) {
    const a = random() * TAU;
    const up = Math.min(0.96, -0.3 + 1.3 * random());
    const ring = Math.sqrt(1 - up * up);
    const r = reach(a) * 0.8;
    clumps.push({ c: new V3(Math.cos(a) * ring * r, crownY + up * o.rise * 0.5, Math.sin(a) * ring * r), r: o.clump * (0.8 + 0.5 * random()) });
  }

  // ---- wood ----
  const pos = [];
  const nor = [];
  const bark = [];
  const idx = [];
  for (const tube of tubes) {
    const curve = tube.curve ?? new THREE.CatmullRomCurve3(tube.pts);
    const rows = tube.root ? 14 : 10;
    const frames = curve.computeFrenetFrames(rows, false);
    const first = pos.length / 3;
    const phase = random() * TAU;
    const along = curve.getLength();
    for (let j = 0; j <= rows; j++) {
      const t = j / rows;
      const p = curve.getPoint(t);
      for (let i = 0; i <= tube.sides; i++) {
        const a = (i / tube.sides) * TAU;
        let r = tube.r0 + (tube.r1 - tube.r0) * t;
        // a bole spreads into its roots, and is never round
        if (tube.root) r *= 1 + 1.15 * Math.exp(-t * 7.5) * (0.62 + 0.38 * Math.sin(a * 5 + phase) * Math.sin(a * 2 + 1)) + 0.07 * Math.sin(a * 3 + t * 6 + phase);
        const n = frames.normals[j].clone().multiplyScalar(Math.cos(a)).addScaledVector(frames.binormals[j], Math.sin(a));
        pos.push(p.x + n.x * r, p.y + n.y * r, p.z + n.z * r);
        nor.push(n.x, n.y, n.z);
        bark.push(i / tube.sides, t * along);
      }
    }
    for (let j = 0; j < rows; j++) {
      for (let i = 0; i < tube.sides; i++) {
        const a = first + j * (tube.sides + 1) + i;
        const b = a + tube.sides + 1;
        idx.push(a, b, a + 1, a + 1, b, b + 1);
      }
    }
  }
  const wood = new THREE.BufferGeometry();
  wood.setAttribute('position', new THREE.Float32BufferAttribute(pos, 3));
  wood.setAttribute('normal', new THREE.Float32BufferAttribute(nor, 3));
  wood.setAttribute('aBark', new THREE.Float32BufferAttribute(bark, 2));
  wood.setIndex(idx);

  // ---- leaves ----
  const lp = [];
  const luv = [];
  const lball = [];
  const lrand = [];
  const lidx = [];
  const cp = [];
  const cball = [];
  const crand = [];
  const ico = new THREE.IcosahedronGeometry(1, 1).attributes.position;
  const centre = new V3(0, crownY, 0);
  const x = new V3();
  const y = new V3();
  const n = new V3();
  for (const clump of clumps) {
    const phase = random();
    // the mass of the clump: a dark ball the sprays stand on, so a crown is solid and not lace
    {
      const e = Math.hypot((clump.c.x - centre.x) / o.crown, (clump.c.y - centre.y) / (o.rise * 0.5), (clump.c.z - centre.z) / o.crown);
      const low = THREE.MathUtils.smoothstep((clump.c.y - centre.y) / (o.rise * 0.5), -0.9, 0.2);
      const open = THREE.MathUtils.clamp((0.22 + 0.78 * THREE.MathUtils.smoothstep(e, 0.4, 0.95)) * (0.55 + 0.45 * low), 0, 1);
      const tone = random();
      const r = clump.r * 0.7;
      for (let i = 0; i < ico.count; i++) {
        const nx = ico.getX(i);
        const ny = ico.getY(i);
        const nz = ico.getZ(i);
        // (a little squashed, and lumpy)
        const k = r * (0.9 + 0.2 * Math.sin(nx * 5 + phase * 9) * Math.sin(nz * 4 + ny * 3));
        cp.push(clump.c.x + nx * k, clump.c.y + ny * k * 0.82, clump.c.z + nz * k);
        cball.push(nx, ny, nz);
        crand.push(tone, phase, open, clump.c.y);
      }
    }
    for (let k = 0; k < o.cards; k++) {
      // points spread evenly over the ball, then shaken
      const up = 1 - (2 * (k + 0.5)) / o.cards;
      const ring = Math.sqrt(1 - up * up);
      const a = k * 2.39996 + phase * TAU;
      const dir = new V3(Math.cos(a) * ring, up, Math.sin(a) * ring).add(new V3(random() - 0.5, random() - 0.5, random() - 0.5).multiplyScalar(0.5)).normalize();
      const at = clump.c.clone().addScaledVector(dir, clump.r * (0.62 + 0.42 * random()));
      // the spray lies roughly on the ball's skin, tipped every which way
      n.copy(dir).add(new V3(random() - 0.5, random() - 0.5, random() - 0.5).multiplyScalar(1.1)).normalize();
      x.set(random() - 0.5, random() - 0.5, random() - 0.5).cross(n).normalize();
      y.crossVectors(n, x);
      const size = o.spray * (0.75 + 0.6 * random());
      const first = lp.length / 3;
      // how deep in the crown it sits: the inside and the underside are dark
      const e = Math.hypot((at.x - centre.x) / o.crown, (at.y - centre.y) / (o.rise * 0.5), (at.z - centre.z) / o.crown);
      const low = THREE.MathUtils.smoothstep((at.y - centre.y) / (o.rise * 0.5), -0.9, 0.2);
      const open = THREE.MathUtils.clamp((0.22 + 0.78 * THREE.MathUtils.smoothstep(e, 0.4, 0.95)) * (0.55 + 0.45 * low), 0, 1);
      const tone = random();
      for (const [sx, sy] of [[-1, -1], [1, -1], [1, 1], [-1, 1]]) {
        lp.push(at.x + (x.x * sx + y.x * sy) * size * 0.5, at.y + (x.y * sx + y.y * sy) * size * 0.5, at.z + (x.z * sx + y.z * sy) * size * 0.5);
        luv.push(sx * 0.5 + 0.5, sy * 0.5 + 0.5);
        lball.push(dir.x, dir.y, dir.z);
        lrand.push(tone, phase, open, at.y);
      }
      lidx.push(first, first + 1, first + 2, first, first + 2, first + 3);
    }
  }
  const leaves = new THREE.BufferGeometry();
  leaves.setAttribute('position', new THREE.Float32BufferAttribute(lp, 3));
  leaves.setAttribute('uv', new THREE.Float32BufferAttribute(luv, 2));
  leaves.setAttribute('aBall', new THREE.Float32BufferAttribute(lball, 3));
  leaves.setAttribute('aRand', new THREE.Float32BufferAttribute(lrand, 4));
  leaves.setIndex(lidx);
  const core = new THREE.BufferGeometry();
  core.setAttribute('position', new THREE.Float32BufferAttribute(cp, 3));
  core.setAttribute('aBall', new THREE.Float32BufferAttribute(cball, 3));
  core.setAttribute('aRand', new THREE.Float32BufferAttribute(crand, 4));
  return { wood, leaves, core, crown: o.crown, crownY, height: o.trunk + o.rise };
}

const leafVertex = /* glsl */ `
${COMMON}
in vec3 aBall;
in vec4 aRand;   // tone, its clump's phase, how open to the sky, height in the tree
out vec2 vUv;
out vec3 vWorld;
out vec3 vBall;
out vec3 vRand;
void main() {
  mat4 m = modelMatrix * instanceMatrix;
  vec4 w = m * vec4(position, 1.0);
  vBall = normalize(mat3(m) * aBall);
  // the crown rides the gusts, each clump in its own time, each spray trembling
  float gust = gustAt(w.xz);
  float U = uWind.z;
  float high = smoothstep(1.0, 9.0, aRand.w);
  float sway = sin(uTime * (0.9 + 0.5 * aRand.y) + aRand.y * 37.0);
  w.xz += uWind.xy * (U * (0.012 + 0.05 * gust) + sway * 0.012 * U) * high;
  w.xyz += vBall * sin(uTime * (2.6 + 2.0 * aRand.x) + aRand.x * 61.0) * 0.006 * U * (0.4 + gust);
  vWorld = w.xyz;
  vUv = uv;
  vRand = aRand.xyz;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`;

const leafFragment = /* glsl */ `
${COMMON}
layout(location = 0) out vec4 outC;
uniform sampler2D tLeaf;
in vec2 vUv;
in vec3 vWorld;
in vec3 vBall;
in vec3 vRand;
void main() {
  float dist = distance(vWorld, uCamPos);
  vec4 leaf = texture(tLeaf, vUv);
  float a = leaf.g;
  // far off the sprays thin to nothing as their picture blurs: keep the crown whole
  float cut = mix(0.5, 0.16, smoothstep(20.0, 100.0, dist));
  a = clamp((a - cut) / max(fwidth(a), 1e-4) + 0.5, 0.0, 1.0);
  if (a < 0.04) discard;
  vec3 N = normalize(vBall);
  vec3 V = normalize(uCamPos - vWorld);
  // each clump is lit as a ball; each spray is a shade of its own, as brush marks are
  float tone = dot(N, uSunDir) * 0.5 + 0.5;
  // (from far off a crown is its lobes, not its leaves: the brush gets broader)
  float fine = mix(1.0, 0.25, smoothstep(14.0, 90.0, dist));
  tone = tone * mix(0.5, 1.0, vRand.z) + ((vRand.x - 0.5) * 0.36 + (leaf.r / max(leaf.g, 0.05) - 0.75) * 0.5) * fine;
  vec3 col = mix(vec3(0.014, 0.1, 0.1), vec3(0.035, 0.2, 0.12), smoothstep(0.3, 0.42, tone));
  col = mix(col, vec3(0.1, 0.36, 0.12), smoothstep(0.56, 0.68, tone));
  col = mix(col, vec3(0.3, 0.56, 0.15), smoothstep(0.84, 0.92, tone));
  float cloud = cloudAt(vWorld.xz + uSunDir.xz * 30.0);
  vec3 light = uSkyColor * 0.9 + uSunColor * (0.2 + 0.16 * smoothstep(0.5, 0.9, tone)) * mix(0.45, 1.0, clamp((cloud - 0.5) * 2.0, 0.0, 1.0));
  // seen against the sun, leaves glow
  float through = pow(max(dot(-V, uSunDir), 0.0), 2.0);
  col = col * light + vec3(0.3, 0.5, 0.05) * uSunColor * through * (0.05 + 0.24 * vRand.x * vRand.x);
  outC = vec4(applyFog(col, vWorld), a);
}
`;

const coreFragment = /* glsl */ `
${COMMON}
layout(location = 0) out vec4 outC;
in vec2 vUv;
in vec3 vWorld;
in vec3 vBall;
in vec3 vRand;
uniform sampler2D tLeaf;
void main() {
  vec3 N = normalize(vBall);
  // not a solid: a shell of leaves, cut from the same sprays, that you see into
  vec3 w = abs(N);
  vec2 uv = w.y > max(w.x, w.z) ? N.xz : w.x > w.z ? N.zy : N.xy;
  if (texture(tLeaf, uv * 2.3 + vRand.y * 7.0).g < 0.4) discard;
  if (!gl_FrontFacing) N = -N;
  float n = texture(tNoise, vWorld.xz * 0.9 + vWorld.y * 0.37).r;
  // lit as the sprays on it are, a tone deeper: it is the inside of the crown
  float tone = dot(N, uSunDir) * 0.5 + 0.5;
  tone = tone * mix(0.5, 1.0, vRand.z) + (n - 0.5) * 0.36 - 0.04;
  vec3 col = mix(vec3(0.014, 0.1, 0.1), vec3(0.035, 0.19, 0.115), smoothstep(0.3, 0.42, tone));
  col = mix(col, vec3(0.09, 0.33, 0.11), smoothstep(0.58, 0.7, tone));
  float cloud = cloudAt(vWorld.xz + uSunDir.xz * 30.0);
  col *= uSkyColor * 0.9 + uSunColor * (0.2 + 0.14 * smoothstep(0.5, 0.9, tone)) * mix(0.45, 1.0, clamp((cloud - 0.5) * 2.0, 0.0, 1.0));
  outC = vec4(applyFog(col, vWorld), 1.0);
}
`;

const woodVertex = /* glsl */ `
${COMMON}
in vec2 aBark;
out vec3 vWorld;
out vec3 vNormal;
out vec2 vBark;
void main() {
  mat4 m = modelMatrix * instanceMatrix;
  vec4 w = m * vec4(position, 1.0);
  vWorld = w.xyz;
  vNormal = normalize(mat3(m) * normal);
  vBark = aBark;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`;

const woodFragment = /* glsl */ `
${COMMON}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec2 vBark;
void main() {
  vec3 N = normalize(vNormal);
  // bark in long strokes, greyer in places, green with moss where it faces up or away from the sun
  float stroke = texture(tNoise, vec2(vBark.x * 3.0, vBark.y * 0.16)).r;
  float patchy = texture(tNoise, vec2(vBark.x * 0.9 + 0.3, vBark.y * 0.07)).g;
  vec3 col = mix(vec3(0.085, 0.062, 0.05), vec3(0.2, 0.16, 0.125), smoothstep(0.3, 0.7, stroke));
  col = mix(col, vec3(0.24, 0.23, 0.2), smoothstep(0.55, 0.75, patchy) * 0.6);
  float moss = smoothstep(0.45, 0.7, patchy * 0.6 + N.y * 0.5 - dot(N, uSunDir) * 0.2 + 0.25) * 0.75;
  col = mix(col, vec3(0.06, 0.13, 0.035), moss);
  // it stands in its own shade: light reaches it in spots
  float spot = smoothstep(0.56, 0.7, texture(tNoise, vec2(vBark.x * 1.7, vBark.y * 0.21) + uTime * vec2(0.011, 0.007)).b);
  float sun = smoothstep(-0.15, 0.6, dot(N, uSunDir)) * mix(0.1, 1.0, spot);
  vec3 c = col * (uSunColor * sun * 0.62 + mix(uBounceColor * 2.0, uSkyColor, N.y * 0.5 + 0.5) * 0.85);
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;

/** Where the trees stand: [x, z, kind, scale, turn]. Kind 0 is the great tree. */
function plant() {
  const random = mulberry(20);
  const trees = [[TREE[0], TREE[1], 0, 1, 2.4]];
  const clear = (x, z, gap) => {
    if (tarnDistance(x, z) < 1.75 || pathDistance(x, z) < 6 || inGarden(x, z)) return false;
    for (const t of trees) if (Math.hypot(t[0] - x, t[1] - z) < gap * (t[2] === 0 ? 5 : 1)) return false;
    return true;
  };
  /**
   * A copse: n trees packed round (u, v) so their crowns run together into one mass, longer one way than the other,
   * big trees in its middle and small ones at its edge.
   */
  const copse = (u, v, n, spread) => {
    const turn = random() * TAU;
    const long = 1.3 + 0.9 * random();
    let tries = 0;
    for (let i = 0; i < n && tries < n * 40; tries++) {
      const a = random() * TAU;
      const d = random() ** 0.7;
      const du = Math.cos(a) * d * spread * long;
      const dv = (Math.sin(a) * d * spread) / long;
      const [x, z] = fromValley(u + du * Math.cos(turn) - dv * Math.sin(turn), v + du * Math.sin(turn) + dv * Math.cos(turn));
      if (Math.hypot(x, z) > 596 || !clear(x, z, 4.4)) continue;
      trees.push([x, z, 1 + Math.floor(random() * 3), (1.95 - 1.05 * d) * (0.75 + 0.5 * random()), random() * TAU]);
      i++;
    }
  };
  /** One big tree standing alone. */
  const lone = (u, v) => {
    const [x, z] = fromValley(u, v);
    if (clear(x, z, 9)) trees.push([x, z, 1 + Math.floor(random() * 3), 1.9 + 0.5 * random(), random() * TAU]);
  };
  // The hill itself is kept clear, so the records' flowers are seen from its crown: only the great tree stands on
  // it. Woods lie in a broken ring round its foot, thickest behind the start; ahead, where the country opens to
  // the mountains, there are few, and those stand to either side as a painter frames a view.
  const edge = gardenRadius();
  for (let k = 0; k < 26; k++) {
    // (a bearing: 0 is straight ahead of the start)
    let a = (random() - 0.5) * TAU;
    if (Math.abs(a) < 0.55 && random() < 0.8) a += Math.sign(a || 1) * 0.6;
    const d = edge + 36 + 250 * random() ** 1.5;
    copse(Math.cos(a) * d, Math.sin(a) * d, 5 + Math.floor(random() * 10), 11 + 15 * random());
  }
  copse(edge + 70, -(edge * 0.6 + 46), 9, 16);
  copse(edge + 96, edge * 0.6 + 52, 8, 15);
  // a few big ones alone on the downs
  for (let k = 0; k < 7; k++) {
    const a = random() * TAU;
    const d = edge + 24 + 110 * random();
    lone(Math.cos(a) * d, Math.sin(a) * d);
  }
  // and the country's rim, wooded, so the walk ends among trees
  for (let k = 0; k < 12; k++) {
    const a = random() * TAU;
    copse((455 + 95 * random()) * Math.cos(a), (455 + 95 * random()) * Math.sin(a), 5 + Math.floor(random() * 9), 16 + 14 * random());
  }
  return trees;
}

/** @param options.cards 0 to 1, the share of leaf sprays drawn */
export function createTrees(field, { cards = 1 } = {}) {
  const kinds = [
    growTree(11, { trunk: 3.6, girth: 0.92, crown: 11.5, rise: 12.5, limbs: 6, clump: 2.7, fill: 26, cards: Math.round(92 * cards), spray: 1.05 }),
    growTree(23, { trunk: 2.6, girth: 0.42, crown: 5.6, rise: 8.2, limbs: 4, clump: 1.9, fill: 9, cards: Math.round(48 * cards), spray: 1.15 }),
    growTree(37, { trunk: 3.3, girth: 0.36, crown: 4.6, rise: 9.6, limbs: 4, clump: 1.7, fill: 10, cards: Math.round(48 * cards), spray: 1.1 }),
    growTree(51, { trunk: 2.2, girth: 0.5, crown: 6.6, rise: 7.2, limbs: 5, clump: 2.1, fill: 8, cards: Math.round(48 * cards), spray: 1.15 }),
  ];
  const trees = plant();
  // past the walk: the same trees, drawn with a third of the sprays, each bigger
  for (let k = 1; k <= 3; k++) {
    const seed = [0, 23, 37, 51][k];
    const o = [null, { trunk: 2.6, girth: 0.42, crown: 5.6, rise: 8.2, limbs: 4, clump: 1.9, fill: 9 }, { trunk: 3.3, girth: 0.36, crown: 4.6, rise: 9.6, limbs: 4, clump: 1.7, fill: 10 }, { trunk: 2.2, girth: 0.5, crown: 6.6, rise: 7.2, limbs: 5, clump: 2.1, fill: 8 }][k];
    kinds.push(growTree(seed, { ...o, cards: 22, spray: 1.85 }));
  }
  {
    const random = mulberry(808);
    const [dx, dz] = [0.6, 0.8];
    let made = 0;
    for (let g = 0; g < 34 && made < 330; g++) {
      // woods and spinneys out on the plain and up the valley's sides
      const u = -700 + 1900 * random();
      const v = (random() - 0.5) * 2200;
      const n = 4 + Math.floor(random() * 13);
      const spread = 7 + n * 1.7;
      for (let i = 0; i < n; i++) {
        const a = random() * TAU;
        const d = spread * random() ** 0.7;
        const x = (u + Math.cos(a) * d) * dx - (v + Math.sin(a) * d) * dz;
        const z = (u + Math.cos(a) * d) * dz + (v + Math.sin(a) * d) * dx;
        if (Math.hypot(x, z) < 640 || Math.hypot(x, z) > 1250 || riverDistance(x, z) < 26) continue;
        trees.push([x, z, 4 + Math.floor(random() * 3), 0.9 + 0.9 * random() ** 1.5, random() * TAU]);
        made++;
      }
    }
  }
  // bushes: the same leaves in low mounds, among the stones, beside the path and round the tarn
  const BUSH = kinds.length;
  kinds.push(
    growTree(71, { trunk: 0.25, girth: 0.07, crown: 1.5, rise: 1.9, limbs: 3, clump: 0.8, fill: 7, cards: Math.round(30 * cards), spray: 0.62 }),
    growTree(83, { trunk: 0.2, girth: 0.06, crown: 2.1, rise: 1.5, limbs: 4, clump: 0.85, fill: 8, cards: Math.round(30 * cards), spray: 0.62 }),
  );
  {
    const random = mulberry(313);
    const bush = (x, z, scale) => {
      if (pathDistance(x, z) < 2.4 || tarnDistance(x, z) < 0.9 || inGarden(x, z)) return;
      if (Math.hypot(x - TREE[0], z - TREE[1]) < 9) return;
      trees.push([x, z, BUSH + (random() < 0.5 ? 0 : 1), scale, random() * TAU]);
    };
    // low mounds scattered over the downs near the hill
    const spots = [];
    for (let k = 0; k < 24; k++) {
      const a = random() * TAU;
      const d = gardenRadius() + 14 + 150 * random();
      spots.push([Math.cos(a) * d, Math.sin(a) * d]);
    }
    for (const [u, v] of spots) {
      const n = 1 + Math.floor(random() * 4);
      for (let i = 0; i < n; i++) {
        const a = random() * TAU;
        const d = 1 + 5 * random();
        const [x, z] = fromValley(u + Math.cos(a) * d, v + Math.sin(a) * d);
        bush(x, z, 0.6 + 0.9 * random());
      }
    }
    // round the tarn, on its bank
    for (let tries = 0, made = 0; tries < 600 && made < 16; tries++) {
      const x = TARN.at[0] + (random() - 0.5) * TARN.rx * 3;
      const z = TARN.at[1] + (random() - 0.5) * TARN.rx * 3;
      const t = tarnDistance(x, z);
      if (t < 0.93 || t > 1.16) continue;
      bush(x, z, 0.6 + 0.8 * random());
      made++;
    }
    // at the feet of the copses, where they hide the trunks
    for (const t of trees.slice()) {
      if (t[2] < 1 || t[2] > 3 || random() > 0.5) continue;
      const a = random() * TAU;
      const d = 2.5 + 3.5 * random();
      bush(t[0] + Math.cos(a) * d, t[1] + Math.sin(a) * d, 0.6 + 0.8 * random());
    }
    // thickets: bushes run together into masses, longer one way than the other, with open grass between
    const thickets = [
      [250, -60, 14, 15], [330, 112, 12, 13], [382, -150, 16, 17], [472, 44, 12, 13], [204, 150, 10, 11], [524, -92, 14, 15], [162, -152, 10, 11],
      [432, 222, 12, 13], [292, -232, 10, 11], [562, 152, 12, 13], [352, 12, 8, 8], [452, -232, 10, 11], [232, 62, 8, 8], [584, -12, 10, 10],
      [140, 212, 9, 10], [402, 300, 10, 11], [498, -300, 10, 11], [310, -20, 6, 7],
    ];
    for (const [u, v, n, spread] of thickets) {
      const turn = random() * TAU;
      const long = 1.4 + random();
      for (let i = 0; i < n; i++) {
        const a = random() * TAU;
        const d = Math.sqrt(random());
        const du = Math.cos(a) * d * spread * long;
        const dv = (Math.sin(a) * d * spread) / long;
        const [x, z] = fromValley(u + du * Math.cos(turn) - dv * Math.sin(turn), v + du * Math.sin(turn) + dv * Math.cos(turn));
        bush(x, z, (1.5 - 0.7 * d) * (0.6 + 0.5 * random()));
      }
    }
    // and rows of them wandering across the meadow, old field edges, with gaps
    for (let k = 0; k < 10; k++) {
      const u0 = 170 + 400 * random();
      const v0 = -270 + 540 * random();
      const way = Math.PI / 2 + (random() - 0.5) * 1.3;
      const length = 70 + 90 * random();
      for (let s = -length / 2; s < length / 2; s += 2.6 + 2.6 * random()) {
        if (Math.sin(s / 15 + k * 2.1) > 0.7) continue;
        const bend = 7 * Math.sin(s / 26 + k);
        const [x, z] = fromValley(u0 + Math.cos(way) * s - Math.sin(way) * bend, v0 + Math.sin(way) * s + Math.cos(way) * bend);
        bush(x, z, 0.55 + 0.6 * random());
      }
    }
    // and a few out on the open meadow, in twos and threes
    for (let k = 0; k < 24; k++) {
      const a = random() * TAU;
      const r = 90 + 500 * Math.sqrt(random());
      const n = 1 + Math.floor(random() * 3.5);
      for (let i = 0; i < n; i++) bush(Math.cos(a) * r + (random() - 0.5) * 9, Math.sin(a) * r + (random() - 0.5) * 9, 0.5 + 0.9 * random());
    }
  }
  const group = new THREE.Group();
  const leafMaterial = new THREE.ShaderMaterial({
    glslVersion: THREE.GLSL3,
    vertexShader: leafVertex,
    fragmentShader: leafFragment,
    uniforms: { ...field.uniforms, tLeaf: { value: leafTexture() } },
    side: THREE.DoubleSide,
    alphaToCoverage: true,
  });
  const woodMaterial = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: woodVertex, fragmentShader: woodFragment, uniforms: field.uniforms });
  const coreMaterial = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: leafVertex, fragmentShader: coreFragment, uniforms: { ...field.uniforms, tLeaf: leafMaterial.uniforms.tLeaf }, side: THREE.DoubleSide });
  const matrix = new THREE.Matrix4();
  const q = new THREE.Quaternion();
  const up = new V3(0, 1, 0);
  kinds.forEach((kind, k) => {
    const mine = trees.filter((t) => t[2] === k);
    if (!mine.length) return;
    const leaves = new THREE.InstancedMesh(kind.leaves, leafMaterial, mine.length);
    const wood = new THREE.InstancedMesh(kind.wood, woodMaterial, mine.length);
    const core = new THREE.InstancedMesh(kind.core, coreMaterial, mine.length);
    mine.forEach(([x, z, , scale, turn], i) => {
      q.setFromAxisAngle(up, turn);
      matrix.compose(new V3(x, heightAt(x, z), z), q, new V3(scale, scale * (0.94 + 0.12 * ((i * 7) % 5) * 0.25), scale));
      leaves.setMatrixAt(i, matrix);
      wood.setMatrixAt(i, matrix);
      core.setMatrixAt(i, matrix);
    });
    leaves.frustumCulled = wood.frustumCulled = core.frustumCulled = false;
    group.add(wood, core, leaves);
  });

  // ---- their shade on the ground ----
  const RES = 1024;
  const data = new Uint8Array(RES * RES);
  const shade = new THREE.DataTexture(data, RES, RES, THREE.RedFormat, THREE.UnsignedByteType);
  shade.minFilter = shade.magFilter = THREE.LinearFilter;
  shade.unpackAlignment = 1;
  const extentMin = field.uniforms.uExtentMin.value;
  const extent = 1 / field.uniforms.uExtentInv.value;
  field.uniforms.tShade.value = shade;

  return {
    group,
    trees,
    /** Lays every crown's shadow on the ground for this sun. Call when the hour changes. */
    cast(sunDir) {
      data.fill(0);
      const flat = Math.hypot(sunDir.x, sunDir.z) || 1e-3;
      const sx = sunDir.x / flat;
      const sz = sunDir.z / flat;
      const slope = flat / Math.max(sunDir.y, 0.12); // metres of shadow to a metre of height
      const texel = extent / RES;
      for (const [x, z, k, scale] of trees) {
        if (k > 3 && k < BUSH) continue;
        const kind = kinds[k];
        const r = kind.crown * scale * 1.02;
        const long = Math.hypot(r, kind.height * 0.28 * scale * slope);
        const cx = x - sx * kind.crownY * scale * slope;
        const cz = z - sz * kind.crownY * scale * slope;
        const reach = long + 2;
        const i0 = Math.max(0, Math.floor((cx - reach - extentMin.x) / texel));
        const i1 = Math.min(RES - 1, Math.ceil((cx + reach - extentMin.x) / texel));
        const j0 = Math.max(0, Math.floor((cz - reach - extentMin.y) / texel));
        const j1 = Math.min(RES - 1, Math.ceil((cz + reach - extentMin.y) / texel));
        for (let j = j0; j <= j1; j++) {
          for (let i = i0; i <= i1; i++) {
            const dx = extentMin.x + (i + 0.5) * texel - cx;
            const dz = extentMin.y + (j + 0.5) * texel - cz;
            // stretched along the sun's line
            const al = (dx * sx + dz * sz) / long;
            const ac = (-dx * sz + dz * sx) / r;
            const d = Math.hypot(al, ac);
            if (d >= 1.08) continue;
            // a ragged edge: the outline of a crown is clumps
            const ang = Math.atan2(ac, al);
            const edge = 0.9 + 0.09 * Math.sin(ang * 5 + x) + 0.06 * Math.sin(ang * 9 + z);
            const v = 255 * (1 - THREE.MathUtils.smoothstep(d, edge - 0.16, edge + 0.04));
            const at = i + j * RES;
            if (v > data[at]) data[at] = v;
          }
        }
      }
      shade.needsUpdate = true;
    },
  };
}
