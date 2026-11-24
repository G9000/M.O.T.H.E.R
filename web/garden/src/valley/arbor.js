// A double-ribbed trellis at the entrance. The opening faces the valley; its
// feet stand between the lantern ring and the first turn of the record path.
import * as THREE from 'three';
import { COMMON } from '../lib/index.js';
import { fromValley, HERO } from './world.js';

const vertexShader = `
${COMMON}
out vec3 vColour;
out vec3 vWorld;
out vec3 vNormal;
out vec2 vSurface;
uniform float uBlossom;
void main() {
  mat4 m = modelMatrix * instanceMatrix;
  vec4 world = m * vec4(position, 1.);
  // Each pendant moves as a spray, with the lowest florets swinging furthest.
  world.xz += uWind.xy * uBlossom * (0.014 + 0.009 * sin(world.y * 3.))
    * sin(uTime * 1.2 + world.z * 1.6 + world.x);
  vWorld = world.xyz;
  vColour = instanceColor;
  vSurface = uv;
  mat3 basis = mat3(m);
  vNormal = normalize(basis * (normal / vec3(dot(basis[0], basis[0]), dot(basis[1], basis[1]), dot(basis[2], basis[2]))));
  gl_Position = projectionMatrix * viewMatrix * world;
}`;
const fragmentShader = `
${COMMON}
in vec3 vColour;
in vec3 vWorld;
in vec3 vNormal;
in vec2 vSurface;
uniform float uDetail;
layout(location = 0) out vec4 outC;
void main() {
  vec3 n = normalize(vNormal);
  float sun = smoothstep(-0.4, 0.8, dot(n, uSunDir));
  vec3 light = uSkyColor * (0.68 + 0.32 * n.y) + uSunColor * (0.30 + 0.65 * sun) * cloudAt(vWorld.xz);
  vec3 paint = vColour;
  if (uDetail > 0.5) {
    if (!gl_FrontFacing) n = -n;
    float t = vSurface.y, s = vSurface.x * 2.0 - 1.0;
    float ridge = cos(23.0 * s + 1.7 * sin(3.3 * t)) * (1.0 - smoothstep(0.02, 0.22, fwidth(s)));
    paint = mix(paint * vec3(0.47, 0.32, 0.64), paint, smoothstep(0.0, 0.73, t));
    paint *= 1.0 + 0.09 * ridge;
    if (uDetail < 1.5) {
      // Veins fan out from the base, splitting as the petal widens.
      float fan = s * (2.2 + 1.65 * t) + 0.15 * sin(5.0 * t + 2.5 * s);
      float footprint = max(fwidth(fan), 0.001);
      float vein = (1.0 - smoothstep(0.018, 0.07 + footprint, abs(fract(fan + 0.5) - 0.5)))
        * (1.0 - smoothstep(0.09, 0.42, footprint));
      vec2 cell = texture(tNoise, vSurface * vec2(2.6, 4.3)).rg;
      paint *= 0.89 + 0.22 * cell.x;
      paint = mix(paint, paint * vec3(0.70, 0.43, 0.61), vein * (0.75 - 0.4 * t));
      float rim = smoothstep(0.74, 1.0, t) * (0.45 + 0.55 * abs(s));
      paint = mix(paint, vec3(0.96, 0.79, 0.77), 0.42 * rim);
      float throat = (1.0 - smoothstep(0.04, 0.48, abs(s))) * (1.0 - smoothstep(0.20, 0.52, t));
      paint = mix(paint, vec3(0.98, 0.78, 0.45), 0.75 * throat);
    }
    light = uSkyColor * 0.64 + uSunColor * (0.28 + 0.65 * abs(dot(n, uSunDir))) * cloudAt(vWorld.xz);
    if (uDetail < 1.5) {
      float backlit = max(dot(-normalize(uCamPos - vWorld), uSunDir), 0.0);
      light += uSunColor * vec3(0.25, 0.14, 0.13) * backlit * backlit * smoothstep(0.3, 1.0, t);
    }
  }
  outC = vec4(applyFog(paint * light, vWorld), 1.);
}`;

export function paintedInstances(field, geometry, batch, blossom = 0, detail = 0) {
  const material = new THREE.ShaderMaterial({
    glslVersion: THREE.GLSL3, vertexShader, fragmentShader,
    uniforms: { ...field.uniforms, uBlossom: { value: blossom }, uDetail: { value: detail } },
    side: detail ? THREE.DoubleSide : THREE.FrontSide,
  });
  const mesh = new THREE.InstancedMesh(geometry, material, batch.length);
  batch.forEach(([matrix, colour], i) => {
    mesh.setMatrixAt(i, matrix);
    mesh.setColorAt(i, colour);
  });
  return mesh;
}

// A banner petal spreads above two curved wings and a forward keel.
// The same strips also give the compound leaves a folded, tapered blade.
function wisteriaGeometry(leaf = false) {
  const positions = [], uvs = [], indices = [];
  const petals = leaf ? [[0, 1, 1, 0.2]] : [[0, 1.28, 1.15, 0.88], [-1.22, 0.84, 0.65, -0.32],
    [1.22, 0.84, 0.65, -0.32], [0, 0.78, 0.48, -0.82]];
  for (const [turn, length, width, lift] of petals) {
    const start = positions.length / 3;
    for (let i = 0; i <= 6; i++) for (let j = 0; j <= 4; j++) {
      const t = i / 6, s = (j - 2) / 2;
      const w = width * (0.02 + Math.sin(Math.PI * t * (leaf ? 1 : 0.87)) ** 0.75);
      const x = s * w, z = t * length * (leaf ? 1.8 : 1) - 0.15 * w * s * s;
      positions.push(x * Math.cos(turn) + z * Math.sin(turn),
        t * length * lift + 0.27 * w * s * s - 0.22 * t * t, z * Math.cos(turn) - x * Math.sin(turn));
      uvs.push(j / 4, t);
      const k = start + i * 5 + j;
      if (i < 6 && j < 4) indices.push(k, k + 5, k + 1, k + 1, k + 5, k + 6);
    }
  }
  const g = new THREE.BufferGeometry();
  g.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
  g.setAttribute('uv', new THREE.Float32BufferAttribute(uvs, 2));
  g.setIndex(indices);
  g.computeVertexNormals();
  return g;
}

