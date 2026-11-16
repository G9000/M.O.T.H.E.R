// The grass itself: three rings of instanced blades that follow the camera. Nothing about a blade is stored: where
// it stands, how tall it is, which way it leans and what colour it is all come from the number of the world cell
// it grows in, so the same blade is there when you come back. Near the camera every blade is drawn, eight rows of
// vertices each; further out fewer and wider ones; furthest, tufts. A blade fades from one ring to the next on its
// own random distance, so nothing pops.
import * as THREE from 'three';
import { COMMON, FIRE_RAMP, LEAF_LIGHT } from './glsl.js';
import { mulberry } from './noise.js';

/** cell: metres between blades. tile × tiles: the square of ground a ring covers. fade: in from, to; out from, to. */
export const RINGS = [
  { name: 'near', cell: 0.05, tile: 3, tiles: 6, rows: 8, sub: 1, fade: [-2, -1, 2.8, 6], width: 1, tops: 0 },
  { name: 'mid', cell: 0.09, tile: 7.2, tiles: 8, rows: 4, sub: 1, fade: [2.8, 6, 13, 21.6], width: 2.0, tops: 0.05 },
  { name: 'wide', cell: 0.2, tile: 16, tiles: 8, rows: 3, sub: 1, fade: [13, 21.6, 34, 48], width: 4.6, tops: 0.22 },
  { name: 'far', cell: 0.625, tile: 40, tiles: 8, rows: 3, sub: 3, fade: [34, 48, 96, 120], width: 10, tops: 0.4 },
];

