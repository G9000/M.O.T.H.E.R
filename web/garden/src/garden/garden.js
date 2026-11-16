// The memory garden: the owner's records, planted along the path that winds down the hill. The path is time: the
// newest record stands where the walk begins, under the great tree, and each step further along it is a step
// further back, to the first record of all beside the pool at the path's end. Each record is one tall flower, its
// species telling its kind, drawn in flat colour with an inked line so it stands out from the painted meadow.
// Pinned records are not on the path: they are the stone lanterns standing round the start.
//
// Read-only: this file fetches one snapshot from the local server that served the page, and sends nothing else.
import * as THREE from 'three';
import { mergeGeometries } from 'three/addons/utils/BufferGeometryUtils.js';
import { COMMON } from '../lib/index.js';
import { pathAt, pathLength, START } from '../valley/world.js';

const TAU = Math.PI * 2;
const KINDS = ['memory', 'decision', 'task', 'checkpoint', 'artifact', 'action'];
const BED = { memory: 'Memories', decision: 'Decisions', task: 'Tasks', checkpoint: 'Checkpoints', artifact: 'Artifacts', action: 'Actions', pinned: 'Pinned' };
const FLOWER = { memory: 'daisy', decision: 'tulip', task: 'poppy', checkpoint: 'bellflower', artifact: 'sunflower', action: 'rose', pinned: 'stone lantern' };
const CSS = { memory: '#c6abf2', decision: '#86d4c0', task: '#f2a66e', checkpoint: '#8db6f2', artifact: '#e3cf7d', action: '#ef93ae', pinned: '#f0d9a0' };
const RETIRED = new Set(['retracted', 'superseded', 'cancelled']);
const unsettled = (r) => (r.kind === 'task' && (r.status === 'doing' || r.status === 'blocked')) || (r.kind === 'action' && (r.status === 'pending' || r.status === 'unknown'));

// Paint, as [in the light, in the shade]: a shade is a colour of its own, not the light one made darker.
const PAINT = {
  memory: [[0.74, 0.6, 0.98], [0.4, 0.3, 0.7]],
  decision: [[0.42, 0.86, 0.72], [0.17, 0.5, 0.5]],
  task: [[0.99, 0.56, 0.22], [0.68, 0.27, 0.2]],
  checkpoint: [[0.4, 0.58, 0.98], [0.2, 0.29, 0.68]],
  artifact: [[1.0, 0.8, 0.2], [0.74, 0.46, 0.14]],
  action: [[0.98, 0.45, 0.62], [0.66, 0.22, 0.42]],
  pinned: [[1.0, 0.94, 0.74], [0.82, 0.62, 0.42]],
  gold: [[1.0, 0.82, 0.3], [0.72, 0.46, 0.16]],
  dark: [[0.26, 0.14, 0.2], [0.12, 0.07, 0.12]],
  brown: [[0.46, 0.28, 0.14], [0.24, 0.14, 0.1]],
  green: [[0.38, 0.62, 0.28], [0.15, 0.33, 0.22]],
  stone: [[0.74, 0.72, 0.66], [0.4, 0.41, 0.52]],
  flame: [[2.2, 1.5, 0.7], [1.9, 1.2, 0.55]],
};
// The line is drawn where two different small alphas meet (see lib/frame.js): one per part of a flower.
const INK = { outer: 0.05, inner: 0.08, core: 0.11, green: 0.17 };

// ---- the snapshot ---------------------------------------------------------------------------------------------
/**
 * The link from the terminal carries a single-use token after the "#". It is traded once for this tab's session
 * and taken out of the address bar; a reload uses the session again.
 */
export async function loadSnapshot() {
  try {
    const launch = location.hash.slice(1);
    let session = null;
    try {
      session = sessionStorage.getItem('garden-session');
    } catch {
      // storage may be blocked: this page still has the session it is about to be given
    }
    if (launch) {
      history.replaceState(null, '', location.pathname);
      const res = await fetch('./api/session', { method: 'POST', headers: { Authorization: `Bearer ${launch}` } });
      if (res.ok) {
        session = (await res.json()).session;
        try {
          sessionStorage.setItem('garden-session', session);
        } catch {
          // as above
        }
      }
    }
    if (!session) return null;
    const res = await fetch('./api/garden', { headers: { Authorization: `Bearer ${session}` } });
    return res.ok ? await res.json() : null;
  } catch {
    return null;
  }
}