export function createArbor(field, quality) {
  const group = new THREE.Group();
  group.name = 'Wisteria arbor';
  const [x, z] = fromValley(6.4, 0);
  group.position.set(x, field.heightAt(x, z), z);
  group.rotation.y = Math.atan2(HERO[0], HERO[1]);
  const solid = [];
  const metal = [], stone = [], flowers = [], leaves = [];
  const dummy = new THREE.Object3D();
  const up = new THREE.Vector3(0, 1, 0);
  const brass = '#bb9667', iron = '#51665b';
  const half = 2.48, spring = 2.36, rise = 1.54;
  const v = (x, y, z) => new THREE.Vector3(x, y, z);
  const arch = (a, depth, radius = half) => v(radius * Math.cos(a), spring + rise * Math.sin(a), depth);
  function piece(batch, colour) {
    dummy.updateMatrix();
    batch.push([dummy.matrix.clone(), new THREE.Color(colour)]);
  }
  function rod(a, b, radius, colour = iron) {
    const direction = b.clone().sub(a);
    dummy.position.copy(a).lerp(b, 0.5);
    dummy.scale.set(radius, direction.length(), radius);
    dummy.quaternion.setFromUnitVectors(up, direction.normalize());
    piece(metal, colour);
  }
  function block(batch, at, scale, colour, turn = 0, tilt = 0) {
    dummy.position.copy(at);
    dummy.scale.copy(scale);
    dummy.rotation.set(0, turn, tilt);
    piece(batch, colour);
  }
  // Piers and collars, paired curved rails and a diagonal lattice between ribs.
  for (const side of [-1, 1]) {
    for (const depth of [-0.56, 0.56]) {
      const foot = v(side * half, 0, depth);
      const world = group.localToWorld(foot.clone());
      const base = field.heightAt(world.x, world.z) - group.position.y;
      solid.push([world.x, world.z, 0.30]);
      block(stone, v(foot.x, base + 0.18, depth), v(0.46, 0.50, 0.46), '#b4aa97');
      block(stone, v(foot.x, base + 0.43, depth), v(0.53, 0.12, 0.53), '#dfd3b2');
      rod(v(foot.x, base + 0.45, depth), v(foot.x, spring, depth), 0.074);
      for (const y of [0.60, 2.20, spring]) {
        rod(v(foot.x, y, depth), v(foot.x, y + 0.075, depth), 0.105, brass);
      }
    }
    for (let j = 0; j < 3; j++) {
      const y = 0.57 + j * 0.55;
      for (const d of [-0.56, 0.56]) rod(v(side * half, y, d), v(side * half, y + 0.55, -d), 0.021);
    }
  }
  for (let i = 0; i < 32; i++) {
    const a = Math.PI * i / 32, b = Math.PI * (i + 1) / 32;
    for (const depth of [-0.56, 0.56]) rod(arch(a, depth), arch(b, depth), 0.062);
    if (i % 2 === 0) rod(arch(a, -0.56), arch(a, 0.56), 0.038, brass);
    rod(arch(a, i % 2 ? 0.56 : -0.56), arch(b, i % 2 ? -0.56 : 0.56), 0.022);
  }
  const palette = ['#9572aa', '#b6a0cf', '#d8c3dc', '#f1deeb', '#ab88b9'];
  const stems = quality === 'low' ? 65 : 112;
  for (let i = 0; i < stems; i++) {
    const angle = Math.PI * (i + 0.5) / stems;
    const depth = Math.sin(i * 2.4) * 0.55;
    const top = arch(angle, depth);
    const sway = Math.sin(i * 13.17);
    const length = 0.26 + (0.45 + 0.30 * Math.cos(angle * 2)) * (0.7 + 0.3 * sway);
    for (const side of [-1, 1]) {
      block(leaves, top, v(0.105, 0.15, 0.135), i % 2 ? '#73925a' : '#496f53', i * 2.4 + side, sway * 0.5);
    }
    rod(top, v(top.x + sway * 0.07, top.y - length, depth), 0.006, '#75805a');
    const steps = quality === 'low' ? 7 : 12;
    for (let j = 0; j < steps; j++) {
      const t = (j + 0.5) / steps, spiral = j * 2.40 + i;
      const radius = 0.095 * (1 - t) + 0.016;
      const at = v(top.x + Math.cos(spiral) * radius + sway * t * 0.07, top.y - length * t, depth + Math.sin(spiral) * radius);
      block(flowers, at, v(radius, radius * 0.8, radius * 0.68), palette[(i + j) % palette.length], spiral, sway * 0.7);
    }
  }
  group.add(
    paintedInstances(field, new THREE.CylinderGeometry(1, 1, 1, 8), metal),
    paintedInstances(field, new THREE.BoxGeometry(), stone),
    paintedInstances(field, wisteriaGeometry(), flowers, 1, 1),
    paintedInstances(field, wisteriaGeometry(true), leaves, 1, 2),
  );
  return { group, solid };
}
