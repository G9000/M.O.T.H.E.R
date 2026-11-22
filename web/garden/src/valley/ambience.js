// The valley's ambience, made here rather than recorded. Each voice stands in for one of the recordings that can't be
// shipped (see sounds.js): wind in the grass, leaves overhead, birds of a summer day, water lapping at the pool and
// crickets as the light goes. Each returns a gain node: sounds.js turns it up and down by where you stand and the hour.

/** A few seconds of soft noise that loops without a seam: brown-ish, so it sounds like air rather than hiss. */
function noiseBuffer(ctx, seconds = 6, colour = 0.985) {
  const length = Math.floor(ctx.sampleRate * seconds);
  const buffer = ctx.createBuffer(2, length, ctx.sampleRate);
  for (let ch = 0; ch < 2; ch++) {
    const d = buffer.getChannelData(ch);
    let b = 0;
    for (let i = 0; i < length; i++) {
      b = colour * b + (Math.random() * 2 - 1) * (1 - colour) * 6;
      d[i] = b;
    }
    // cross-fade the ends so the loop has no click
    const fade = Math.floor(ctx.sampleRate * 0.25);
    for (let i = 0; i < fade; i++) {
      const k = i / fade;
      d[i] = d[i] * k + d[length - fade + i] * (1 - k);
    }
  }
  return buffer;
}
function loop(ctx, buffer, rate = 1) {
  const s = ctx.createBufferSource();
  s.buffer = buffer;
  s.loop = true;
  s.playbackRate.value = rate;
  s.start(0, Math.random() * buffer.duration);
  return s;
}
function filter(ctx, type, frequency, q = 0.7) {
  const f = ctx.createBiquadFilter();
  f.type = type;
  f.frequency.value = frequency;
  f.Q.value = q;
  return f;
}
/** A slow wandering control signal (0 centred), for things that rise and fall on their own. */
function wander(ctx, hz, depth, target) {
  const o = ctx.createOscillator();
  o.frequency.value = hz;
  const g = ctx.createGain();
  g.gain.value = depth;
  o.connect(g).connect(target);
  o.start();
  return o;
}

