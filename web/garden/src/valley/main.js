// A walk through the memory garden, in first person: this file is the walker, the camera and the page. The grass
// comes from src/lib; the land, the sky, the clouds and the far country from the files beside this one; the
// records and what is said about them from ../garden/garden.js. (The page was written as a Patch block's demo and
// still reports its progress through patch-block, which here goes nowhere.)
import { patchBlock } from 'patch-block';
import * as THREE from 'three';
import { createGarden, fetchSnapshot, loadMedia, loadSnapshot, pathLengthFor } from '../garden/garden.js';
import { bakeGround, createField, createFrame, loadGroundTextures } from '../lib/index.js';
import { createFluff, createStones } from './dressing.js';
import { createFloor, FLOOR, paintFloor } from './floor.js';
import { createHorizon } from './horizon.js';
import { createSky, createSkyParts, SUNSET } from './sky.js';
import { createSounds } from './sounds.js';
import { createStrokes } from './strokes.js';
import { createTrees } from './trees.js';
import { createTarn } from './water.js';
import { configure, fromValley, heightAt, HERO, paint, START, tarnDistance } from './world.js';

const params = new URLSearchParams(location.search);
// how long each part of getting ready took, ms: for tests
const timings = {};
let lap = performance.now();
const mark = (name) => {
  const now = performance.now();
  timings[name] = Math.round(now - lap);
  lap = now;
};
const stepped = params.has('step'); // a test drives the clock: nothing moves until it says
const $ = (id) => document.getElementById(id);

let loaded = false;
let shown = true;
const block = patchBlock({
  onVisible(visible) {
    shown = visible;
    if (loaded) run();
  },
});
/** Says how far along the loading is, and gives the browser a turn so the word gets out and the page stays alive. */
const stage = async (value, label) => {
  block.progress(value, label);
  $('loading').firstElementChild.textContent = label;
  await new Promise((done) => setTimeout(done));
};
/**
 * One of the slow sums done on another thread (see bake.worker.js), so the page stays alive and the graphics card
 * can be given its own work meanwhile. Where a worker can't be had, `here` does it on this one.
 */
const offThread = (message, here) =>
  new Promise((done) => {
    let worker = null;
    let settled = false;
    const fallback = () => {
      if (settled) return;
      settled = true;
      worker?.terminate();
      Promise.resolve(here()).then(done);
    };
    try {
      worker = new Worker(new URL('./bake.worker.js', import.meta.url), { type: 'module' });
      worker.onmessage = (e) => {
        if (settled) return;
        settled = true;
        worker.terminate();
        done(e.data);
      };
      worker.onerror = fallback;
      worker.postMessage(message);
    } catch {
      fallback();
    }
  });
// The owner's records come first: the path through the garden is as long as they need, and the land is made round it.
let snapshot = (await loadSnapshot()) ?? (import.meta.env.DEV ? (await import('../garden/demo.js')).demoSnapshot() : null);
if (!snapshot) {
  $('loading').firstElementChild.textContent = 'This garden link has already been used, or the garden was closed. Run the command again for a new link.';
  throw new Error('no snapshot');
}
const needed = pathLengthFor(snapshot);
// Leave room for new flowers so ordinary updates don't have to repaint the terrain.
const WORLD = { length: snapshot.liveRevision ? Math.ceil((needed + 16) / 32) * 32 : needed };
configure(WORLD);
const quality = params.get('quality') ?? 'high';
const EXTENT = 1280;
const GROUND = { extent: EXTENT, extentRes: EXTENT, centre: [0, 0] };
const FLOOR_SIZE = quality === 'low' ? 1024 : 1536;
// the garden is seen in the low gold light of a late summer evening; T and 1 to 4 change it
const hour0 = params.has('hour') ? Number(params.get('hour')) : 19.2;
const groundJob = offThread({ job: 'ground', world: WORLD, ...GROUND }, () => bakeGround({ heightAt, paint, ...GROUND }));
const floorJob = offThread({ job: 'floor', world: WORLD, size: FLOOR_SIZE }, () => ({ pixels: paintFloor(FLOOR_SIZE) }));
await stage(0.04, 'Painting the sky');
// Sound is made ready now, while the land is being made: opening the PC's audio can hold a page's thread for a
// second or more, and here nothing is waiting on it. It only starts playing on the first click.
const sounds = createSounds();
const soundsReady = params.has('nosound') ? null : sounds.load();

