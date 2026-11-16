// Draws a frame with the field in it: your scene, then the field's flames as a volume marched through wherever its
// fire map is hot, then smoke and sparks, then bloom and a filmic tone map. Use frame.render(scene, camera) in
// place of renderer.render. The pass structure follows Patch's fire block.
import * as THREE from 'three';
import { COMMON, FIRE_RAMP } from './glsl.js';
import { mulberry, volumeNoiseTexture } from './noise.js';

const fullscreenVertex = /* glsl */ `
out vec2 vUv;
void main() {
  vUv = position.xy * 0.5 + 0.5;
  gl_Position = vec4(position.xy, 0.0, 1.0);
}
`;

const marchFragment = /* glsl */ `
${COMMON}
${FIRE_RAMP}
precision highp sampler3D;
layout(location = 0) out vec4 outC;
in vec2 vUv;
uniform sampler2D tDepth;
uniform sampler3D tVol;
uniform vec3 uCamRight;
uniform vec3 uCamUp;
uniform vec3 uCamFwd;
uniform vec2 uTan;
uniform mat4 uProj;
uniform vec4 uBox;     // x0, z0, x1, z1 of everything alight, or x1 < x0 for nothing
uniform vec2 uBoxY;    // the ground under it, lowest and highest
uniform float uFlameH; // metres, a tall flame
uniform vec4 uTorch;   // a flame held in the hand: its foot xyz and its height, 0 for none

float vol(vec3 p) { return texture(tVol, p).r; }

// density and temperature of the grass fire at p, hag metres above the ground
vec2 flame(vec3 p, float hag) {
  float U = uWind.z;
  // flames lean with the wind, more the higher they reach, and wander as they rise
  vec2 q = p.xz - uWind.xy * min(U * 0.15, 1.4) * hag * (0.45 + 0.4 * hag);
  float w1 = vol(vec3(q.x * 0.19, hag * 0.15 - uTime * 0.33, q.y * 0.19));
  float w2 = vol(vec3(q.y * 0.19 + 0.5, hag * 0.15 - uTime * 0.29, q.x * 0.19 + 0.25));
  q += (vec2(w1, w2) - 0.5) * 0.85 * hag;
  float heat = texture(tFire, stateUv(q)).r;
  if (heat < 0.012) return vec2(0.0);
  float n2 = vol(vec3(q.x * 0.47, hag * 0.24 - uTime * 0.66, q.y * 0.47));
  float n3 = vol(vec3(q.x * 1.55 + 0.3, hag * 0.72 - uTime * 2.2, q.y * 1.55));
  float tall = uFlameH * (0.2 + heat) * (0.25 + 1.5 * n2) * (1.0 + 0.05 * U);
  float u = hag / max(tall, 0.05);
  float f = heat * 1.3 * (1.0 - u) + (n3 - 0.5) * (0.45 + 0.6 * u) - 0.16 - 0.1 * u;
  if (f <= 0.0) return vec2(0.0);
  // nothing loose above the flames: scraps that small only show as speckle
  float dens = smoothstep(0.0, 0.22, f) * smoothstep(-0.04, 0.1, hag) * (1.0 - smoothstep(0.85, 1.2, u));
  float T = 0.16 + 0.72 * f + 0.3 * heat * (1.0 - u) * (1.0 - u) + 0.36 * (n3 - 0.5);
  return vec2(dens, T);
}

// the hand torch: one teardrop of flame
vec2 torch(vec3 l) {
  float y = l.y;
  if (y < -0.05 || y > 1.0) return vec2(0.0);
  float n = vol(vec3(l.x * 0.5, y * 0.3 - uTime * 0.85, l.z * 0.5));
  float n3 = vol(vec3(l.x * 1.7 + 0.4, y * 0.8 - uTime * 2.6, l.z * 1.7));
  vec2 axis = (vec2(n, n3) - 0.5) * 0.3 * y + uWind.xy * min(uWind.z * 0.03, 0.3) * y * y;
  float rr = length(l.xz - axis);
  float radius = 0.17 * (1.0 - y) * sqrt(max(1.0 - y, 0.0)) * (1.0 + 1.6 * y) + 0.02;
  float f = radius - rr + (n3 - 0.5) * 0.13;
  if (f <= 0.0) return vec2(0.0);
  return vec2(smoothstep(0.0, 0.05, f), 0.42 + 2.4 * f + 0.25 * (1.0 - y));
}

void main() {
  vec2 ndc = vUv * 2.0 - 1.0;
  vec3 rd = normalize(uCamFwd + uCamRight * ndc.x * uTan.x + uCamUp * ndc.y * uTan.y);
  float z = texture(tDepth, vUv).r * 2.0 - 1.0;
  float tMax = z >= 0.99999 ? 1e5 : (uProj[3][2] / (z + uProj[2][2])) / dot(rd, uCamFwd);
  vec3 ro = uCamPos;
  vec3 col = vec3(0.0);
  float Tr = 1.0;
  float jitter = fract(52.9829189 * fract(dot(gl_FragCoord.xy, vec2(0.06711056, 0.00583715))) + fract(uTime * 5.3) * 0.618);

  if (uTorch.w > 0.0) {
    vec3 c = uTorch.xyz + vec3(0.0, uTorch.w * 0.5, 0.0);
    float R = uTorch.w * 0.62;
    vec3 oc = ro - c;
    float b = dot(oc, rd);
    float disc = b * b - (dot(oc, oc) - R * R);
    if (disc > 0.0) {
      float sq = sqrt(disc);
      float ta = max(-b - sq, 0.0);
      float tb = min(-b + sq, tMax);
      if (tb > ta) {
        float dt = (tb - ta) / 16.0;
        float t = ta + dt * jitter;
        for (int i = 0; i < 16; i++) {
          vec2 f = torch((ro + rd * t - uTorch.xyz) / uTorch.w);
          if (f.x > 0.0) {
            float a = 1.0 - exp(-f.x * dt / uTorch.w * 26.0);
            col += Tr * a * fireRamp(f.y) * 1.2;
            Tr *= 1.0 - a;
          }
          t += dt;
        }
      }
    }
  }

  if (uBox.z > uBox.x) {
    float topH = uFlameH * 2.6;
    vec3 lo = vec3(uBox.x - 3.0, uBoxY.x - 0.3, uBox.y - 3.0);
    vec3 hi = vec3(uBox.z + 3.0, uBoxY.y + topH, uBox.w + 3.0);
    vec3 inv = 1.0 / (rd + vec3(1e-6) * sign(rd + 1e-9));
    vec3 a0 = (lo - ro) * inv;
    vec3 a1 = (hi - ro) * inv;
    vec3 tn = min(a0, a1);
    vec3 tf = max(a0, a1);
    float t0 = max(max(tn.x, tn.y), max(tn.z, 0.0));
    float t1 = min(min(min(tf.x, tf.y), tf.z), min(tMax, 280.0));
    if (t1 > t0) {
      float t = t0 + max(0.045, t0 * 0.011) * jitter;
      for (int i = 0; i < 150; i++) {
        if (t >= t1 || Tr < 0.02) break;
        vec3 p = ro + rd * t;
        float ds = max(0.045, t * 0.011);
        // 2 m cells say whether any fire is near: step straight over the ones with none
        if (texture(tGlow, stateUv(p.xz)).r < 0.004) { t += max(ds, 0.8); continue; }
        float hag = p.y - groundAt(p.xz);
        if (hag > topH) {
          if (rd.y > -1e-3) break;
          t += max(ds, (hag - topH) / -rd.y * 0.9);
          continue;
        }
        if (hag < -0.2) {
          if (rd.y < 1e-3) break;
          t += max(ds, (-0.2 - hag) / rd.y * 0.9);
          continue;
        }
        vec2 f = flame(p, hag);
        if (f.x > 0.0) {
          float a = 1.0 - exp(-f.x * min(ds, t1 - t) * 3.4);
          col += Tr * a * fireRamp(f.y) * 1.5;
          Tr *= 1.0 - a;
          t += ds * (0.88 + 0.24 * fract(jitter + float(i) * 0.61803399));
        } else {
          t += max(ds * 1.6, 0.2);
        }
      }
    }
  }
  outC = vec4(col, 1.0 - Tr);
}
`;

