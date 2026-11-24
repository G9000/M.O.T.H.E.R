import * as THREE from 'three';
import { fromValley, gardenRadius, HERO, START, TARN, tarnDistance, TREE } from '../valley/world.js';

/** Viewpoints use the configured hill. A landing is a dry spot at walking height. */
export function atlasStops(heightAt) {
  const ground = ([x, z], lift = 0) => new THREE.Vector3(x, heightAt(x, z) + lift, z);
  const pool = new THREE.Vector3(TARN.at[0], TARN.level + 2.5, TARN.at[1]);
  const bank = new THREE.Vector2(-pool.x, -pool.z).normalize();
  const unit = tarnDistance(pool.x + bank.x, pool.z + bank.y);
  const shore = [pool.x + bank.x * 1.29 / unit, pool.z + bank.y * 1.29 / unit];
  const centre = ground(TARN.at.map(value => value * 0.32));
  const radius = Math.max(gardenRadius() + Math.hypot(centre.x, centre.z) + 8,
    Math.hypot(pool.x - centre.x, pool.z - centre.z) + 22);
  return [
    { name: 'Whole garden', target: centre, radius, bearing: Math.atan2(-HERO[0], -HERO[1]) + 0.65, elevation: 0.77 },
    { name: 'Wisteria arbor', target: ground(fromValley(6.4, 0), 2.05), radius: 7.8,
      bearing: Math.atan2(-HERO[0], -HERO[1]) + 0.30, elevation: 0.045, landing: ground(START) },
    { name: 'Blossom tree', target: ground(TREE, 9.0), radius: 34,
      bearing: Math.atan2(HERO[0], HERO[1]) - 0.49, elevation: 0.06, landing: ground(fromValley(-1, -4)) },
    { name: 'Cherry blossoms', target: ground(TREE, 3.17).add(new THREE.Vector3(0, 0, 4.15)),
      radius: 2.05, bearing: 1.08, elevation: -0.10, landing: ground([TREE[0] + 1.9, TREE[1] + 5.2]) },
    { name: 'Bark close-up', target: ground(TREE, 2.9), radius: 4.8,
      bearing: 0.22, elevation: -0.09, landing: ground(fromValley(-5, -6)) },
    { name: 'Glasshouse pool', target: pool, radius: 21, bearing: Math.atan2(bank.x, bank.y) + 0.22,
      elevation: 0.13, landing: ground(shore) },
  ];
}

export function positionAtlasCamera(camera, stop, rotation, heightAt) {
  const halfFov = THREE.MathUtils.degToRad(camera.fov * 0.5);
  const narrow = Math.min(halfFov, Math.atan(Math.tan(halfFov) * camera.aspect));
  const distance = stop.landing ? stop.radius * Math.max(1, 1 / camera.aspect) : stop.radius / Math.sin(narrow);
  const bearing = stop.bearing + rotation;
  const horizontal = Math.cos(stop.elevation) * distance;
  const x = stop.target.x + Math.sin(bearing) * horizontal;
  const z = stop.target.z + Math.cos(bearing) * horizontal;
  camera.position.set(x, Math.max(heightAt(x, z) + 1.75, stop.target.y + Math.sin(stop.elevation) * distance), z);
  camera.lookAt(stop.target);
  camera.updateMatrixWorld();
}

export function positionSpecimenCamera(camera, record, rotation, whole, heightAt) {
  const a = record.appearance;
  const height = record.tall;
  const targetY = record.y + height * (whole || a.stone ? 0.54 : a.companion ? 0.75 : a.form === 'bellflower' ? 0.87 : 1.04);
  const size = whole || a.stone ? Math.max(height * 0.84, a.width * 0.77) : Math.max(a.width, a.depth) * (a.companion ? 0.65 : a.form === 'rose' ? 0.27 : 0.43);
  const angle = a.turn + rotation;
  const half = THREE.MathUtils.degToRad(camera.fov / 2);
  const distance = size / Math.sin(Math.min(half, Math.atan(Math.tan(half) * camera.aspect)));
  const elevation = whole ? 0.14 : a.form === 'bellflower' ? -0.12 : 0.48;
  const horizontal = Math.cos(elevation) * distance;
  const x = record.x + Math.sin(angle) * horizontal, z = record.z + Math.cos(angle) * horizontal;
  camera.position.set(x, Math.max(heightAt(x, z) + 0.18, targetY + Math.sin(elevation) * distance), z);
  camera.lookAt(record.x, targetY, record.z);
  camera.updateMatrixWorld();
}