/** The snapshot as the garden uses it. History entries: [revision, harness, time, changed fields, earlier text]. */
function readRecords(snapshot) {
  const known = new Set(snapshot.records.map((r) => r.id));
  return snapshot.records
    .filter((r) => KINDS.includes(r.kind))
    .map((r) => {
      const refs = (r.references ?? []).filter((id) => known.has(id) && id !== r.id);
      return {
        id: r.id,
        kind: r.kind,
        status: r.status,
        title: r.title,
        content: r.content,
        truncated: r.truncated,
        revision: r.revision,
        pinned: !!r.pinned,
        created: Date.parse(r.createdAt) || 0,
        refs,
        hiddenRefs: (r.references ?? []).length - refs.length,
        assigned: r.assignment ? [r.assignment.assignee, r.assignment.assignedBy, r.assignment.assignedAt] : null,
        next: r.nextStep ?? '',
        h: r.history.map((e) => [e.revision, e.actor, e.at, e.changed, e.revision !== r.revision && (!e.changed || e.changed.includes('content')) ? e.excerpt : '']),
      };
    });
}

// ---- the flowers ----------------------------------------------------------------------------------------------
const SPHERE = new THREE.SphereGeometry(1, 12, 7);
const TUBE = new THREE.CylinderGeometry(1, 1, 1, 8);
const CONE = new THREE.ConeGeometry(1, 1, 10, 1, true);
const M = () => new THREE.Matrix4();

/** One part of a flower: a shape, put in its place, in one paint and with one number for the line. */
function part(shape, matrix, paint, ink) {
  const g = shape.clone().applyMatrix4(matrix);
  g.deleteAttribute('uv');
  const n = g.attributes.position.count;
  const lit = new Float32Array(n * 3);
  const shade = new Float32Array(n * 3);
  for (let i = 0; i < n; i++) {
    lit.set(PAINT[paint][0], i * 3);
    shade.set(PAINT[paint][1], i * 3);
  }
  g.setAttribute('aLit', new THREE.BufferAttribute(lit, 3));
  g.setAttribute('aShade', new THREE.BufferAttribute(shade, 3));
  g.setAttribute('aInk', new THREE.BufferAttribute(new Float32Array(n).fill(ink), 1));
  return g;
}
const place = (x, y, z, sx, sy, sz, rz = 0, rx = 0) => M().makeTranslation(x, y, z).multiply(M().makeRotationZ(rz)).multiply(M().makeRotationX(rx)).multiply(M().makeScale(sx, sy, sz));

/** A ring of petals round the top of the stem, each hinged at the middle and tilted up. */
function ring(parts, n, len, wid, tilt, paint, ink, { y = 0, r0 = 0, turn = 0, thick = 0.022 } = {}) {
  for (let i = 0; i < n; i++) {
    const m = M()
      .makeTranslation(0, 1 + y, 0)
      .multiply(M().makeRotationY((i / n) * TAU + turn))
      .multiply(M().makeRotationZ(tilt))
      .multiply(M().makeTranslation(r0 + len / 2, 0, 0))
      .multiply(M().makeScale(len / 2, thick, wid / 2));
    parts.push(part(SPHERE, m, paint, ink));
  }
}

