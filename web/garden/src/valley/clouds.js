// The clouds: summer cumulus painted the way a background artist would, as a few big simple masses with bright
// tops, blue shadow sides and flat darker bases. Each cloud is a heap of spheres whose surface is pushed about by
// billowing noise; a ray marcher draws them into a cube map round the valley, one strip a frame, so they can cost
// what they like and the sky itself is a single lookup.
import * as THREE from 'three';

const MAX_PUFFS = 240;
const MAX_CLOUDS = 14;
const PER_CLOUD = 44;

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

/** Billows to push a cloud's surface with: tiling 3D noise, rounded lumps at three sizes and a smooth field. */
function billowTexture(size = 64) {
  const random = mulberry(90210);
  const grid = (f) => {
    const p = new Float32Array(f * f * f * 3);
    for (let i = 0; i < p.length; i++) p[i] = random();
    return p;
  };
  // distance to the nearest of one point a cell, wrapped: 1 at a point, falling away, so lumps are round
  const lumps = (pts, f, x, y, z) => {
    const px = x * f;
    const py = y * f;
    const pz = z * f;
    const ix = Math.floor(px);
    const iy = Math.floor(py);
    const iz = Math.floor(pz);
    let best = 9;
    for (let dz = -1; dz <= 1; dz++) {
      const cz = (iz + dz + f) % f;
      for (let dy = -1; dy <= 1; dy++) {
        const cy = (iy + dy + f) % f;
        for (let dx = -1; dx <= 1; dx++) {
          const cx = (ix + dx + f) % f;
          const k = (cx + (cy + cz * f) * f) * 3;
          const ex = ix + dx + pts[k] - px;
          const ey = iy + dy + pts[k + 1] - py;
          const ez = iz + dz + pts[k + 2] - pz;
          const d = ex * ex + ey * ey + ez * ez;
          if (d < best) best = d;
        }
      }
    }
    return Math.sqrt(Math.max(0, 1 - best * 1.5));
  };
  const lattice = (f) => {
    const p = new Float32Array(f * f * f);
    for (let i = 0; i < p.length; i++) p[i] = random();
    return p;
  };
  const value = (lat, f, x, y, z) => {
    const px = x * f;
    const py = y * f;
    const pz = z * f;
    const ix = Math.floor(px);
    const iy = Math.floor(py);
    const iz = Math.floor(pz);
    const s = (t) => t * t * (3 - 2 * t);
    const fx = s(px - ix);
    const fy = s(py - iy);
    const fz = s(pz - iz);
    const at = (i, j, k) => lat[((ix + i) % f) + (((iy + j) % f) + ((iz + k) % f) * f) * f];
    const x00 = at(0, 0, 0) + (at(1, 0, 0) - at(0, 0, 0)) * fx;
    const x10 = at(0, 1, 0) + (at(1, 1, 0) - at(0, 1, 0)) * fx;
    const x01 = at(0, 0, 1) + (at(1, 0, 1) - at(0, 0, 1)) * fx;
    const x11 = at(0, 1, 1) + (at(1, 1, 1) - at(0, 1, 1)) * fx;
    const y0 = x00 + (x10 - x00) * fy;
    const y1 = x01 + (x11 - x01) * fy;
    return y0 + (y1 - y0) * fz;
  };
  const w = [grid(3), grid(6), grid(12), grid(24)];
  const v = [lattice(4), lattice(8), lattice(16)];
  const data = new Uint16Array(size * size * size * 4);
  const half = THREE.DataUtils.toHalfFloat;
  let k = 0;
  for (let zi = 0; zi < size; zi++) {
    const z = (zi + 0.5) / size;
    for (let yi = 0; yi < size; yi++) {
      const y = (yi + 0.5) / size;
      for (let xi = 0; xi < size; xi++) {
        const x = (xi + 0.5) / size;
        const a = lumps(w[0], 3, x, y, z);
        const b = lumps(w[1], 6, x, y, z);
        const c = lumps(w[2], 12, x, y, z);
        const d = lumps(w[3], 24, x, y, z);
        data[k++] = half(Math.min(1, a * 0.6 + b * 0.28 + c * 0.12));
        data[k++] = half(Math.min(1, b * 0.55 + c * 0.3 + d * 0.15));
        data[k++] = half(value(v[0], 4, x, y, z) * 0.57 + value(v[1], 8, x, y, z) * 0.29 + value(v[2], 16, x, y, z) * 0.14);
        data[k++] = half(Math.min(1, c * 0.6 + d * 0.4));
      }
    }
  }
  const texture = new THREE.Data3DTexture(data, size, size, size);
  texture.format = THREE.RGBAFormat;
  texture.type = THREE.HalfFloatType;
  texture.minFilter = texture.magFilter = THREE.LinearFilter;
  texture.wrapS = texture.wrapT = texture.wrapR = THREE.RepeatWrapping;
  texture.unpackAlignment = 1;
  texture.needsUpdate = true;
  return texture;
}

