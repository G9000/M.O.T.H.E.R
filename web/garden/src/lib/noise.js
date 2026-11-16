// Noise the shaders read as textures, made once on the CPU. All of it tiles.
import * as THREE from 'three';

export function mulberry(seed) {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const fade = (t) => t * t * t * (t * (t * 6 - 15) + 10);

/** Value noise on a lattice of `period` cells that wraps, in 2D. Returns a function of (x, y) in cells, 0..1. */
function lattice2(period, random) {
  const v = new Float32Array(period * period);
  for (let i = 0; i < v.length; i++) v[i] = random();
  return (x, y) => {
    const xi = Math.floor(x);
    const yi = Math.floor(y);
    const fx = fade(x - xi);
    const fy = fade(y - yi);
    const x0 = ((xi % period) + period) % period;
    const y0 = ((yi % period) + period) % period;
    const x1 = (x0 + 1) % period;
    const y1 = (y0 + 1) % period;
    const a = v[x0 + y0 * period];
    const b = v[x1 + y0 * period];
    const c = v[x0 + y1 * period];
    const d = v[x1 + y1 * period];
    return a + (b - a) * fx + (c - a) * fy + (a - b - c + d) * fx * fy;
  };
}

/**
 * A size × size RGBA texture of four different smooth fields (fbm, 5 octaves), each tiling. With `fine` it is
 * 16 bits a channel and has no mipmaps: for reading it stretched over a large area, where 8 bits show as steps.
 */
export function smoothNoiseTexture(size = 256, seed = 11, fine = false) {
  const random = mulberry(seed);
  const data = fine ? new Uint16Array(size * size * 4) : new Uint8Array(size * size * 4);
  for (let c = 0; c < 4; c++) {
    const octaves = [];
    let period = 4;
    for (let o = 0; o < 5; o++) {
      octaves.push({ period, at: lattice2(period, random) });
      period *= 2;
    }
    const field = new Float32Array(size * size);
    let lo = Infinity;
    let hi = -Infinity;
    for (let y = 0; y < size; y++) {
      for (let x = 0; x < size; x++) {
        let sum = 0;
        let amp = 0.5;
        for (const o of octaves) {
          sum += amp * o.at((x / size) * o.period, (y / size) * o.period);
          amp *= 0.5;
        }
        field[x + y * size] = sum;
        lo = Math.min(lo, sum);
        hi = Math.max(hi, sum);
      }
    }
    for (let i = 0; i < field.length; i++) {
      const v = (field[i] - lo) / (hi - lo);
      data[i * 4 + c] = fine ? THREE.DataUtils.toHalfFloat(v) : Math.round(v * 255);
    }
  }
  const tex = new THREE.DataTexture(data, size, size, THREE.RGBAFormat, fine ? THREE.HalfFloatType : THREE.UnsignedByteType);
  tex.wrapS = tex.wrapT = THREE.RepeatWrapping;
  tex.minFilter = fine ? THREE.LinearFilter : THREE.LinearMipmapLinearFilter;
  tex.magFilter = THREE.LinearFilter;
  tex.generateMipmaps = !fine;
  tex.needsUpdate = true;
  return tex;
}

/** White noise, four channels, read with nearest so each texel is one clump of grass. */
export function whiteNoiseTexture(size = 256, seed = 5) {
  const random = mulberry(seed);
  const data = new Uint8Array(size * size * 4);
  for (let i = 0; i < data.length; i++) data[i] = Math.floor(random() * 256);
  const tex = new THREE.DataTexture(data, size, size, THREE.RGBAFormat, THREE.UnsignedByteType);
  tex.wrapS = tex.wrapT = THREE.RepeatWrapping;
  tex.minFilter = tex.magFilter = THREE.NearestFilter;
  tex.needsUpdate = true;
  return tex;
}

/** A size³ one-channel volume of smooth noise (fbm, 3 octaves) that tiles on every axis, for the flames. */
export function volumeNoiseTexture(size = 64, seed = 23) {
  const random = mulberry(seed);
  const octaves = [];
  let period = 4;
  for (let o = 0; o < 3; o++) {
    const v = new Float32Array(period * period * period);
    for (let i = 0; i < v.length; i++) v[i] = random();
    octaves.push({ period, v });
    period *= 2;
  }
  const field = new Float32Array(size * size * size);
  let lo = Infinity;
  let hi = -Infinity;
  for (let z = 0; z < size; z++) {
    for (let y = 0; y < size; y++) {
      for (let x = 0; x < size; x++) {
        let sum = 0;
        let amp = 0.5;
        for (const { period: P, v } of octaves) {
          const px = (x / size) * P;
          const py = (y / size) * P;
          const pz = (z / size) * P;
          const xi = Math.floor(px);
          const yi = Math.floor(py);
          const zi = Math.floor(pz);
          const fx = fade(px - xi);
          const fy = fade(py - yi);
          const fz = fade(pz - zi);
          const x1 = (xi + 1) % P;
          const y1 = (yi + 1) % P;
          const z1 = (zi + 1) % P;
          const at = (a, b, c) => v[a + P * (b + P * c)];
          const c00 = at(xi, yi, zi) + (at(x1, yi, zi) - at(xi, yi, zi)) * fx;
          const c10 = at(xi, y1, zi) + (at(x1, y1, zi) - at(xi, y1, zi)) * fx;
          const c01 = at(xi, yi, z1) + (at(x1, yi, z1) - at(xi, yi, z1)) * fx;
          const c11 = at(xi, y1, z1) + (at(x1, y1, z1) - at(xi, y1, z1)) * fx;
          const c0 = c00 + (c10 - c00) * fy;
          const c1 = c01 + (c11 - c01) * fy;
          sum += amp * (c0 + (c1 - c0) * fz);
          amp *= 0.5;
        }
        field[x + size * (y + size * z)] = sum;
        lo = Math.min(lo, sum);
        hi = Math.max(hi, sum);
      }
    }
  }
  const data = new Uint8Array(field.length);
  for (let i = 0; i < field.length; i++) data[i] = Math.round(((field[i] - lo) / (hi - lo)) * 255);
  const tex = new THREE.Data3DTexture(data, size, size, size);
  tex.format = THREE.RedFormat;
  tex.type = THREE.UnsignedByteType;
  tex.wrapS = tex.wrapT = tex.wrapR = THREE.RepeatWrapping;
  tex.minFilter = tex.magFilter = THREE.LinearFilter;
  tex.unpackAlignment = 1;
  tex.needsUpdate = true;
  return tex;
}
