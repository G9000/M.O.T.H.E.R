// The valley's slow sums, off the page's thread: the ground read into the meadow's maps (1.6 million points) and
// the painting of the plain's fields and roads. Done here they take as long, but the page stays alive and the
// graphics card paints the sky and the mountains meanwhile. main.js does them itself if a worker can't be had.
import { bakeRows } from '../lib/bake.js';
import { paintFloor } from './floor.js';
import { configure, heightAt, paint } from './world.js';

self.onmessage = ({ data }) => {
  // (the land depends on how long the garden's path is: see world.js)
  configure(data.world);
  if (data.job === 'ground') {
    const { extent, extentRes, centre } = data;
    const heights = new Float32Array(extentRes * extentRes);
    const map = new Uint8Array(extentRes * extentRes * 4);
    bakeRows({ heightAt, paint, extentMin: [centre[0] - extent / 2, centre[1] - extent / 2], texel: extent / extentRes, extentRes }, heights, map, 0, extentRes);
    self.postMessage({ heights, map }, [heights.buffer, map.buffer]);
  } else if (data.job === 'floor') {
    const pixels = paintFloor(data.size);
    self.postMessage({ pixels }, [pixels.buffer]);
  }
};