/**
 * The sky's clouds, laid out by hand round the way the valley runs: a tower over the mountains ahead, two more
 * heaped clouds to either side of it, fair-weather cumulus scattered nearer, and a bank lying along the mountains'
 * feet. Metres, with the start of the walk at the origin.
 */
function layout(hero) {
  const random = mulberry(4);
  const [dx, dz] = hero;
  const at = (u, v) => [u * dx - v * dz, u * dz + v * dx];
  const clouds = [];
  /** A rounded mound of puffs, `w` across and `h` tall, its middle at (x, y, z): one head of a cloud. */
  const heap = (list, x, y, z, w, h, n) => {
    for (let i = 0; i < n; i++) {
      // points in the upper part of a ball, most of them out near its skin, so the skin is all bulges
      const a = random() * Math.PI * 2;
      const up = -0.15 + 1.15 * random();
      const out = Math.sqrt(Math.max(0, 1 - up * up)) * (0.35 + 0.65 * Math.sqrt(random()));
      const r = Math.min(w * 0.3, h * 0.62) * (0.72 + 0.5 * random());
      list.push([x + Math.cos(a) * out * (w * 0.5 - r * 0.6), y + up * (h - r * 0.7), z + Math.sin(a) * out * (w * 0.5 - r * 0.6) * 0.85, r]);
    }
  };
  /**
   * A cumulus: a flat-based cloud `width` across and `height` tall. `heads` more mounds are heaped on the first,
   * each smaller and higher, leaning one way, as a real tower builds.
   */
  const cumulus = (u, v, base, width, height, puffs, heads = 0) => {
    const [cx, cz] = at(u, v);
    const list = [];
    const each = Math.max(4, Math.floor(puffs / (heads + 1.6)));
    const first = heads ? height * 0.42 : height;
    heap(list, cx, base + first * 0.1, cz, width, first, Math.round(each * 1.6));
    const lean = random() * Math.PI * 2;
    let x = cx;
    let z = cz;
    let y = base + first * 0.55;
    let w = width;
    for (let k = 0; k < heads; k++) {
      w *= 0.66 + 0.12 * random();
      const a = lean + (random() - 0.5) * 1.6;
      x += Math.cos(a) * w * 0.2;
      z += Math.sin(a) * w * 0.2;
      const h = ((height - first * 0.55) / heads) * 1.25;
      heap(list, x, y, z, w, h, each);
      y += h * 0.72;
    }
    clouds.push({ puffs: list.slice(0, PER_CLOUD), base, scale: width * 0.07 });
  };
  // the tower, a little right of where the valley points
  cumulus(19000, 5200, 1500, 14500, 10800, 42, 3);
  cumulus(15500, -12500, 1450, 8200, 5200, 28, 2);
  cumulus(11500, 15000, 1400, 6800, 3900, 24, 2);
  // behind and to the sides, so every way you turn has one
  cumulus(-13000, 9000, 1500, 8400, 5600, 26, 2);
  cumulus(-9000, -15000, 1450, 7200, 3400, 20, 1);
  // fair-weather cumulus, nearer
  cumulus(6200, -2900, 1250, 2500, 1050, 10);
  cumulus(4300, 3900, 1200, 1800, 800, 8);
  cumulus(2600, -5200, 1220, 2100, 900, 8);
  cumulus(-1200, 2400, 1180, 1500, 660, 7);
  cumulus(1100, -1800, 1160, 1200, 540, 6);
  cumulus(-4200, -3300, 1230, 2200, 920, 8);
  cumulus(8600, 7800, 1300, 3000, 1300, 10, 1);
  cumulus(9800, 12500, 1350, 4400, 2300, 14, 1);
  cumulus(7400, -9000, 1300, 3600, 1700, 12, 1);
  // (a bank along the mountains' feet was here; the painted mountains carry their own mist now)
  if (false) {
    const list = [];
    for (let i = 0; i < 14; i++) {
      const v = -6500 + i * 1000 + (random() - 0.5) * 500;
      const u = 11800 + 900 * Math.sin(i * 0.9) + (random() - 0.5) * 500;
      const [x, z] = at(u, v);
      const r = 420 + 340 * random();
      list.push([x, 900 + 200 * random(), z, r]);
    }
    clouds.push({ puffs: list, base: 640, scale: 240 });
  }
  return clouds;
}

