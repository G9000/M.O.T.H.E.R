import * as THREE from 'three';
import { COMMON } from '../lib/glsl.js';
import { paintedInstances } from './arbor.js';
import { TARN } from './world.js';

// Each pair is a radius and elevation above the water. Ribs and glazing use
// the same profile so every pane meets its frame, including the roof lantern.
const profile = [
  [3.85, 0.48], [3.85, 2.65], [3.85, 3.45], [3.70, 3.68],
  [3.38, 4.40], [2.68, 5.10], [1.72, 5.60], [0.78, 5.86],
  [0.78, 6.44], [0.44, 6.82], [0.06, 7.08],
];
const bays = 16;
const up = new THREE.Vector3(0, 1, 0);
const at = (a, r, y) => new THREE.Vector3(Math.cos(a) * r, y, Math.sin(a) * r);

export function createGlasshouse(field) {
  const house = new THREE.Group();
  house.name = 'Water garden glasshouse';
  house.position.set(TARN.at[0], TARN.level, TARN.at[1]);
  const ribs = [], plinth = [], fronds = [];
  const verts = [], tints = [];
  const pose = new THREE.Object3D();
  const iron = new THREE.Color('#456a61'), brass = new THREE.Color('#d4ad72');
  function rod(a, b, radius, colour) {
    const dir = b.clone().sub(a);
    pose.position.copy(a).lerp(b, 0.5);
    pose.scale.set(radius, dir.length(), radius);
    pose.quaternion.setFromUnitVectors(up, dir.normalize());
    pose.updateMatrix();
    ribs.push([pose.matrix.clone(), colour]);
  }
  function pane(a, b, c, d, tint) {
    for (const p of [a, b, c, c, b, d]) {
      verts.push(p.x, p.y, p.z);
      tints.push(tint.r, tint.g, tint.b);
    }
  }
  for (let i = 0; i < bays; i++) {
    const a = Math.PI * 2 * i / bays, b = Math.PI * 2 * (i + 1) / bays;
    for (let j = 0; j < profile.length - 1; j++) {
      const p = at(a, ...profile[j]), q = at(b, ...profile[j]);
      const r = at(a, ...profile[j + 1]), s = at(b, ...profile[j + 1]);
      rod(p, r, i % 2 === 0 ? 0.056 : 0.032, iron);
      rod(p, q, j === 2 || j === 7 ? 0.074 : 0.026, j === 2 || j === 7 ? brass : iron);
      const accent = j === 1 || j === 7;
      pane(p, r, q, s, new THREE.Color(accent ? (i % 4 < 2 ? '#d3b3d7' : '#ebc991') : '#8ac7ba'));
    }
    // A pointed fanlight above each bay, traced in brass.
    const crown = at((a + b) / 2, 3.80, 3.40);
    for (const angle of [a, b]) {
      const corner = at(angle, 3.85, 2.65);
      const shoulder = at(angle * 0.68 + (a + b) * 0.16, 3.82, 3.11);
      rod(corner, shoulder, 0.026, brass);
      rod(shoulder, crown, 0.026, brass);
    }
  }
  rod(at(0, 0, 7), at(0, 0, 7.88), 0.047, brass);
  for (const [r, y, h, tint] of [[4.40, -0.93, 2.3, '#7b8276'], [4.48, 0.22, 0.18, '#c4b9a2'], [4.05, 0.38, 0.24, '#a59985']]) {
    const m = new THREE.Matrix4().makeScale(r, h, r).setPosition(0, y, 0);
    plinth.push([m, new THREE.Color(tint)]);
  }
  // The palm silhouettes give the transparent lower bays depth.
  for (const [x, z, height] of [[0.4, 0.2, 3.6], [-1.5, 0.8, 2.3], [1.6, -0.9, 2.65]]) {
    const top = new THREE.Vector3(x, height, z);
    rod(new THREE.Vector3(x - 0.18, 0.46, z), top, 0.09, iron);
    for (let i = 0; i < 13; i++) {
      const angle = i * 2.4 + height;
      const span = height * (0.29 + 0.12 * Math.cos(i * 1.8));
      const leaf = (t) => top.clone().add(at(angle, span * t, 0.52 * Math.sin(t * Math.PI) - 0.28 * t));
      for (let j = 0; j < 4; j++) {
        const base = leaf(j / 4), tip = leaf((j + 1) / 4), direction = tip.clone().sub(base);
        pose.position.copy(base).lerp(tip, 0.5);
        pose.scale.set(0.10 + 0.11 * Math.sin(Math.PI * (j + 0.5) / 4), direction.length() * 0.72, 0.06);
        pose.quaternion.setFromUnitVectors(up, direction.normalize());
        pose.updateMatrix();
        fronds.push([pose.matrix.clone(), new THREE.Color(i % 3 ? '#599278' : '#929e6a')]);
      }
    }
  }
  // Climbers wrap alternating pillars. Leave the other bays clear so a walker
  // on either bank can see through the glass to the palms.
  const vine = new THREE.Color('#47674f');
  const blossom = ['#cfaccd', '#b395c2', '#e6d5df'].map(hex => new THREE.Color(hex));
  for (let i = 0; i < bays; i += 4) {
    const angle = i * 2 * Math.PI / bays;
    const root = at(angle, 3.58, 0.50);
    const pot = new THREE.Matrix4().makeScale(0.35, 0.42, 0.35).setPosition(root.x, 0.66, root.z);
    plinth.push([pot, new THREE.Color('#b58369')]);
    let last = root;
    for (let j = 1; j <= 26; j++) {
      const t = j / 26, phase = angle + t * 13;
      const bud = at(angle + 0.25 * t * t, 3.57 + 0.12 * Math.cos(phase), 0.65 + 2.28 * t);
      bud.x += 0.12 * Math.sin(phase);
      rod(last, bud, 0.024, vine);
      pose.position.copy(bud);
      pose.rotation.set(0.3, phase, 0.65 * Math.sin(phase));
      pose.scale.set(0.20, 0.10, 0.11);
      pose.updateMatrix();
      fronds.push([pose.matrix.clone(), vine]);
      if (j > 11 && j % 3 === 0) {
        for (let k = 0; k < 8; k++) {
          const size = 0.085 - k * 0.007;
          pose.position.set(bud.x + size * Math.cos(k * 2.4), bud.y - k * 0.045, bud.z + size * Math.sin(k * 2.4));
          pose.scale.set(size, size * 0.9, size);
          pose.updateMatrix();
          fronds.push([pose.matrix.clone(), blossom[(k + j) % blossom.length]]);
        }
      }
      last = bud;
    }
  }
  const glass = new THREE.BufferGeometry();
  glass.setAttribute('position', new THREE.Float32BufferAttribute(verts, 3));
  glass.setAttribute('color', new THREE.Float32BufferAttribute(tints, 3));
  glass.computeVertexNormals();
  const panes = new THREE.Mesh(glass, new THREE.ShaderMaterial({
    glslVersion: THREE.GLSL3, uniforms: field.uniforms, vertexColors: true,
    side: THREE.DoubleSide, transparent: true, depthWrite: false, forceSinglePass: true,
    vertexShader: `
      out vec3 vWorld, vNormal, vTint;
      void main() {
        vec4 w = modelMatrix * vec4(position, 1.0);
        vWorld = w.xyz; vNormal = mat3(modelMatrix) * normal; vTint = color;
        gl_Position = projectionMatrix * viewMatrix * w;
      }`,
    fragmentShader: `
      ${COMMON}
      in vec3 vWorld, vNormal, vTint;
      layout(location = 0) out vec4 outC;
      void main() {
        vec3 n = normalize(vNormal), v = normalize(uCamPos - vWorld);
        float rim = pow(1.0 - abs(dot(n, v)), 2.0);
        float glint = pow(max(dot(reflect(-v, n), uSunDir), 0.0), 50.0);
        float dusk = 1.0 - smoothstep(0.03, 0.28, uSunDir.y);
        vec3 colour = vTint * (uSkyColor + 0.23) + rim * uSkyColor * 0.65;
        colour += uSunColor * glint * 1.6 + vec3(0.56, 0.29, 0.11) * dusk;
        outC = vec4(applyFog(colour, vWorld), 0.27 + 0.39 * rim + 0.13 * dusk);
      }`,
  }));
  panes.renderOrder = 6;
  house.add(paintedInstances(field, new THREE.CylinderGeometry(1, 1, 1, 8), ribs),
    paintedInstances(field, new THREE.CylinderGeometry(1, 1, 1, bays), plinth),
    paintedInstances(field, new THREE.IcosahedronGeometry(1, 0), fronds), panes);
  return house;
}
