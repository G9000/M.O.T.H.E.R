// Wildflowers among the grass, in drifts of one kind: ox-eye daisies, buttercups, knapweed and poppies. Like the
// blades, a flower is only the number of its cell: its stem leans with the wind, lies down when trodden, loses its
// head to the blade and shrivels in a fire, all from the same maps.
import * as THREE from 'three';
import { COMMON, LEAF_LIGHT } from './glsl.js';

const RING = { cell: 0.4, tile: 12, tiles: 6, fade: [27, 35] };
const PETALS = 8;

const vertex = /* glsl */ `
${COMMON}
in vec2 aCell;
uniform float uCell;
uniform vec2 uFade;
uniform float uSalt;
uniform float uHeight;
out vec3 vWorld;
out vec3 vNormal;
out vec3 vAlbedo;
out vec3 vLight;  // cloud shade, fire light, how far up

void main() {
  float part = position.z;           // 0 stem, 1..8 a petal, 9 the middle
  float sub = floor(part / 16.0);
  part -= sub * 16.0;
  vec2 origin = modelMatrix[3].xz;
  vec2 cell = floor(origin / uCell + 0.5) + aCell;
  vec4 r = cellRand(cell, uSalt + sub * 5.0);
  vec4 q = cellRand(cell, uSalt + 53.0 + sub * 5.0);
  vec2 xz = (cell + r.xy) * uCell;
  float dist = distance(xz, uCamPos.xz);
  vec4 m = texture(tMap, extentUv(xz));
  // they grow in drifts
  float drift = smoothstep(0.42, 0.7, texture(tNoise, xz / 23.0).g);
  float chance = m.a * (0.16 + 0.84 * drift) * step(0.5, m.g);
  float live = step(q.x, chance) * (1.0 - smoothstep(uFade.x, uFade.y, dist + q.y * 3.0));
  if (live <= 0.0) {
    gl_Position = vec4(2.0, 2.0, 2.0, 1.0);
    return;
  }
  vec2 suv = stateUv(xz);
  vec4 st = texture(tState, suv);
  vec4 fr = texture(tFire, suv);
  vec2 bend = (st.rg * 255.0 - 128.0) / 127.0;
  float flat_ = min(length(bend), 1.0);
  float charred = smoothstep(0.1, 0.6, fr.g);

  // one kind to a drift, with a few strays
  float kindN = texture(tWhite, xz / (7.0 * 256.0)).r;
  if (q.z > 0.8) kindN = q.w;
  int kind = int(kindN * 3.999);
  vec3 petal = vec3(0.80, 0.80, 0.74);
  vec3 middle = vec3(0.75, 0.50, 0.03);
  float count = 8.0, pl = 0.024, pw = 0.0075, cup = 0.12, mid = 0.0095, tall = 1.0;
  if (kind == 1) { petal = vec3(0.85, 0.62, 0.02); middle = vec3(0.6, 0.45, 0.02); count = 5.0; pl = 0.012; pw = 0.011; cup = 0.6; mid = 0.004; tall = 0.85; }
  if (kind == 2) { petal = vec3(0.86, 0.34, 0.55); middle = vec3(0.7, 0.5, 0.08); count = 8.0; pl = 0.019; pw = 0.005; cup = 0.55; mid = 0.007; tall = 1.1; }
  if (kind == 3) { petal = vec3(0.93, 0.88, 0.8); middle = vec3(0.8, 0.55, 0.06); count = 4.0; pl = 0.03; pw = 0.03; cup = 0.75; mid = 0.006; tall = 1.05; }
  float size = mix(2.0, 2.7, r.z);

  float gy = groundAt(xz);
  float h = uHeight * tall * mix(0.98, 1.32, r.w) * (1.0 - 0.7 * charred) * tallAt(xz);
  float yaw = q.w * TAU;
  vec2 L = vec2(cos(yaw), sin(yaw)) * 0.18;
  float gust = gustAt(xz);
  float U = uWind.z;
  float nod = sin(r.w * 40.0 + uTime * (2.4 + 2.0 * r.z)) * (0.03 + 0.012 * U);
  L += uWind.xy * (U * mix(0.02, 0.07, gust) + nod);
  for (int i = 0; i < 6; i++) {
    vec4 p = uPushers[i];
    if (p.w <= 0.0) continue;
    vec2 d = xz - p.xz;
    float dd = length(d);
    L += d / max(dd, 1e-3) * (1.0 - smoothstep(p.w * 0.7, p.w * 1.9, dd)) * (1.0 - smoothstep(gy + h * 0.5, gy + h + 0.12, p.y - p.w)) * 0.95;
  }
  L = L * (1.0 - 0.8 * flat_) + bend * 1.42;
  float lean = length(L);
  vec2 dir = L / max(lean, 1e-4);
  lean = min(lean, 1.5);
  float theta0 = lean * mix(0.3, 0.85, flat_);
  float A = lean - theta0;
  float top = A > 1e-3 ? h * (sin(theta0 + A) - sin(theta0)) / A : h * cos(theta0);
  float sCut = clamp(st.b / max(top, 1e-3), 0.0, 1.0);
  bool headless = sCut < 0.97 || charred > 0.5;

  float s = (part < 0.5 ? position.y : 1.0) * sCut;
  float phi = theta0 + A * s;
  float along = A > 1e-3 ? h * (cos(theta0) - cos(phi)) / A : h * s * sin(theta0);
  float rise = A > 1e-3 ? h * (sin(phi) - sin(theta0)) / A : h * s * cos(theta0);
  vec3 tangent = vec3(dir.x * sin(phi), cos(phi), dir.y * sin(phi));
  vec3 at = vec3(xz.x + dir.x * along, gy + rise, xz.y + dir.y * along);
  vec3 world;
  vec3 N;
  vec3 col;
  if (part < 0.5) {
    vec3 across = normalize(cross(tangent, uCamPos - at));
    world = at + across * position.x * 0.0016;
    N = cross(across, tangent);
    col = mix(vec3(0.16, 0.4, 0.1), vec3(0.24, 0.52, 0.13), s);
  } else {
    // the head looks up and a little towards the sun
    vec3 axis = normalize(tangent * 0.6 + vec3(0.0, 0.5, 0.0) + uSunDir * 0.25);
    vec3 u = normalize(cross(axis, vec3(0.31, 0.0, 0.95)));
    vec3 v = cross(axis, u);
    if (headless) {
      world = at;
      N = axis;
      col = vec3(0.0);
    } else if (part > 8.5) {
      world = at + (u * position.x + v * position.y) * mid * size + axis * 0.002;
      N = normalize(axis + (u * position.x + v * position.y) * 0.6);
      col = middle;
    } else {
      float k = part - 1.0;
      float a = (k + r.x) / count * TAU;
      vec3 out_ = cos(a) * u + sin(a) * v;
      vec3 side = -sin(a) * u + cos(a) * v;
      float t = position.y;
      float wide = sin(3.14159 * clamp(t * 0.85 + 0.12, 0.0, 1.0));
      float droop = cup - 0.5 * t * t * (1.0 - cup);
      world = at + (out_ * cos(droop) + axis * sin(droop)) * t * pl * size + side * position.x * pw * size * wide;
      if (k >= count) world = at;
      N = normalize(axis * cos(droop) - out_ * sin(droop));
      col = petal * mix(0.8, 1.05, t);
    }
  }
  col = mix(col, vec3(0.011, 0.010, 0.009), charred);
  vWorld = world;
  vNormal = N;
  vAlbedo = col;
  vLight = vec3(cloudAt(xz), texture(tGlow, suv).g + lampAt(world), clamp(rise / (uHeight * 0.85), 0.0, 1.0));
  gl_Position = projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`;