const vertex = /* glsl */ `
${COMMON}
in vec2 aCell;
uniform float uCell;
uniform vec4 uFade;
uniform float uWidth;
uniform float uSalt;
uniform float uRows;
uniform float uHeight;      // metres, a tall blade in good ground
uniform float uTops;        // 0 for a ring of single blades; more where one blade stands for many and you mostly see their tops

out vec3 vWorld;
out vec3 vNormal;
out vec3 vAcross;
out vec3 vAlbedo;
out vec4 vMisc;   // side, up, gloss, heat
out vec2 vLight;  // cloud shade, fire light

// where a seed stalk's rows sit: most of them in the head
float stalkRow(float t) {
  float k = t * (uRows - 1.0);
  if (uRows > 6.5) {
    float r[8] = float[8](0.0, 0.3, 0.55, 0.74, 0.8, 0.87, 0.94, 1.0);
    return r[int(k + 0.5)];
  }
  if (uRows > 3.5) {
    float r[4] = float[4](0.0, 0.76, 0.88, 1.0);
    return r[int(k + 0.5)];
  }
  return t;
}

void main() {
  float side = position.x;
  float t = position.y;
  float sub = position.z;
  vec2 origin = modelMatrix[3].xz;
  vec2 cell = floor(origin / uCell + 0.5) + aCell;
  vec4 r = cellRand(cell, uSalt + sub * 7.0);
  vec4 q = cellRand(cell, uSalt + 31.0 + sub * 7.0);
  vec2 xz = (cell + r.xy) * uCell;
  float dist = distance(xz, uCamPos.xz);
  float keep = (uFade.y < 0.0 ? 1.0 : smoothstep(uFade.x, uFade.y, dist)) * (1.0 - smoothstep(uFade.z, uFade.w, dist));
  float live = smoothstep(0.0, 0.18, keep - q.x * 0.82);
  vec4 m = texture(tMap, extentUv(xz));
  float growth = m.g;
  live *= smoothstep(0.0, 0.22, growth - q.y * 0.78);
  if (live <= 0.0) {
    gl_Position = vec4(2.0, 2.0, 2.0, 1.0);
    return;
  }

  vec2 suv = stateUv(xz);
  vec4 st = texture(tState, suv);
  vec4 fr = texture(tFire, suv);
  vec2 bend = (st.rg * 255.0 - 128.0) / 127.0;
  float flat_ = min(length(bend), 1.0);
  float cutH = st.b + (q.y - 0.5) * 0.024;
  // stubble is too short and stiff to tread flat: trodden, mown ground was showing as pale combed stripes
  bend *= smoothstep(0.08, 0.26, st.b);
  flat_ = min(length(bend), 1.0);
  float charred = smoothstep(0.12, 0.7, fr.g + (q.x - 0.5) * 0.3);
  float heat = fr.r;

  // neighbours share a look: a clump is one texel of white noise, its edge broken up blade by blade
  vec4 cl = texture(tWhite, (xz + (r.zw - 0.5) * 0.34) / (0.44 * 256.0));
  float dry = smoothstep(0.3, 0.7, m.r + (q.z - 0.5) * 0.5 + (cl.g - 0.5) * 0.26);
  bool stalk = uRows > 3.5 && q.w < 0.012 + 0.045 * m.r;
  float h = uHeight * mix(0.5, 1.0, cl.r) * mix(0.7, 1.22, r.z) * mix(0.7, 1.0, growth) * (1.0 + 0.7 * m.b) * mix(0.55, 1.0, live);
  if (stalk) h *= 1.5;
  // (broad patches: long, deeper grass in some, short sunlit grass in others)
  float patchT = (textureLod(tStrokes, xz.yx / 27.0 + 0.37, 0.0).r - 0.5) * 0.7 + (textureLod(tStrokes, xz / 95.0 + 0.11, 0.0).r - 0.5) * 0.9;
  h *= tallAt(xz) * mix(1.32, 0.78, clamp(0.5 + patchT * 1.5, 0.0, 1.0));
  h *= 1.0 + 0.3 * uTops;
  h *= 1.0 - 0.8 * charred;

  float gy = groundAt(xz);
  vec3 base = vec3(xz.x, gy, xz.y);

  // which way it leans, and how far, as an angle from upright
  float yaw = (cl.b * 0.45 + r.w * 0.55) * TAU;
  vec2 L = vec2(cos(yaw), sin(yaw)) * mix(0.22, 0.85, q.z) * (stalk ? 0.3 : 1.0);
  float gust = gustAt(xz);
  float U = uWind.z;
  float flutter = sin(r.w * 40.0 + uTime * (5.0 + 4.0 * r.z)) * (0.028 + 0.013 * U);
  L += uWind.xy * (U * mix(0.028, 0.105, gust) + flutter * 0.6) + vec2(-uWind.y, uWind.x) * flutter * 0.5;
  L += vec2(sin(uTime * 0.7 + r.x * 30.0), cos(uTime * 0.6 + r.y * 30.0)) * 0.03;
  for (int i = 0; i < 6; i++) {
    vec4 p = uPushers[i];
    if (p.w <= 0.0) continue;
    vec2 d = xz - p.xz;
    float dd = length(d);
    float k = (1.0 - smoothstep(p.w * 0.7, p.w * 1.9, dd)) * (1.0 - smoothstep(gy + h * 0.5, gy + h + 0.12, p.y - p.w));
    L += d / max(dd, 1e-3) * k * 0.95;
  }
  L = L * (1.0 - 0.8 * flat_) + bend * 1.42;
  L *= 1.0 + 0.9 * charred;
  float lean = length(L);
  vec2 dir = L / max(lean, 1e-4);
  lean = min(lean, 1.5);
  float theta0 = lean * mix(0.25, 0.82, flat_);
  float A = lean - theta0;

  // cut: the blade stops where the edge went through it
  float top = A > 1e-3 ? h * (sin(theta0 + A) - sin(theta0)) / A : h * cos(theta0);
  float sCut = clamp(cutH / max(top, 1e-3), 0.0, 1.0);
  bool isCut = sCut < 0.995;
  float s = (stalk ? stalkRow(t) : t) * sCut;

  float w = uWidth * 0.0115 * mix(0.72, 1.3, r.x) * (uWidth < 1.5 ? 2.1 : uWidth < 3.0 ? 1.45 : 1.0);
  if (stalk) w *= s < 0.78 ? 0.22 : 0.22 + 0.62 * sin(3.14159 * (s - 0.78) / 0.22);
  else w *= min(1.0, (1.0 - s) * 3.2) * (0.72 + 0.28 * (1.0 - s));
  w *= (1.0 - 0.45 * charred) * mix(0.5, 1.0, live);
  // stubble is the thick foot of the blade
  if (isCut) w *= 1.0 + 0.7 * (1.0 - sCut);

  float phi = theta0 + A * s;
  float along = A > 1e-3 ? h * (cos(theta0) - cos(phi)) / A : h * s * sin(theta0);
  float rise = A > 1e-3 ? h * (sin(phi) - sin(theta0)) / A : h * s * cos(theta0);
  vec3 tangent = vec3(dir.x * sin(phi), cos(phi), dir.y * sin(phi));
  vec3 across = vec3(-dir.y, 0.0, dir.x);
  // turned partly to face the camera, so a blade seen edge on doesn't vanish
  vec3 toCam = normalize(uCamPos - base);
  vec3 facing = cross(tangent, toCam);
  float fl = length(facing);
  if (fl > 1e-3) {
    facing /= fl;
    across = normalize(mix(across, facing * (dot(facing, across) < 0.0 ? -1.0 : 1.0), 0.68));
  }
  vec3 world = base + vec3(dir.x * along, rise, dir.y * along) + across * side * w * 0.5;

  vWorld = world;
  vAcross = across;
  vNormal = cross(across, tangent);

  float open = max(1.0 - smoothstep(0.1, 0.4, st.b), charred);
  float up = mix(clamp(rise / (uHeight * 0.85), 0.0, 1.0), 0.8, open * 0.8);
  // a tuft stands for many blades, and of those you mostly see the tops
  float sc = mix(s, 1.0, uTops);
  up = max(up, uTops * 1.2);
  vec3 col = grassColor(dry * (1.0 - 0.5 * m.b), sc);
  col *= mix(0.88, 1.14, cl.a) * mix(0.94, 1.06, q.z) * (1.0 + 0.3 * gust * min(U / 6.0, 1.0));
  // some clumps are yellower, some bluer: more than one kind of grass grows here
  col *= mix(vec3(1.08, 1.0, 0.82), vec3(0.9, 1.0, 1.08), cl.g);
  col *= meadowTint(xz);
  // each blade is a stroke of the brush and takes the tone of the dab it stands in: light dabs yellower, dark bluer
  float dab = (textureLod(tStrokes, xz / 6.0, 0.0).r - 0.5) * 0.5 + patchT;
  // only some blades are drawn: one in a dozen is a darker stroke, one in fifteen catches the light
  float tone = 0.58 + dab * 0.7 + (cl.a - 0.5) * 0.08 + (sc - 0.6) * 0.1 + 0.1 * gust * min(U / 6.0, 1.0);
  tone += q.z < 0.045 ? -0.26 : q.z > 0.965 ? 0.18 : 0.0;
  float rel = slopeRel(landNormal(xz));
  tone += (rel - 0.5) * 0.6;
  // the grass nearest you is the picture's foreground, and a tone deeper
  tone -= 0.2 * (1.0 - smoothstep(1.5, 9.0, dist));
  col = meadowGreen(tone) * mix(vec3(1.0), meadowTint(xz), 0.5);
  col = mix(col, vec3(0.44, 0.47, 0.17), dry * 0.45);
  if (stalk) col = mix(col, mix(vec3(0.46, 0.52, 0.18), vec3(0.4, 0.46, 0.2), step(0.72, cl.g)), smoothstep(0.75, 0.82, s));
  // far off, a few tufts carry the colour of the flowers among them
  if (uTops > 0.35 && q.w < m.a * 0.1) col = mix(col, mix(vec3(0.75, 0.72, 0.6), vec3(0.7, 0.5, 0.05), step(0.5, q.z)), smoothstep(0.4, 1.0, s));
  if (isCut) col = mix(col, col * 1.35 + vec3(0.025, 0.028, 0.008), smoothstep(0.7, 1.0, t) * 0.7);
  col = mix(col, col * 1.2 + 0.008, flat_ * 0.7);
  col = mix(col, vec3(0.011, 0.010, 0.009), charred);
  vAlbedo = col;
  float gloss = 0.015 + 0.05 * gust * min(U / 8.0, 1.0) + 0.05 * flat_;
  vMisc = vec4(side, up, gloss * (1.0 - charred), heat * (1.0 - 0.5 * charred) * (0.35 + 0.65 * s));
  // a low sun lights the tall clumps and leaves the short ones behind them in shade
  float among = mix(mix(0.45, 1.0, smoothstep(0.15, 0.75, cl.r)), 1.0, clamp(uSunDir.y * 2.5, 0.0, 1.0));
  vLight = vec2(cloudAt(xz) * mix(among, 1.0, open) * slopeSun(rel), texture(tGlow, suv).g + lampAt(world));
  gl_Position = projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`;