const canvas = $('view');
let renderer;
try {
  renderer = new THREE.WebGLRenderer({ canvas, antialias: false, powerPreference: 'high-performance', preserveDrawingBuffer: stepped });
} catch {
  $('loading').firstElementChild.textContent = 'The garden needs WebGL 2, which this browser does not offer.';
  throw new Error('no WebGL 2');
}
renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, params.has('dpr') ? Number(params.get('dpr')) : 1.5));
renderer.setSize(window.innerWidth, window.innerHeight);

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(68, window.innerWidth / window.innerHeight, 0.12, 40000);
camera.rotation.order = 'YXZ';
// Listened for from the start, not once loaded: the page may go fullscreen while the valley is still being made.
// The frame's targets follow the renderer's size on their own, each time it draws.
function fitView() {
  renderer.setSize(window.innerWidth, window.innerHeight);
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
}
window.addEventListener('resize', fitView);

const groundTextures = loadGroundTextures(renderer, './textures/');
// The sky's clouds and mountains need nothing from the land and are the graphics card's work, most of the wait on
// a fast PC: begun now, they are painted while the land is still being worked out.
const skyParts = createSkyParts(renderer, {
  cloudSize: quality === 'low' ? 512 : quality === 'medium' ? 1024 : 2048,
  cloudSteps: quality === 'low' ? 150 : 240,
  liveClouds: quality !== 'low',
  backdropWidth: quality === 'high' ? 12288 : 4096,
});
const skyReady = (async () => {
  const from = performance.now();
  const lapse = (name) => {
    timings[name] = Math.round(performance.now() - from);
  };
  await skyParts.clouds.compile();
  await skyParts.backdrop.compile();
  lapse('skyShadersAt');
  skyParts.backdrop.paint();
  await gpuDone();
  lapse('mountainsAt');
  skyParts.light(hour0);
  await gpuDone();
  lapse('cloudsAt');
})();
mark('skyBegun');
await stage(0.12, 'Laying out the valley');
const baked = await groundJob;
mark('groundWait');
const field = createField({
  heightAt,
  paint,
  ...GROUND,
  // (the part that remembers your footsteps lies along the path down)
  size: 256,
  baked,
  textures: await groundTextures,
  density: quality === 'low' ? 0.45 : quality === 'medium' ? 0.7 : 1,
  height: 0.5,
  seed: 3,
});
scene.add(field.group);
mark('field');
field.uniforms.tFloor.value = createFloor(renderer, FLOOR_SIZE, (await floorJob).pixels);
mark('floorPaint');
field.uniforms.uFloor.value.set(HERO[0], HERO[1], FLOOR.u0, FLOOR.v0);
field.uniforms.uFloorInv.value = 1 / FLOOR.size;
field.uniforms.tStrokes.value = createStrokes(renderer);
mark('strokes');
{
  // the ground's slope, for the light on the meadow: taken over a few metres, so it is the lie of the land and
  // not every bump in it (a hummock lit on its own is a blotch)
  const RES = 512;
  const data = new Uint8Array(RES * RES * 2);
  const step = EXTENT / RES;
  const byte = (slope) => Math.round(255 * Math.min(1, Math.max(0, 0.5 + slope * 2)));
  for (let j = 0, k = 0; j < RES; j++) {
    const z = -EXTENT / 2 + (j + 0.5) * step;
    for (let i = 0; i < RES; i++) {
      const x = -EXTENT / 2 + (i + 0.5) * step;
      data[k++] = byte((field.heightAt(x + 24, z) - field.heightAt(x - 24, z)) / 48);
      data[k++] = byte((field.heightAt(x, z + 24) - field.heightAt(x, z - 24)) / 48);
    }
  }
  const slope = new THREE.DataTexture(data, RES, RES, THREE.RGFormat, THREE.UnsignedByteType);
  slope.minFilter = slope.magFilter = THREE.LinearFilter;
  slope.unpackAlignment = 1;
  slope.needsUpdate = true;
  field.uniforms.tSlope.value = slope;
  field.uniforms.uHeroEye.value.set(START[0], field.heightAt(START[0], START[1]) + 1.68, START[1]);
}
mark('floor');
await stage(0.3, 'Building the village');
const frame = createFrame(renderer, field, { samples: quality === 'low' ? 0 : 4, bloom: 0.1 });
mark('frame');
const sky = createSky(renderer, field, { parts: skyParts });
mark('sky');
const trees = createTrees(field, { cards: quality === 'low' ? 0.6 : quality === 'medium' ? 0.8 : 1 });
mark('trees');
scene.add(sky.mesh, createHorizon(field, EXTENT / 2), trees.group, createTarn(field, sky));
// the finish: the frame laid in as brush marks. ?paint=0 for none; P turns it on and off.
frame.paint = params.has('paint') ? Number(params.get('paint')) : quality === 'low' ? 0 : quality === 'medium' ? 2 : 3;
const paintWas = frame.paint || 3;
mark('build');
await stage(0.45, 'Planting trees');
const stones = createStones(field);
scene.add(stones.group);
if (quality !== 'low' && params.get('seeds') !== '0') scene.add(createFluff(field, quality === 'medium' ? 200 : 320));
// time passes: the afternoon you start in is sunset twenty minutes later. ?drift=0 holds the hour; T and 1 to 4
// pick a time of day and hold it there, 0 lets it pass again, and , and . nudge it.
let drift = params.has('drift') ? Number(params.get('drift')) : 0;
const TIMES = [
  { hour: 12.5, name: 'Noon' },
  { hour: 15.5, name: 'Afternoon' },
  { hour: 19, name: 'Golden hour' },
  { hour: 20.3, name: 'Sunset' },
];
const DRIFT = (SUNSET - 15.5) / (20 * 60);
let castAt = 0;
/** The records, planted: made once the land is. */
let garden = null;
let clock = 0;