const puffVertex = /* glsl */ `
${COMMON}
in vec4 aPos;    // centre, size
in vec4 aLook;   // turn, opacity, warmth (lit by fire from below), seed
uniform vec3 uCamRight;
uniform vec3 uCamUp;
uniform vec3 uCamFwd;
uniform vec2 uTan;
uniform vec2 uRes;
uniform float uMinPx;   // never drawn smaller than this many pixels across: dimmer instead (0 for no limit)
out vec2 vUv;
out vec4 vLook;
out vec3 vWorld;
out vec3 vRight;
out vec3 vUp;
void main() {
  float c = cos(aLook.x), s = sin(aLook.x);
  vec2 p = vec2(position.x * c - position.y * s, position.x * s + position.y * c);
  float size = aPos.w;
  float dim = 1.0;
  if (uMinPx > 0.0) {
    float depth = max(dot(aPos.xyz - uCamPos, uCamFwd), 0.1);
    float px = size * uRes.y / (uTan.y * depth);
    float k = max(1.0, uMinPx / px);
    size *= k;
    dim = 1.0 / (k * k);
  }
  vec3 world = aPos.xyz + (uCamRight * p.x + uCamUp * p.y) * size;
  vUv = position.xy;
  vLook = vec4(aLook.x, aLook.y * dim, aLook.zw);
  vWorld = world;
  vRight = uCamRight * c + uCamUp * s;
  vUp = -uCamRight * s + uCamUp * c;
  gl_Position = aLook.y <= 0.0 ? vec4(2.0, 2.0, 2.0, 1.0) : projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`;

