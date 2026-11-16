// Reading a ground into the field's two maps. In a file of its own, with nothing imported, so a worker can do it
// off the page's thread without carrying the rest of the library.

const clamp01 = (v) => Math.min(1, Math.max(0, v));

/** Rows from..to of the height map and the map of what grows where. */
export function bakeRows({ heightAt, paint, extentMin, texel, extentRes }, heights, map, from, to) {
  for (let j = from; j < to; j++) {
    for (let i = 0; i < extentRes; i++) {
      const x = extentMin[0] + (i + 0.5) * texel;
      const z = extentMin[1] + (j + 0.5) * texel;
      const k = i + j * extentRes;
      heights[k] = heightAt(x, z);
      const p = paint(x, z);
      map[k * 4] = Math.round(255 * clamp01(p.dry ?? 0.3));
      map[k * 4 + 1] = Math.round(255 * clamp01(p.growth ?? 1));
      map[k * 4 + 2] = Math.round(255 * clamp01(p.wet ?? 0));
      map[k * 4 + 3] = Math.round(255 * clamp01(p.flowers ?? 0));
    }
  }
}
