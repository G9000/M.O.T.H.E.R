// What the field remembers, kept on the CPU so a game can read it back at once, and sent to the GPU as textures
// for the blades and the ground to read:
//   tState  (0.125 m texels): which way the grass is flattened and how far, and how short it has been cut.
//   tFire   (0.25 m cells):   heat, char and embers. The fire that spreads through it is simulated here.
//   tGlow   (2 m cells):      where fire is, and the light it throws.
import * as THREE from 'three';

const UNCUT = 255;
const FRESH = 0;
const BURNING = 1;
const SMOULDER = 2;
const BURNT = 3;
const SMOULDER_TIME = 9;
const STEP = 1 / 30;

const smooth = (a, b, x) => {
  const t = Math.min(1, Math.max(0, (x - a) / (b - a)));
  return t * t * (3 - 2 * t);
};

function byteTexture(data, n, filter = THREE.LinearFilter) {
  const tex = new THREE.DataTexture(data, n, n, THREE.RGBAFormat, THREE.UnsignedByteType);
  tex.wrapS = tex.wrapT = THREE.ClampToEdgeWrapping;
  tex.minFilter = tex.magFilter = filter;
  tex.generateMipmaps = false;
  tex.needsUpdate = true;
  return tex;
}

/**
 * The parts of a texture that changed since the last upload. Given the renderer, each changed rectangle goes up
 * in one call; without it, three sends them a row at a time, or the whole texture when that is a lot of rows.
 */
class Dirty {
  constructor(tex, n) {
    this.tex = tex;
    this.n = n;
    this.rects = [];
    this.rows = 0;
    this.area = 0;
    this.all = false;
  }
  add(i0, j0, i1, j1) {
    if (this.all) return;
    this.rects.push(i0, j0, i1, j1);
    this.rows += j1 - j0 + 1;
    this.area += (j1 - j0 + 1) * (i1 - i0 + 1);
    if (this.area > this.n * this.n * 0.6) this.all = true;
  }
  flush(renderer) {
    if (!this.all && !this.rects.length) return;
    const { tex, n } = this;
    tex.clearUpdateRanges();
    const made = renderer?.properties.get(tex).__webglTexture;
    if (made && !this.all && tex.version > 0 && !tex.needsUpdate) {
      // straight to WebGL, through three's own state so its caches stay right
      const gl = renderer.getContext();
      const state = renderer.state;
      state.activeTexture(gl.TEXTURE0);
      state.bindTexture(gl.TEXTURE_2D, made);
      state.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false);
      state.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, false);
      state.pixelStorei(gl.UNPACK_ALIGNMENT, 4);
      state.pixelStorei(gl.UNPACK_ROW_LENGTH, n);
      for (let k = 0; k < this.rects.length; k += 4) {
        const i0 = this.rects[k];
        const j0 = this.rects[k + 1];
        state.pixelStorei(gl.UNPACK_SKIP_PIXELS, i0);
        state.pixelStorei(gl.UNPACK_SKIP_ROWS, j0);
        gl.texSubImage2D(gl.TEXTURE_2D, 0, i0, j0, this.rects[k + 2] - i0 + 1, this.rects[k + 3] - j0 + 1, gl.RGBA, gl.UNSIGNED_BYTE, tex.image.data);
      }
      state.pixelStorei(gl.UNPACK_ROW_LENGTH, 0);
      state.pixelStorei(gl.UNPACK_SKIP_PIXELS, 0);
      state.pixelStorei(gl.UNPACK_SKIP_ROWS, 0);
      this.rects.length = 0;
      this.rows = 0;
      this.area = 0;
      return;
    }
    if (this.rows > 420) this.all = true;
    if (!this.all) {
      for (let k = 0; k < this.rects.length; k += 4) {
        const i0 = this.rects[k];
        const w = this.rects[k + 2] - i0 + 1;
        for (let j = this.rects[k + 1]; j <= this.rects[k + 3]; j++) tex.addUpdateRange((j * n + i0) * 4, w * 4);
      }
    }
    tex.needsUpdate = true;
    this.rects.length = 0;
    this.rows = 0;
    this.area = 0;
    this.all = false;
  }
}