const smokeFragment = /* glsl */ `
${COMMON}
layout(location = 0) out vec4 outC;
uniform sampler2D tDepth;
uniform vec2 uRes;
uniform mat4 uProj;
in vec2 vUv;
in vec4 vLook;
in vec3 vWorld;
in vec3 vRight;
in vec3 vUp;
float lin(float d) { return uProj[3][2] / (d * 2.0 - 1.0 + uProj[2][2]); }
void main() {
  float r2 = dot(vUv, vUv);
  if (r2 > 1.0) discard;
  // a ragged ball of smoke: two noises eat into its edge
  vec2 nuv = vUv * 0.075 + vLook.w * vec2(7.3, 3.1);
  float n = texture(tNoiseHi, nuv).r * 0.62 + texture(tNoiseHi, nuv * 2.6 + 0.4).g * 0.38;
  float body = (1.0 - r2) * smoothstep(0.34, 0.72, n * 0.85 + (1.0 - r2) * 0.42);
  // where it meets the ground or the grass it fades instead of cutting
  float scene = lin(texture(tDepth, gl_FragCoord.xy / uRes).r);
  float mine = lin(gl_FragCoord.z);
  float soft = clamp((scene - mine) / 0.6, 0.0, 1.0) * clamp((mine - 0.4) / 3.0, 0.0, 1.0);
  float a = body * vLook.y * soft;
  if (a < 0.003) discard;
  // lit as a ball: the sun on one side, the sky on top, the fire underneath
  vec3 N = normalize(vRight * vUv.x + vUp * vUv.y + cross(vRight, vUp) * sqrt(max(1.0 - r2, 0.0)));
  float sun = clamp(dot(N, uSunDir) * 0.6 + 0.4, 0.0, 1.0);
  vec3 albedo = mix(vec3(0.2, 0.185, 0.17), vec3(0.58, 0.56, 0.54), smoothstep(0.2, 0.8, n)) * mix(0.5, 1.0, 1.0 - vLook.z);
  vec3 c = albedo * (uSunColor * sun * 0.5 + mix(uBounceColor, uSkyColor, N.y * 0.5 + 0.5) * 1.1);
  c += uFireColor * (vLook.z * 0.7 + 0.3 * texture(tGlow, stateUv(vWorld.xz)).g) * (0.05 + 0.2 * clamp(-N.y * 0.5 + 0.5, 0.0, 1.0)) * albedo * 2.6;
  c = applyFog(c, vWorld);
  outC = vec4(c * a, a);
}
`;

const sparkFragment = /* glsl */ `
layout(location = 0) out vec4 outC;
uniform sampler2D tDepth;
uniform vec2 uRes;
in vec2 vUv;
in vec4 vLook;
void main() {
  if (gl_FragCoord.z > texture(tDepth, gl_FragCoord.xy / uRes).r) discard;
  float d = length(vUv);
  float dot_ = exp(-d * d * 5.0) * (1.0 - smoothstep(0.7, 1.0, d));
  vec3 c = mix(vec3(0.9, 0.08, 0.004), vec3(3.0, 1.1, 0.16), vLook.z);
  outC = vec4(c * dot_ * vLook.y, 1.0);
}
`;

const SMOKE = 720;
const SPARKS = 640;

