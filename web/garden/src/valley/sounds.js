// The valley's sound: recordings only, nothing synthesized. Footsteps are five recordings by Kenney (CC0), which are
// in this repo. The five ambiences (grass in the wind, leaves, a summer evening, water at a bank, crickets) are cut
// from the Sonniss GDC bundles, which may be used in a game but not passed on as files: they are in the released
// demo and not in this repo. MISSING_ASSETS.md says which recordings they are and how to put them back.
//
// The build writes ./audio/manifest.json listing the recordings that are there, so nothing is asked for that isn't.
// Sound starts on the first click, as browsers ask; everything is fetched and decoded before that, so the click
// itself costs nothing.

const LOOPS = ['grass', 'leaves', 'summer', 'lake', 'crickets'];

export function createSounds(base = './audio/') {
  let ctx = null;
  let master = null;
  const steps = [];
  const buffers = {};
  const loops = {};
  let begun = false;
  let lastStep = -1;

  return {
    /** Fetches and decodes the recordings. Call while loading; resolves whether or not there is any sound to have. */
    async load() {
      const Ctx = window.AudioContext || window.webkitAudioContext;
      if (!Ctx) return;
      let names;
      try {
        const res = await fetch(`${base}manifest.json`);
        if (!res.ok) return;
        names = await res.json();
      } catch {
        return;
      }
      try {
        ctx = new Ctx();
        master = ctx.createGain();
        master.gain.value = 0.8;
        master.connect(ctx.destination);
        await Promise.all(
          names.map(async (name) => {
            try {
              const buffer = await ctx.decodeAudioData(await (await fetch(`${base}${name}.ogg`)).arrayBuffer());
              if (name.startsWith('footstep')) steps.push(buffer);
              else buffers[name] = buffer;
            } catch {
              // a missing recording is just a missing sound
            }
          }),
        );
      } catch {
        ctx = null;
      }
    },
    /** Call from a click. */
    start() {
      if (!ctx) return;
      if (ctx.state === 'suspended') ctx.resume();
      if (begun) return;
      begun = true;
      for (const name of LOOPS) {
        if (!buffers[name]) continue;
        const source = ctx.createBufferSource();
        source.buffer = buffers[name];
        source.loop = true;
        const gain = ctx.createGain();
        gain.gain.value = 0;
        source.connect(gain).connect(master);
        // (not all from their first second, or every visit begins the same)
        source.start(0, Math.random() * source.buffer.duration);
        loops[name] = gain;
      }
    },
    /** One footfall, 0 to 1 loud. Never the same recording twice running. */
    step(volume) {
      if (!ctx || ctx.state !== 'running' || !steps.length) return;
      let pick = Math.floor(Math.random() * steps.length);
      if (pick === lastStep) pick = (pick + 1) % steps.length;
      lastStep = pick;
      const source = ctx.createBufferSource();
      source.buffer = steps[pick];
      source.playbackRate.value = 0.92 + Math.random() * 0.16;
      const gain = ctx.createGain();
      gain.gain.value = volume;
      const pan = ctx.createStereoPanner();
      pan.pan.value = (Math.random() - 0.5) * 0.3;
      source.connect(gain).connect(pan).connect(master);
      source.start();
    },
    /**
     * @param here.wind m/s; here.gust 0 to 1, the gust passing now; here.tree 0 to 1, how far under a tree you
     *   are; here.tarn 0 to 1, how near the water; here.dusk 0 to 1
     */
    update(dt, here) {
      if (!begun) return;
      const ease = Math.min(1, dt * 1.6);
      const to = (name, value) => {
        if (loops[name]) loops[name].gain.value += (value - loops[name].gain.value) * ease;
      };
      to('grass', (0.1 + Math.min(0.5, here.wind * 0.055)) * (0.55 + 0.45 * here.gust));
      to('leaves', here.tree * (0.3 + 0.5 * here.gust));
      to('summer', 0.55 * (1 - 0.5 * here.dusk));
      to('lake', here.tarn * 0.5);
      to('crickets', here.dusk * 0.3);
    },
  };
}