// ---- the walker ----------------------------------------------------------------------------------------------
const EYE = 1.68;
const REACH = EXTENT / 2 - 110; // metres from the middle you can walk
const WALK = 4.2;
const RUN = 6.3;
const player = {
  x: START[0],
  z: START[1],
  y: 0,
  vy: 0,
  vx: 0,
  vz: 0,
  yaw: Math.atan2(-HERO[0], -HERO[1]),
  pitch: 0.04,
  onGround: true,
  walked: 0,
  stride: 0,
  foot: 1,
  bob: 0,
  dip: 0,
};
player.y = field.heightAt(player.x, player.z);
const keys = new Set();

function updatePlayer(dt) {
  const fx = -Math.sin(player.yaw);
  const fz = -Math.cos(player.yaw);
  const rx = -fz;
  const rz = fx;
  let mx = 0;
  let mz = 0;
  if (keys.has('KeyW') || keys.has('ArrowUp')) mz += 1;
  if (keys.has('KeyS') || keys.has('ArrowDown')) mz -= 1;
  if (keys.has('KeyD') || keys.has('ArrowRight')) mx += 1;
  if (keys.has('KeyA') || keys.has('ArrowLeft')) mx -= 1;
  const len = Math.hypot(mx, mz) || 1;
  const speed = keys.has('ShiftLeft') || keys.has('ShiftRight') ? RUN : WALK;
  const wantX = ((fx * mz + rx * mx) / len) * speed;
  const wantZ = ((fz * mz + rz * mx) / len) * speed;
  const grip = player.onGround ? 9 : 2;
  player.vx += (wantX - player.vx) * Math.min(1, dt * grip);
  player.vz += (wantZ - player.vz) * Math.min(1, dt * grip);
  const fromX = player.x;
  const fromZ = player.z;
  player.x += player.vx * dt;
  player.z += player.vz * dt;
  // the tarn's edge is as far as you go: no wading
  if (tarnDistance(player.x, player.z) < 0.88 && tarnDistance(player.x, player.z) < tarnDistance(fromX, fromZ)) {
    player.x = fromX;
    player.z = fromZ;
    player.vx *= 0.5;
    player.vz *= 0.5;
  }
  // nor through a tree
  for (const [sx, sz, gap] of stones.solid) {
    const dx = player.x - sx;
    const dz = player.z - sz;
    const d = Math.hypot(dx, dz);
    if (d < gap && d > 1e-4) {
      player.x = sx + (dx / d) * gap;
      player.z = sz + (dz / d) * gap;
    }
  }
  for (const [tx, tz, kind, scale] of trees.trees) {
    if (kind > 3) break;
    const gap = (kind === 0 ? 1.25 : 0.6) * scale;
    const dx = player.x - tx;
    const dz = player.z - tz;
    const d = Math.hypot(dx, dz);
    if (d < gap && d > 1e-4) {
      player.x = tx + (dx / d) * gap;
      player.z = tz + (dz / d) * gap;
    }
  }
  // the valley goes on, but the walk doesn't: the ground turns you back gently at its edge
  const r = Math.hypot(player.x, player.z);
  if (r > REACH) {
    player.x *= REACH / r;
    player.z *= REACH / r;
  }
  const ground = field.heightAt(player.x, player.z);
  if (player.onGround) {
    player.y = ground;
    if (keys.has('Space')) {
      player.vy = 4.4;
      player.onGround = false;
    }
  } else {
    player.vy -= 12.5 * dt;
    player.y += player.vy * dt;
    if (player.y <= ground) {
      field.bend(player.x, player.z, 0.55, 0, 0, Math.min(1, -player.vy / 5));
      sounds.step(0.7);
      player.dip = Math.min(0.1, -player.vy * 0.016);
      player.y = ground;
      player.vy = 0;
      player.onGround = true;
    }
  }
  tread(dt, true);
}