const vertex = /* glsl */ `
out vec2 vUv;
void main() {
  vUv = position.xy;
  gl_Position = vec4(position.xy, 0.0, 1.0);
}
`;

const fragment = /* glsl */ `
precision highp float;
precision highp sampler3D;
layout(location = 0) out vec4 outC;
in vec2 vUv;
uniform sampler3D tBillow;
uniform vec4 uPuffs[${MAX_PUFFS}];
uniform vec4 uClouds[${MAX_CLOUDS}];   // centre, radius of everything in it
uniform vec4 uCloudInfo[${MAX_CLOUDS}]; // first puff, how many, base (metres), scale of its billows
uniform int uCount;
uniform int uFace;
uniform vec3 uSunDir;
uniform vec3 uLit, uMid, uShade, uUnder, uHaze;
uniform float uTime;
uniform int uSteps;
uniform float uTexel;

// The billows, read so they have no creases where one texel meets the next.
vec4 billow(vec3 q) {
  q = q * 64.0 + 0.5;
  vec3 i = floor(q);
  vec3 f = q - i;
  f = f * f * (3.0 - 2.0 * f);
  return texture(tBillow, (i + f - 0.5) / 64.0);
}

float smin(float a, float b, float k) {
  float h = max(k - abs(a - b), 0.0) / k;
  return min(a, b) - h * h * k * 0.25;
}

// Signed distance to the clouds' surface, about: negative inside. scale comes back as the size of the billows there.
// fine: 1 for the surface as it is drawn, 0 for its big forms alone.
float map(vec3 p, float fine, out float base, out float scale) {
  float d = 1e9;
  base = 1200.0;
  scale = 300.0;
  for (int c = 0; c < ${MAX_CLOUDS}; c++) {
    if (c >= uCount) break;
    vec4 cl = uClouds[c];
    float db = length(p - cl.xyz) - cl.w;
    if (db > 0.0) {
      d = min(d, db + 150.0);
      continue;
    }
    vec4 info = uCloudInfo[c];
    int first = int(info.x);
    int n = int(info.y);
    float dc = 1e9;
    for (int i = 0; i < ${PER_CLOUD}; i++) {
      if (i >= n) break;
      vec4 s = uPuffs[first + i];
      dc = smin(dc, length(p - s.xyz) - s.w, s.w * 0.55);
    }
    // billows: big round lumps, smaller ones riding on them, and a fine edge
    float sc = info.w;
    vec3 q = p / (sc * 7.5) + vec3(0.0, -uTime * 0.004, uTime * 0.002) + float(c) * 0.37;
    vec4 n1 = billow(q);
    // heaps on heaps on heaps: the same round lumps at three sizes, which is what a cumulus is
    float lump = (0.55 - n1.r) * 2.4;
    if (fine > 0.0) lump += (0.55 - billow(q * 2.7 + 0.3).r) * 1.0 + (0.55 - billow(q * 7.3 + 0.7).r) * 0.36;
    dc += sc * lump;
    // a level base, rounded into the sides and gently uneven (in plain waves: seen edge on, a base shows every
    // crease its surface has)
    float under = info.z - p.y + sc * (0.55 * sin(p.x / (sc * 2.3) + float(c)) * sin(p.z / (sc * 1.9) + 1.7) + 0.3 * sin(p.x / (sc * 0.9) + p.z / (sc * 1.2)));
    float kk = sc * 1.3;
    float hh = max(kk - abs(dc - under), 0.0) / kk;
    dc = max(dc, under) + hh * hh * kk * 0.25;
    if (dc < d) {
      d = dc;
      base = info.z;
      scale = sc;
    }
  }
  return d;
}

// The colour of a cloud's surface at p, as it is painted: white where its heaps face the light, with a clean edge
// to the pale shade beside them, and a deeper tone down in the folds and underneath. No greys, and nothing soft.
vec3 paint(vec3 p, float base, float scale) {
  float b2, s2;
  const vec3 k1 = vec3(1.0, -1.0, -1.0);
  const vec3 k2 = vec3(-1.0, -1.0, 1.0);
  const vec3 k3 = vec3(-1.0, 1.0, -1.0);
  const vec3 k4 = vec3(1.0, 1.0, 1.0);
  // which way its big forms face, and which way the heap you are looking at does
  float e = scale * 1.2;
  vec3 Ns = normalize(k1 * map(p + k1 * e, 0.0, b2, s2) + k2 * map(p + k2 * e, 0.0, b2, s2) + k3 * map(p + k3 * e, 0.0, b2, s2) + k4 * map(p + k4 * e, 0.0, b2, s2));
  e = scale * 0.65;
  vec3 Nf = normalize(k1 * map(p + k1 * e, 1.0, b2, s2) + k2 * map(p + k2 * e, 1.0, b2, s2) + k3 * map(p + k3 * e, 1.0, b2, s2) + k4 * map(p + k4 * e, 1.0, b2, s2));
  vec3 N = normalize(Ns * 0.8 + Nf);
  // A painter keeps a cloud mostly white whatever the hour: its light comes from the sun's side, but raised, and
  // turned a little towards whoever is looking.
  vec3 L = normalize(uSunDir + vec3(0.0, 0.5, 0.0) - normalize(vec3(p.x, 0.0, p.z)) * 0.3);
  // how much cloud lies between here and that light
  float shadow = clamp(-map(p + L * scale * 2.6, 0.0, b2, s2) / (scale * 0.8), 0.0, 1.0) * 0.5;
  shadow += clamp(-map(p + L * scale * 6.5, 0.0, b2, s2) / (scale * 1.4), 0.0, 1.0) * 0.7;
  // how open it is here: 0 down in a fold between heaps, 1 on the crown of one
  float open = clamp(map(p + Nf * scale * 1.2, 1.0, b2, s2) / (scale * 1.2), 0.0, 1.0);
  float up = smoothstep(0.0, scale * 7.0, p.y - base);
  float light = (dot(N, L) * 0.5 + 0.5) * exp(-shadow * 1.3) + 0.1 * up + 0.04 * (open - 0.5) + 0.1 * N.y;
  // white where the light is, with a clean edge to it; in the shade it is modelled softly, from pale to deeper
  // The shade is one quiet tone that deepens slowly towards the base and away from the light. Its modelling comes
  // from the cloud's big forms alone: taken from every small heap, it breaks into stains.
  float broad = (dot(Ns, L) * 0.5 + 0.5) * exp(-shadow) + 0.25 * up + 0.1 * Ns.y + 0.34 * (billow(p / (scale * 16.0) + 0.3).b - 0.5);
  vec3 inShade = mix(uShade, uMid, smoothstep(0.25, 0.85, broad));
  // (a touch deeper down in the folds between heaps, with an edge to it)
  inShade *= 1.0 - 0.07 * (1.0 - smoothstep(0.1, 0.18, open));
  vec3 col = mix(inShade, uLit, smoothstep(0.55, 0.6, light));
  return mix(uUnder, col, smoothstep(-0.2, 0.5, (p.y - base) / (scale * 2.2) + light));
}

vec3 faceDir(vec2 st) {
  if (uFace == 0) return vec3(1.0, -st.y, -st.x);
  if (uFace == 1) return vec3(-1.0, -st.y, st.x);
  if (uFace == 2) return vec3(st.x, 1.0, st.y);
  if (uFace == 3) return vec3(st.x, -1.0, -st.y);
  if (uFace == 4) return vec3(st.x, -st.y, 1.0);
  return vec3(-st.x, -st.y, -1.0);
}

void main() {
  vec3 rd = normalize(faceDir(vUv));
  outC = vec4(0.0);
  if (rd.y < 0.012) return;
  vec3 ro = vec3(0.0, 70.0, 0.0);
  float t = (520.0 - ro.y) / rd.y;
  float tEnd = min((14500.0 - ro.y) / rd.y, 56000.0);
  float base, scale;
  float before = t;
  // the nearest the ray comes to cloud it misses, as an angle: that is what gives a painted edge and not a stair
  float near = 1e9;
  float tNear = t;
  bool hit = false;
  for (int i = 0; i < 260; i++) {
    if (i >= uSteps || t > tEnd) break;
    float d = map(ro + rd * t, 1.0, base, scale);
    if (d <= 0.0) { hit = true; break; }
    if (d < near * t) { near = d / t; tNear = t; }
    before = t;
    t += max(d * 0.5, 30.0);
  }
  float alpha = 0.0;
  if (hit) {
    // onto the skin itself, by halving between the last point outside and this one
    float lo = before;
    float hi = t;
    for (int k = 0; k < 9; k++) {
      float mid = 0.5 * (lo + hi);
      if (map(ro + rd * mid, 1.0, base, scale) > 0.0) lo = mid; else hi = mid;
    }
    t = hi;
    alpha = 1.0;
  } else if (near < uTexel) {
    t = tNear;
    alpha = 1.0 - near / uTexel;
  }
  if (alpha <= 0.0) return;
  vec3 p = ro + rd * t;
  map(p, 1.0, base, scale);
  vec3 col = paint(p, base, scale);
  // far clouds sink into the haze
  float haze = (1.0 - exp(-t / 46000.0)) * mix(1.0, 0.45, smoothstep(0.03, 0.35, rd.y));
  outC = vec4(mix(col, uHaze, haze) * alpha, alpha);
}
`;

