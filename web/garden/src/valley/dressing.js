// What a painter puts in a hillside so it is a place: grey stones lying in the grass in loose groups, and the
// wind made visible, as seed fluff and petals carried past on the gusts.
import * as THREE from 'three';
import { COMMON } from '../lib/index.js';
import { fromValley, gardenRadius, heightAt, inGarden, pathDistance, tarnDistance } from './world.js';

const TAU = Math.PI * 2;

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

/** Smooth noise over space, 0 to 1. */
function noise3(x, y, z, seed) {
  const h = (i, j, k) => {
    let n = Math.imul(i, 374761393) ^ Math.imul(j, 668265263) ^ Math.imul(k, 2147483647) ^ Math.imul(seed, 1274126177);
    n = Math.imul(n ^ (n >>> 13), 1274126177);
    return ((n ^ (n >>> 16)) >>> 0) / 4294967296;
  };
  const ix = Math.floor(x);
  const iy = Math.floor(y);
  const iz = Math.floor(z);
  const s = (t) => t * t * (3 - 2 * t);
  const fx = s(x - ix);
  const fy = s(y - iy);
  const fz = s(z - iz);
  const l = (a, b, t) => a + (b - a) * t;
  return l(
    l(l(h(ix, iy, iz), h(ix + 1, iy, iz), fx), l(h(ix, iy + 1, iz), h(ix + 1, iy + 1, iz), fx), fy),
    l(l(h(ix, iy, iz + 1), h(ix + 1, iy, iz + 1), fx), l(h(ix, iy + 1, iz + 1), h(ix + 1, iy + 1, iz + 1), fx), fy),
    fz,
  );
}

/** A stone about a metre across: a lumpy ball with a few flat faces broken off it, lying on its widest side. */
function stoneGeometry(seed) {
  const random = mulberry(seed);
  const cuts = [];
  for (let i = 0; i < 3; i++) {
    const a = random() * TAU;
    const up = -0.2 + 1.1 * random();
    const ring = Math.sqrt(Math.max(0, 1 - up * up));
    cuts.push([Math.cos(a) * ring, up, Math.sin(a) * ring, 0.62 + 0.25 * random()]);
  }
  const shape = (x, y, z) => {
    const r = 0.78 + 0.34 * noise3(x * 1.4 + 9, y * 1.4, z * 1.4, seed) + 0.1 * noise3(x * 4.1, y * 4.1 + 3, z * 4.1, seed + 7);
    let px = x * r;
    let py = y * r * 0.72;
    let pz = z * r;
    for (const [nx, ny, nz, d] of cuts) {
      const over = px * nx + py * ny + pz * nz - d;
      if (over > 0) {
        px -= nx * over;
        py -= ny * over;
        pz -= nz * over;
      }
    }
    return [px, Math.max(py, -0.3), pz];
  };
  const base = new THREE.IcosahedronGeometry(1, 3).attributes.position;
  const position = new Float32Array(base.count * 3);
  const normal = new Float32Array(base.count * 3);
  const v = new THREE.Vector3();
  const t1 = new THREE.Vector3();
  const t2 = new THREE.Vector3();
  for (let i = 0; i < base.count; i++) {
    v.fromBufferAttribute(base, i).normalize();
    const p = shape(v.x, v.y, v.z);
    position.set(p, i * 3);
    // its slope there, from two points close by on the ball
    t1.set(v.y, -v.x, 0);
    if (t1.lengthSq() < 1e-4) t1.set(1, 0, 0);
    t1.normalize();
    t2.crossVectors(v, t1);
    const e = 0.045;
    const a = shape(...v.clone().addScaledVector(t1, e).normalize().toArray());
    const b = shape(...v.clone().addScaledVector(t2, e).normalize().toArray());
    t1.set(a[0] - p[0], a[1] - p[1], a[2] - p[2]);
    t2.set(b[0] - p[0], b[1] - p[1], b[2] - p[2]);
    t1.cross(t2).normalize();
    if (t1.dot(v) < 0) t1.negate();
    normal.set([t1.x, t1.y, t1.z], i * 3);
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.BufferAttribute(position, 3));
  geometry.setAttribute('normal', new THREE.BufferAttribute(normal, 3));
  return geometry;
}

const stoneVertex = /* glsl */ `
${COMMON}
out vec3 vWorld;
out vec3 vNormal;
out vec3 vObj;
void main() {
  mat4 m = modelMatrix * instanceMatrix;
  vec4 w = m * vec4(position, 1.0);
  vWorld = w.xyz;
  vNormal = normalize(mat3(m) * normal);
  vObj = position * length(m[0].xyz) + m[3].xyz * 0.37;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`;