/** What walking does to the meadow and, for a walker with no feet to be seen, what it sounds like. */
function tread(dt, heard) {
  const pace = Math.hypot(player.vx, player.vz);
  const moved = pace * dt;
  if (player.onGround && moved > 0) {
    player.walked += moved;
    player.stride += moved;
    // each foot presses the grass down the way it's going
    if (player.stride > 0.16 && pace > 0.05) {
      player.stride = 0;
      const side = player.foot * 0.12;
      field.bend(player.x - (player.vz / pace) * side, player.z + (player.vx / pace) * side, 0.27, player.vx, player.vz, 0.8);
    }
    // about a step a metre, soft
    if (player.walked > (pace > 5 ? 1.2 : 0.95)) {
      player.walked = 0;
      player.foot = -player.foot;
      if (heard) sounds.step(pace > 5 ? 0.42 : 0.28);
    }
    player.bob += moved * (Math.PI / 0.95);
  }
  player.dip *= Math.exp(-dt * 9);
  field.setPusher(0, player.x, player.y + 0.2, player.z, 0.17);
}

/** Stands the walker at a place, looking a way. */
function goTo(x, z, yaw, pitch = -0.06) {
  Object.assign(player, { x, z, y: field.heightAt(x, z), yaw, pitch, vx: 0, vz: 0, vy: 0, onGround: true });
}

function placeCamera() {
  // a test or a film can take the camera off the walker: __tg.view = { x, y, z, yaw, pitch }, y above the ground
  const view = window.__tg?.view;
  if (view) {
    camera.position.set(view.x, field.heightAt(view.x, view.z) + (view.y ?? EYE), view.z);
    camera.rotation.set(view.pitch ?? 0, view.yaw ?? 0, 0);
    if (view.fov && view.fov !== camera.fov) {
      camera.fov = view.fov;
      camera.updateProjectionMatrix();
    }
    camera.updateMatrixWorld();
    return;
  }
  const pace = Math.min(1, Math.hypot(player.vx, player.vz) / RUN);
  const bob = player.onGround ? Math.sin(player.bob) * 0.022 * pace : 0;
  camera.position.set(player.x, player.y + EYE + bob - player.dip, player.z);
  camera.rotation.set(player.pitch, player.yaw, 0);
  camera.updateMatrixWorld();
}

// ---- the page ------------------------------------------------------------------------------------------------
const wind = { angle: Math.atan2(HERO[1], HERO[0]) + 0.9, speed: 4.5 };

function setHour(hour) {
  sky.setHour(Math.min(SUNSET, Math.max(12, hour)));
  trees.cast(sky.sunDir);
  castAt = sky.hour;
}
let noteTimer = 0;
/** A word or two at the top of the screen, gone in a moment. */
function note(text) {
  const el = $('note');
  el.textContent = text;
  el.classList.add('on');
  clearTimeout(noteTimer);
  noteTimer = setTimeout(() => el.classList.remove('on'), 1400);
}
/** Goes to one of the times of day and holds it there. */
function pickTime(index) {
  const time = TIMES[(index + TIMES.length) % TIMES.length];
  drift = 0;
  setHour(time.hour);
  note(time.name);
}
function setWind(speed) {
  wind.speed = Math.min(12, Math.max(0, speed));
  field.setWind(wind.angle, wind.speed);
}

