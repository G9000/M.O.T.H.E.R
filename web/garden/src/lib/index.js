// Grass you can touch: a field of grass for three.js (WebGL2) that bends under what crosses it, can be cut and
// burnt, and remembers all of it. createField() makes the field; createFrame() (frame.js) draws a scene with it,
// its flames and smoke.
//
//   const field = createField({ heightAt: (x, z) => 0, textures: await loadGroundTextures(renderer) });
//   scene.add(field.group);
//   field.bend(x, z, 0.3, dx, dz);  field.cutLine(ax, az, bx, bz, 0.2, 0.07);  field.ignite(x, z, 0.4);
//   field.stateAt(x, z);            // { burning, heat, burnt, cut, flat, fuel, ... }
//   field.update(dt, camera);       // every frame
import * as THREE from 'three';
import { createBlades } from './blades.js';
import { createClippings } from './clippings.js';
import { createFlowers } from './flowers.js';
import { createGround, flatTextures } from './ground.js';
import { mulberry, smoothNoiseTexture, whiteNoiseTexture } from './noise.js';
import { FieldState } from './state.js';
import { bakeRows } from './bake.js';

export { loadGroundTextures } from './ground.js';
export { createFrame } from './frame.js';
export { bakeRows } from './bake.js';
export { COMMON, LEAF_LIGHT, FIRE_RAMP } from './glsl.js';

const clamp01 = (v) => Math.min(1, Math.max(0, v));

/**
 * Reads heightAt and paint into the field's maps a slice at a time, giving the browser a turn between slices, so a
 * million calls of each don't stop the page. Pass what it resolves to createField as `baked`, with the same options.
 * @param onProgress called with 0 to 1 as it goes
 */
export async function bakeGround({ heightAt = () => 0, paint = () => ({}), extent = 1024, extentRes = 1024, centre = [0, 0] } = {}, onProgress) {
  const o = { heightAt, paint, extentMin: [centre[0] - extent / 2, centre[1] - extent / 2], texel: extent / extentRes, extentRes };
  const heights = new Float32Array(extentRes * extentRes);
  const map = new Uint8Array(extentRes * extentRes * 4);
  let from = 0;
  while (from < extentRes) {
    const until = performance.now() + 24;
    while (from < extentRes && performance.now() < until) {
      bakeRows(o, heights, map, from, Math.min(extentRes, from + 4));
      from += 4;
    }
    onProgress?.(Math.min(1, from / extentRes));
    await new Promise((r) => {
      // (a message, not a timer: timers wait at least 4 ms each once they nest)
      const c = new MessageChannel();
      c.port1.onmessage = () => r();
      c.port2.postMessage(0);
    });
  }
  return { heights, map };
}

/**
 * @param o.heightAt (x, z) => metres: your ground. Read once, into a map `extent` metres across.
 * @param o.paint (x, z) => { dry, growth, wet, flowers }, each 0 to 1: where grass is straw or green, where it
 *   grows at all (0 is bare ground), where the ground is too wet to burn, where flowers grow. Read once.
 * @param o.extent side of the square of ground the field knows, metres; o.extentRes its map's texels
 * @param o.size side of the square inside it that reacts and remembers (0.125 m a texel); o.centre its middle
 * @param o.textures from loadGroundTextures(), or leave out for plain soil
 * @param o.height metres, a tall blade. o.density 0 to 1, how many blades are drawn.
 * @param o.recover seconds for flattened grass to stand halfway up (0 never); o.regrow seconds for cut grass to grow back (0 never)
 * @param o.baked what bakeGround made of the same heightAt, paint, extent, extentRes and centre, so making the field doesn't stop the page
 */
