import * as THREE from 'three';
import { mulberry } from '../lib/noise.js';

// A repeatable two-by-one metre bark tile. RG stores signed surface slopes,
// B the exposed wood and papery lips, A the lichen. No network asset needed.
export function cherryBarkTexture() {
  const width = 1024, height = 512;
  const elevation = new Float32Array(width * height);
  const pixels = new Uint8Array(width * height * 4);
  const random = mulberry(8643);
  const field = Float32Array.from({ length: 128 * 128 }, random);
  // Sample toroidally at every octave, including the displacement. Wrapped
  // lattice coordinates preserve both the value and gradient at tile edges.
  function tile(u, v, nx, ny) {
    const x = u * nx, y = v * ny;
    const ix = Math.floor(x), iy = Math.floor(y);
    const wx = ix & (nx - 1), wy = iy & (ny - 1);
    const fx = x - ix, fy = y - iy;
    const sx = fx * fx * (3 - 2 * fx), sy = fy * fy * (3 - 2 * fy);
    const nx1 = (wx + 1) & (nx - 1), ny1 = (wy + 1) & (ny - 1);
    const a = THREE.MathUtils.lerp(field[wy * 128 + wx], field[wy * 128 + nx1], sx);
    const b = THREE.MathUtils.lerp(field[ny1 * 128 + wx], field[ny1 * 128 + nx1], sx);
    return THREE.MathUtils.lerp(a, b, sy);
  }
  const smooth = THREE.MathUtils.smoothstep;
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    const i = y * width + x, u = x / width, v = y / height;
    const curl = tile(u, v, 8, 4) - 0.5;
    const plate = tile(u + 0.033 * curl, v + 0.08 * curl, 32, 8);
    const fibres = tile(u + curl * 0.025, v, 128, 16);
    const grit = tile(u + 0.29, v + 0.16, 128, 128);
    const splitPhase = 19 * u + 1.15 * curl + 0.15 * fibres;
    const split = Math.abs(splitPhase - Math.round(splitPhase));
    const cleft = (1 - smooth(split, 0.012, 0.105)) * smooth(plate, 0.16, 0.65);
    const lip = smooth(split, 0.042, 0.095) * (1 - smooth(split, 0.095, 0.19));
    const lichen = smooth(tile(u + 0.37, v + 0.42, 8, 8) + 0.30 * grit + 0.16 * fibres, 0.90, 1.07);
    elevation[i] = -12 * cleft + 4.8 * lip + 3.8 * fibres + 1.4 * grit + 2 * lichen;
    pixels[4 * i + 2] = 92 + 34 * plate + 43 * fibres + 32 * lip - 57 * cleft + 12 * grit;
    pixels[4 * i + 3] = 220 * lichen;
  }
  // Short, interrupted horizontal cork marks. Each has a lifted pale rim
  // over a rust-coloured split, with varied length, tilt and curvature.
  for (let k = 0; k < 225; k++) {
    const cx = random() * width, cy = random() * height;
    const rx = 3 + 31 * random() ** 2, ry = 0.9 + 2.8 * random(), lean = (random() - 0.5) * 0.24;
    const phase = random() * 6.28, length = Math.ceil(rx + 1);
    for (let dx = -length; dx <= length; dx++) {
      const s = dx / rx;
      if (Math.abs(s) >= 1) continue;
      const radius = ry * Math.sqrt(1 - s * s);
      const line = cy + lean * dx + 1.1 * Math.sin(3 * s + phase);
      for (let y = Math.floor(line - radius - 1); y <= Math.ceil(line + radius + 1); y++) {
        const d = (y - line) / Math.max(radius, 0.6);
        const lip = Math.max(0, 1 - Math.abs(d - 0.40) * 1.35);
        const slit = Math.max(0, 1 - Math.abs(d + 0.38) * 1.7);
        const i = (y & (height - 1)) * width + ((Math.floor(cx) + dx) & (width - 1));
        elevation[i] += 3.6 * lip - 3.0 * slit;
        pixels[4 * i + 2] = Math.max(0, Math.min(255, pixels[4 * i + 2] + 90 * lip - 71 * slit));
      }
    }
  }
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    const i = y * width + x;
    const du = elevation[y * width + ((x + 1) & (width - 1))] - elevation[y * width + ((x - 1) & (width - 1))];
    const dv = elevation[((y + 1) & (height - 1)) * width + x] - elevation[((y - 1) & (height - 1)) * width + x];
    pixels[4 * i] = 128 - 119 * du / (2 + Math.abs(du));
    pixels[4 * i + 1] = 128 - 119 * dv / (2 + Math.abs(dv));
  }
  const texture = new THREE.DataTexture(pixels, width, height);
  texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
  texture.minFilter = THREE.LinearMipmapLinearFilter;
  texture.magFilter = THREE.LinearFilter;
  texture.generateMipmaps = true;
  texture.anisotropy = 4;
  texture.needsUpdate = true;
  return texture;
}