/**
 * @param hero the way the valley runs, (x, z)
 * @param options.size texels along a side of each face of the cube; options.steps the marcher's
 * @param options.live false to draw the clouds once and leave them
 */
export function createClouds(renderer, hero, { size = 1024, steps = 110, live = true } = {}) {
  const clouds = layout(hero);
  const puffs = [];
  const spheres = [];
  const info = [];
  for (const cloud of clouds) {
    const first = puffs.length;
    const box = new THREE.Box3();
    for (const p of cloud.puffs) {
      puffs.push(new THREE.Vector4(...p));
      box.expandByPoint(new THREE.Vector3(p[0] - p[3], p[1] - p[3], p[2] - p[3]));
      box.expandByPoint(new THREE.Vector3(p[0] + p[3], p[1] + p[3], p[2] + p[3]));
    }
    const sphere = box.getBoundingSphere(new THREE.Sphere());
    // room for the billows to push out
    spheres.push(new THREE.Vector4(sphere.center.x, sphere.center.y, sphere.center.z, sphere.radius + cloud.scale * 4));
    info.push(new THREE.Vector4(first, cloud.puffs.length, cloud.base, cloud.scale));
  }
  while (puffs.length < MAX_PUFFS) puffs.push(new THREE.Vector4());
  while (spheres.length < MAX_CLOUDS) {
    spheres.push(new THREE.Vector4());
    info.push(new THREE.Vector4());
  }

  const target = new THREE.WebGLCubeRenderTarget(size, { type: THREE.HalfFloatType, format: THREE.RGBAFormat, depthBuffer: false, generateMipmaps: false });
  target.texture.minFilter = target.texture.magFilter = THREE.LinearFilter;
  const uniforms = {
    tBillow: { value: billowTexture() },
    uPuffs: { value: puffs.slice(0, MAX_PUFFS) },
    uClouds: { value: spheres.slice(0, MAX_CLOUDS) },
    uCloudInfo: { value: info.slice(0, MAX_CLOUDS) },
    uCount: { value: Math.min(clouds.length, MAX_CLOUDS) },
    uFace: { value: 0 },
    uSunDir: { value: new THREE.Vector3(0, 1, 0) },
    uLit: { value: new THREE.Color(1.22, 1.2, 1.14) },
    uMid: { value: new THREE.Color(0.7, 0.79, 0.88) },
    uShade: { value: new THREE.Color(0.31, 0.46, 0.63) },
    uUnder: { value: new THREE.Color(0.23, 0.33, 0.52) },
    uHaze: { value: new THREE.Color(0.5, 0.72, 0.92) },
    uTime: { value: 0 },
    uSteps: { value: steps },
    // about a texel and a half of the cube, as an angle: how near a miss still shades the edge
    uTexel: { value: 3 / size },
  };
  const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: vertex, fragmentShader: fragment, uniforms, depthTest: false, depthWrite: false });
  const triangle = new THREE.BufferGeometry();
  triangle.setAttribute('position', new THREE.Float32BufferAttribute([-1, -1, 0, 3, -1, 0, -1, 3, 0], 3));
  const mesh = new THREE.Mesh(triangle, material);
  mesh.frustumCulled = false;
  const scene = new THREE.Scene();
  scene.add(mesh);
  const camera = new THREE.Camera();

  // the five faces that can hold cloud (not the one underfoot), each in four strips
  const FACES = [0, 1, 2, 4, 5];
  const STRIPS = 8;
  let unit = 0;
  let cycleTime = 0;
  let time = 0;

  function draw(face, strip) {
    const previous = renderer.getRenderTarget();
    const h = size / STRIPS;
    target.scissorTest = strip >= 0;
    if (strip >= 0) target.scissor.set(0, strip * h, size, h);
    uniforms.uFace.value = face;
    renderer.setRenderTarget(target, face);
    renderer.render(scene, camera);
    target.scissorTest = false;
    renderer.setRenderTarget(previous);
  }

  return {
    texture: target.texture,
    uniforms,
    /** The light on them. Any of: sunDir, lit, mid, shade, under, haze (colours in linear light). */
    setLight(l) {
      if (l.sunDir) uniforms.uSunDir.value.copy(l.sunDir).normalize();
      for (const [k, u] of [['lit', 'uLit'], ['mid', 'uMid'], ['shade', 'uShade'], ['under', 'uUnder'], ['haze', 'uHaze']]) if (l[k]) uniforms[u].value.copy(l[k]);
    },
    /** Every face, now: at the start, and whenever the light changes. */
    drawAll() {
      // (they hold their shape: redrawn a strip at a time, clouds that billowed moved in jumps)
      uniforms.uTime.value = 0;
      for (const face of FACES) draw(face, -1);
      unit = 0;
    },
    /** One strip of one face a frame: the clouds billow slowly, and nobody pays for it. */
    update(dt) {
      time += dt;
      if (!live) return;
      if (unit === 0) cycleTime = time;
      uniforms.uTime.value = 0;
      draw(FACES[Math.floor(unit / STRIPS)], unit % STRIPS);
      unit = (unit + 1) % (FACES.length * STRIPS);
    },
    compile: () => renderer.compileAsync(scene, camera),
  };
}