export class FieldState {
  /**
   * @param o.size side of the square that reacts, metres. o.centre its middle [x, z].
   * @param o.heightAt, o.growthAt, o.dryAt, o.wetAt functions of (x, z): metres, and 0 to 1
   * @param o.recover seconds for flattened grass to stand halfway back up (0: never)
   * @param o.regrow seconds for cut grass to grow back (0: never)
   */
  constructor({ size = 256, centre = [0, 0], heightAt, growthAt, dryAt, wetAt, recover = 70, regrow = 0 }) {
    this.size = size;
    this.minX = centre[0] - size / 2;
    this.minZ = centre[1] - size / 2;
    this.heightAt = heightAt;
    this.recover = recover;
    this.regrow = regrow;

    // flattening and cutting
    this.N = Math.round(size / 0.125);
    this.cell = size / this.N;
    this.data = new Uint8Array(this.N * this.N * 4);
    this.tex = byteTexture(this.data, this.N);
    this.dirty = new Dirty(this.tex, this.N);
    this.scanRow = 0;
    /** Texels cut since you last emptied it: x, z, the height they were, the height they are (metres). */
    this.cutEvents = [];

    // fire
    this.M = this.N / 2;
    this.fcell = size / this.M;
    const cells = this.M * this.M;
    this.fdata = new Uint8Array(cells * 4);
    this.ftex = byteTexture(this.fdata, this.M);
    this.fdirty = new Dirty(this.ftex, this.M);
    this.fState = new Uint8Array(cells);
    this.fAge = new Float32Array(cells);
    this.fAcc = new Float32Array(cells);
    this.fPeak = new Float32Array(cells);
    this.fDur = new Float32Array(cells);
    this.fGrow = new Uint8Array(cells);
    this.fDry = new Uint8Array(cells);
    this.fWet = new Uint8Array(cells);
    this.fRand = new Uint8Array(cells);
    this.fDone = new Uint8Array(cells);
    this.active = new Int32Array(cells);
    this.activeCount = 0;
    this.burning = 0;
    this.burntCells = 0;
    this.acc = 0;
    this.time = 0;
    /** The box holding everything alight: for drawing flames only where there are any. */
    this.fireBox = { x0: 0, z0: 0, x1: 0, z1: 0, y0: 0, y1: 0 };

    // glow
    this.G = Math.round(size / 2);
    this.gcell = size / this.G;
    this.gsum = new Float32Array(this.G * this.G);
    this.gtmp = new Float32Array(this.G * this.G);
    this.gwide = new Float32Array(this.G * this.G);
    this.gdata = new Uint8Array(this.G * this.G * 4);
    this.gtex = byteTexture(this.gdata, this.G);
    this.glowLive = false;

    let seed = 1234567;
    for (let j = 0; j < this.M; j++) {
      for (let i = 0; i < this.M; i++) {
        const k = i + j * this.M;
        const x = this.minX + (i + 0.5) * this.fcell;
        const z = this.minZ + (j + 0.5) * this.fcell;
        this.fGrow[k] = Math.round(255 * Math.min(1, Math.max(0, growthAt(x, z))));
        this.fDry[k] = Math.round(255 * Math.min(1, Math.max(0, dryAt(x, z))));
        this.fWet[k] = Math.round(255 * Math.min(1, Math.max(0, wetAt(x, z))));
        seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0;
        this.fRand[k] = seed >>> 24;
      }
    }
    this.kernel = [];
    this.setWind(1, 0, 0);
    this.reset();
  }

  /** Everything back as it grew. */
  reset() {
    const d = this.data;
    for (let k = 0; k < d.length; k += 4) {
      d[k] = 128;
      d[k + 1] = 128;
      d[k + 2] = UNCUT;
      d[k + 3] = 255;
    }
    this.dirty.all = true;
    this.fdata.fill(0);
    this.fdirty.all = true;
    this.fState.fill(FRESH);
    this.fAge.fill(0);
    this.fAcc.fill(0);
    this.fDone.fill(0);
    this.activeCount = 0;
    this.burning = 0;
    this.burntCells = 0;
    this.gdata.fill(0);
    this.gtex.needsUpdate = true;
    this.glowLive = false;
    this.cutEvents.length = 0;
  }

