// The tarn on the valley floor: still water that holds the sky. It mirrors the dome and its clouds, shows its bed
// where you look down into it, and thins to nothing at its shore.
import * as THREE from 'three';
import { COMMON } from '../lib/index.js';
import { HERO, TARN } from './world.js';

const vertex = /* glsl */ `
out vec3 vWorld;
void main() {
  vec4 w = modelMatrix * vec4(position, 1.0);
  vWorld = w.xyz;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`;

const fragment = /* glsl */ `
${COMMON}
precision highp samplerCube;
layout(location = 0) out vec4 outC;
in vec3 vWorld;
uniform vec3 uZenith, uMid, uHorizon, uGlow;
uniform samplerCube tClouds;
uniform vec4 uTarn;   // centre x, z, and its half-lengths across and along the valley
uniform vec2 uAlong;  // the way the valley runs

vec3 skyAt(vec3 d) {
  float h = max(d.y, 0.0);
  vec3 col = mix(uHorizon, uMid, smoothstep(0.0, 0.3, pow(h, 0.8)));
  col = mix(col, uZenith, smoothstep(0.18, 0.85, h));
  vec4 c = texture(tClouds, d);
  return col * (1.0 - c.a) + c.rgb;
}

void main() {
  vec2 xz = vWorld.xz;
  float depth = vWorld.y - groundAt(xz);
  if (depth <= 0.0) discard;
  // only the tarn: low ground beyond its banks stays dry
  vec2 off = xz - uTarn.xy;
  if (length(vec2(dot(off, vec2(-uAlong.y, uAlong.x)) / uTarn.z, dot(off, uAlong) / uTarn.w)) > 1.32) discard;
  vec3 V = normalize(uCamPos - vWorld);
  float dist = distance(uCamPos, vWorld);
  // the wind's small ripples, travelling; calmer in the lee of the bank
  float U = uWind.z * (0.35 + 0.65 * gustAt(xz));
  vec2 flow = uWind.xy * uTime;
  vec2 r1 = texture(tNoiseHi, (xz - flow * 0.22) / 7.3).rg - 0.5;
  vec2 r2 = texture(tNoiseHi, (xz.yx + flow * 0.31) / 2.1 + 0.4).ba - 0.5;
  vec2 rip = (r1 + r2 * 0.6) * 0.035 * U * smoothstep(0.0, 0.5, depth) / (1.0 + dist * 0.012);
  vec3 N = normalize(vec3(rip.x, 1.0, rip.y));
  vec3 R = reflect(-V, N);
  R.y = abs(R.y);
  // (it holds more of the sky's blue than a true mirror seen this low would: that is how it is painted)
  vec3 mirror = mix(skyAt(normalize(R + vec3(0.0, 0.3, 0.0))), uMid * 1.05, 0.5);
  // what stands low on the far side is mirrored too: the bank, the hills
  float shore = 1.0 - smoothstep(0.015, 0.11, R.y + r1.x * 0.03);
  mirror = mix(mirror, vec3(0.03, 0.115, 0.05) * (uSkyColor + uSunColor * 0.25), shore * 0.9);
  float fres = 0.03 + 0.97 * pow(1.0 - max(dot(N, V), 0.0), 5.0);
  float shade = cloudAt(xz);
  vec3 bed = mix(vec3(0.1, 0.19, 0.11), vec3(0.01, 0.06, 0.065), smoothstep(0.0, 1.5, depth)) * (uSkyColor + uSunColor * 0.3 * shade);
  vec3 col = mix(bed, mirror, clamp(fres * 1.15 + 0.16, 0.0, 1.0));
  // the sun's path on it, in sparks
  float glint = pow(max(dot(R, uSunDir), 0.0), 420.0) * shade;
  col += uSunColor * glint * 2.5;
  // a dark line where the bank meets the water
  col = mix(vec3(0.045, 0.12, 0.075) * (uSkyColor + uSunColor * 0.25), col, smoothstep(0.04, 0.3, depth));
  float alpha = smoothstep(0.0, 0.1, depth);
  outC = vec4(applyFog(col, vWorld), alpha);
}
`;

/** @param sky from createSky: the water mirrors its dome */
export function createTarn(field, sky) {
  const reach = Math.max(TARN.rx, TARN.rz) * 1.5;
  const geometry = new THREE.PlaneGeometry(reach * 2, reach * 2, 1, 1);
  geometry.rotateX(-Math.PI / 2);
  const u = sky.uniforms;
  const uniforms = { ...field.uniforms, uZenith: u.uZenith, uMid: u.uMid, uHorizon: u.uHorizon, uGlow: u.uGlow, tClouds: u.tClouds, uTarn: { value: new THREE.Vector4(TARN.at[0], TARN.at[1], TARN.rx, TARN.rz) }, uAlong: { value: new THREE.Vector2(HERO[0], HERO[1]) } };
  const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: vertex, fragmentShader: fragment, uniforms, transparent: true, depthWrite: false });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.position.set(TARN.at[0], TARN.level, TARN.at[1]);
  mesh.renderOrder = 5;
  return mesh;
}