const stoneFragment = /* glsl */ `
${COMMON}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec3 vObj;
void main() {
  vec3 N = normalize(vNormal);
  float n1 = texture(tNoise, vObj.xy * 0.31 + vObj.z * 0.17).r;
  float n2 = texture(tNoise, vObj.zx * 1.1 + vObj.y * 0.4).g;
  // light and shade as two paints with a narrow turn; under a tree or a cloud it is all in shade
  float open = cloudAt(vWorld.xz);
  float lit = smoothstep(0.02, 0.2, dot(N, uSunDir) + (n1 - 0.5) * 0.28) * smoothstep(0.42, 0.78, open);
  vec3 light = mix(vec3(0.36, 0.35, 0.33), vec3(0.54, 0.51, 0.45), n2 * 0.6 + n1 * 0.4);
  vec3 shade = mix(vec3(0.27, 0.3, 0.3), vec3(0.36, 0.38, 0.37), n2);
  // lichen where it looks at the sky, moss low down
  float lichen = smoothstep(0.52, 0.66, N.y * 0.45 + n1 * 0.7);
  light = mix(light, vec3(0.42, 0.47, 0.2), lichen * 0.75);
  shade = mix(shade, vec3(0.09, 0.19, 0.12), lichen * 0.75);
  vec3 col = mix(shade * (uSkyColor * 1.25 + dot(uSkyColor, vec3(0.08))), light * (uSunColor * 0.36 + uSkyColor * 0.4), lit);
  // it stands in grass: green comes up it, and its foot is dark
  float foot = 1.0 - smoothstep(0.0, 0.45, vWorld.y - groundAt(vWorld.xz));
  col = mix(col, vec3(0.03, 0.1, 0.035) * (uSkyColor + uSunColor * 0.25 * open), foot * 0.7);
  outC = vec4(applyFog(col, vWorld), 1.0);
}
`;

/** Stones about the walk, in loose groups. Returns the mesh group and the ones too big to walk through. */
export function createStones(field) {
  const random = mulberry(424);
  // loose groups on the downs round the hill, none among the records' flowers
  const groups = [];
  for (let k = 0; k < 28; k++) {
    const a = random() * TAU;
    const d = gardenRadius() + 12 + 230 * random() ** 1.3;
    groups.push([Math.cos(a) * d, Math.sin(a) * d]);
  }
  const stones = [];
  for (const [u, v] of groups) {
    const n = 3 + Math.floor(random() * 7);
    const spread = 4 + random() * 7;
    for (let i = 0; i < n; i++) {
      const a = random() * TAU;
      const d = spread * Math.sqrt(random());
      const [x, z] = fromValley(u + Math.cos(a) * d, v + Math.sin(a) * d);
      if (pathDistance(x, z) < 1.7 || tarnDistance(x, z) < 1.12 || inGarden(x, z)) continue;
      // one or two big ones to a group, the rest small
      const size = i < 2 ? 0.7 + 0.9 * random() : 0.2 + 0.45 * random();
      stones.push([x, z, size, random() * TAU, Math.floor(random() * 3), 0.7 + 0.5 * random(), (random() - 0.5) * 0.3]);
    }
  }
  const group = new THREE.Group();
  const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: stoneVertex, fragmentShader: stoneFragment, uniforms: field.uniforms });
  const matrix = new THREE.Matrix4();
  const q = new THREE.Quaternion();
  const tilt = new THREE.Quaternion();
  for (let kind = 0; kind < 3; kind++) {
    const mine = stones.filter((s) => s[4] === kind);
    if (!mine.length) continue;
    const mesh = new THREE.InstancedMesh(stoneGeometry(11 + kind * 17), material, mine.length);
    mine.forEach(([x, z, size, turn, , tall, lean], i) => {
      q.setFromAxisAngle(new THREE.Vector3(0, 1, 0), turn);
      tilt.setFromAxisAngle(new THREE.Vector3(1, 0, 0), lean);
      q.multiply(tilt);
      // sunk a little, as a stone that has lain there is
      matrix.compose(new THREE.Vector3(x, heightAt(x, z) + size * 0.08, z), q, new THREE.Vector3(size, size * tall, size));
      mesh.setMatrixAt(i, matrix);
    });
    mesh.frustumCulled = false;
    group.add(mesh);
  }
  return { group, solid: stones.filter((s) => s[2] > 0.6).map(([x, z, size]) => [x, z, size * 0.82]) };
}