  /** The wind the fire runs before: a unit direction and a speed in m/s. */
  setWind(dx, dz, speed) {
    this.windX = dx;
    this.windZ = dz;
    this.windSpeed = speed;
    const kernel = [];
    for (let j = -2; j <= 2; j++) {
      for (let i = -2; i <= 2; i++) {
        if (!i && !j) continue;
        const dist = Math.hypot(i, j);
        const c = (i * dx + j * dz) / dist;
        let w;
        if (dist < 1.5) {
          // fast before the wind, slow into it, in between across it
          const shape = c > 0 ? 1 + 1.05 * speed * c * c : 1 / (1 + 0.55 * speed * -c);
          w = (0.55 / dist) * shape;
        } else {
          // a strong wind throws flame over a narrow gap
          w = speed > 1.5 && c > 0.55 ? (0.085 * (speed - 1.5) * c ** 4) / dist : 0;
        }
        if (w > 0) kernel.push(i + j * this.M, w);
      }
    }
    this.kernel = kernel;
  }

  inside(x, z, margin = 0) {
    return x > this.minX + margin && z > this.minZ + margin && x < this.minX + this.size - margin && z < this.minZ + this.size - margin;
  }

  /**
   * Flatten the grass round (x, z). With a direction (dx, dz) it lies that way, as behind a foot or a rolling
   * ball; without, it lies outwards from the middle, as under something dropped on it.
   */
  bend(x, z, radius = 0.3, dx = 0, dz = 0, amount = 1) {
    const { N, cell, data } = this;
    const fx = (x - this.minX) / cell - 0.5;
    const fz = (z - this.minZ) / cell - 0.5;
    const r = radius / cell;
    const i0 = Math.max(2, Math.floor(fx - r));
    const i1 = Math.min(N - 3, Math.ceil(fx + r));
    const j0 = Math.max(2, Math.floor(fz - r));
    const j1 = Math.min(N - 3, Math.ceil(fz + r));
    if (i0 > i1 || j0 > j1) return;
    const len = Math.hypot(dx, dz);
    const radial = len < 1e-6;
    const ux = radial ? 0 : dx / len;
    const uz = radial ? 0 : dz / len;
    for (let j = j0; j <= j1; j++) {
      for (let i = i0; i <= i1; i++) {
        const ox = i - fx;
        const oz = j - fz;
        const d = Math.hypot(ox, oz) / r;
        if (d >= 1) continue;
        const w = (1 - d * d) ** 1.5 * amount;
        let tx = ux;
        let tz = uz;
        if (radial) {
          const l = Math.hypot(ox, oz) || 1;
          tx = ox / l;
          tz = oz / l;
        }
        const k = (i + j * N) * 4;
        const bx = (data[k] - 128) / 127;
        const bz = (data[k + 1] - 128) / 127;
        const nx = bx + (tx - bx) * w;
        const nz = bz + (tz - bz) * w;
        data[k] = 128 + Math.round(Math.max(-1, Math.min(1, nx)) * 127);
        data[k + 1] = 128 + Math.round(Math.max(-1, Math.min(1, nz)) * 127);
      }
    }
    this.dirty.add(i0, j0, i1, j1);
  }