canvas.addEventListener('click', () => {
  sounds.start();
  if (document.pointerLockElement !== canvas) canvas.requestPointerLock()?.catch?.(() => {});
});
document.addEventListener('pointerlockchange', () => {
  const locked = document.pointerLockElement === canvas;
  document.body.classList.toggle('playing', locked);
  if (locked) sounds.start();
  else keys.clear();
});
document.addEventListener('mousemove', (e) => {
  if (document.pointerLockElement !== canvas) return;
  player.yaw -= e.movementX * 0.0021;
  player.pitch = Math.min(1.45, Math.max(-1.45, player.pitch - e.movementY * 0.0021));
});
document.addEventListener('keydown', (e) => {
  // (letters typed into the garden's search box are not the valley's keys)
  if (e.target instanceof HTMLInputElement) return;
  keys.add(e.code);
  // (after the last of them, round to the first again)
  if (e.code === 'KeyT') pickTime(Math.max(0, TIMES.findIndex((time) => time.hour > sky.hour + 0.05)));
  for (let i = 0; i < TIMES.length; i++) if (e.code === `Digit${i + 1}`) pickTime(i);
  if (e.code === 'Digit0') {
    drift = 1;
    note('Time passing');
  }
  if (e.code === 'Comma') setHour(sky.hour - 0.25);
  if (e.code === 'Period') setHour(sky.hour + 0.25);
  if (e.code === 'KeyP') frame.paint = frame.paint ? 0 : paintWas;
  if (e.code === 'Space') e.preventDefault();
});
document.addEventListener('keyup', (e) => keys.delete(e.code));
window.addEventListener('blur', () => keys.clear());

/** Resolves once the GPU has done everything asked of it so far, without holding the page up while it does. */
function gpuDone() {
  const gl = renderer.getContext();
  const sync = gl.fenceSync(gl.SYNC_GPU_COMMANDS_COMPLETE, 0);
  gl.flush();
  return new Promise((done) => {
    const check = () => {
      const state = gl.clientWaitSync(sync, 0, 0);
      if (state === gl.TIMEOUT_EXPIRED) return void setTimeout(check, 2);
      gl.deleteSync(sync);
      done();
    };
    check();
  });
}

// ---- the loop ------------------------------------------------------------------------------------------------
function tick(dt) {
  if (garden?.typing()) keys.clear();
  updatePlayer(dt);
  placeCamera();
  field.update(dt, camera);
  frame.update(dt, camera);
  clock += dt;
  if (drift > 0 && sky.hour < SUNSET && !window.__tg?.view) {
    sky.setHour(Math.min(SUNSET, sky.hour + dt * drift * DRIFT), true);
    if (Math.abs(sky.hour - castAt) > 0.04) {
      trees.cast(sky.sunDir);
      castAt = sky.hour;
    }
  }
  // what you hear depends on where you stand: leaves under a tree, water by the tarn, crickets as the light goes
  let tree = 0;
  for (const [tx, tz, kind, scale] of trees.trees) {
    if (kind > 3) break;
    const reach = (kind === 0 ? 11.5 : 5.6) * scale;
    const d = Math.hypot(player.x - tx, player.z - tz);
    tree = Math.max(tree, 1 - Math.min(1, Math.max(0, (d - reach * 0.8) / (reach * 2.2))));
  }
  const tarn = 1 - Math.min(1, Math.max(0, (tarnDistance(player.x, player.z) - 1) / 1.6));
  const gust = 0.5 + 0.5 * Math.sin(clock * 0.23) * Math.sin(clock * 0.37 + 1);
  sounds.update(dt, { wind: wind.speed, gust, tree, tarn, dusk: Math.min(1, Math.max(0, (sky.hour - 18.8) / 1.4)) });
  sky.update(dt, camera);
  garden?.update(dt, camera);
  frame.exposure = sky.exposure;
  frame.render(scene, camera);
}

