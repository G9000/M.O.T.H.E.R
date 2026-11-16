// What lies past the field's own ground, out to where the painted backdrop takes over: the valley's plain, with
// forest in masses, meadows between, a river winding through and fields round a hamlet. It is real ground, so it
// shifts as you walk; what stands on it (farwoods.js) is cheap. At its inner edge it is the same grass, lit the
// same way, as the ground it meets; at its outer edge it is lost in the same haze as the backdrop.
import * as THREE from 'three';
import { COMMON, LEAF_LIGHT } from '../lib/index.js';
import { fieldsAt, forestAt, heightAt, HERO } from './world.js';

/** How far the plain reaches, metres. The backdrop begins a little inside this. */
export const REACH = 6200;

const vertex = /* glsl */ `
out vec3 vWorld;
out vec3 vNormal;
void main() {
  vWorld = position;
  vNormal = normal;
  gl_Position = projectionMatrix * viewMatrix * vec4(position, 1.0);
}
`;

const fragment = /* glsl */ `
${COMMON}
${LEAF_LIGHT}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
uniform sampler2D tFar;     // forest, fields, over the square the plain covers
uniform float uFarInv;      // 1 / that square's side
uniform vec2 uAlong;        // the way the valley runs

void main() {
  vec3 N = normalize(vNormal);
  if (N.y < 0.0) N = -N;
  vec2 xz = vWorld.xz;
  float dist = distance(vWorld, uCamPos);
  vec3 V = normalize(uCamPos - vWorld);
  float cloud = cloudAt(xz);
  float rel = slopeRel(N);
  float sunny = cloud * slopeSun(rel);

  // near: the meadow, exactly as the field's own ground paints it far off
  // the farmland, as it is painted (floor.js): fields, hedges, the road
  vec4 fl = floorAt(xz);
  float sx = strokeCoord(vWorld);
  float sw = fwidth(sx);
  float sy = strokeCoord2(xz);
  float syw = fwidth(sy);
  vec2 hq = heroAngle(vWorld);
  vec2 hw = fwidth(hq);
  vec3 g = paintGround(mix(meadowGreen(0.6 + (rel - 0.5) * 0.4), fl.rgb, fl.a), xz, dist, 0.0);
  g = heroPaint(landStrokes(landStrokes(g, xz, dist, sx, sw), xz, dist, sy, syw), vWorld, hq, hw, dist);
  vec3 near = g * meadowLight(sunny);

  vec4 a = texture(tNoiseHi, xz / 2300.0);
  vec4 b = texture(tNoiseHi, xz / 810.0 + 0.31);
  vec4 c = texture(tNoiseHi, xz / 230.0 + 0.57);
  vec4 e = texture(tNoiseHi, xz / 47.0 + 0.13);
  vec4 far4 = texture(tFar, xz * uFarInv + 0.5);

  // meadow: the same greens in broad soft patches, yellower on the rises, with a brush's streaks in it
  vec3 col = meadowGreen(0.5 + 0.2 * smoothstep(0.3, 0.7, a.g) + (rel - 0.5) * 0.4);

  col = paintGround(mix(col, fl.rgb, fl.a), xz, dist, 0.0);
  col = heroPaint(landStrokes(landStrokes(col, xz, dist, sx, sw), xz, dist, sy, syw), vWorld, hq, hw, dist);

  // forest floor: dark under the trees that stand on it, its edge ragged
  float forest = smoothstep(0.38, 0.62, far4.r + (c.r - 0.5) * 0.34 + (e.g - 0.5) * 0.2);
  col = mix(col, mix(vec3(0.03, 0.14, 0.07), vec3(0.06, 0.2, 0.08), e.b), forest);

  float sun = max(dot(N, uSunDir), 0.0);
  // (green land seen from far off is lit a little from every side)
  vec3 far = col * meadowLight(sunny);

  vec3 outCol = mix(near, far, smoothstep(650.0, 1400.0, dist));
  outC = vec4(applyFog(outCol, vWorld), mix(1.0, 0.4, smoothstep(520.0, 700.0, dist)));
}
`;

/** The far land's map, baked once: forest in red, fields in green, over the square the plain covers. */
function farMap(size = 1024) {
  const data = new Uint8Array(size * size * 4);
  const side = REACH * 2;
  for (let j = 0; j < size; j++) {
    for (let i = 0; i < size; i++) {
      const x = ((i + 0.5) / size - 0.5) * side;
      const z = ((j + 0.5) / size - 0.5) * side;
      const k = (i + j * size) * 4;
      data[k] = 255 * forestAt(x, z);
      data[k + 1] = 255 * fieldsAt(x, z);
      data[k + 3] = 255;
    }
  }
  const texture = new THREE.DataTexture(data, size, size, THREE.RGBAFormat, THREE.UnsignedByteType);
  texture.minFilter = texture.magFilter = THREE.LinearFilter;
  texture.needsUpdate = true;
  return texture;
}

/** @param inner half the side of the square the field's ground covers; the land starts just inside it */
export function createHorizon(field, inner = 512) {
  const around = 1440;
  const rings = 120;
  const edge = inner - 6;
  const reach = REACH / edge;
  const positions = new Float32Array(around * (rings + 1) * 3);
  let p = 0;
  for (let k = 0; k <= rings; k++) {
    const scale = Math.pow(reach, k / rings);
    for (let i = 0; i < around; i++) {
      // a point on the square's edge, pushed outwards
      const t = (i / around) * 4;
      const side = Math.floor(t);
      const u = (t - side) * 2 - 1;
      let x = side === 0 ? u : side === 1 ? 1 : side === 2 ? -u : -1;
      let z = side === 0 ? -1 : side === 1 ? u : side === 2 ? 1 : -u;
      // further out the square rounds off into a circle, so the plain ends as far off whichever way you look
      const round = Math.min(1, k / (rings * 0.4));
      const len = Math.hypot(x, z);
      x *= edge * scale * (1 + (1 / len - 1) * round);
      z *= edge * scale * (1 + (1 / len - 1) * round);
      positions[p++] = x;
      // (over its last two kilometres it settles to level ground, so its edge is no hill standing against the
      // backdrop, and never a wall)
      const out = Math.min(1, Math.max(0, (Math.hypot(x, z) - 4000) / 2000));
      positions[p++] = heightAt(x, z) * (1 - 0.9 * out * out * (3 - 2 * out));
      positions[p++] = z;
    }
  }
  const indices = new Uint32Array(rings * around * 6);
  let q = 0;
  for (let k = 0; k < rings; k++) {
    for (let i = 0; i < around; i++) {
      const a = k * around + i;
      const b = k * around + ((i + 1) % around);
      indices[q++] = a;
      indices[q++] = a + around;
      indices[q++] = b;
      indices[q++] = b;
      indices[q++] = a + around;
      indices[q++] = b + around;
    }
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.setIndex(new THREE.BufferAttribute(indices, 1));
  geometry.computeVertexNormals();
  const far = farMap();
  const uniforms = { ...field.uniforms, tFar: { value: far }, uFarInv: { value: 1 / (REACH * 2) }, uAlong: { value: new THREE.Vector2(HERO[0], HERO[1]) } };
  const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: vertex, fragmentShader: fragment, uniforms, side: THREE.DoubleSide, polygonOffset: true, polygonOffsetFactor: 2, polygonOffsetUnits: 12 });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.frustumCulled = false;
  return mesh;
}