  /** Cut everything within `width / 2` of the line from a to b down to `height` metres (h0 at a, h1 at b). */
  cutLine(ax, az, bx, bz, width = 0.2, h0 = 0.07, h1 = h0) {
    const { N, cell, data } = this;
    const r = width / 2 / cell;
    const pax = (ax - this.minX) / cell - 0.5;
    const paz = (az - this.minZ) / cell - 0.5;
    const pbx = (bx - this.minX) / cell - 0.5;
    const pbz = (bz - this.minZ) / cell - 0.5;
    const i0 = Math.max(2, Math.floor(Math.min(pax, pbx) - r));
    const i1 = Math.min(N - 3, Math.ceil(Math.max(pax, pbx) + r));
    const j0 = Math.max(2, Math.floor(Math.min(paz, pbz) - r));
    const j1 = Math.min(N - 3, Math.ceil(Math.max(paz, pbz) + r));
    if (i0 > i1 || j0 > j1) return 0;
    const ex = pbx - pax;
    const ez = pbz - paz;
    const ee = ex * ex + ez * ez || 1e-9;
    let cut = 0;
    for (let j = j0; j <= j1; j++) {
      for (let i = i0; i <= i1; i++) {
        const t = Math.min(1, Math.max(0, ((i - pax) * ex + (j - paz) * ez) / ee));
        const ox = i - (pax + ex * t);
        const oz = j - (paz + ez * t);
        if (ox * ox + oz * oz > r * r) continue;
        const k = (i + j * N) * 4 + 2;
        const to = Math.max(1, Math.min(254, Math.round((h0 + (h1 - h0) * t) * 255)));
        const was = data[k];
        if (to >= was) continue;
        data[k] = to;
        // only what was standing tall enough to lose a top is worth telling anyone about
        if (was - to > 20 && this.fState[(i >> 1) + (j >> 1) * this.M] === FRESH) {
          this.cutEvents.push(this.minX + (i + 0.5) * cell, this.minZ + (j + 0.5) * cell, was / 255, to / 255);
          cut++;
        }
      }
    }
    this.dirty.add(i0, j0, i1, j1);
    return cut;
  }

  /** Cut a disc of grass down to `height` metres. */
  cut(x, z, radius = 0.3, height = 0.07) {
    return this.cutLine(x, z, x, z, radius * 2, height);
  }

  /** How much there is to burn in a fire cell, 0 to 1: nothing where it's bare, wet, mown or already burnt. */
  fuel(k) {
    if (this.fState[k] !== FRESH || this.fWet[k] > 140) return 0;
    const i = (k % this.M) * 2;
    const j = Math.floor(k / this.M) * 2;
    const d = this.data;
    const N = this.N;
    const cutH = (d[(i + j * N) * 4 + 2] + d[(i + 1 + j * N) * 4 + 2] + d[(i + (j + 1) * N) * 4 + 2] + d[(i + 1 + (j + 1) * N) * 4 + 2]) / (4 * 255);
    return (this.fGrow[k] / 255) * smooth(0.08, 0.26, cutH) * (1 - this.fWet[k] / 280);
  }

  light(k, force = false) {
    const fuel = this.fuel(k);
    if (fuel < (force ? 0.06 : 0.22)) {
      this.fAcc[k] = 0;
      return false;
    }
    const dry = this.fDry[k] / 255;
    const rnd = this.fRand[k] / 255;
    this.fState[k] = BURNING;
    this.fAge[k] = 0;
    this.fPeak[k] = Math.min(1, (0.42 + 0.58 * fuel) * (0.72 + 0.4 * dry));
    this.fDur[k] = (2.0 + 2.6 * fuel) * (0.75 + 0.5 * rnd);
    this.active[this.activeCount++] = k;
    return true;
  }

  /** Set the grass alight within `radius` of (x, z). Returns how many cells caught. */
  ignite(x, z, radius = 0.3) {
    const { M, fcell } = this;
    const fx = (x - this.minX) / fcell - 0.5;
    const fz = (z - this.minZ) / fcell - 0.5;
    const r = Math.max(radius / fcell, 0.75);
    let lit = 0;
    for (let j = Math.max(3, Math.floor(fz - r)); j <= Math.min(M - 4, Math.ceil(fz + r)); j++) {
      for (let i = Math.max(3, Math.floor(fx - r)); i <= Math.min(M - 4, Math.ceil(fx + r)); i++) {
        if ((i - fx) ** 2 + (j - fz) ** 2 > r * r) continue;
        if (this.light(i + j * M, true)) lit++;
      }
    }
    return lit;
  }

  /** What's at (x, z) now. Heights in metres, the rest 0 to 1. */
  stateAt(x, z) {
    if (!this.inside(x, z)) return { inside: false, burning: false, heat: 0, burnt: 0, cut: 1, flat: 0, fuel: 0 };
    const i = Math.min(this.N - 1, Math.floor((x - this.minX) / this.cell));
    const j = Math.min(this.N - 1, Math.floor((z - this.minZ) / this.cell));
    const k = (i + j * this.N) * 4;
    const f = (i >> 1) + (j >> 1) * this.M;
    const d = this.data;
    return {
      inside: true,
      burning: this.fState[f] === BURNING,
      heat: this.fdata[f * 4] / 255,
      burnt: this.fdata[f * 4 + 1] / 255,
      cut: d[k + 2] / 255,
      flat: Math.min(1, Math.hypot(d[k] - 128, d[k + 1] - 128) / 127),
      fuel: this.fuel(f),
      dry: this.fDry[f] / 255,
      wet: this.fWet[f] / 255,
      growth: this.fGrow[f] / 255,
    };
  }