const fragment = /* glsl */ `
${COMMON}
${LEAF_LIGHT}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec3 vAlbedo;
in vec3 vLight;
void main() {
  vec3 V = normalize(uCamPos - vWorld);
  vec3 N = normalize(vNormal + vec3(0.0, 1e-4, 0.0));
  if (dot(N, V) < 0.0) N = -N;
  // petals are matt and a little see-through: lit like a leaf, but they never dazzle
  vec3 c = min(leafLight(vAlbedo, N, V, vLight.z, 0.04, vLight.x, vLight.y) * 0.62, vec3(1.25));
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;

function flowerGeometry(subs = 2) {
  const positions = [];
  const indices = [];
  for (let sub = 0; sub < subs; sub++) {
    const z = sub * 16;
    // the stem: a strip of four rows
    let first = positions.length / 3;
    for (let j = 0; j < 4; j++) {
      positions.push(-1, j / 3, z, 1, j / 3, z);
      if (j < 3) indices.push(first + j * 2, first + j * 2 + 1, first + j * 2 + 3, first + j * 2, first + j * 2 + 3, first + j * 2 + 2);
    }
    // the petals: foot, two shoulders, two further out, the tip
    for (let k = 0; k < PETALS; k++) {
      first = positions.length / 3;
      const part = z + 1 + k;
      positions.push(0, 0, part, -1, 0.4, part, 1, 0.4, part, -1, 0.75, part, 1, 0.75, part, 0, 1, part);
      indices.push(first, first + 2, first + 1, first + 1, first + 2, first + 4, first + 1, first + 4, first + 3, first + 3, first + 4, first + 5);
    }
    // the middle: a six-sided disc
    first = positions.length / 3;
    positions.push(0, 0, z + 9);
    for (let k = 0; k < 6; k++) positions.push(Math.cos((k / 6) * Math.PI * 2), Math.sin((k / 6) * Math.PI * 2), z + 9);
    for (let k = 0; k < 6; k++) indices.push(first, first + 1 + k, first + 1 + ((k + 1) % 6));
  }
  const n = Math.round(RING.tile / RING.cell);
  const cells = new Float32Array(n * n * 2);
  for (let j = 0; j < n; j++) {
    for (let i = 0; i < n; i++) {
      cells[(i + j * n) * 2] = i;
      cells[(i + j * n) * 2 + 1] = j;
    }
  }
  const geometry = new THREE.InstancedBufferGeometry();
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
  geometry.setIndex(indices);
  geometry.setAttribute('aCell', new THREE.InstancedBufferAttribute(cells, 2));
  geometry.instanceCount = n * n;
  geometry.boundingSphere = new THREE.Sphere(new THREE.Vector3(RING.tile / 2, 0.3, RING.tile / 2), RING.tile * 0.75 + 2.5);
  return geometry;
}

export function createFlowers(shared, { height = 0.52, seed = 1 } = {}, heightAt) {
  const group = new THREE.Group();
  const geometry = flowerGeometry();
  const uniforms = {
    ...shared,
    uCell: { value: RING.cell },
    uFade: { value: new THREE.Vector2(...RING.fade) },
    uSalt: { value: 400 + seed },
    uHeight: { value: height },
  };
  const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: vertex, fragmentShader: fragment, uniforms, side: THREE.DoubleSide });
  const tiles = [];
  for (let j = 0; j < RING.tiles; j++) {
    for (let i = 0; i < RING.tiles; i++) {
      const mesh = new THREE.Mesh(geometry, material);
      mesh.matrixAutoUpdate = false;
      tiles.push({ mesh, i: i - RING.tiles / 2, j: j - RING.tiles / 2 });
      group.add(mesh);
    }
  }
  let at = [NaN, NaN];
  return {
    group,
    uniforms,
    follow(x, z) {
      const sx = Math.floor(x / RING.tile);
      const sz = Math.floor(z / RING.tile);
      if (sx === at[0] && sz === at[1]) return;
      at = [sx, sz];
      for (const tile of tiles) {
        const ox = (sx + tile.i) * RING.tile;
        const oz = (sz + tile.j) * RING.tile;
        tile.mesh.position.set(ox, heightAt(ox + RING.tile / 2, oz + RING.tile / 2), oz);
        tile.mesh.updateMatrix();
        tile.mesh.updateMatrixWorld(true);
      }
    },
  };
}