/** A whole flower one metre tall, its foot at the origin: a stem, two leaves and the head of its kind. */
function flowerGeometry(kind) {
  if (kind === 'pinned') {
    // the pinned core: a stone lantern with a light in it
    const stone = [
      part(TUBE, place(0, 0.05, 0, 0.2, 0.1, 0.2), 'stone', INK.green),
      part(TUBE, place(0, 0.38, 0, 0.075, 0.56, 0.075), 'stone', INK.inner),
      part(TUBE, place(0, 0.68, 0, 0.19, 0.05, 0.19), 'stone', INK.green),
      part(TUBE, place(0, 0.8, 0, 0.12, 0.2, 0.12), 'flame', INK.core),
      part(CONE, place(0, 0.99, 0, 0.27, 0.2, 0.27), 'stone', INK.outer),
      part(SPHERE, place(0, 1.1, 0, 0.045, 0.045, 0.045), 'stone', INK.inner),
    ];
    const lantern = mergeGeometries(stone);
    for (const g of stone) g.dispose();
    return lantern;
  }
  const parts = [part(TUBE, place(0, 0.5, 0, 0.016, 1, 0.016), 'green', INK.green)];
  parts.push(part(SPHERE, place(0.13, 0.3, 0, 0.15, 0.014, 0.055, 0.55), 'green', INK.green));
  parts.push(part(SPHERE, place(-0.12, 0.5, 0.02, 0.14, 0.014, 0.05, -0.6), 'green', INK.green));
  if (kind === 'memory') {
    ring(parts, 13, 0.25, 0.085, 0.16, kind, INK.outer);
    parts.push(part(SPHERE, place(0, 1.02, 0, 0.075, 0.045, 0.075), 'gold', INK.core));
  } else if (kind === 'decision') {
    ring(parts, 3, 0.25, 0.2, 1.2, kind, INK.outer, { r0: 0.02, thick: 0.03 });
    ring(parts, 3, 0.24, 0.19, 1.28, kind, INK.inner, { r0: 0.015, turn: TAU / 6, thick: 0.03 });
  } else if (kind === 'task') {
    ring(parts, 4, 0.23, 0.26, 0.5, kind, INK.outer);
    ring(parts, 4, 0.19, 0.22, 0.8, kind, INK.inner, { y: 0.01, turn: TAU / 8 });
    parts.push(part(SPHERE, place(0, 1.04, 0, 0.05, 0.05, 0.05), 'dark', INK.core));
  } else if (kind === 'checkpoint') {
    for (let i = 0; i < 3; i++) {
      const a = i * 2.1 + 0.4;
      const x = Math.cos(a) * 0.11;
      const z = Math.sin(a) * 0.11;
      parts.push(part(CONE, place(x, 0.9 - i * 0.03, z, 0.085, 0.17, 0.085), kind, i === 1 ? INK.inner : INK.outer));
    }
  } else if (kind === 'artifact') {
    const head = [];
    ring(head, 16, 0.19, 0.065, 0.08, kind, INK.outer, { r0: 0.13 });
    head.push(part(TUBE, place(0, 1.0, 0, 0.15, 0.05, 0.15), 'brown', INK.core));
    // a sunflower looks out, not up
    const tip = M().makeTranslation(0, 1, 0).multiply(M().makeRotationX(0.85)).multiply(M().makeTranslation(0, -1, 0.04));
    for (const g of head) parts.push(g.applyMatrix4(tip));
  } else if (kind === 'action') {
    ring(parts, 5, 0.2, 0.19, 0.6, kind, INK.outer);
    ring(parts, 4, 0.15, 0.15, 1.0, kind, INK.inner, { y: 0.02, turn: 0.5 });
    ring(parts, 3, 0.1, 0.11, 1.3, kind, INK.outer, { y: 0.035, turn: 1.1 });
  }
  const merged = mergeGeometries(parts);
  for (const g of parts) g.dispose();
  return merged;
}

const vertex = /* glsl */ `
uniform float uTime;
uniform float uFocus;
in vec3 aLit;
in vec3 aShade;
in float aInk;
in float aState;
out vec3 vWorld;
out vec3 vN;
out vec3 vLit;
out vec3 vShade;
out float vInk;
out float vState;
out float vFocus;
void main() {
  vFocus = abs(float(gl_InstanceID) - uFocus) < 0.5 ? 1.0 : 0.0;
  mat4 m = modelMatrix * instanceMatrix;
  // (the flower you are looking at stands a little taller)
  vec4 w = m * vec4(position * (1.0 + 0.1 * vFocus), 1.0);
  vec3 foot = m[3].xyz;
  // a slow nod in the wind, more at the head than the foot
  float up = max(w.y - foot.y, 0.0);
  float sway = sin(uTime * 1.5 + foot.x * 0.61 + foot.z * 0.47) + 0.5 * sin(uTime * 2.7 + foot.z * 0.9);
  // (stone does not nod)
  if (aState > 2.5) sway = 0.0;
  w.x += sway * 0.03 * up * up;
  w.z += sway * 0.018 * up * up;
  vWorld = w.xyz;
  vN = normalize(mat3(m) * normal);
  vLit = aLit;
  vShade = aShade;
  vInk = aInk;
  vState = aState;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`;