  /** A burning cell picked at random, as [x, z, heat], or null: where to put smoke and sparks. */
  sample(random) {
    for (let tries = 0; tries < 6 && this.activeCount; tries++) {
      const k = this.active[Math.floor(random() * this.activeCount)];
      const heat = this.fdata[k * 4] / 255;
      const smoulder = this.fdata[k * 4 + 2] / 255;
      if (heat < 0.12 && smoulder < 0.3) continue;
      const i = k % this.M;
      const j = Math.floor(k / this.M);
      return [this.minX + (i + random()) * this.fcell, this.minZ + (j + random()) * this.fcell, heat, smoulder];
    }
    return null;
  }

  stepFire(dt) {
    const { M, fState, fAge, fAcc, fPeak, fDur, fdata, fDry, fRand, active, kernel } = this;
    let i0 = M;
    let i1 = -1;
    let j0 = M;
    let j1 = -1;
    let burning = 0;
    const gsum = this.gsum;
    gsum.fill(0);
    const G = this.G;
    const perG = M / G;
    for (let a = this.activeCount - 1; a >= 0; a--) {
      const k = active[a];
      const i = k % M;
      const j = (k - i) / M;
      if (i < i0) i0 = i;
      if (i > i1) i1 = i;
      if (j < j0) j0 = j;
      if (j > j1) j1 = j;
      const age = (fAge[k] += dt);
      const o = k * 4;
      if (fState[k] === BURNING) {
        const dur = fDur[k];
        if (age >= dur) {
          fState[k] = SMOULDER;
          fAge[k] = 0;
          fdata[o] = 0;
          fdata[o + 1] = 255;
          fdata[o + 2] = 255;
          this.burntCells++;
          continue;
        }
        // it catches at once: the front is where the flames are tallest
        const heat = fPeak[k] * smooth(0, 0.14, age) * (1 - smooth(dur * 0.45, dur, age));
        fdata[o] = Math.round(heat * 255);
        fdata[o + 1] = Math.round(255 * smooth(0.1, dur * 0.55, age));
        fdata[o + 2] = Math.round(200 * smooth(dur * 0.4, dur, age));
        burning++;
        gsum[Math.floor(i / perG) + Math.floor(j / perG) * G] += heat;
        if (i < 3 || j < 3 || i > M - 4 || j > M - 4 || this.fDone[k]) continue;
        const give = heat * dt;
        let fresh = 0;
        for (let n = 0; n < kernel.length; n += 2) {
          const q = k + kernel[n];
          if (fState[q] !== FRESH) continue;
          fresh++;
          const got = (fAcc[q] += give * kernel[n + 1]);
          // dry grass catches sooner; no two tufts catch alike, which keeps the front ragged
          if (got * (0.5 + fDry[q] / 255) > 0.62 + fRand[q] / 300) this.light(q);
        }
        // nothing left round it to set alight: it only has to burn out
        if (!fresh) this.fDone[k] = 1;
      } else {
        if (age >= SMOULDER_TIME) {
          fState[k] = BURNT;
          fdata[o + 2] = 0;
          active[a] = active[--this.activeCount];
          continue;
        }
        const left = 1 - age / SMOULDER_TIME;
        fdata[o + 2] = Math.round(255 * left * left);
      }
    }
    this.burning = burning;
    if (i1 >= i0) this.fdirty.add(i0, j0, i1, j1);
    if (burning) {
      const b = this.fireBox;
      b.x0 = this.minX + i0 * this.fcell;
      b.x1 = this.minX + (i1 + 1) * this.fcell;
      b.z0 = this.minZ + j0 * this.fcell;
      b.z1 = this.minZ + (j1 + 1) * this.fcell;
      let lo = Infinity;
      let hi = -Infinity;
      const stride = Math.max(1, Math.floor(this.activeCount / 96));
      for (let a = 0; a < this.activeCount; a += stride) {
        const k = active[a];
        const h = this.heightAt(this.minX + ((k % M) + 0.5) * this.fcell, this.minZ + (Math.floor(k / M) + 0.5) * this.fcell);
        if (h < lo) lo = h;
        if (h > hi) hi = h;
      }
      b.y0 = lo - 0.6;
      b.y1 = hi + 0.6;
    }
    this.stepGlow(burning > 0);
  }

