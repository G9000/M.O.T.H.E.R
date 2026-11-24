import assert from 'node:assert/strict';
import test from 'node:test';
import { appearanceFromSeed, COMPANIONS, companionAppearance, FORMS, gardenAppearance } from '../src/garden/seed.js';

test('existing record IDs keep their appearance and their decimal seeds reproduce it', () => {
  for (const [id, seed, form] of [
    ['memory-001', '2699649361508293808', 'river stone'],
    ['memory-002', '2699652660043178441', 'bellflower'],
    ['memory-003', '2699651560531550230', 'tulip'],
    ['garden🌿', '5953555404226732416', 'rose'],
  ]) {
    const a = gardenAppearance(id);
    assert.equal(a.seed, seed);
    assert.equal(a.form, form);
    assert.deepEqual(appearanceFromSeed(seed), a);
  }
  const specimen = gardenAppearance('memory-002');
  assert.deepEqual([specimen.width, specimen.height, specimen.curve, specimen.structure].map(v => Number(v.toFixed(6))), [1.228014, 1.131821, 0.058495, 0]);
});

test('the same record seed recreates a companion independently of its original specimen', () => {
  const companions = Array.from({length: 512}, (_, i) => {
    const original = gardenAppearance('memory-' + i);
    const companion = companionAppearance(original.seed);
    assert.deepEqual(companionAppearance(BigInt(original.seed)), companion);
    assert.deepEqual(appearanceFromSeed(companion.seed), original);
    return companion;
  });
  for (const form of COMPANIONS) {
    assert.equal(new Set(companions.filter(a => a.form === form).map(a => a.structure)).size, 3);
  }
});

test('a collection of memories spans every form and varies within each form', () => {
  const samples = Array.from({length: 512}, (_, i) => gardenAppearance('memory-' + i));
  assert.equal(new Set(samples.map(a => a.seed)).size, samples.length);
  for (const form of FORMS) {
    const group = samples.filter(a => a.form === form);
    assert.ok(new Set(group.map(a => a.width)).size > 1);
    if (form !== 'river stone') assert.equal(new Set(group.map(a => a.structure)).size, 3);
  }
});