const fluffVertex = /* glsl */ `
${COMMON}
in vec4 aSeed;
uniform vec3 uBox;
out vec2 vQ;
out vec3 vWorld;
out vec2 vLook;  // how much of it shows, petal or fluff
void main() {
  float petal = step(0.72, aSeed.w);
  // carried on the wind, each at its own pace, in a box that goes with you
  vec2 gone = uWind.xy * uWind.z * uTime * (0.4 + 0.5 * fract(aSeed.w * 7.0));
  vec2 xz = aSeed.xz * uBox.xz + gone;
  xz = mod(xz - uCamPos.xz + uBox.xz * 0.5, uBox.xz) - uBox.xz * 0.5 + uCamPos.xz;
  float gust = gustAt(xz);
  xz += vec2(sin(uTime * 0.9 + aSeed.z * 30.0), cos(uTime * 0.7 + aSeed.x * 30.0)) * 0.45;
  float y = groundAt(xz) + 0.35 + aSeed.y * aSeed.y * uBox.y + sin(uTime * (0.5 + aSeed.w) + aSeed.x * 40.0) * 0.4 + gust * 0.7;
  vec3 at = vec3(xz.x, y, xz.y);
  float dist = distance(at, uCamPos);
  // never smaller than a pixel or so, or it flickers
  float size = max(mix(0.011, 0.019, fract(aSeed.w * 13.0)), dist * 0.001);
  vec3 cr = vec3(viewMatrix[0][0], viewMatrix[1][0], viewMatrix[2][0]);
  vec3 cu = vec3(viewMatrix[0][1], viewMatrix[1][1], viewMatrix[2][1]);
  // a petal turns over as it falls
  float turn = uTime * (2.0 + 3.0 * aSeed.x) + aSeed.z * 20.0;
  vec2 q = position.xy;
  if (petal > 0.5) q = vec2(q.x * cos(turn) - q.y * sin(turn), (q.x * sin(turn) + q.y * cos(turn)) * (0.35 + 0.65 * abs(sin(turn * 0.7))));
  vec3 world = at + (cr * q.x + cu * q.y) * size;
  vQ = position.xy;
  vWorld = world;
  // they come with the gusts, and are gone in the lulls
  vLook = vec2((1.0 - smoothstep(13.0, 19.0, dist)) * smoothstep(1.2, 3.5, dist) * (0.25 + 0.75 * smoothstep(0.3, 0.8, gust)), petal);
  gl_Position = projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`;

const fluffFragment = /* glsl */ `
${COMMON}
layout(location = 0) out vec4 outC;
in vec2 vQ;
in vec3 vWorld;
in vec2 vLook;
void main() {
  float r = length(vQ);
  float a = (vLook.y > 0.5 ? 1.0 - smoothstep(0.75, 1.0, r) : (1.0 - smoothstep(0.15, 1.0, r)) * 0.85) * vLook.x;
  if (a < 0.01) discard;
  vec3 col = mix(vec3(0.95, 0.94, 0.86), vec3(0.96, 0.66, 0.74), vLook.y);
  col *= uSunColor * 0.33 * cloudAt(vWorld.xz) + uSkyColor * 0.55;
  outC = vec4(applyFog(col, vWorld), a);
}
`;

/** Seed fluff and petals in the air round the camera. @param count how many */
export function createFluff(field, count = 320) {
  const random = mulberry(99);
  const seeds = new Float32Array(count * 4);
  for (let i = 0; i < seeds.length; i++) seeds[i] = random();
  const geometry = new THREE.InstancedBufferGeometry();
  geometry.setAttribute('position', new THREE.Float32BufferAttribute([-1, -1, 0, 1, -1, 0, 1, 1, 0, -1, 1, 0], 3));
  geometry.setIndex([0, 1, 2, 0, 2, 3]);
  geometry.setAttribute('aSeed', new THREE.InstancedBufferAttribute(seeds, 4));
  geometry.instanceCount = count;
  const material = new THREE.ShaderMaterial({
    glslVersion: THREE.GLSL3,
    vertexShader: fluffVertex,
    fragmentShader: fluffFragment,
    uniforms: { ...field.uniforms, uBox: { value: new THREE.Vector3(40, 6, 40) } },
    transparent: true,
    depthWrite: false,
  });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.frustumCulled = false;
  mesh.renderOrder = 6;
  return mesh;
}