const fragment = /* glsl */ `
${COMMON}
in vec3 vWorld;
in vec3 vN;
in vec3 vLit;
in vec3 vShade;
in float vInk;
in float vState;
in float vFocus;
layout(location = 0) out vec4 outC;
void main() {
  vec3 N = normalize(vN);
  if (dot(N, uCamPos - vWorld) < 0.0) N = -N;
  // in the light or out of it, and nothing between, as the wanderer is drawn
  float open = smoothstep(0.35, 0.6, openAt(vWorld.xz));
  float lit = smoothstep(0.0, 0.035, dot(N, uSunDir) + 0.06) * open;
  vec3 litPaint = vLit;
  vec3 shadePaint = vShade;
  if (vState > 0.5 && vState < 1.5) {
    // retired: the colour has gone out of it
    litPaint = mix(vec3(dot(litPaint, vec3(0.333))), litPaint, 0.18) * vec3(0.78, 0.76, 0.84);
    shadePaint = mix(vec3(dot(shadePaint, vec3(0.333))), shadePaint, 0.18) * vec3(0.8, 0.78, 0.9);
  }
  vec3 sun = uSunColor * 0.42 + uSkyColor * 0.3;
  float grey = dot(uSkyColor, vec3(0.333));
  vec3 sky = mix(uSkyColor, vec3(grey), 0.8) * 1.15 + 0.2;
  vec3 col = mix(shadePaint * sky, litPaint * sun, lit);
  // unsettled work breathes
  if (vState > 1.5 && vState < 2.5) col *= 1.0 + 0.22 * sin(uTime * 2.6 + vWorld.x);
  col *= 1.0 + 0.3 * vFocus;
  outC = vec4(applyFog(col, vWorld), vInk);
}
`;

// ---- where things grow ----------------------------------------------------------------------------------------
const HEAD = 7; // metres of path before the newest record
const STEP = 0.8; // metres of path to a record
const TAIL = 9;
const MONTH = { month: 'long', year: 'numeric' };
const month = (t) => (t ? new Date(t).toLocaleDateString('en-GB', MONTH) : 'Undated');

/** How long a path these records need, metres: asked before the land is made. */
export function pathLengthFor(snapshot) {
  const onPath = snapshot.records.filter((r) => KINDS.includes(r.kind) && !r.pinned).length;
  return Math.max(70, HEAD + onPath * STEP + TAIL);
}

function layOut(records, heightAt) {
  // the lanterns: a ring round where you start
  const pinned = records.filter((r) => r.pinned);
  pinned.forEach((r, i) => {
    const a = (i / pinned.length) * TAU + 0.5;
    r.x = START[0] + Math.cos(a) * 4.6;
    r.z = START[1] + Math.sin(a) * 4.6;
    r.s = null;
  });
  // everything else along the path, newest first, on alternate sides of it and never quite in a row
  const line = records.filter((r) => !r.pinned).sort((a, b) => b.created - a.created);
  const marks = [];
  let last = null;
  line.forEach((r, i) => {
    r.s = HEAD + i * STEP;
    const p = pathAt(r.s);
    const side = i % 2 ? 1 : -1;
    const off = side * (1.45 + ((i * 7) % 5) * 0.34);
    // (across the path is square to the way it runs)
    r.x = p.x + p.tz * off;
    r.z = p.z - p.tx * off;
    const name = month(r.created);
    if (name !== last) {
      marks.push({ s: Math.max(2, r.s - STEP / 2), text: i ? name : `Today · ${name}` });
      last = name;
    }
  });
  if (line.length) marks.push({ s: Math.min(pathLength(), HEAD + line.length * STEP + 2), text: 'The first record' });
  for (const r of records) r.y = heightAt(r.x, r.z);
  for (const m of marks) {
    const p = pathAt(m.s);
    Object.assign(m, { x: p.x, z: p.z, y: heightAt(p.x, p.z) });
  }
  return { marks, line };
}

