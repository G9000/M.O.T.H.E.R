import assert from 'node:assert/strict';
import test from 'node:test';
import { PerspectiveCamera, Vector3 } from 'three';
import { positionSpecimenCamera } from '../src/garden/atlas.js';
import { gardenAppearance } from '../src/garden/seed.js';

test('the lens centres the same seeded object in portrait and landscape, above the terrain', () => {
  for (const id of ['memory-001', 'memory-002', 'memory-003']) {
    const record = { x: 13, y: 4, z: -20, tall: 0.86, appearance: gardenAppearance(id) };
    for (const aspect of [0.6, 1.7]) {
      const camera = new PerspectiveCamera(68, aspect, 0.12, 40000);
      for (const rotation of [0, 1, 3]) for (const whole of [false, true]) {
        positionSpecimenCamera(camera, record, rotation, whole, () => record.y);
        const a = record.appearance;
        const h = whole || a.stone ? 0.54 : a.form === 'bellflower' ? 0.87 : 1.04;
        const centre = new Vector3(record.x, record.y + record.tall * h, record.z).project(camera);
        assert.ok(Math.abs(centre.x) + Math.abs(centre.y) < 1e-8);
        assert.ok(centre.z > -1 && centre.z < 1);
        assert.ok(camera.position.y >= record.y + 0.18);
      }
    }
  }
});
