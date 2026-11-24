// A botanical plate for the reading and loading screens. Drawn once per
// canvas so the text-only Garden has artwork even without a WebGL context.
export function illustrateWisteria(canvas) {
  const g = canvas.getContext('2d');
  if (!g) return;
  canvas.width = 720;
  canvas.height = 340;
  const sprig = (x, y, angle, length) => {
    g.save();
    g.translate(x, y);
    g.rotate(angle);
    g.strokeStyle = '#baad81';
    g.lineWidth = 1.4;
    g.beginPath();
    g.moveTo(0, 0);
    g.quadraticCurveTo(length * 0.4, -6, length, 0);
    g.stroke();
    for (let i = 1; i <= 6; i++) {
      for (const side of [-1, 1]) {
        g.save();
        g.translate(length * (i - 0.4) / 7, -2);
        g.scale(1, side);
        const h = 20 + 14 * Math.sin(i / 6 * Math.PI);
        g.fillStyle = side < 0 ? '#53786c' : '#42695d';
        g.beginPath();
        g.moveTo(0, 0);
        g.bezierCurveTo(-2, -12, 13, -h, 25, -h);
        g.quadraticCurveTo(23, -8, 0, 0);
        g.fill();
        g.strokeStyle = '#b0b491';
        g.lineWidth = 0.8;
        g.stroke();
        g.beginPath();
        g.moveTo(0, 0);
        g.quadraticCurveTo(13, -12, 25, -h);
        g.stroke();
        g.restore();
      }
    }
    g.restore();
  };
  const cluster = (x, y, length, phase) => {
    g.lineWidth = 1.4;
    g.strokeStyle = '#a79786';
    g.beginPath();
    g.moveTo(x, y);
    g.quadraticCurveTo(x - 12, y + length * 0.6, x + 4, y + length);
    g.stroke();
    const colours = ['#af96b6', '#d0b9c8', '#8e799c', '#dbc4d5', '#aa93b3'];
    for (let i = 0; i < 20; i++) {
      const t = i / 20;
      const size = 4 + 12 * Math.sin(Math.PI * (0.16 + t * 0.83));
      const turn = i * 2.4 + phase;
      g.save();
      g.translate(x - 7 * Math.sin(t * Math.PI) + Math.cos(turn) * size, y + length * t);
      g.rotate(Math.cos(turn) * 0.60);
      g.fillStyle = colours[(i + phase) % colours.length];
      g.strokeStyle = '#665b71';
      g.lineWidth = 0.85;
      g.beginPath();
      g.moveTo(0, 0);
      g.bezierCurveTo(-size * 1.3, -size / 2, -size, size * 0.9, 0, size);
      g.bezierCurveTo(size * 0.8, size * 0.75, size, -size * 0.35, 0, 0);
      g.fill();
      g.stroke();
      g.beginPath();
      g.moveTo(0, size * 0.18);
      g.quadraticCurveTo(-size * 0.2, size * 0.6, 0, size * 0.82);
      g.stroke();
      g.restore();
    }
  };
  g.strokeStyle = '#ac9975';
  g.lineWidth = 4;
  g.beginPath();
  g.moveTo(760, -16);
  g.bezierCurveTo(570, 12, 424, 98, 133, 65);
  g.stroke();
  sprig(273, 72, -2.77, 156);
  sprig(399, 63, -2.32, 116);
  sprig(461, 53, -0.32, 123);
  for (const args of [[265, 72, 89, 3], [340, 71, 151, 1], [390, 66, 113, 4], [448, 60, 202, 2], [508, 46, 134, 1], [575, 25, 193, 3], [636, 13, 120, 0]]) cluster(...args);
  sprig(615, 16, 0.73, 163);
}