// ---- the page -------------------------------------------------------------------------------------------------
const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
const DAY = { day: 'numeric', month: 'short', year: 'numeric' };
const when = (t) => {
  const d = new Date(t);
  return Number.isNaN(d.getTime()) ? '' : `${d.toLocaleDateString('en-GB', DAY)} ${d.toTimeString().slice(0, 5)}`;
};
const $ = (id) => document.getElementById(id);

/**
 * @param field the valley's meadow: its uniforms are the light the flowers stand in
 * @param goTo (x, z, yaw, pitch) stands the walker somewhere, looking a way
 */
export function createGarden({ snapshot, field, scene, canvas, goTo }) {
  const records = readRecords(snapshot);
  const byId = new Map(records.map((r) => [r.id, r]));
  const refBy = (id) => records.filter((r) => r.refs.includes(id)).map((r) => r.id);
  const { marks, line } = layOut(records, field.heightAt);

  // one mesh per kind of flower, however many records there are
  const meshes = [];
  const dummy = new THREE.Object3D();
  for (const kind of [...KINDS, 'pinned']) {
    const items = records.filter((r) => (kind === 'pinned' ? r.pinned : r.kind === kind && !r.pinned));
    if (!items.length) continue;
    const geometry = flowerGeometry(kind);
    const state = new Float32Array(items.length);
    const material = new THREE.ShaderMaterial({
      glslVersion: THREE.GLSL3,
      side: THREE.DoubleSide,
      uniforms: { ...field.uniforms, uFocus: { value: -1 } },
      vertexShader: vertex,
      fragmentShader: fragment,
    });
    const mesh = new THREE.InstancedMesh(geometry, material, items.length);
    items.forEach((r, i) => {
      const retired = RETIRED.has(r.status);
      // taller the more often it has been revised; a retired record has sunk back
      r.tall = kind === 'pinned' ? 1.2 : retired ? 0.62 : 0.85 + 0.09 * Math.min(r.revision, 5);
      r.mesh = mesh;
      r.index = i;
      state[i] = kind === 'pinned' ? 3 : retired ? 1 : unsettled(r) ? 2 : 0;
      dummy.position.set(r.x, r.y - 0.02, r.z);
      dummy.rotation.set(0, (i * 2.39996) % TAU, 0);
      dummy.scale.setScalar(r.tall);
      dummy.updateMatrix();
      mesh.setMatrixAt(i, dummy.matrix);
    });
    geometry.setAttribute('aState', new THREE.InstancedBufferAttribute(state, 1));
    mesh.frustumCulled = false;
    scene.add(mesh);
    meshes.push(mesh);
  }

  // ---- what is said on the page ---------------------------------------------------------------------------------
  const n = records.length;
  $('count').textContent = !n ? 'Read-only · no records yet' : n < snapshot.total ? `Read-only · ${n} of ${snapshot.total} records` : `Read-only · ${n} ${n === 1 ? 'record' : 'records'}`;
  const labels = $('labels');
  const markLabels = marks.map((m) => {
    const el = document.createElement('div');
    el.className = 'label bed';
    el.textContent = m.text;
    labels.appendChild(el);
    return { el, at: new THREE.Vector3(m.x, m.y + 2.3, m.z) };
  });
  const era = $('era');
  const focusLabel = document.createElement('div');
  focusLabel.className = 'label';
  focusLabel.hidden = true;
  labels.appendChild(focusLabel);
  const card = $('card');
  const list = $('list');
  const search = $('q');
  let focus = null;
  let open = null;

  const dot = (r) => `<span class="dot" style="background:${CSS[r.pinned ? 'pinned' : r.kind]}"></span>`;
  const chips = (ids) => ids.map((id) => `<button class="chip" data-go="${esc(id)}">${dot(byId.get(id))}${esc(byId.get(id).title)}</button>`).join('');

  function showCard(r) {
    open = r;
    list.hidden = true;
    const warn = r.status === 'blocked' || r.status === 'unknown' || r.status === 'pending';
    const last = r.h[r.h.length - 1];
    const linked = [...r.refs, ...refBy(r.id)];
    card.innerHTML = `
      <button class="close" aria-label="Close" data-close>×</button>
      <div class="pills"><span class="pill kind" style="background:${CSS[r.kind]}">${esc(r.kind)}</span><span class="pill ${warn ? 'warn' : RETIRED.has(r.status) ? 'bad' : ''}">${esc(r.status)}</span>${r.pinned ? '<span class="pill pinned">pinned</span>' : ''}</div>
      <h2>${esc(r.title)}</h2>
      <p class="sub">rev ${esc(r.revision)}${last ? ` · ${esc(last[1])} · ${when(last[2])}` : ''}</p>
      <p class="content">${esc(r.content)}${r.truncated ? ' …' : ''}</p>
      ${r.truncated ? '<p class="sub">A long record: only its beginning is shown here.</p>' : ''}
      ${r.assigned ? `<div class="note"><b>Waiting for ${esc(r.assigned[0])}</b>Handed over by ${esc(r.assigned[1])}, ${when(r.assigned[2])}</div>` : ''}
      ${r.next ? `<div class="note"><b>Next step</b>${esc(r.next)}</div>` : ''}
      ${linked.length ? `<div class="rel"><h3>Linked · walk there</h3><div class="chips">${chips(linked)}</div></div>` : ''}
      ${r.hiddenRefs ? `<p class="sub">${r.hiddenRefs === 1 ? '1 linked record is' : `${r.hiddenRefs} linked records are`} outside this snapshot.</p>` : ''}
      <details><summary>${r.revision === 1 ? '1 revision' : `${esc(r.revision)} revisions`}${r.h.length < r.revision ? `, latest ${r.h.length} shown` : ''}</summary><div class="hist">${[...r.h]
        .reverse()
        .map((e) => `<div><span class="r">rev ${esc(e[0])}</span><span>${esc(e[1])}, ${when(e[2])}${e[3] ? `<br><span class="x">changed ${e[3].map(esc).join(', ')}</span>` : ''}${e[4] ? `<br><span class="x">“${esc(e[4])}”</span>` : ''}</span></div>`)
        .join('')}</div></details>`;
    card.hidden = false;
  }
  function closePanels() {
    open = null;
    card.hidden = list.hidden = true;
  }
  function renderList() {
    const q = search.value.trim().toLowerCase();
    const groups = [['Pinned core', records.filter((r) => r.pinned)], ...KINDS.map((k) => [BED[k], records.filter((r) => r.kind === k && !r.pinned)])];
    $('items').innerHTML =
      groups
        .map(([name, rs]) => {
          const items = rs.filter((r) => !q || `${r.title} ${r.content}`.toLowerCase().includes(q));
          if (!items.length) return '';
          return `<h3>${name}</h3>${items.map((r) => `<button class="item ${RETIRED.has(r.status) ? 'off' : ''}" data-go="${esc(r.id)}">${dot(r)}${esc(r.title)}</button>`).join('')}`;
        })
        .join('') || '<p class="none">Nothing matches.</p>';
  }
  function showList() {
    open = null;
    card.hidden = true;
    renderList();
    list.hidden = false;
    search.focus();
  }
  /** Stands the walker a few steps short of a record's flower, looking at it, and opens its card. */
  function visit(r) {
    let x;
    let z;
    if (r.s === null) {
      // a lantern: come at it from the middle of the ring
      const d = Math.hypot(r.x - START[0], r.z - START[1]) || 1;
      x = r.x - ((r.x - START[0]) / d) * 3;
      z = r.z - ((r.z - START[1]) / d) * 3;
    } else {
      // a flower: stand on the path beside it
      const p = pathAt(r.s);
      x = p.x;
      z = p.z;
    }
    const dx = r.x - x;
    const dz = r.z - z;
    const far = Math.hypot(dx, dz) || 1;
    // looking at the bloom, which is below the eye
    const drop = field.heightAt(x, z) + 1.68 - (r.y + r.tall);
    goTo(x, z, Math.atan2(-dx, -dz), -Math.atan2(drop, far));
    showCard(r);
  }
  const unlock = () => document.pointerLockElement && document.exitPointerLock();
  function read() {
    if (!focus) return;
    showCard(focus);
    unlock();
  }
  for (const panel of [card, list]) {
    panel.addEventListener('click', (e) => {
      const go = e.target.closest('[data-go]');
      if (go) visit(byId.get(go.dataset.go));
      else if (e.target.closest('[data-close]')) closePanels();
    });
  }
  search.addEventListener('input', renderList);
  search.addEventListener('keydown', (e) => {
    if (e.key !== 'Enter') return;
    const first = $('items').querySelector('[data-go]');
    if (first) visit(byId.get(first.dataset.go));
  });
  $('listBtn').addEventListener('click', () => (list.hidden ? showList() : closePanels()));
  document.addEventListener('keydown', (e) => {
    if (e.target === search) {
      if (e.key === 'Escape') closePanels();
      return;
    }
    if (e.code === 'KeyE') read();
    else if (e.code === 'KeyL') {
      e.preventDefault();
      unlock();
      if (list.hidden) showList();
      else closePanels();
    } else if (e.key === 'Escape') closePanels();
  });
  // a click while walking reads the flower in front of you
  canvas.addEventListener('mousedown', (e) => {
    if (e.button === 0 && document.pointerLockElement === canvas) read();
  });
  // walking on puts the reading away
  document.addEventListener('pointerlockchange', () => {
    if (document.pointerLockElement === canvas) closePanels();
  });

  // ---- each frame: which flower is being looked at ---------------------------------------------------------------
  const eye = new THREE.Vector3();
  const gaze = new THREE.Vector3();
  const to = new THREE.Vector3();
  const v = new THREE.Vector3();
  function pin(el, at, camera) {
    v.copy(at).project(camera);
    if (v.z > 1 || v.z < -1) {
      el.hidden = true;
      return;
    }
    el.hidden = false;
    el.style.left = `${((v.x + 1) / 2) * innerWidth}px`;
    el.style.top = `${((1 - v.y) / 2) * innerHeight}px`;
  }
  function update(dt, camera) {
    camera.getWorldPosition(eye);
    camera.getWorldDirection(gaze);
    let best = null;
    let bestScore = 0.94; // within about twenty degrees of where you look
    for (const r of records) {
      to.set(r.x - eye.x, r.y + r.tall * 0.9 - eye.y, r.z - eye.z);
      const d = to.length();
      if (d > 16 || d < 0.3) continue;
      const score = to.dot(gaze) / d - d * 0.0012;
      if (score > bestScore) {
        bestScore = score;
        best = r;
      }
    }
    // (with a card open, its flower stays the marked one)
    const marked = open ?? best;
    if (best !== focus) {
      focus = best;
      $('hint').hidden = !focus;
    }
    for (const mesh of meshes) mesh.material.uniforms.uFocus.value = marked && marked.mesh === mesh ? marked.index : -1;
    if (marked) {
      focusLabel.textContent = marked.title;
      pin(focusLabel, v.set(marked.x, marked.y + marked.tall + 0.42, marked.z).clone(), camera);
    } else focusLabel.hidden = true;
    for (const m of markLabels) {
      const far = Math.hypot(m.at.x - eye.x, m.at.z - eye.z);
      if (far > 34) m.el.hidden = true;
      else pin(m.el, m.at, camera);
    }
    // where on the path you are, as a time: that of the nearest flower
    let near = null;
    let nearest = 9;
    for (const r of line) {
      const d = Math.hypot(r.x - eye.x, r.z - eye.z);
      if (d < nearest) {
        nearest = d;
        near = r;
      }
    }
    const text = near ? month(near.created) : '';
    if (era.textContent !== text) era.textContent = text;
  }

  return {
    records,
    marks,
    update,
    visit,
    /** True while the owner is typing in the search box: the walker must not take the letters as steps. */
    typing: () => document.activeElement === search,
    flowers: FLOWER,
  };
}
