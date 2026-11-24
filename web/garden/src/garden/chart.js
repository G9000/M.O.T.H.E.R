import { gardenRadius, pathAt, pathLength, START, TARN, tarnDistance } from '../valley/world.js';

/** The fixed survey of this hill, shared by successive live record snapshots. */
export function createGardenChart(heightAt) {
  const reach = Math.max(gardenRadius() + 14, Math.hypot(...TARN.at) + Math.max(TARN.rx, TARN.rz) * 1.6);
  const trail = [START];
  for (let t = 0; t <= pathLength(); t += 1.5) {
    const p = pathAt(t);
    trail.push([p.x, p.z]);
  }
  const toMap = (x, z, size) => [size / 2 + ((x - START[0]) / reach) * (size / 2), size / 2 + ((z - START[1]) / reach) * (size / 2)];
  // Paint the terrain and the trail once. The live layer only draws the
  // selectable records, photographs and walker at the current zoom.
  const chart = document.createElement('canvas');
  const res = 320;
  chart.width = chart.height = res;
  const ink = chart.getContext('2d');
  const relief = ink.createImageData(res, res);
  for (let row = 0; row < res; row++) {
    for (let col = 0; col < res; col++) {
      const x = START[0] + ((col + 0.5) / res - 0.5) * 2 * reach;
      const z = START[1] + ((row + 0.5) / res - 0.5) * 2 * reach;
      const height = heightAt(x, z);
      const band = height / 0.85;
      const contour = 1 - Math.min(1, Math.abs(band - Math.round(band)) * 7);
      const underwater = tarnDistance(x, z) < 1.32 && height < TARN.level;
      const tint = Math.min(1, Math.max(0, height / 24));
      const i = (row * res + col) * 4;
      relief.data[i] = underwater ? 49 : 36 + tint * 29 + contour * 19;
      relief.data[i + 1] = underwater ? 100 : 65 + tint * 26 + contour * 18;
      relief.data[i + 2] = underwater ? 107 : 60 + tint * 17 + contour * 13;
      relief.data[i + 3] = 255;
    }
  }
  ink.putImageData(relief, 0, 0);
  ink.strokeStyle = '#e0c69b';
  ink.lineWidth = 2.5;
  ink.lineCap = ink.lineJoin = 'round';
  ink.beginPath();
  trail.forEach(([x, z], i) => {
    const [u, w] = toMap(x, z, res);
    if (i) ink.lineTo(u, w);
    else ink.moveTo(u, w);
  });
  ink.stroke();
  return { chart, toMap };
}