/** Smoke and sparks: a few hundred points of each, moved on the CPU, born where the field is burning. */
function createParticles(field, uniforms) {
  const random = mulberry(4242);
  const quad = new THREE.Float32BufferAttribute([-1, -1, 0, 1, -1, 0, -1, 1, 0, 1, 1, 0], 3);
  const make = (count, fragmentShader, blending, minPx) => {
    const geometry = new THREE.InstancedBufferGeometry();
    geometry.setAttribute('position', quad);
    geometry.setIndex([0, 1, 3, 0, 3, 2]);
    const aPos = new THREE.InstancedBufferAttribute(new Float32Array(count * 4), 4);
    const aLook = new THREE.InstancedBufferAttribute(new Float32Array(count * 4), 4);
    aPos.setUsage(THREE.DynamicDrawUsage);
    aLook.setUsage(THREE.DynamicDrawUsage);
    geometry.setAttribute('aPos', aPos);
    geometry.setAttribute('aLook', aLook);
    geometry.instanceCount = 0;
    const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: puffVertex, fragmentShader, uniforms: { ...uniforms, uMinPx: { value: minPx } }, transparent: true, depthTest: false, depthWrite: false, ...blending });
    const mesh = new THREE.Mesh(geometry, material);
    mesh.frustumCulled = false;
    return { geometry, aPos, aLook, mesh };
  };
  const smoke = make(SMOKE, smokeFragment, { blending: THREE.CustomBlending, blendSrc: THREE.OneFactor, blendDst: THREE.OneMinusSrcAlphaFactor, blendSrcAlpha: THREE.OneFactor, blendDstAlpha: THREE.OneMinusSrcAlphaFactor }, 0);
  const sparks = make(SPARKS, sparkFragment, { blending: THREE.AdditiveBlending }, 2.2);
  const scene = new THREE.Scene();
  scene.add(smoke.mesh, sparks.mesh);

  // smoke: x y z, vx vy vz, age, life, size, turn, spin, warmth, seed, thin
  const S = 14;
  const puffs = new Float32Array(SMOKE * S);
  let puffCount = 0;
  const order = new Uint16Array(SMOKE);
  const depth = new Float32Array(SMOKE);
  // sparks: x y z, vx vy vz, age, life, size, seed
  const K = 10;
  const dots = new Float32Array(SPARKS * K);
  let dotCount = 0;
  let owedSmoke = 0;
  let owedSparks = 0;

  return {
    scene,
    get count() {
      return puffCount + dotCount;
    },
    clear() {
      puffCount = dotCount = 0;
      smoke.geometry.instanceCount = sparks.geometry.instanceCount = 0;
    },
    update(dt, camera) {
      const state = field.state;
      const wind = field.uniforms.uWind.value;
      const wx = wind.x * wind.z;
      const wz = wind.y * wind.z;
      // more fire, more smoke, up to what the pools hold
      const alight = state.burning;
      owedSmoke += dt * Math.min(55, alight * 0.012 + (alight ? 4 : 0));
      owedSparks += dt * Math.min(140, alight * 0.04);
      while (owedSmoke >= 1) {
        owedSmoke -= 1;
        const at = state.sample(random);
        if (!at || puffCount >= SMOKE) continue;
        const o = puffCount++ * S;
        const strength = Math.max(at[2], at[3] * 0.4);
        puffs[o] = at[0];
        puffs[o + 1] = field.heightAt(at[0], at[1]) + 0.5 + 0.9 * random() * strength;
        puffs[o + 2] = at[1];
        puffs[o + 3] = (random() - 0.5) * 0.6;
        puffs[o + 4] = 1.4 + 2.2 * random() * (0.4 + strength);
        puffs[o + 5] = (random() - 0.5) * 0.6;
        puffs[o + 6] = 0;
        puffs[o + 7] = 7 + 7 * random();
        puffs[o + 8] = 0.6 + 0.6 * random();
        puffs[o + 9] = random() * 6.283;
        puffs[o + 10] = (random() - 0.5) * 0.5;
        puffs[o + 11] = at[2];
        puffs[o + 12] = random();
        puffs[o + 13] = at[2] > 0.12 ? 1 : 0.45;
      }
      while (owedSparks >= 1) {
        owedSparks -= 1;
        const at = state.sample(random);
        if (!at || at[2] < 0.15 || dotCount >= SPARKS) continue;
        const o = dotCount++ * K;
        dots[o] = at[0];
        dots[o + 1] = field.heightAt(at[0], at[1]) + 0.15 + 0.5 * random();
        dots[o + 2] = at[1];
        dots[o + 3] = (random() - 0.5) * 1.6;
        dots[o + 4] = 1.2 + 2.4 * random();
        dots[o + 5] = (random() - 0.5) * 1.6;
        dots[o + 6] = 0;
        dots[o + 7] = 0.5 + 1.3 * random();
        dots[o + 8] = 0.008 + 0.014 * random() * random();
        dots[o + 9] = random() * 100;
      }

      const cam = camera.getWorldPosition(new THREE.Vector3());
      const fwd = camera.getWorldDirection(new THREE.Vector3());
      for (let i = puffCount - 1; i >= 0; i--) {
        const o = i * S;
        const age = (puffs[o + 6] += dt);
        if (age >= puffs[o + 7]) {
          puffs.copyWithin(o, (puffCount - 1) * S, puffCount * S);
          puffCount--;
          continue;
        }
        // rises fast off the flames, then slows and goes with the wind
        const carried = Math.min(1, age * 0.5);
        puffs[o + 3] += (wx * 0.85 - puffs[o + 3]) * Math.min(1, dt * 0.9) * carried;
        puffs[o + 5] += (wz * 0.85 - puffs[o + 5]) * Math.min(1, dt * 0.9) * carried;
        puffs[o + 4] += (0.9 - puffs[o + 4]) * Math.min(1, dt * 0.45);
        puffs[o] += puffs[o + 3] * dt;
        puffs[o + 1] += puffs[o + 4] * dt;
        puffs[o + 2] += puffs[o + 5] * dt;
        puffs[o + 8] += dt * (0.55 + 0.07 * wind.z);
        puffs[o + 9] += puffs[o + 10] * dt;
        puffs[o + 11] *= Math.exp(-dt * 1.3);
      }
      for (let i = 0; i < puffCount; i++) {
        order[i] = i;
        depth[i] = (puffs[i * S] - cam.x) * fwd.x + (puffs[i * S + 1] - cam.y) * fwd.y + (puffs[i * S + 2] - cam.z) * fwd.z;
      }
      const sorted = Array.from(order.subarray(0, puffCount)).sort((a, b) => depth[b] - depth[a]);
      for (let n = 0; n < puffCount; n++) {
        const o = sorted[n] * S;
        const age = puffs[o + 6];
        const life = puffs[o + 7];
        const fadeIn = Math.min(1, age / 0.7);
        const fadeOut = 1 - Math.min(1, Math.max(0, (age - life * 0.45) / (life * 0.55)));
        smoke.aPos.array.set([puffs[o], puffs[o + 1], puffs[o + 2], puffs[o + 8]], n * 4);
        smoke.aLook.array.set([puffs[o + 9], 0.42 * fadeIn * fadeOut * fadeOut * puffs[o + 13], Math.min(1, puffs[o + 11] * 1.4), puffs[o + 12]], n * 4);
      }
      smoke.geometry.instanceCount = puffCount;
      smoke.aPos.needsUpdate = smoke.aLook.needsUpdate = puffCount > 0;

      for (let i = dotCount - 1; i >= 0; i--) {
        const o = i * K;
        const age = (dots[o + 6] += dt);
        if (age >= dots[o + 7]) {
          dots.copyWithin(o, (dotCount - 1) * K, dotCount * K);
          dotCount--;
          continue;
        }
        // tossed about by the hot air, then left to the wind
        const seed = dots[o + 9];
        dots[o + 3] += (wx * 0.7 - dots[o + 3]) * Math.min(1, dt * 1.6) + Math.sin(age * 9 + seed) * 4 * dt;
        dots[o + 5] += (wz * 0.7 - dots[o + 5]) * Math.min(1, dt * 1.6) + Math.cos(age * 7 + seed * 1.7) * 4 * dt;
        dots[o + 4] += (0.4 - dots[o + 4]) * Math.min(1, dt * 0.8);
        dots[o] += dots[o + 3] * dt;
        dots[o + 1] += dots[o + 4] * dt;
        dots[o + 2] += dots[o + 5] * dt;
      }
      for (let i = 0; i < dotCount; i++) {
        const o = i * K;
        const life = dots[o + 6] / dots[o + 7];
        const twinkle = 0.7 + 0.3 * Math.sin(dots[o + 6] * 23 + dots[o + 9]);
        sparks.aPos.array.set([dots[o], dots[o + 1], dots[o + 2], dots[o + 8] * (1 - 0.4 * life)], i * 4);
        sparks.aLook.array.set([0, Math.min(1, life * 8) * (1 - life * life) * twinkle, 1 - life, 0], i * 4);
      }
      sparks.geometry.instanceCount = dotCount;
      sparks.aPos.needsUpdate = sparks.aLook.needsUpdate = dotCount > 0;
    },
  };
}