export function createAtlas({ heightAt, goTo, beforeOpen, beforeTravel, times, hour, pickTime, specimens }) {
  const dialog = document.getElementById('atlas');
  const opener = document.getElementById('atlasBtn');
  const choices = dialog.querySelector('.atlas-stops');
  const visit = dialog.querySelector('[data-travel]');
  const light = dialog.querySelector('select');
  const lens = document.getElementById('lensBtn');
  const browser = dialog.querySelector('[data-specimens]');
  const caption = dialog.querySelector('[data-caption]');
  const zoom = dialog.querySelector('[data-zoom]');
  const title = dialog.querySelector('h2');
  const stops = atlasStops(heightAt);
  let selected = 0;
  let rotation = 0;
  let collection = [], index = 0, whole = false;
  const buttons = stops.map((stop, i) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.textContent = stop.name;
    button.addEventListener('click', () => select(i));
    choices.append(button);
    return button;
  });
  function select(index) {
    selected = index;
    rotation = 0;
    buttons.forEach((button, i) => button.setAttribute('aria-pressed', String(i === selected)));
    visit.hidden = !stops[index].landing;
  }
  times.forEach((time, i) => light.add(new Option(time.name, String(i))));
  light.addEventListener('change', () => pickTime(Number(light.value)));
  function open() {
    beforeOpen();
    const current = hour();
    light.selectedIndex = times.reduce((best, time, i) =>
      Math.abs(time.hour - current) < Math.abs(times[best].hour - current) ? i : best, 0);
    choices.hidden = collection.length > 0;
    browser.hidden = !choices.hidden;
    caption.hidden = !choices.hidden;
    dialog.showModal();
  }
  function showSpecimen() {
    const record = collection[index];
    title.textContent = record.appearance.form;
    caption.textContent = record.title;
    zoom.textContent = whole ? 'Close-up' : 'Whole specimen';
  }
  function inspect(record) {
    const records = specimens();
    if (record && !records.includes(record)) records.unshift(record);
    if (!records.length) return;
    // Lead with different families, then allow browsing through every remaining record.
    const seen = new Set(), rest = [];
    collection = [];
    for (const r of records) {
      if (seen.has(r.appearance.form)) rest.push(r);
      else { seen.add(r.appearance.form); collection.push(r); }
    }
    collection.push(...rest);
    index = Math.max(0, collection.indexOf(record));
    whole = false;
    rotation = 0;
    visit.hidden = true;
    browser.querySelectorAll('[data-step]').forEach(b => { b.disabled = collection.length < 2; });
    showSpecimen();
    open();
  }
  function showTree() {
    collection = [];
    title.textContent = 'Garden atlas';
    select(stops.findIndex(stop => stop.name === 'Blossom tree'));
    open();
  }
  document.getElementById('treeBtn').addEventListener('click', showTree);
  lens.addEventListener('click', () => inspect());
  for (const b of browser.querySelectorAll('[data-step]')) b.addEventListener('click', () => {
    index = (index + Number(b.dataset.step) + collection.length) % collection.length;
    rotation = 0;
    showSpecimen();
  });
  zoom.addEventListener('click', () => { whole = !whole; showSpecimen(); });
  opener.addEventListener('click', () => {
    collection = [];
    title.textContent = 'Garden atlas';
    select(0);
    open();
  });
  dialog.querySelector('[data-return]').addEventListener('click', () => dialog.close());
  dialog.querySelector('[data-left]').addEventListener('click', () => { rotation -= Math.PI / 8; });
  dialog.querySelector('[data-right]').addEventListener('click', () => { rotation += Math.PI / 8; });
  visit.addEventListener('click', () => {
    const { target, landing } = stops[selected];
    if (!landing) return;
    const dx = landing.x - target.x, dz = landing.z - target.z;
    beforeTravel();
    goTo(landing.x, landing.z, Math.atan2(dx, dz), Math.atan2(target.y - landing.y - 1.68, Math.hypot(dx, dz)));
    dialog.close();
    document.getElementById('walkBtn').focus();
  });
  // Native dialog navigation and Escape remain active. Garden hotkeys are
  // document listeners, so keep keystrokes from reaching the walk beneath it.
  dialog.addEventListener('keydown', e => e.stopPropagation());
  return {
    get open() { return dialog.open; },
    inspect,
    showTree,
    placeCamera(camera) {
      if (collection.length) positionSpecimenCamera(camera, collection[index], rotation, whole, heightAt);
      else positionAtlasCamera(camera, stops[selected], rotation, heightAt);
    },
  };
}
