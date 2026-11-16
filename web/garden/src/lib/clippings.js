// What comes off when grass is cut. Thrown by a blade, the tops fly, tumble, are slowed by the air and carried a
// little by the wind. Laid by a scythe, the stems are carried to the end of the stroke and dropped there, where they
// come to rest on the ground among the stubble and on each other: a low loose heap, thick in the middle and thinning
// out to nothing at its edges. Either way a piece is as green as the grass it was, and pales slowly as it lies.
import * as THREE from 'three';
import { COMMON, LEAF_LIGHT } from './glsl.js';

const MAX = 49152;
const LIE = 30; // seconds a thrown piece lies before it goes
const LIE_LAID = 300; // and a laid one

const vertex = /* glsl */ `
${COMMON}
in vec3 aPos;
in vec3 aDir;
in vec3 aSide;
in vec4 aColor;   // its colour when cut, and when that was
out vec3 vWorld;
out vec3 vNormal;
out vec3 vAlbedo;
out vec3 vLight;
void main() {
  float t = position.y * 0.5 + 0.5;
  float taper = 1.0 - 0.8 * t * t;
  vec3 world = aPos + aDir * position.y + aSide * position.x * taper;
  vWorld = world;
  vNormal = cross(aSide, aDir);
  // green when cut; hay by the time it has lain a few minutes
  float lain = smoothstep(20.0, 240.0, uTime - aColor.a);
  float lum = dot(aColor.rgb, vec3(0.3, 0.6, 0.1));
  vAlbedo = mix(aColor.rgb, vec3(0.62, 0.56, 0.30) * (0.35 + 1.6 * lum), lain * 0.75);
  // deep in the heap or down in the stubble it is in shade; on top it is in the light
  float up = clamp(0.3 + (world.y - groundAt(world.xz)) / 0.22, 0.0, 0.8);
  vLight = vec3(cloudAt(world.xz), texture(tGlow, stateUv(world.xz)).g + lampAt(world), up);
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
  // a cut stem is curled and creased, not a flat plate: some of it always faces the light
  N = normalize(N + uSunDir * 0.28 + vec3(0.0, 0.2, 0.0));
  vec3 c = leafLight(vAlbedo, N, V, vLight.z, 0.05, vLight.x, vLight.y);
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;

/** The same colours as grassColor in glsl.js, for the pieces. */
function grassColor(dry, s, out) {
  const mix = (a, b, t) => a + (b - a) * t;
  out[0] = mix(mix(0.02, 0.118, s), mix(0.095, 0.39, s), dry);
  out[1] = mix(mix(0.042, 0.19, s), mix(0.076, 0.325, s), dry);
  out[2] = mix(mix(0.011, 0.038, s), mix(0.034, 0.17, s), dry);
  return out;
}

export function createClippings(shared, state, heightAt, random) {
  const geometry = new THREE.InstancedBufferGeometry();
  geometry.setAttribute('position', new THREE.Float32BufferAttribute([-1, -1, 0, 1, -1, 0, -1, 1, 0, 1, 1, 0], 3));
  geometry.setIndex([0, 1, 3, 0, 3, 2]);
  const aPos = new THREE.InstancedBufferAttribute(new Float32Array(MAX * 3), 3);
  const aDir = new THREE.InstancedBufferAttribute(new Float32Array(MAX * 3), 3);
  const aSide = new THREE.InstancedBufferAttribute(new Float32Array(MAX * 3), 3);
  const aColor = new THREE.InstancedBufferAttribute(new Float32Array(MAX * 4), 4);
  for (const a of [aPos, aDir, aSide]) a.setUsage(THREE.DynamicDrawUsage);
  geometry.setAttribute('aPos', aPos);
  geometry.setAttribute('aDir', aDir);
  geometry.setAttribute('aSide', aSide);
  geometry.setAttribute('aColor', aColor);
  geometry.instanceCount = 0;
  const material = new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: vertex, fragmentShader: fragment, uniforms: shared, side: THREE.DoubleSide });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.frustumCulled = false;

  // per piece: velocity, the axis it spins about and how fast, its length and width, how long it has lain
  const vel = new Float32Array(MAX * 3);
  const axis = new Float32Array(MAX * 3);
  const spin = new Float32Array(MAX);
  const len = new Float32Array(MAX);
  const wid = new Float32Array(MAX);
  const age = new Float32Array(MAX);
  const life = new Float32Array(MAX);
  const mode = new Uint8Array(MAX); // 0 free, 1 flying, 2 lying
  // laid in a row: how near the middle of the heap it lands (0 at its edge, 1 in the middle), and which way the row runs
  const pile = new Float32Array(MAX).fill(-1);
  const rowYaw = new Float32Array(MAX);
  let next = 0;
  let high = 0;
  let moving = 0;
  let lying = 0;
  const colour = [0, 0, 0];

  function rotate(arr, i, ax, ay, az, angle) {
    // Rodrigues: turn the vector at arr[i..i+2] about the unit axis by angle
    const c = Math.cos(angle);
    const s = Math.sin(angle);
    const x = arr[i];
    const y = arr[i + 1];
    const z = arr[i + 2];
    const d = (ax * x + ay * y + az * z) * (1 - c);
    arr[i] = x * c + (ay * z - az * y) * s + ax * d;
    arr[i + 1] = y * c + (az * x - ax * z) * s + ay * d;
    arr[i + 2] = z * c + (ax * y - ay * x) * s + az * d;
  }

  function spawn(x, z, was, to, vx, vz, lay) {
    const k = next;
    next = (next + 1) % MAX;
    if (k >= high) high = k + 1;
    if (mode[k] === 1) moving--;
    if (mode[k] === 2) lying--;
    mode[k] = 1;
    moving++;
    age[k] = 0;
    life[k] = lay ? LIE_LAID : LIE;
    // what stood above the cut: the map says "uncut" for grass that was never touched, which is about this tall
    const stood = Math.min(was, 0.26 + 0.3 * random());
    // laid by a scythe the stems stay nearly whole; thrown by a blade they come off in short tops
    const piece = lay ? Math.max(0.14, Math.min(0.46, (stood - to) * (0.7 + 0.5 * random()))) : Math.max(0.04, Math.min(0.2, (stood - to) * (0.35 + 0.5 * random())));
    len[k] = piece / 2;
    wid[k] = lay ? 0.0045 + 0.0045 * random() : 0.0028 + 0.0028 * random();
    const o = k * 3;
    aPos.array[o] = x + (random() - 0.5) * 0.12;
    aPos.array[o + 1] = heightAt(x, z) + to + piece * (0.3 + random());
    aPos.array[o + 2] = z + (random() - 0.5) * 0.12;
    if (lay) {
      // each stem to its own place in the row: anywhere along it, most of them near its middle line
      const g = (random() + random() + random() - 1.5) / 0.5;
      const s = (random() * 2 - 1) * lay.along;
      const tx = lay.x + lay.dx * s - lay.dz * g * lay.width;
      const tz = lay.z + lay.dz * s + lay.dx * g * lay.width;
      // the ends of the row thin out as its sides do
      pile[k] = Math.exp(-0.5 * g * g) * Math.min(1, (1 - Math.abs(s) / lay.along) * 3 + 0.25);
      rowYaw[k] = Math.atan2(lay.dz, lay.dx);
      // as fast as the blade that pushes it, so it runs ahead of the edge and never trails over the blade
      const T = Math.min(0.34, Math.max(0.09, Math.hypot(tx - aPos.array[o], tz - aPos.array[o + 2]) / 13)) * (0.8 + 0.4 * random());
      vel[o] = (tx - aPos.array[o]) / T;
      vel[o + 1] = (heightAt(tx, tz) + 0.05 - aPos.array[o + 1] + 4.9 * T * T) / T;
      vel[o + 2] = (tz - aPos.array[o + 2]) / T;
    } else {
      pile[k] = -1;
      const throwIt = 0.25 + 0.5 * random();
      vel[o] = vx * throwIt + (random() - 0.5) * 1.6;
      vel[o + 1] = 1.0 + 2.6 * random() + 0.12 * Math.hypot(vx, vz);
      vel[o + 2] = vz * throwIt + (random() - 0.5) * 1.6;
    }
    // starts upright, as it grew, and tumbles about some sideways axis
    const a = random() * Math.PI * 2;
    aDir.array[o] = 0.25 * Math.cos(a) * len[k];
    aDir.array[o + 1] = 0.97 * len[k];
    aDir.array[o + 2] = 0.25 * Math.sin(a) * len[k];
    aSide.array[o] = -Math.sin(a) * wid[k];
    aSide.array[o + 1] = 0;
    aSide.array[o + 2] = Math.cos(a) * wid[k];
    const b = random() * Math.PI * 2;
    const tilt = (random() - 0.5) * 0.9;
    axis[o] = Math.cos(b) * Math.cos(tilt);
    axis[o + 1] = Math.sin(tilt);
    axis[o + 2] = Math.sin(b) * Math.cos(tilt);
    spin[k] = (lay ? 3 + 7 * random() : 4 + 12 * random()) * (random() < 0.5 ? -1 : 1);
    // the colour of the grass it was: a whole stem runs from its dim foot to its bright top, so it is the middle of that
    const f = state.stateAt(x, z);
    grassColor(Math.min(1, Math.max(0, ((f.dry ?? 0.3) - 0.3) / 0.4 + (random() - 0.5) * 0.4)), lay ? 0.4 + 0.4 * random() : 0.55 + 0.4 * random(), colour);
    const shade = 0.88 + 0.26 * random();
    aColor.array[k * 4] = colour[0] * shade;
    aColor.array[k * 4 + 1] = colour[1] * shade;
    aColor.array[k * 4 + 2] = colour[2] * shade;
    aColor.array[k * 4 + 3] = shared.uTime.value;
    aColor.needsUpdate = true;
  }

  function hide(k) {
    const o = k * 3;
    aDir.array[o] = aDir.array[o + 1] = aDir.array[o + 2] = 0;
    aSide.array[o] = aSide.array[o + 1] = aSide.array[o + 2] = 0;
    if (mode[k] === 1) moving--;
    if (mode[k] === 2) lying--;
    mode[k] = 0;
  }

  /** Sets piece k down where it is: lying along `yaw`, both its ends on the ground or on the heap under them. */
  function rest(k, yaw, lift, pitch) {
    const o = k * 3;
    const l = len[k];
    const cx = Math.cos(yaw);
    const cz = Math.sin(yaw);
    const x = aPos.array[o];
    const z = aPos.array[o + 2];
    const ga = heightAt(x - cx * l, z - cz * l);
    const gb = heightAt(x + cx * l, z + cz * l);
    // a blade has some thickness and never lies quite flat: a centimetre clear of the soil at least
    aPos.array[o + 1] = (ga + gb) / 2 + 0.01 + lift;
    const rise = (gb - ga) / 2 + pitch * l;
    const flat = Math.sqrt(Math.max(l * l - rise * rise, 1e-8));
    aDir.array[o] = cx * flat;
    aDir.array[o + 1] = rise;
    aDir.array[o + 2] = cz * flat;
    // its face up, give or take: rolled a little either way
    const roll = (random() - 0.5) * 1.2;
    aSide.array[o] = -cz * wid[k] * Math.cos(roll);
    aSide.array[o + 1] = wid[k] * Math.sin(roll);
    aSide.array[o + 2] = cx * wid[k] * Math.cos(roll);
  }

  return {
    mesh,
    get flying() {
      return moving;
    },
    /**
     * Makes pieces from what the state says was just cut, thrown along (vx, vz) m/s. With `lay` they are carried,
     * nearly whole, to a row and dropped there, as a scythe leaves a windrow: { x, z } a point on the row's middle
     * line, { dx, dz } the way it runs (unit), `along` how far either side of the point they are strewn, `width`
     * how far from the line most of them land (metres).
     */
    take(vx, vz, lay = null) {
      const ev = state.cutEvents;
      // a wide cut can't have every blade: enough that it reads, spread over all of it
      const each = Math.min(lay ? 3.2 : 3, Math.max(0.35, 900 / Math.max(ev.length / 4, 1)));
      for (let i = 0; i < ev.length; i += 4) {
        let n = Math.floor(each) + (random() < each % 1 ? 1 : 0);
        while (n-- > 0) spawn(ev[i], ev[i + 1], ev[i + 2], ev[i + 3], vx, vz, lay);
      }
      ev.length = 0;
      geometry.instanceCount = high;
    },
    clear() {
      for (let k = 0; k < high; k++) hide(k);
      high = 0;
      next = 0;
      geometry.instanceCount = 0;
      aDir.needsUpdate = aSide.needsUpdate = true;
    },
    update(dt) {
      if (!moving && !lying) return;
      const wind = shared.uWind.value;
      const drag = Math.exp(-2.4 * dt);
      let touched = false;
      for (let k = 0; k < high; k++) {
        const m = mode[k];
        if (!m) continue;
        const o = k * 3;
        if (m === 2) {
          age[k] += dt;
          if (age[k] > life[k]) {
            // shrinks away rather than blinking out
            if (age[k] > life[k] + 2) hide(k);
            else for (let c = 0; c < 3; c++) aDir.array[o + c] *= 1 - Math.min(1, dt * 1.5);
            touched = true;
          }
          continue;
        }
        touched = true;
        age[k] += dt;
        const laid = pile[k] >= 0;
        vel[o + 1] -= 9.8 * dt;
        // light as it is, the air stops it quickly and the wind takes it
        if (!laid) {
          vel[o] = vel[o] * drag + wind.x * wind.z * 0.5 * (1 - drag);
          vel[o + 1] *= drag;
          vel[o + 2] = vel[o + 2] * drag + wind.y * wind.z * 0.5 * (1 - drag);
        }
        const flutter = laid ? 0 : Math.sin(age[k] * 9 + k) * 0.6 * dt;
        aPos.array[o] += vel[o] * dt + axis[o + 2] * flutter;
        aPos.array[o + 1] += vel[o + 1] * dt;
        aPos.array[o + 2] += vel[o + 2] * dt - axis[o] * flutter;
        const angle = spin[k] * dt;
        rotate(aDir.array, o, axis[o], axis[o + 1], axis[o + 2], angle);
        rotate(aSide.array, o, axis[o], axis[o + 1], axis[o + 2], angle);
        const floor = heightAt(aPos.array[o], aPos.array[o + 2]) + 0.03;
        if (aPos.array[o + 1] <= floor && vel[o + 1] < 0) {
          const f = state.stateAt(aPos.array[o], aPos.array[o + 2]);
          if (laid) {
            // a stem in a row lies more or less across it, as the blade swept it, but no two alike; in the middle
            // of the heap it rests on the stems under it, at the edges on the ground between the stubble
            const yaw = rowYaw[k] + Math.PI / 2 + (random() + random() + random() - 1.5) * 1.5;
            const lift = pile[k] * 0.15 * random() ** 1.3;
            rest(k, yaw, lift, (random() - 0.5) * 0.7 * pile[k]);
          } else {
            // lands flat, lying whichever way it was pointing
            rest(k, Math.atan2(aDir.array[o + 2], aDir.array[o]), Math.min(f.cut ?? 1, 0.12) * 0.4 * random(), (random() - 0.5) * 0.3);
          }
          mode[k] = 2;
          moving--;
          lying++;
          age[k] = random() * 6;
          // nothing lies on a fire
          if (f.burning || f.burnt > 0.5) hide(k);
        }
      }
      if (touched) aPos.needsUpdate = aDir.needsUpdate = aSide.needsUpdate = true;
    },
  };
}