export function createField({
  heightAt = () => 0,
  paint = () => ({ dry: 0.3, growth: 1, wet: 0, flowers: 0.3 }),
  extent = 1024,
  extentRes = 1024,
  size = 256,
  centre = [0, 0],
  textures = flatTextures(),
  height = 0.52,
  density = 1,
  seed = 1,
  recover = 70,
  regrow = 0,
  flowers = true,
  baked = null,
} = {}) {
  const extentMin = [centre[0] - extent / 2, centre[1] - extent / 2];
  const texel = extent / extentRes;

  // the ground and what grows where, baked once (or already, by bakeGround)
  const heights = baked?.heights ?? new Float32Array(extentRes * extentRes);
  const map = baked?.map ?? new Uint8Array(extentRes * extentRes * 4);
  if (!baked) bakeRows({ heightAt, paint, extentMin, texel, extentRes }, heights, map, 0, extentRes);
  const tHeight = new THREE.DataTexture(heights, extentRes, extentRes, THREE.RedFormat, THREE.FloatType);
  tHeight.minFilter = tHeight.magFilter = THREE.NearestFilter;
  tHeight.needsUpdate = true;
  const tMap = new THREE.DataTexture(map, extentRes, extentRes, THREE.RGBAFormat, THREE.UnsignedByteType);
  tMap.minFilter = tMap.magFilter = THREE.LinearFilter;
  tMap.wrapS = tMap.wrapT = THREE.ClampToEdgeWrapping;
  tMap.needsUpdate = true;

  /** Bilinear between texel centres, the way the shaders read the same map. */
  const sampler = (data, stride, offset, scale) => (x, z) => {
    const px = Math.min(extentRes - 1, Math.max(0, (x - extentMin[0]) / texel - 0.5));
    const pz = Math.min(extentRes - 1, Math.max(0, (z - extentMin[1]) / texel - 0.5));
    const i = Math.min(extentRes - 2, Math.floor(px));
    const j = Math.min(extentRes - 2, Math.floor(pz));
    const fx = px - i;
    const fz = pz - j;
    const a = (i + j * extentRes) * stride + offset;
    const b = a + extentRes * stride;
    return ((data[a] * (1 - fx) + data[a + stride] * fx) * (1 - fz) + (data[b] * (1 - fx) + data[b + stride] * fx) * fz) * scale;
  };
  const groundAt = sampler(heights, 1, 0, 1);
  const dryAt = sampler(map, 4, 0, 1 / 255);
  const growthAt = sampler(map, 4, 1, 1 / 255);
  const wetAt = sampler(map, 4, 2, 1 / 255);

  const state = new FieldState({ size, centre, heightAt: groundAt, growthAt, dryAt, wetAt, recover, regrow });

  // (a new Vector4 has w = 1, which here would be a pusher a metre across at the origin)
  const pushers = Array.from({ length: 6 }, () => new THREE.Vector4(0, 0, 0, 0));
  const shared = {
    uTime: { value: 0 },
    uWind: { value: new THREE.Vector4(1, 0, 3, 1) },
    uWindScroll: { value: new THREE.Vector2() },
    uExtentMin: { value: new THREE.Vector2(...extentMin) },
    uExtentInv: { value: 1 / extent },
    uHeightRes: { value: extentRes },
    uStateMin: { value: new THREE.Vector2(state.minX, state.minZ) },
    uStateInv: { value: 1 / size },
    tHeight: { value: tHeight },
    tMap: { value: tMap },
    tState: { value: state.tex },
    tFire: { value: state.ftex },
    tGlow: { value: state.gtex },
    tNoise: { value: smoothNoiseTexture(256, seed * 13 + 1) },
    tNoiseHi: { value: smoothNoiseTexture(256, seed * 13 + 1, true) },
    tWhite: { value: whiteNoiseTexture(256, seed * 7 + 2) },
    uSunDir: { value: new THREE.Vector3(0.6, 0.25, 0.3).normalize() },
    uSunColor: { value: new THREE.Color(3.2, 2.5, 1.6) },
    uSkyColor: { value: new THREE.Color(0.34, 0.42, 0.6) },
    uBounceColor: { value: new THREE.Color(0.1, 0.09, 0.05) },
    uFogColor: { value: new THREE.Color(0.55, 0.6, 0.7) },
    uFogSunColor: { value: new THREE.Color(1.0, 0.75, 0.45) },
    uFogDensity: { value: 0.0016 },
    uFireColor: { value: new THREE.Color(0, 0, 0) },
    uCamPos: { value: new THREE.Vector3() },
    uPushers: { value: pushers },
    uCloudShade: { value: 0.55 },
    // the shade of what stands on the field (trees): 1 where it falls, over the same ground as tMap
    tShade: { value: new THREE.DataTexture(new Uint8Array([0, 0, 0, 255]), 1, 1) },
    // farmland painted from above (the valley's floor): its picture, the way its square lies (along x, z; its
    // corner), and 1 / its side
    tFloor: { value: new THREE.DataTexture(new Uint8Array([0, 0, 0, 0]), 1, 1) },
    // brushwork (strokes.js): tone in red, stones in blue
    tStrokes: { value: new THREE.DataTexture(new Uint8Array([128, 0, 0, 255]), 1, 1) },
    // the ground's slope east and south, over the same ground as tMap: half is level, and all of it is a rise of 1 in 4
    tSlope: { value: new THREE.DataTexture(new Uint8Array([128, 128]), 1, 1, THREE.RGFormat) },
    // the eye the far meadow is painted for: where the walk starts
    uHeroEye: { value: new THREE.Vector3(0, 60, 0) },
    uFloor: { value: new THREE.Vector4(1, 0, 0, 0) },
    uFloorInv: { value: 0 },
    uLamp: { value: new THREE.Vector4(0, 0, 0, 0) },
    uWalker: { value: new THREE.Vector4(0, 0, 0, 0) },
  };

  const group = new THREE.Group();
  const ground = createGround(shared, textures, { height, extentMin, texel });
  const blades = createBlades(shared, { height, density, seed }, groundAt);
  blades.init();
  group.add(ground.mesh, blades.group);
  const random = mulberry(seed * 31 + 9);
  const clippings = createClippings(shared, state, groundAt, random);
  group.add(clippings.mesh);
  const flowerBed = flowers ? createFlowers(shared, { height, seed }, groundAt) : null;
  if (flowerBed) group.add(flowerBed.group);

  const wind = { angle: 0, speed: 3, gust: 1 };
  const field = {
    group,
    uniforms: shared,
    state,
    blades,
    ground,
    clippings,
    flowers: flowerBed,
    size,
    extent,
    wind,
    /** Set by createFrame, or by you: with the renderer, the field sends its maps to the GPU faster. */
    renderer: null,
    /** The ground's height at (x, z), exactly where the grass stands on it. */
    heightAt: groundAt,
    growthAt,
    dryAt,
    wetAt,
    bend: (x, z, radius, dx, dz, amount) => state.bend(x, z, radius, dx, dz, amount),
    /**
     * Cut along a line. (vx, vz) is how fast the edge was moving, which throws the clippings. With `lay` the cut
     * stems are carried to a row and dropped there instead, as a scythe lays a windrow: { x, z, dx, dz, along,
     * width }, a point on the row, the way it runs, how far along it and how far to either side they fall.
     */
    cutLine(ax, az, bx, bz, width, h0, h1, vx = 0, vz = 0, lay = null) {
      const n = state.cutLine(ax, az, bx, bz, width, h0, h1);
      if (n) clippings.take(vx, vz, lay);
      else state.cutEvents.length = 0;
      return n;
    },
    cut(x, z, radius = 0.3, h = 0.07) {
      return this.cutLine(x, z, x, z, radius * 2, h, h);
    },
    ignite: (x, z, radius) => state.ignite(x, z, radius),
    stateAt: (x, z) => state.stateAt(x, z),
    /** Wind from a compass angle in radians (the direction it blows towards, round Y) at `speed` m/s. */
    setWind(angle, speed, gust = 1) {
      wind.angle = angle;
      wind.speed = speed;
      wind.gust = gust;
      shared.uWind.value.set(Math.cos(angle), Math.sin(angle), speed, gust);
      state.setWind(Math.cos(angle), Math.sin(angle), speed);
    },
    /** Something standing in the grass that the blades part round: slot 0 to 5, its centre and radius. radius 0 clears it. */
    setPusher(slot, x, y, z, radius) {
      pushers[slot].set(x, y, z, radius);
    },
    /** The light on the field. Any of: sunDir, sunColor, skyColor, bounceColor, fogColor, fogSunColor, fogDensity, cloudShade. */
    setLight(l) {
      if (l.sunDir) shared.uSunDir.value.copy(l.sunDir).normalize();
      if (l.sunColor) shared.uSunColor.value.copy(l.sunColor);
      if (l.skyColor) shared.uSkyColor.value.copy(l.skyColor);
      if (l.bounceColor) shared.uBounceColor.value.copy(l.bounceColor);
      if (l.fogColor) shared.uFogColor.value.copy(l.fogColor);
      if (l.fogSunColor) shared.uFogSunColor.value.copy(l.fogSunColor);
      if (l.fogDensity !== undefined) shared.uFogDensity.value = l.fogDensity;
      if (l.cloudShade !== undefined) shared.uCloudShade.value = l.cloudShade;
    },
    /** A flame carried through the field lights the grass round it: where it is and how bright (about 1; 0 for none). */
    setLamp(x, y, z, strength) {
      shared.uLamp.value.set(x, y, z, strength);
    },
    setDensity: (d) => blades.setDensity(d),
    reset() {
      state.reset();
      clippings.clear();
    },
    update(dt, camera) {
      shared.uTime.value += dt;
      const t = shared.uTime.value;
      // the gust field rides the wind; in still air it barely drifts
      const drift = 0.35 + wind.speed * 0.55;
      shared.uWindScroll.value.x += Math.cos(wind.angle) * drift * dt;
      shared.uWindScroll.value.y += Math.sin(wind.angle) * drift * dt;
      camera.getWorldPosition(shared.uCamPos.value);
      const { x, z } = shared.uCamPos.value;
      blades.follow(x, z);
      ground.follow(x, z);
      if (flowerBed) flowerBed.follow(x, z);
      state.update(dt, field.renderer);
      clippings.update(dt);
      const flick = 1 + 0.1 * Math.sin(t * 9.1) + 0.07 * Math.sin(t * 23.3 + 1) + 0.05 * Math.sin(t * 41.7 + 2);
      shared.uFireColor.value.setRGB(3.2 * flick, 1.15 * flick, 0.28 * flick);
    },
  };
  field.setWind(0.6, 3);
  return field;
}