  blur(src, dst) {
    const G = this.G;
    for (let j = 1; j < G - 1; j++) {
      for (let i = 1; i < G - 1; i++) {
        const k = i + j * G;
        dst[k] = (src[k] * 4 + (src[k - 1] + src[k + 1] + src[k - G] + src[k + G]) * 2 + src[k - G - 1] + src[k - G + 1] + src[k + G - 1] + src[k + G + 1]) / 16;
      }
    }
  }

  stepGlow(any) {
    if (!any && !this.glowLive) return;
    const { G, gsum, gtmp, gwide, gdata } = this;
    for (let j = 1; j < G - 1; j++) {
      for (let i = 1; i < G - 1; i++) {
        const k = i + j * G;
        let near = 0;
        for (let b = -1; b <= 1 && !near; b++) for (let a = -1; a <= 1; a++) if (gsum[k + a + b * G] > 0) near = 1;
        gdata[k * 4] = near ? 255 : 0;
      }
    }
    // the light: a tight pool round the flames, and a wide one that reaches across the field
    this.blur(gsum, gtmp);
    this.blur(gtmp, gsum);
    gwide.set(gsum);
    for (let n = 0; n < 3; n++) {
      this.blur(gwide, gtmp);
      this.blur(gtmp, gwide);
    }
    for (let k = 0; k < G * G; k++) gdata[k * 4 + 1] = Math.min(255, Math.round((gsum[k] * 0.055 + gwide[k] * 0.11) * 255));
    this.gtex.needsUpdate = true;
    this.glowLive = any;
  }

  /** Flattened grass stands back up and cut grass grows, a band of rows a frame, so it costs nearly nothing. */
  scan(dt) {
    const { N, data } = this;
    const pass = 1.5;
    const rows = Math.max(1, Math.round((N * dt) / pass));
    const keep = this.recover > 0 ? Math.pow(0.5, pass / this.recover) : 1;
    const grow = this.regrow > 0 ? (255 * pass) / this.regrow : 0;
    for (let n = 0; n < rows; n++) {
      const j = this.scanRow;
      this.scanRow = (this.scanRow + 1) % N;
      let lo = N;
      let hi = -1;
      let k = j * N * 4;
      for (let i = 0; i < N; i++, k += 4) {
        const bx = data[k];
        const bz = data[k + 1];
        let touched = false;
        if (keep < 1 && (bx !== 128 || bz !== 128)) {
          data[k] = 128 + Math.trunc((bx - 128) * keep);
          data[k + 1] = 128 + Math.trunc((bz - 128) * keep);
          touched = true;
        }
        if (grow > 0 && data[k + 2] < UNCUT) {
          const add = Math.floor(grow) + (Math.random() < grow % 1 ? 1 : 0);
          if (add) {
            data[k + 2] = Math.min(UNCUT, data[k + 2] + add);
            touched = true;
          }
        }
        if (touched) {
          if (i < lo) lo = i;
          hi = i;
        }
      }
      if (hi >= 0) this.dirty.add(lo, j, hi, j);
    }
  }

  /** Move everything on by dt seconds and send what changed to the GPU. */
  update(dt, renderer) {
    this.time += dt;
    this.acc = Math.min(this.acc + dt, STEP * 4);
    while (this.acc >= STEP) {
      this.acc -= STEP;
      if (this.activeCount) this.stepFire(STEP);
      else if (this.glowLive) {
        this.gsum.fill(0);
        this.stepGlow(false);
      }
    }
    if (!this.activeCount) this.burning = 0;
    this.scan(dt);
    this.dirty.flush(renderer);
    this.fdirty.flush(renderer);
  }
}
