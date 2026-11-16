// Brushwork, as a picture that repeats: dabs and short strokes, some lighter than the ground they lie on and some
// darker, and a scatter of stones. The ground reads it at several sizes, so a meadow seen from off is made of
// marks at every scale, as a painted one is. Red is the tone (half is none); blue is where a stone lies.
import * as THREE from 'three';

function mulberry(seed) {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export function createStrokes(renderer, size = 512) {
  const canvas = document.createElement('canvas');
  canvas.width = canvas.height = size;
  const g = canvas.getContext('2d');
  const random = mulberry(8128);
  g.fillStyle = 'rgb(128,0,0)';
  g.fillRect(0, 0, size, size);
  /** An ellipse, drawn again wherever it crosses an edge, so the picture repeats without a seam. */
  const mark = (x, y, rx, ry, turn, style) => {
    g.fillStyle = style;
    for (const ox of [-size, 0, size]) {
      for (const oy of [-size, 0, size]) {
        if (x + ox < -rx - ry || x + ox > size + rx + ry || y + oy < -rx - ry || y + oy > size + rx + ry) continue;
        g.beginPath();
        g.ellipse(x + ox, y + oy, rx, ry, turn, 0, Math.PI * 2);
        g.fill();
      }
    }
  };
  // broad soft patches first, then strokes on them, then small dabs
  for (let i = 0; i < 70; i++) mark(random() * size, random() * size, 26 + random() * 44, 12 + random() * 20, random() * Math.PI, `rgba(${random() < 0.5 ? 86 : 172},0,0,0.5)`);
  for (let i = 0; i < 420; i++) {
    const tone = random() < 0.52 ? 40 + random() * 60 : 160 + random() * 80;
    mark(random() * size, random() * size, 7 + random() * 16, 2 + random() * 4, random() * Math.PI, `rgba(${Math.round(tone)},0,0,0.85)`);
  }
  for (let i = 0; i < 900; i++) {
    const tone = random() < 0.5 ? 30 + random() * 60 : 170 + random() * 85;
    mark(random() * size, random() * size, 2 + random() * 4, 1 + random() * 2, random() * Math.PI, `rgb(${Math.round(tone)},0,0)`);
  }
  // stones, in loose groups
  g.globalCompositeOperation = 'lighter';
  for (let k = 0; k < 9; k++) {
    const cx = random() * size;
    const cy = random() * size;
    const n = 2 + Math.floor(random() * 5);
    for (let i = 0; i < n; i++) {
      mark(cx + (random() - 0.5) * 90, cy + (random() - 0.5) * 90, 4 + random() * 9, 3 + random() * 5, random() * Math.PI, 'rgb(0,0,255)');
    }
  }
  const texture = new THREE.CanvasTexture(canvas);
  texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
  texture.generateMipmaps = true;
  texture.minFilter = THREE.LinearMipmapLinearFilter;
  texture.magFilter = THREE.LinearFilter;
  // (always seen at a low angle)
  texture.anisotropy = renderer.capabilities.getMaxAnisotropy();
  return texture;
}