export function createAmbience(ctx, destination) {
  // (a gentle limiter, so no gust or chime can ever clip)
  const out = ctx.createDynamicsCompressor();
  out.threshold.value = -10;
  out.knee.value = 8;
  out.ratio.value = 6;
  out.attack.value = 0.005;
  out.release.value = 0.25;
  out.connect(destination);
  const air = noiseBuffer(ctx, 7, 0.985);
  const fine = noiseBuffer(ctx, 5, 0.6);
  const voices = {};
  const bus = (name, level = 1) => {
    const g = ctx.createGain();
    g.gain.value = 0;
    const trim = ctx.createGain();
    trim.gain.value = level;
    g.connect(trim).connect(out);
    voices[name] = g;
    return g;
  };

  // wind through grass: a broad, breathing rush
  {
    const g = bus('grass', 0.9);
    const band = filter(ctx, 'bandpass', 700, 0.45);
    const top = filter(ctx, 'lowpass', 2400);
    wander(ctx, 0.07, 280, band.frequency);
    loop(ctx, air).connect(band).connect(top).connect(g);
  }

  // leaves overhead: a high patter that flutters with the wind
  {
    const g = bus('leaves', 0.2);
    const high = filter(ctx, 'highpass', 1800);
    const band = filter(ctx, 'bandpass', 4200, 0.8);
    const flutter = ctx.createGain();
    flutter.gain.value = 0.5;
    const shake = filter(ctx, 'lowpass', 9);
    const depth = ctx.createGain();
    depth.gain.value = 1.2;
    loop(ctx, fine, 0.37).connect(shake).connect(depth).connect(flutter.gain);
    loop(ctx, fine).connect(high).connect(band).connect(flutter).connect(g);
  }

  // water at the pool's edge: a low wash and a few small lapping sounds
  {
    const g = bus('lake', 0.45);
    const low = filter(ctx, 'lowpass', 420);
    const swell = ctx.createGain();
    swell.gain.value = 0.7;
    wander(ctx, 0.23, 0.35, swell.gain);
    loop(ctx, air, 0.8).connect(low).connect(swell).connect(g);
    const lap = filter(ctx, 'bandpass', 1100, 3);
    const lapGain = ctx.createGain();
    lapGain.gain.value = 0.15;
    const ripple = filter(ctx, 'lowpass', 4);
    const rdepth = ctx.createGain();
    rdepth.gain.value = 1.2;
    loop(ctx, fine, 0.21).connect(ripple).connect(rdepth).connect(lapGain.gain);
    wander(ctx, 0.31, 300, lap.frequency);
    loop(ctx, fine, 0.9).connect(lap).connect(lapGain).connect(g);
  }

  // crickets as the light goes: two of them, each chirping in its own rhythm, one to each side
  {
    const g = bus('crickets', 0.22);
    for (const [hz, rate, group, side] of [
      [4650, 16, 0.55, -0.6],
      [4380, 19, 0.8, 0.55],
    ]) {
      const tone = ctx.createOscillator();
      tone.frequency.value = hz;
      const trill = ctx.createGain();
      trill.gain.value = 0.5;
      const pulse = ctx.createOscillator();
      pulse.type = 'square';
      pulse.frequency.value = rate;
      const pulseDepth = ctx.createGain();
      pulseDepth.gain.value = 0.5;
      pulse.connect(pulseDepth).connect(trill.gain);
      const phrase = ctx.createGain();
      phrase.gain.value = 0.5;
      const beat = ctx.createOscillator();
      beat.type = 'square';
      beat.frequency.value = group;
      const beatDepth = ctx.createGain();
      beatDepth.gain.value = 0.5;
      beat.connect(beatDepth).connect(phrase.gain);
      const pan = ctx.createStereoPanner();
      pan.pan.value = side;
      tone.connect(trill).connect(phrase).connect(pan).connect(g);
      for (const o of [tone, pulse, beat]) o.start(ctx.currentTime + Math.random());
    }
  }

  // a summer day: a faint hum of insects, and birds, sung one phrase at a time (see sing below)
  const birds = bus('summer', 1.4);
  {
    const hum = filter(ctx, 'bandpass', 5600, 6);
    const humGain = ctx.createGain();
    humGain.gain.value = 0.02;
    loop(ctx, fine).connect(hum).connect(humGain).connect(birds);
  }
  let nextPhrase = 1;
  /** One bird's phrase: a few quick whistles, gliding, each a little different, from one place in the air. */
  function sing() {
    const t0 = ctx.currentTime + 0.05;
    const pan = ctx.createStereoPanner();
    pan.pan.value = Math.random() * 1.6 - 0.8;
    const far = ctx.createGain();
    far.gain.value = 0.25 + Math.random() * 0.5;
    pan.connect(far).connect(birds);
    const notes = 2 + Math.floor(Math.random() * 5);
    const base = 2400 + Math.random() * 2200;
    let t = t0;
    for (let i = 0; i < notes; i++) {
      const o = ctx.createOscillator();
      const e = ctx.createGain();
      const len = 0.05 + Math.random() * 0.12;
      const from = base * (0.85 + Math.random() * 0.4);
      const to = from * (Math.random() < 0.5 ? 1.35 : 0.72);
      o.frequency.setValueAtTime(from, t);
      o.frequency.exponentialRampToValueAtTime(to, t + len);
      e.gain.setValueAtTime(0, t);
      e.gain.linearRampToValueAtTime(0.5, t + 0.012);
      e.gain.exponentialRampToValueAtTime(0.001, t + len);
      o.connect(e).connect(pan);
      o.start(t);
      o.stop(t + len + 0.02);
      t += len + 0.03 + Math.random() * 0.09;
    }
  }

  return {
    voices,
    /** Keeps the birds singing while their voice can be heard. */
    update(dt) {
      if (birds.gain.value < 0.02) return;
      nextPhrase -= dt;
      if (nextPhrase > 0) return;
      sing();
      nextPhrase = 1.2 + Math.random() * 4.5;
    },
    /** A soft bell, for a record or a photo opening. */
    chime() {
      const t = ctx.currentTime;
      const root = [784, 880, 988, 1175][Math.floor(Math.random() * 4)];
      for (const [mult, level, decay] of [
        [1, 0.16, 2.4],
        [2.01, 0.06, 1.4],
        [3.0, 0.03, 0.9],
        [4.12, 0.015, 0.6],
      ]) {
        const o = ctx.createOscillator();
        o.frequency.value = root * mult;
        const e = ctx.createGain();
        e.gain.setValueAtTime(0, t);
        e.gain.linearRampToValueAtTime(level, t + 0.006);
        e.gain.exponentialRampToValueAtTime(0.0001, t + decay);
        o.connect(e).connect(out);
        o.start(t);
        o.stop(t + decay + 0.05);
      }
    },
  };
}