/**
 * @param options.samples MSAA for the scene. options.fireScale the flames' pixels against the screen's.
 * @param options.bloom, options.exposure the finish. Both can be changed later on the frame.
 */
export function createFrame(renderer, field, { samples = 4, fireScale = 0.5, bloom = 0.22, exposure = 1, flameHeight = 1.5 } = {}) {
  const hdr = { type: THREE.HalfFloatType, format: THREE.RGBAFormat, minFilter: THREE.LinearFilter, magFilter: THREE.LinearFilter, generateMipmaps: false };
  const depth = new THREE.DepthTexture(1, 1, THREE.UnsignedIntType);
  depth.minFilter = depth.magFilter = THREE.NearestFilter;
  const rtScene = new THREE.WebGLRenderTarget(1, 1, { ...hdr, samples, depthBuffer: true, depthTexture: depth });
  const rtComposite = new THREE.WebGLRenderTarget(1, 1, { ...hdr, depthBuffer: false });
  const rtFire = new THREE.WebGLRenderTarget(1, 1, { ...hdr, depthBuffer: false });
  const rtDown = [];
  const rtUp = [];
  for (let i = 0; i < 5; i++) {
    rtDown.push(new THREE.WebGLRenderTarget(1, 1, { ...hdr, depthBuffer: false }));
    rtUp.push(new THREE.WebGLRenderTarget(1, 1, { ...hdr, depthBuffer: false }));
  }

  const fsGeometry = new THREE.BufferGeometry();
  fsGeometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array([-1, -1, 0, 3, -1, 0, -1, 3, 0]), 3));
  const fsMesh = new THREE.Mesh(fsGeometry);
  fsMesh.frustumCulled = false;
  const fsScene = new THREE.Scene();
  fsScene.add(fsMesh);
  const fsCamera = new THREE.Camera();
  const fs = (fragmentShader, uniforms) =>
    new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: fullscreenVertex, fragmentShader, uniforms, depthTest: false, depthWrite: false, blending: THREE.NoBlending });
  const head = /* glsl */ `precision highp float;\nlayout(location = 0) out vec4 outC;\nin vec2 vUv;\n`;

  const view = {
    tDepth: { value: depth },
    tVol: { value: volumeNoiseTexture(64) },
    uCamRight: { value: new THREE.Vector3() },
    uCamUp: { value: new THREE.Vector3() },
    uCamFwd: { value: new THREE.Vector3() },
    uTan: { value: new THREE.Vector2() },
    uProj: { value: new THREE.Matrix4() },
    uBox: { value: new THREE.Vector4(0, 0, -1, 0) },
    uBoxY: { value: new THREE.Vector2() },
    uFlameH: { value: flameHeight },
    uTorch: { value: new THREE.Vector4(0, 0, 0, 0) },
    uRes: { value: new THREE.Vector2() },
  };
  const marchMaterial = fs(marchFragment, { ...field.uniforms, ...view });
  const particles = createParticles(field, { ...field.uniforms, ...view });

  const compositeMaterial = fs(
    head +
      /* glsl */ `
    uniform sampler2D tScene, tFire;
    uniform vec2 uFireRes;
    void main() {
      // a one pixel tent evens out the march's jitter
      vec2 px = 0.75 / uFireRes;
      vec4 f = 0.25 * (texture(tFire, vUv + px * vec2(-1, -1)) + texture(tFire, vUv + px * vec2(1, -1)) + texture(tFire, vUv + px * vec2(-1, 1)) + texture(tFire, vUv + px * vec2(1, 1)));
      outC = vec4(texture(tScene, vUv).rgb * (1.0 - f.a) + f.rgb, texture(tScene, vUv).a);
    }`,
    { tScene: { value: rtScene.texture }, tFire: { value: rtFire.texture }, uFireRes: { value: new THREE.Vector2(1, 1) } },
  );
  const bloomFirst = fs(
    head +
      /* glsl */ `
    uniform sampler2D tSrc; uniform vec2 uTexel; uniform float uThreshold;
    vec3 pick(vec3 c) { float m = max(c.r, max(c.g, c.b)); float k = smoothstep(uThreshold * 0.7, uThreshold * 1.6, m); return min(c, vec3(14.0)) * k; }
    void main() {
      vec3 c = pick(texture(tSrc, vUv + uTexel * vec2(-1, -1)).rgb) + pick(texture(tSrc, vUv + uTexel * vec2(1, -1)).rgb)
        + pick(texture(tSrc, vUv + uTexel * vec2(-1, 1)).rgb) + pick(texture(tSrc, vUv + uTexel * vec2(1, 1)).rgb);
      outC = vec4(c * 0.25, 1.0);
    }`,
    { tSrc: { value: null }, uTexel: { value: new THREE.Vector2() }, uThreshold: { value: 1.5 } },
  );
  const bloomDown = fs(
    head +
      /* glsl */ `
    uniform sampler2D tSrc; uniform vec2 uTexel;
    void main() {
      vec3 c = texture(tSrc, vUv).rgb * 4.0;
      c += texture(tSrc, vUv + uTexel * vec2(-1, -1)).rgb + texture(tSrc, vUv + uTexel * vec2(1, -1)).rgb
        + texture(tSrc, vUv + uTexel * vec2(-1, 1)).rgb + texture(tSrc, vUv + uTexel * vec2(1, 1)).rgb;
      outC = vec4(c / 8.0, 1.0);
    }`,
    { tSrc: { value: null }, uTexel: { value: new THREE.Vector2() } },
  );
  const bloomUp = fs(
    head +
      /* glsl */ `
    uniform sampler2D tSrc, tPrev; uniform vec2 uTexel; uniform float uMix;
    void main() {
      vec3 c = texture(tPrev, vUv + uTexel * vec2(-1, 0)).rgb + texture(tPrev, vUv + uTexel * vec2(1, 0)).rgb
        + texture(tPrev, vUv + uTexel * vec2(0, -1)).rgb + texture(tPrev, vUv + uTexel * vec2(0, 1)).rgb;
      c += (texture(tPrev, vUv + uTexel * vec2(-1, -1)).rgb + texture(tPrev, vUv + uTexel * vec2(1, -1)).rgb
        + texture(tPrev, vUv + uTexel * vec2(-1, 1)).rgb + texture(tPrev, vUv + uTexel * vec2(1, 1)).rgb) * 0.5;
      outC = vec4(c / 6.0 * uMix + texture(tSrc, vUv).rgb, 1.0);
    }`,
    { tSrc: { value: null }, tPrev: { value: null }, uTexel: { value: new THREE.Vector2() }, uMix: { value: 0.85 } },
  );
  const finalMaterial = fs(
    head +
      /* glsl */ `
    uniform sampler2D tHdr, tBloom; uniform float uBloom, uExposure, uVignette, uPaint; uniform vec2 uPaintTexel; uniform highp sampler2D tDepth;
    // The frame as paint: round each point, four overlapping patches; the calmest of them gives its colour. Edges
    // stay where they are and everything between them goes flat, as a loaded brush leaves it.
    vec3 painted(vec2 uv) {
      // a finer brush for what is near: grass at your feet keeps its blades, the far country goes to flat paint
      float away = 0.12 / max(1.0 - textureLod(tDepth, uv, 0.0).r, 1e-6);
      int r = int(uPaint);
      // (what is painted already keeps its detail)
      bool done = textureLod(tHdr, uv, 0.0).a < 0.6;
      // The brush is broadest in the middle distance, where grass is a mass. Near, it leaves the strokes of the
      // blades alone; far off, where a house is a few pixels across, it leaves those alone too.
      r = min(r, 2);
      if (away < 16.0 || away > 140.0) r = 1;
      if (done) r = 0;
      vec3 m[4];
      vec3 s[4];
      for (int k = 0; k < 4; k++) { m[k] = vec3(0.0); s[k] = vec3(0.0); }
      for (int j = -r; j <= r; j++) {
        for (int i = -r; i <= r; i++) {
          vec3 c = textureLod(tHdr, uv + vec2(float(i), float(j)) * uPaintTexel, 0.0).rgb;
          c = c / (1.0 + c);
          vec3 cc = c * c;
          if (i <= 0 && j <= 0) { m[0] += c; s[0] += cc; }
          if (i >= 0 && j <= 0) { m[1] += c; s[1] += cc; }
          if (i <= 0 && j >= 0) { m[2] += c; s[2] += cc; }
          if (i >= 0 && j >= 0) { m[3] += c; s[3] += cc; }
        }
      }
      float n = float((r + 1) * (r + 1));
      vec3 sum = vec3(0.0);
      float total = 0.0;
      for (int k = 0; k < 4; k++) {
        vec3 mean = m[k] / n;
        vec3 var = abs(s[k] / n - mean * mean);
        float v = var.r + var.g + var.b;
        // (the calmest patch nearly alone, but never a hard choice: a hard one flickers)
        float w = 1.0 / (1e-7 + v * v * v * v * 1e12);
        sum += mean * w;
        total += w;
      }
      vec3 o = sum / total;
      return o / (1.0 - min(o, vec3(0.985)));
    }
    // The inked line round a figure. A figure's parts each write a small alpha of their own (valley/wanderer.js),
    // smaller than anything else writes; where one of those meets a different alpha, there is a line.
    float inkAt(vec2 uv) {
      float a = textureLod(tHdr, uv, 0.0).a;
      float da = textureLod(tDepth, uv, 0.0).r;
      vec2 w = uPaintTexel * max(1.0, 1.0 / (uPaintTexel.y * 1080.0));
      float e = 0.0;
      for (int k = 0; k < 4; k++) {
        vec2 o = k == 0 ? vec2(1.0, 0.0) : k == 1 ? vec2(-1.0, 0.0) : k == 2 ? vec2(0.0, 1.0) : vec2(0.0, -1.0);
        float b = textureLod(tHdr, uv + o * w, 0.0).a;
        if (min(a, b) >= 0.23) continue;
        // (a line where the figure stands in front of something, or one of its parts meets another: none where a
        // blade of grass stands in front of it, or every blade would be inked)
        float db = textureLod(tDepth, uv + o * w, 0.0).r;
        if (max(a, b) >= 0.23 && (a < b ? db < da : da < db)) continue;
        e = max(e, abs(a - b));
      }
      return smoothstep(0.008, 0.022, e);
    }
    vec3 aces(vec3 x) { return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), 0.0, 1.0); }
    vec3 toSrgb(vec3 c) { return mix(c * 12.92, 1.055 * pow(c, vec3(1.0 / 2.4)) - 0.055, step(0.0031308, c)); }
    void main() {
      vec3 c = (uPaint > 0.5 ? painted(vUv) : texture(tHdr, vUv).rgb) + texture(tBloom, vUv).rgb * uBloom;
      // (the line is the colour under it, gone dark and a little warm: never a black wire)
      c = mix(c, min(c, vec3(1.2)) * vec3(0.2, 0.14, 0.15), inkAt(vUv) * 0.92);
      vec2 d = vUv - 0.5;
      c *= 1.0 - uVignette * dot(d, d) * 1.6;
      c *= uExposure;
      // a painter's curve: what is below white stays as it was mixed, and only the brightest lights are eased in
      vec3 hi = max(c - 0.62, 0.0);
      c = min(c, 0.62) + 0.38 * (1.0 - exp(-hi / 0.38));
      float l = dot(c, vec3(0.2126, 0.7152, 0.0722));
      c = toSrgb(max(mix(vec3(l), c, 1.06), 0.0));
      float n = fract(52.9829189 * fract(dot(gl_FragCoord.xy, vec2(0.06711056, 0.00583715))));
      outC = vec4(c + (n - 0.5) / 255.0, 1.0);
    }`,
    { tHdr: { value: rtComposite.texture }, tBloom: { value: rtUp[0].texture }, uBloom: { value: bloom }, uExposure: { value: exposure }, uVignette: { value: 0.14 }, uPaint: { value: 0 }, tDepth: { value: depth }, uPaintTexel: { value: new THREE.Vector2(1, 1) } },
  );

  field.renderer = renderer;
  const size = new THREE.Vector2();
  let width = 0;
  let height = 0;
  const pass = (material, target) => {
    fsMesh.material = material;
    renderer.setRenderTarget(target);
    renderer.render(fsScene, fsCamera);
  };
  const fit = (w, h) => {
    if (w === width && h === height) return;
    width = w;
    height = h;
    depth.image.width = w;
    depth.image.height = h;
    rtScene.setSize(w, h);
    rtComposite.setSize(w, h);
    const fw = Math.max(1, Math.round(w * frame.fireScale));
    const fh = Math.max(1, Math.round(h * frame.fireScale));
    rtFire.setSize(fw, fh);
    compositeMaterial.uniforms.uFireRes.value.set(fw, fh);
    view.uRes.value.set(w, h);
    let bw = w;
    let bh = h;
    for (let i = 0; i < rtDown.length; i++) {
      bw = Math.max(1, bw >> 1);
      bh = Math.max(1, bh >> 1);
      rtDown[i].setSize(bw, bh);
      rtUp[i].setSize(bw, bh);
    }
  };

  const frame = {
    bloom,
    exposure,
    fireScale,
    particles,
    /** A flame to carry: set .position (its foot) and .size (its height in metres, 0 for none). */
    torch: { position: new THREE.Vector3(), size: 0 },
    rtScene,
    update(dt, camera) {
      particles.update(dt, camera);
    },
    render(scene, camera) {
      renderer.getDrawingBufferSize(size);
      fit(size.x, size.y);
      camera.updateMatrixWorld();
      const e = camera.matrixWorld.elements;
      view.uCamRight.value.set(e[0], e[1], e[2]).normalize();
      view.uCamUp.value.set(e[4], e[5], e[6]).normalize();
      view.uCamFwd.value.set(-e[8], -e[9], -e[10]).normalize();
      const tan = Math.tan(THREE.MathUtils.degToRad(camera.fov) / 2);
      view.uTan.value.set(tan * camera.aspect, tan);
      view.uProj.value.copy(camera.projectionMatrix);
      const state = field.state;
      const b = state.fireBox;
      if (state.burning > 0) {
        view.uBox.value.set(b.x0, b.z0, b.x1, b.z1);
        view.uBoxY.value.set(b.y0, b.y1);
      } else view.uBox.value.set(0, 0, -1, 0);
      view.uTorch.value.set(frame.torch.position.x, frame.torch.position.y, frame.torch.position.z, frame.torch.size);

      renderer.setRenderTarget(rtScene);
      renderer.render(scene, camera);
      const anyFire = state.burning > 0 || frame.torch.size > 0;
      if (anyFire) pass(marchMaterial, rtFire);
      else {
        renderer.setRenderTarget(rtFire);
        renderer.setClearColor(0x000000, 0);
        renderer.clear();
      }
      pass(compositeMaterial, rtComposite);
      if (particles.count) {
        const auto = renderer.autoClear;
        renderer.autoClear = false;
        renderer.setRenderTarget(rtComposite);
        renderer.render(particles.scene, camera);
        renderer.autoClear = auto;
      }

      let src = rtComposite;
      for (let i = 0; i < rtDown.length; i++) {
        const m = i === 0 ? bloomFirst : bloomDown;
        m.uniforms.tSrc.value = src.texture;
        m.uniforms.uTexel.value.set(0.5 / rtDown[i].width, 0.5 / rtDown[i].height);
        pass(m, rtDown[i]);
        src = rtDown[i];
      }
      let prev = rtDown[rtDown.length - 1];
      for (let i = rtDown.length - 2; i >= 0; i--) {
        bloomUp.uniforms.tSrc.value = rtDown[i].texture;
        bloomUp.uniforms.tPrev.value = prev.texture;
        bloomUp.uniforms.uTexel.value.set(0.5 / prev.width, 0.5 / prev.height);
        pass(bloomUp, rtUp[i]);
        prev = rtUp[i];
      }
      finalMaterial.uniforms.uBloom.value = frame.bloom;
      finalMaterial.uniforms.uExposure.value = frame.exposure;
      finalMaterial.uniforms.uPaint.value = frame.paint ?? 0;
      finalMaterial.uniforms.uPaintTexel.value.set(1 / size.x, 1 / size.y);
      pass(finalMaterial, null);
    },
    /**
     * Compiles every shader the frame and the scene use, for the scene's own target, and sends every texture they
     * read to the GPU. Await it behind your loading screen. Each material is started in a task of its own and each
     * texture sent in one, so the page keeps answering while it goes: three's own compile does a whole scene in one
     * go, which here held the page for half a second. `onStep(done, of)` is called as it goes.
     */
    async compile(scene, camera, onStep) {
      renderer.getDrawingBufferSize(size);
      fit(size.x, size.y);
      const turn = () =>
        new Promise((r) => {
          const c = new MessageChannel();
          c.port1.onmessage = () => r();
          c.port2.postMessage(0);
        });
      // one drawable for each material in the scene, and every texture any of them reads
      const first = new Map();
      const textures = new Set([view.tVol.value]);
      const collect = (material) => {
        for (const u of Object.values(material.uniforms ?? {})) if (u?.value?.isTexture && !u.value.isRenderTargetTexture && !u.value.isDepthTexture) textures.add(u.value);
        for (const k of ['map', 'normalMap', 'roughnessMap', 'metalnessMap', 'aoMap', 'emissiveMap', 'alphaMap']) if (material[k]?.isTexture) textures.add(material[k]);
      };
      scene.traverse((o) => {
        if (!(o.isMesh || o.isPoints || o.isLine) || !o.material || Array.isArray(o.material)) return;
        if (!first.has(o.material)) first.set(o.material, o);
        collect(o.material);
      });
      const steps = first.size + textures.size + 8;
      let done = 0;
      const step = async () => {
        onStep?.(++done, steps);
        await turn();
      };
      // textures first: once the shaders are linking the GPU's thread is busy for a while, and a big upload sent
      // then waits for it, with the page
      for (const texture of textures) {
        renderer.initTexture(texture);
        await step();
      }
      const jobs = [];
      const target = renderer.getRenderTarget();
      for (const object of first.values()) {
        renderer.setRenderTarget(rtScene);
        jobs.push(renderer.compileAsync(object, camera, scene));
        renderer.setRenderTarget(target);
        await step();
      }
      renderer.setRenderTarget(rtComposite);
      jobs.push(renderer.compileAsync(particles.scene, camera));
      renderer.setRenderTarget(target);
      await step();
      for (const m of [marchMaterial, compositeMaterial, bloomFirst, bloomDown, bloomUp, finalMaterial]) {
        fsMesh.material = m;
        renderer.setRenderTarget(m === marchMaterial ? rtFire : rtComposite);
        jobs.push(renderer.compileAsync(fsScene, fsCamera));
        renderer.setRenderTarget(target);
        await step();
      }
      await Promise.all(jobs);
    },
  };
  return frame;
}
