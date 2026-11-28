import test from 'node:test';
import assert from 'node:assert/strict';
import { rankMoments } from '../src/garden/moments.js';

test('clues rank matching words, showing keepsakes only when requested', () => {
  const records = [
    {id: 'a', title: 'Presented without a script'},
    {id: 'b', title: 'Script notes'},
    {id: 'c', title: 'Presented without a script', keepsake: true},
    {id: 'd', title: 'Presented without a script', status: 'retracted'},
    {id: 'e', title: 'Presented without a script', sourceReview: {state: 'needs_review'}},
  ];
  const clue = 'Show me the day I presented without a script';
  assert.deepEqual(rankMoments(records, clue).map((h) => h.record.id), ['a', 'b']);
  assert.deepEqual(rankMoments(records, clue)[0].matched, ['presented', 'without', 'script']);
  assert.deepEqual(rankMoments(records, clue, true).map((h) => h.record.id), ['a', 'c', 'b']);
  assert.deepEqual(rankMoments(records, 'show me the day'), []);
});