mark('land');
await stage(0.48, 'Planting the garden');
// the owner's photos and recordings, for the grove by the pool (invented ones under the dev server)
let media = import.meta.env.DEV && snapshot.workspaceId === 'demo' ? await (await import('../garden/demo.js')).demoMedia() : await loadMedia();
const plantGarden = (kinds) => createGarden({ snapshot, field, scene, canvas, goTo, media, chime: () => sounds.chime(), kinds });
garden = plantGarden();
// sound on or off: the button by the search, or M; off until asked for, and remembered in this browser only
{
  const button = $('soundBtn');
  const show = () => {
    button.setAttribute('aria-pressed', String(!sounds.muted));
    button.setAttribute('aria-label', sounds.muted ? 'Sound is off' : 'Sound is on');
    button.classList.toggle('off', sounds.muted);
  };
  sounds.muted = true;
  try {
    sounds.muted = localStorage.getItem('garden-sound') !== 'on';
  } catch {
    // storage may be blocked: the garden stays quiet
  }
  show();
  const flip = () => {
    sounds.start();
    sounds.muted = !sounds.muted;
    try {
      localStorage.setItem('garden-sound', sounds.muted ? 'off' : 'on');
    } catch {
      // as above
    }
    show();
  };
  button.addEventListener('click', flip);
  document.addEventListener('keydown', (e) => {
    if (e.code === 'KeyM' && !(e.target instanceof HTMLInputElement || e.target instanceof HTMLSelectElement)) flip();
  });
}
mark('garden');
await stage(0.5, 'Painting the mountains');
await skyReady;
mark('skyWait');
await stage(0.64, 'Painting the clouds');
setHour(hour0);
setWind(params.has('wind') ? Number(params.get('wind')) : 4.5);
placeCamera();
field.update(0, camera);
sky.update(0, camera);
await gpuDone();
mark('clouds');
await stage(0.78, 'Mixing paint');
await Promise.all([frame.compile(scene, camera), soundsReady]);
mark('sceneShaders');
await stage(0.9, 'Warming up');
// real frames, each waited for, so the first the walker sees isn't the one that compiles: a full turn of the
// clouds' strips, a few steps taken and the grass trodden
{
  const [x0, z0] = [player.x, player.z];
  keys.add('KeyW');
  for (let i = 0; i < 44; i++) {
    tick(1 / 30);
    await gpuDone();
    if (i % 11 === 10) await stage(0.9 + 0.08 * (i / 44), 'Warming up');
  }
  keys.clear();
  Object.assign(player, { x: x0, z: z0, vx: 0, vz: 0, walked: 0, stride: 0, bob: 0, yaw: Math.atan2(-HERO[0], -HERO[1]), pitch: 0.04 });
  field.reset();
  clock = 0;
  setHour(hour0);
  tick(1 / 60);
  await gpuDone();
}
mark('warm');

if (snapshot.liveRevision) {
  try {
    const walk = JSON.parse(sessionStorage.getItem('garden-walk') ?? 'null');
    sessionStorage.removeItem('garden-walk');
    if (walk?.length === 4 && walk.every(Number.isFinite)) goTo(...walk);
  } catch {
    // Visiting still works without browser storage.
  }
}

// for tests and filming: everything the page is made of, and a clock to drive by hand
window.__tg = { garden, goTo, sounds, pickTime, timings, view: null, wind, THREE, renderer, scene, camera, field, frame, sky, trees, stones, player, keys, tick, setHour, setWind, fromValley, ready: true };

// ---- the loop's switch ----------------------------------------------------------------------------------------
let last = 0;
let running = false;
/** Draws while the page says the demo can be seen, and not at all while it can't. */
function run() {
  const want = shown && !stepped;
  if (want === running) return;
  running = want;
  last = performance.now();
  renderer.setAnimationLoop(
    want
      ? (now) => {
          const dt = Math.min(0.05, Math.max(0, (now - last) / 1000));
          last = now;
          tick(dt);
        }
      : null,
  );
}

fitView();
loaded = true;
run();
$('loading').remove();
document.body.classList.add('ready');
block.ready();
// for the measuring tools
window.__block = { ready: true, block, timings };

async function refreshGarden() {
  try {
    if (document.hidden || !garden.canRefresh()) return;
    const next = await fetchSnapshot(snapshot.liveRevision);
    if (!next || next.unchanged || !garden.canRefresh()) return;
    if (pathLengthFor(next) > WORLD.length) {
      if (document.pointerLockElement) return;
      try {
        sessionStorage.setItem('garden-walk', JSON.stringify([player.x, player.z, player.yaw, player.pitch]));
      } catch {
        // A new hill can still be loaded if browser storage is unavailable.
      }
      location.reload();
      return;
    }
    const nextMedia = next.liveMediaRevision === snapshot.liveMediaRevision ? media : await loadMedia();
    if (!garden.canRefresh()) return;
    const kinds = garden.selectedKinds();
    garden.dispose();
    snapshot = next;
    media = nextMedia;
    garden = plantGarden(kinds);
    window.__tg.garden = garden;
  } catch {
    // Keep the current view if the owner or browser is temporarily unavailable.
  } finally {
    setTimeout(refreshGarden, 5000);
  }
}

if (snapshot.liveRevision) setTimeout(refreshGarden, 5000);