const fragment = /* glsl */ `
${COMMON}
${LEAF_LIGHT}
${FIRE_RAMP}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec3 vAcross;
in vec3 vAlbedo;
in vec4 vMisc;
in vec2 vLight;

void main() {
  vec3 V = normalize(uCamPos - vWorld);
  vec3 N = normalize(vNormal);
  if (dot(N, V) < 0.0) N = -N;
  // a blade is folded along its middle, not flat
  N = normalize(N + normalize(vAcross) * vMisc.x * 0.5);
  vec3 c = vAlbedo * meadowLight(vLight.x) * mix(0.84, 1.0, vMisc.y) + vAlbedo * uFireColor * vLight.y * 0.5;
  c += fireRamp(0.3 + 0.55 * vMisc.w) * vMisc.w * 1.5;
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;

function ringGeometry(ring, random) {
  const { rows, sub, tile, cell } = ring;
  const positions = [];
  const indices = [];
  for (let b = 0; b < sub; b++) {
    const first = positions.length / 3;
    for (let j = 0; j < rows; j++) {
      const t = j / (rows - 1);
      positions.push(-1, t, b, 1, t, b);
      if (j < rows - 1) {
        const a = first + j * 2;
        indices.push(a, a + 1, a + 3, a, a + 3, a + 2);
      }
    }
  }
  const n = Math.round(tile / cell);
  const cells = new Float32Array(n * n * 2);
  for (let j = 0; j < n; j++) {
    for (let i = 0; i < n; i++) {
      cells[(i + j * n) * 2] = i;
      cells[(i + j * n) * 2 + 1] = j;
    }
  }
  // shuffled, so drawing fewer instances thins the grass evenly
  for (let i = n * n - 1; i > 0; i--) {
    const k = Math.floor(random() * (i + 1));
    for (let c = 0; c < 2; c++) {
      const tmp = cells[i * 2 + c];
      cells[i * 2 + c] = cells[k * 2 + c];
      cells[k * 2 + c] = tmp;
    }
  }
  const geometry = new THREE.InstancedBufferGeometry();
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
  geometry.setIndex(indices);
  geometry.setAttribute('aCell', new THREE.InstancedBufferAttribute(cells, 2));
  geometry.instanceCount = n * n;
  geometry.boundingSphere = new THREE.Sphere(new THREE.Vector3(tile / 2, 0.3, tile / 2), tile * 0.75 + 2.5);
  return { geometry, count: n * n };
}

/**
 * @param shared the uniforms every material of the field binds (shared.js)
 * @param o.height metres, a tall blade
 * @param o.density 0 to 1, the share of blades drawn
 * @param heightAt the CPU's ground, for placing each tile so it's culled properly
 */
export function createBlades(shared, { height = 0.52, density = 1, seed = 1 } = {}, heightAt) {
  const group = new THREE.Group();
  const random = mulberry(seed * 977 + 3);
  const rings = RINGS.map((ring, index) => {
    const { geometry, count } = ringGeometry(ring, random);
    const uniforms = {
      ...shared,
      uCell: { value: ring.cell },
      uFade: { value: new THREE.Vector4(...ring.fade) },
      uWidth: { value: ring.width },
      uSalt: { value: index * 101 + seed },
      uRows: { value: ring.rows },
      uHeight: { value: height },
      uTops: { value: ring.tops },
    };
    const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: vertex, fragmentShader: fragment, uniforms, side: THREE.DoubleSide });
    const tiles = [];
    for (let j = 0; j < ring.tiles; j++) {
      for (let i = 0; i < ring.tiles; i++) {
        const mesh = new THREE.Mesh(geometry, material);
        mesh.matrixAutoUpdate = false;
        tiles.push({ mesh, i: i - ring.tiles / 2, j: j - ring.tiles / 2 });
        group.add(mesh);
      }
    }
    return { ring, geometry, material, uniforms, tiles, count, at: [NaN, NaN] };
  });

  return {
    group,
    rings,
    /** Fewer blades, evenly: 1 is all of them. */
    setDensity(d) {
      for (const r of rings) r.geometry.instanceCount = Math.max(1, Math.round(r.count * Math.min(1, Math.max(0.05, d))));
    },
    setHeight(h) {
      for (const r of rings) r.uniforms.uHeight.value = h;
    },
    /** Keeps each ring's tiles under the camera. Only does anything when the camera crosses into a new tile. */
    follow(x, z) {
      for (const r of rings) {
        const sx = Math.floor(x / r.ring.tile);
        const sz = Math.floor(z / r.ring.tile);
        if (sx === r.at[0] && sz === r.at[1]) continue;
        r.at = [sx, sz];
        for (const tile of r.tiles) {
          const ox = (sx + tile.i) * r.ring.tile;
          const oz = (sz + tile.j) * r.ring.tile;
          tile.mesh.position.set(ox, heightAt(ox + r.ring.tile / 2, oz + r.ring.tile / 2), oz);
          tile.mesh.updateMatrix();
          tile.mesh.updateMatrixWorld(true);
        }
      }
    },
    init() {
      this.setDensity(density);
    },
  };
}
