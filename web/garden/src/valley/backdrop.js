// The far country, painted once: wooded foothills, a range of snow mountains behind them and a paler range behind
// that, drawn the way a matte painter would, into a strip of picture that wraps the horizon. It stands too far off
// for a walk to shift it, so it can be worked as hard as a painting is. The ground is built for the brush: sharp
// crests with broad clean flanks, a few great peaks with passes between them. It is painted in few tones with
// firm edges, light from the side, snow running down the gullies, and each range sunk further into the air than
// the one in front. What is kept is the paint and not the picture: each point's colour in the light, its colour in
// shade, how lit it is and how much air lies in front of it. The sky's dome mixes those with the hour's light, so
// the hour can change without the mountains being painted again.
//
// It is made in two goes. The ground's height is worked out once into a map; the painting then reads the map,
// which is what lets it look a long way along every ray, cast every shadow, and still be ready in a moment.
import * as THREE from 'three';

/** The side of the square of country the height map covers, km, with the walk at its middle. */
const SPAN = 86;

const vertex = /* glsl */ `
out vec2 vUv;
void main() {
  vUv = position.xy * 0.5 + 0.5;
  gl_Position = vec4(position.xy, 0.0, 1.0);
}
`;

const NOISE = /* glsl */ `
float hash(vec2 p) {
  p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
  return fract(p.x * p.y * (p.x + p.y));
}

float noise(vec2 x) {
  vec2 p = floor(x);
  vec2 f = fract(x);
  vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
  float a = hash(p);
  float b = hash(p + vec2(1.0, 0.0));
  float c = hash(p + vec2(0.0, 1.0));
  float d = hash(p + vec2(1.0, 1.0));
  return a + (b - a) * u.x + (c - a) * u.y + (a - b - c + d) * u.x * u.y;
}

// Cells: how far it is to the edge of the cell a point is in, and the cell's name.
vec3 voro(vec2 x) {
  vec2 n = floor(x);
  vec2 f = x - n;
  float d1 = 8.0;
  float d2 = 8.0;
  vec2 id = n;
  for (int j = -1; j <= 1; j++) {
    for (int i = -1; i <= 1; i++) {
      vec2 g = vec2(float(i), float(j));
      vec2 r = g + vec2(hash(n + g), hash(n + g + 17.3)) - f;
      float d = dot(r, r);
      if (d < d1) { d2 = d1; d1 = d; id = n + g; }
      else if (d < d2) d2 = d;
    }
  }
  return vec3(sqrt(d2) - sqrt(d1), id);
}


// Where the hills are farmed: 0 to 1.
float farmAt(vec2 q) {
  return smoothstep(0.42, 0.52, noise(q * 0.42 + 11.0));
}

// The field a point is in: how far it is to the field's edge, and the field's name.
vec3 fieldOf(vec2 q) {
  return voro(q * 1.9 + 0.6 * vec2(noise(q * 0.7), noise(q * 0.7 + 5.0)));
}

// Trees on a far hill, as paint. x: 1 where there is tree. y: how high the crown stands there, 0 to 1. Woods whose
// edges are whole round crowns; broken rows of trees along the fields' edges; trees alone and in twos and threes
// on the open grass; bushes.
vec2 treesAt(vec2 q) {
  float m = noise(q * 1.3 + 3.0) * 0.6 + noise(q * 4.3 + 9.0) * 0.4 + 0.07 * smoothstep(9.5, 12.5, length(q));
  vec3 c = voro(q * 9.0);
  float mass = max(step(0.66, m), step(0.61, m + (hash(c.yz + 2.0) - 0.5) * 0.12) * step(0.1, c.x));
  vec3 f = fieldOf(q);
  vec3 b = voro(q * 12.0 + 3.0);
  float row = step(f.x, 0.05) * step(0.3, hash(f.yz + 8.2)) * farmAt(q) * step(0.12, b.x) * step(0.3, hash(b.yz + 1.0));
  vec3 one = voro(q * 4.5 + 7.0);
  float alone = step(0.8, hash(one.yz + 5.0)) * step(0.3, one.x) * step(0.36, m);
  vec3 bu = voro(q * 15.0 + 1.0);
  float bush = step(0.95, hash(bu.yz + 3.0)) * step(0.22, bu.x);
  float dome = max(mass * (0.4 + 0.6 * sqrt(clamp(c.x * 2.2, 0.0, 1.0))), alone * sqrt(clamp((one.x - 0.3) * 4.0, 0.0, 1.0)));
  dome = max(dome, row * 0.75 * sqrt(clamp((b.x - 0.12) * 3.5, 0.0, 1.0)));
  dome = max(dome, bush * 0.4 * sqrt(clamp((bu.x - 0.22) * 4.0, 0.0, 1.0)));
  return vec2(max(max(mass, alone), max(row, bush)), dome);
}
`;

const heightFragment = /* glsl */ `
precision highp float;
layout(location = 0) out vec4 outC;
in vec2 vUv;
uniform vec2 uHero;    // the way the valley runs
uniform int uDetail;   // octaves
const mat2 M2 = mat2(0.8, -0.6, 0.6, 0.8);
${NOISE}

// value noise and its slope
vec3 noised(vec2 x) {
  vec2 p = floor(x);
  vec2 f = fract(x);
  vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
  vec2 du = 30.0 * f * f * (f * (f - 2.0) + 1.0);
  float a = hash(p);
  float b = hash(p + vec2(1.0, 0.0));
  float c = hash(p + vec2(0.0, 1.0));
  float d = hash(p + vec2(1.0, 1.0));
  return vec3(a + (b - a) * u.x + (c - a) * u.y + (a - b - c + d) * u.x * u.y, du * (vec2(b - a, c - a) + (a - b - c + d) * u.yx));
}

// rounded, water-worn ground: each finer fold is laid on less where the ground is already steep (the foothills)
float erode(vec2 p, int oct) {
  float a = 0.0;
  float b = 1.0;
  vec2 d = vec2(0.0);
  for (int i = 0; i < 9; i++) {
    if (i >= oct) break;
    vec3 n = noised(p);
    d += n.yz;
    a += b * n.x / (1.0 + dot(d, d));
    b *= 0.5;
    p = M2 * p * 2.0;
  }
  return a;
}

// alpine ground: sharp crests, and the finer folds only along them, so the flanks between are broad and clean
float ridged(vec2 p, int oct) {
  float a = 0.0;
  float amp = 0.5;
  float w = 1.0;
  for (int i = 0; i < 10; i++) {
    if (i >= oct) break;
    float r = 1.0 - abs(2.0 * noise(p) - 1.0);
    r *= r;
    a += r * amp * w;
    w = clamp(r * 1.7, 0.0, 1.0);
    amp *= 0.5;
    p = M2 * p * 2.07 + 3.7;
  }
  return a;
}

void main() {
  vec2 p = (vUv - 0.5) * ${SPAN.toFixed(1)};
  float r = length(p);
  vec2 dir = p / max(r, 1e-3);
  float ahead = 0.5 + 0.5 * smoothstep(-0.6, 0.7, dot(dir, uHero));
  // wooded foothills, in ranks
  // (their detail is painted on them, not modelled into them: lumps are what makes far ground look computed)
  // three ranks of rounded hills, each behind and above the last, so that one's outline lies across the next
  float h = 0.0;
  for (int k = 0; k < 3; k++) {
    float fk = float(k);
    float at = 7.4 + 2.2 * fk + 1.3 * (noise(p * 0.19 + vec2(3.1 + 9.0 * fk, 7.7)) - 0.5) + 0.8 * (noise(p * 0.62 + vec2(5.0 * fk, 2.0)) - 0.5);
    float off = (r - at) / (1.0 + 0.35 * fk);
    // (hills a kilometre or so across, on broader ones: seen from the walk a rank is several hills, not one long back)
    float lump = smoothstep(0.2, 0.75, noise(p * (0.42 - 0.07 * fk) + vec2(1.9, 4.3 + 5.0 * fk)));
    float crest = (noise(p * (1.05 - 0.15 * fk) + vec2(7.3 + 4.0 * fk, 1.1)) - 0.5) * 0.34 + (noise(p * 2.3 + vec2(2.0, 9.0 * fk)) - 0.5) * 0.08;
    h = max(h, max((0.42 + 0.62 * fk) * (0.42 + 0.58 * lump) + crest, 0.0) * exp(-off * off));
  }
  // trees stand up out of them: crowns are part of the ground's shape, so they have outlines and hide what is behind
  vec2 wood = treesAt(p);
  h += 0.055 * wood.x * wood.y * (1.0 - smoothstep(1.45, 1.75, h)) * smoothstep(5.6, 7.0, r) * (1.0 - smoothstep(13.5, 15.0, r));
  // the range: massifs with passes between them
  vec2 q = p + 1.6 * vec2(noise(p * 0.11 + 3.0), noise(p * 0.11 + 9.0));
  float mass = smoothstep(0.34, 0.6, noise(p * 0.052 + 9.3));
  float band = smoothstep(12.0, 17.5, r) * (1.0 - 0.65 * smoothstep(21.0, 26.0, r));
  float m = ridged(q * 0.095 + vec2(11.3, 1.9), uDetail);
  h += pow(m, 1.25) * 4.4 * (0.22 + 0.78 * mass) * band * ahead;
  // two great peaks, where the eye goes: one a little left of the valley's line, one well to its right
  vec2 perp = vec2(-uHero.y, uHero.x);
  float d1 = length(p - (uHero * 18.5 - perp * 4.2));
  float d2 = length(p - (uHero * 17.0 + perp * 11.5));
  h += exp(-d1 * d1 / 7.5) * (0.7 + 1.6 * m) + exp(-d2 * d2 / 8.0) * (0.5 + 1.4 * m);
  // and a further range, seen through the passes
  h += pow(ridged(p * 0.07 + vec2(31.0, 17.0), 7), 1.2) * 3.6 * smoothstep(25.5, 31.0, r);
  outC = vec4(h, 0.0, 0.0, 1.0);
}
`;

const paintFragment = /* glsl */ `
precision highp float;
precision highp sampler2D;
layout(location = 0) out vec4 oLight;   // its colour in the light; 1 where there is land
layout(location = 1) out vec4 oShade;   // its colour in shade; how lit it is
layout(location = 2) out vec4 oAir;     // metres of air before it, thinned with height; mist
in vec2 vUv;
uniform sampler2D tHeight;
uniform float uTexel;  // one texel of the height map, km
uniform vec2 uSide;    // the side the afternoon's light comes from, level
uniform vec2 uHero;    // the way the valley runs
uniform vec2 uEl;      // the strip's lowest and highest elevation, radians
uniform float uEye;    // the eye's height, km

const float TAU = 6.28318531;
${NOISE}

// The land's height at p. Kilometres both.
float height(vec2 p) {
  return textureLod(tHeight, p / ${SPAN.toFixed(1)} + 0.5, 0.0).r;
}

float hash3(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}

// Cells in space: how far it is to the edge of the cell a point is in, and three numbers that are the cell's own.
vec4 voro3(vec3 x) {
  vec3 n = floor(x);
  vec3 f = x - n;
  float d1 = 8.0;
  float d2 = 8.0;
  vec3 id = n;
  for (int k = -1; k <= 1; k++) {
    for (int j = -1; j <= 1; j++) {
      for (int i = -1; i <= 1; i++) {
        vec3 g = vec3(float(i), float(j), float(k));
        vec3 c = n + g;
        vec3 r = g + vec3(hash3(c), hash3(c + 17.3), hash3(c + 41.7)) - f;
        float d = dot(r, r);
        if (d < d1) { d2 = d1; d1 = d; id = c; }
        else if (d < d2) d2 = d;
      }
    }
  }
  return vec4(sqrt(d2) - sqrt(d1), hash3(id + 3.1), hash3(id + 7.7), hash3(id + 12.9));
}

// A bank of cloud lying along the feet of the range, as painted backgrounds have one: the hills in front stand
// dark against it and the peaks rise out of it, and that is three layers where there were two. Returns how much
// of it there is this way; lit comes back as how much of that is its sunlit top.
float bankAt(float az, float el, float u, out float lit) {
  vec2 round_ = vec2(cos(az), sin(az));
  float top = 0.158 + 0.02 * (noise(round_ * 2.3 + 3.0) - 0.5) * 2.0;
  // heaped tops, big heaps with small ones on them, no two alike: rounded above, creased where they meet
  float h1 = sqrt(abs(2.0 * noise(round_ * 9.0 + 1.7) - 1.0));
  float h2 = sqrt(abs(2.0 * noise(round_ * 27.0 + 5.1) - 1.0));
  float h3 = sqrt(abs(2.0 * noise(round_ * 71.0 + 9.4) - 1.0));
  top += 0.022 * h1 + 0.014 * h2 + 0.006 * h3;
  // it lies in lengths, with gaps where the mountains come down to their feet
  // (s: radians right of the valley's line)
  float s = atan(dot(round_, vec2(-uHero.y, uHero.x)), dot(round_, uHero));
  float here = max(1.0 - smoothstep(0.2, 0.46, abs(s - 0.44)), 1.0 - smoothstep(0.14, 0.34, abs(s + 0.98)));
  float peak = 1.0 - smoothstep(0.14, 0.38, abs(s + 0.2));
  here = max(here, peak);
  top += 0.014 * peak;
  here = max(here, smoothstep(0.5, 0.64, noise(round_ * 1.4 + 5.0)) * smoothstep(1.5, 1.9, abs(s)));
  top -= (1.0 - here) * 0.26;
  // inside, it is heaps too: each lit above and shaded below
  // (read along whichever way round the horizon is running here, so it is never stretched)
  float billow = mix(noise(vec2(round_.x * 38.0, el * 70.0)), noise(vec2(round_.y * 38.0 + 7.0, el * 70.0)), round_.x * round_.x);
  // its heaps are lit above and in shade below, and the whole of it darkens downwards
  float heap = mix(noise(vec2(round_.x * 15.0 + 2.0, el * 34.0)), noise(vec2(round_.y * 15.0 + 9.0, el * 34.0)), round_.x * round_.x);
  // (heaps within it too: round, each white above)
  float h4 = sqrt(abs(2.0 * mix(noise(vec2(round_.x * 21.0 + 4.0, el * 46.0)), noise(vec2(round_.y * 21.0 + 8.0, el * 46.0)), round_.x * round_.x) - 1.0));
  float down = top - el;
  lit = step(down - 0.034 * h4 - (billow - 0.5) * 0.02, 0.026) + step(0.6, h4 * (0.55 + 0.45 * heap)) * step(down, 0.09);
  lit = clamp(lit, 0.0, 1.0);
  float cover = smoothstep(0.0, 0.0018, top - el + (billow - 0.5) * 0.008);
  return cover;
}

vec3 normalAt(vec2 p, float e) {
  return normalize(vec3(height(p - vec2(e, 0.0)) - height(p + vec2(e, 0.0)), 2.0 * e, height(p - vec2(0.0, e)) - height(p + vec2(0.0, e))));
}

void main() {
  float az = vUv.x * TAU - 3.14159265;
  float el = mix(uEl.x, uEl.y, vUv.y);
  vec3 rd = vec3(cos(az) * cos(el), sin(el), sin(az) * cos(el));
  vec3 ro = vec3(0.0, uEye, 0.0);
  float flat_ = length(rd.xz);
  float t = 5.0 / flat_;
  float tMax = 41.5 / flat_;
  float before = t;
  bool hit = false;
  for (int i = 0; i < 520; i++) {
    vec3 p = ro + rd * t;
    float dy = p.y - height(p.xz);
    if (dy < 0.0) { hit = true; break; }
    if (t > tMax || (rd.y > 0.0 && p.y > 7.8)) break;
    before = t;
    t += max(0.0045 * t, 0.4 * dy);
  }
  float bankLit;
  float bank = bankAt(az, el, vUv.x, bankLit);
  if (!hit) {
    if (rd.y < 0.0) {
      // below the horizon and nothing struck: the plain goes on under the haze
      oLight = vec4(sqrt(vec3(0.2, 0.3, 0.15)), 1.0);
      oShade = vec4(sqrt(vec3(0.2, 0.3, 0.15)), 1.0);
      oAir = vec4(1.0, 0.0, 0.0, 0.0);
    } else {
      // open sky, but for the bank
      oLight = vec4(0.0, 0.0, 0.0, bank);
      oShade = vec4(0.0);
      oAir = vec4(0.0, 0.0, bank, bank * bankLit);
    }
    return;
  }
  float lo = before;
  float hi = t;
  for (int k = 0; k < 9; k++) {
    float mid = 0.5 * (lo + hi);
    vec3 q = ro + rd * mid;
    if (q.y - height(q.xz) > 0.0) lo = mid; else hi = mid;
  }
  t = hi;
  vec3 p = ro + rd * t;
  float alt = p.y;
  float r = length(p.xz);
  // the faces a brush would lay in: broad
  vec3 N = normalAt(p.xz, max(0.014 + 0.0022 * t, uTexel * 2.0));
  // and the ground as it is, for where rock breaks out and snow cannot lie
  float crag = 1.0 - normalAt(p.xz, uTexel).y;

  // A painter lights far mountains from the side wherever they stand in the picture, so their forms turn: here
  // the light comes from whichever side of the view the sun is on, and low.
  vec2 vh = rd.xz / flat_;
  vec2 rt = vec2(-vh.y, vh.x);
  float side = clamp(dot(rt, uSide) * 3.0, -1.0, 1.0);
  vec2 lh = normalize(rt * side * 0.9 + uSide * 0.45);
  vec3 L = normalize(vec3(lh.x, 0.45, lh.y));

  // the shadow one ridge throws on the next
  float sh = 1.0;
  float ts = 0.05;
  for (int i = 0; i < 40; i++) {
    vec3 q = p + L * ts;
    float dq = q.y - height(q.xz);
    sh = min(sh, 14.0 * dq / ts);
    if (sh < 0.0 || q.y > 7.8) break;
    ts += max(0.04, 0.09 * ts);
  }
  sh = clamp(sh, 0.0, 1.0);

  // gullies: streaks that run down the fall line, for snow to lie in and rock to show between
  vec2 fall = N.xz / max(length(N.xz), 1e-3);
  vec2 contour = vec2(-fall.y, fall.x);
  // (read across the slope whichever way it faces, from marks that stay where they are on the ground)
  float wx = fall.x * fall.x;
  float streak = mix(noise(vec2(p.x * 11.0, alt * 1.3 + 4.0)), noise(vec2(p.z * 11.0, alt * 1.3 + 4.0)), wx) * 0.6 + mix(noise(vec2(p.x * 3.7 + 9.0, alt * 0.6)), noise(vec2(p.z * 3.7 + 9.0, alt * 0.6)), wx) * 0.4;
  float brush = noise(p.xz * 2.3 + 7.0);

  // what grows where
  float treeline = 1.5 + 0.3 * brush;
  // The green hills are open grass, lit in two flat tones off their broad shape, with trees standing on them.
  vec3 Nb = normalAt(p.xz, 0.09);
  // (their light turns softly into shade, as a wash does)
  float hillLit = smoothstep(0.0, 0.22, dot(Nb, L)) * smoothstep(0.25, 0.5, sh);
  float below = (1.0 - smoothstep(treeline - 0.1, treeline + 0.06, alt + crag * 0.3)) * (1.0 - smoothstep(0.5, 0.72, crag));
  // (more of them in the folds and on the shaded sides, fewer high up)
  float wooded = treesAt(p.xz).x * below;
  // the shade a wood throws on the grass beside it
  float thrown = treesAt(p.xz + L.xz * 0.07).x * below * (1.0 - wooded);
  float meadow = (1.0 - smoothstep(1.9, 2.3, alt + (streak - 0.5) * 0.5)) * (1.0 - smoothstep(0.3, 0.55, crag));
  float rocky = (1.0 - wooded) * (1.0 - 0.9 * meadow);

  // Rock as it is painted: broken into facets, large ones and small ones on them, each turned its own way to the
  // light, the joints between them drawn dark. This is where a far mountain gets its detail.
  vec3 fq = vec3(p.x, alt * 1.6, p.z);
  vec4 f1 = voro3(fq * 2.6);
  vec4 f2 = voro3(fq * 7.5 + 5.0);
  vec4 f3 = voro3(fq * 19.0 + 11.0);
  vec2 t1 = f1.yz - 0.5;
  vec2 t2 = f2.yz - 0.5;
  vec2 t3 = f3.yz - 0.5;
  vec3 Nf = normalize(N + (vec3(contour.x, 0.0, contour.y) * (t1.x * 1.0 + t2.x * 0.75 + t3.x * 0.4) + vec3(0.0, t1.y * 0.8 + t2.y * 0.6 + t3.y * 0.35, 0.0)) * rocky);
  float joint = ((1.0 - smoothstep(0.0, 0.05, f1.x)) * 0.8 + (1.0 - smoothstep(0.0, 0.08, f2.x)) * 0.36 + (1.0 - smoothstep(0.0, 0.1, f3.x)) * 0.1) * rocky;
  float tone = f1.w * 0.4 + f2.w * 0.35 + f3.w * 0.25;

  // rock and snow turn from light to shade along an edge; a wooded hill turns slowly
  float ndl = dot(Nf, L);
  // (in three clear tones, and each crown and tuft tips it one way or the other)
  float turn = dot(N, L) + (fract(f2.w * 7.3) - 0.5) * 0.3;
  float soft = smoothstep(-0.05, 0.02, turn) * 0.5 + smoothstep(0.26, 0.33, turn) * 0.5;
  float crownTurn = dot(normalAt(p.xz, uTexel * 1.5), L);
  // a crown is lobes of leaf, each catching the light or not: lighter at its middle, and on the side the sun is
  vec3 lobe = voro(p.xz * 26.0 + 2.0);
  float tscore = crownTurn * 0.9 + (hash(lobe.yz + 6.0) - 0.5) * 0.5 + (lobe.x - 0.14) * 0.9;
  float hillTone = mix(hillLit * (1.0 - 0.9 * thrown), smoothstep(-0.02, 0.05, tscore) * smoothstep(0.3, 0.4, sh), wooded);
  float lit = mix(smoothstep(0.0, 0.12, ndl) * smoothstep(0.1, 0.5, sh), hillTone, max(wooded, meadow));

  // snow: whole only on the highest tops; below them it lies on the ledges, a facet here and a facet there
  float deep = smoothstep(2.6, 5.4, alt + (streak - 0.5) * 0.9 + 0.3 * noise(p.xz * 0.45 + 2.0));
  float snow = max(step(fract(f1.y * 7.31 + f1.w * 3.7), deep * 0.62) * (1.0 - 0.85 * smoothstep(0.5, 0.78, crag)), smoothstep(0.93, 1.0, deep));
  snow = max(snow, step(fract(f2.z * 5.17 + f2.w * 2.3), deep * 0.12)) * (1.0 - wooded) * step(0.02, deep);

  // rock: cream and warm grey in the light; out of it grey-green, not blue
  vec3 light = mix(vec3(0.38, 0.34, 0.26), vec3(0.68, 0.62, 0.47), tone);
  vec3 shade = mix(vec3(0.25, 0.27, 0.23), vec3(0.43, 0.43, 0.36), tone);
  // (lichen and thin grass stain it, lower down most)
  float stain = smoothstep(0.4, 0.64, noise(p.xz * 1.7 + 11.0) * 0.6 + fract(f2.z * 9.1 + f2.w * 3.3) * 0.4) * (1.0 - smoothstep(2.3, 3.9, alt));
  light = mix(light, vec3(0.4, 0.4, 0.17), stain * 0.8);
  shade = mix(shade, vec3(0.2, 0.26, 0.15), stain * 0.7);
  light *= 1.0 - 0.55 * joint;
  shade *= 1.0 - 0.5 * joint;
  // high pasture
  // The grass, as it is painted: nowhere one colour. Soft patches of four greens at two sizes, long strokes laid in
  // two directions, a turn of tone that follows the hill round; then paths, fields and hedges, and small dabs.
  float v1 = noise(p.xz * 0.9 + 1.0);
  float v2 = noise(p.xz * 2.7 + 4.0);
  float s1 = noise(vec2(dot(p.xz, vec2(0.8, 0.6)) * 1.6, dot(p.xz, vec2(-0.6, 0.8)) * 11.0));
  float s2 = noise(vec2(dot(p.xz, vec2(0.96, -0.28)) * 2.2, dot(p.xz, vec2(0.28, 0.96)) * 17.0) + 5.0);
  float round_ = noise(vec2((alt + 0.08 * v1) * 24.0, v2 * 2.0));
  float v = v1 * 0.34 + v2 * 0.2 + s1 * 0.2 + s2 * 0.12 + round_ * 0.14;
  vec3 grassL = mix(vec3(0.17, 0.46, 0.14), vec3(0.27, 0.57, 0.13), smoothstep(0.36, 0.5, v));
  grassL = mix(grassL, vec3(0.42, 0.66, 0.15), smoothstep(0.54, 0.66, v));
  vec3 grassS = mix(vec3(0.08, 0.31, 0.16), vec3(0.13, 0.42, 0.17), smoothstep(0.36, 0.6, v));
  // fields: each a tone of its own, a darker strip under the hedge along most of their edges
  float farmed = farmAt(p.xz) * (1.0 - smoothstep(0.2, 0.4, 1.0 - Nb.y));
  vec3 fld = fieldOf(p.xz);
  float kind = hash(fld.yz + 1.7);
  vec3 tint = kind < 0.25 ? vec3(1.3, 1.12, 0.9) : kind < 0.5 ? vec3(0.8, 0.92, 1.0) : kind < 0.62 ? vec3(1.75, 1.2, 1.1) : vec3(1.0);
  grassL = mix(grassL, grassL * tint, farmed);
  grassS = mix(grassS, grassS * mix(vec3(1.0), tint, 0.6), farmed);
  float hedge = (1.0 - smoothstep(0.02, 0.05, fld.x)) * step(0.3, hash(fld.yz + 8.2)) * farmed;
  grassL = mix(grassL, vec3(0.07, 0.27, 0.11), hedge * 0.8);
  grassS = mix(grassS, vec3(0.04, 0.17, 0.11), hedge * 0.8);
  // paths: pale, winding, here and gone
  float way = 1.0 - smoothstep(0.004, 0.008, abs(noise(p.xz * 0.5 + 7.0) - 0.5));
  way = max(way, (1.0 - smoothstep(0.004, 0.0075, abs(noise(p.xz * 0.9 + 13.0) - 0.46))) * step(0.45, v1));
  grassL = mix(grassL, vec3(0.62, 0.71, 0.3), way * 0.75);
  grassS = mix(grassS, vec3(0.3, 0.5, 0.3), way * 0.6);
  // dabs: a darker tuft, a fleck of sun, a few flowers
  vec3 d1 = voro(p.xz * 38.0);
  float dh = hash(d1.yz + 9.0);
  float dab = step(0.2, d1.x);
  grassL = mix(grassL, grassL * vec3(0.72, 0.84, 0.9), dab * step(0.8, dh));
  grassL = mix(grassL, grassL * vec3(1.25, 1.12, 0.85), dab * step(dh, 0.14));
  grassL = mix(grassL, vec3(0.86, 0.82, 0.46), step(0.3, d1.x) * step(0.955, fract(dh * 7.13)));
  grassS *= 1.0 - 0.2 * dab * step(0.8, dh);
  light = mix(light, grassL, meadow * 0.96);
  shade = mix(shade, grassS, meadow * 0.96);
  // trees: light, half-light and shade by the lobe, a touch of yellow where the sun is full on them, dark between crowns
  vec3 cr = voro(p.xz * 9.0);
  float gap = (1.0 - smoothstep(0.0, 0.05, cr.x)) * 0.6;
  vec3 treeL = mix(vec3(0.055, 0.27, 0.12), vec3(0.11, 0.37, 0.13), smoothstep(0.3, 0.36, tscore));
  treeL = mix(treeL, vec3(0.24, 0.5, 0.15), smoothstep(0.62, 0.68, tscore));
  vec3 treeS = mix(vec3(0.022, 0.11, 0.095), vec3(0.045, 0.19, 0.115), smoothstep(-0.4, -0.3, tscore));
  light = mix(light, mix(treeL, vec3(0.03, 0.14, 0.09), gap), wooded);
  shade = mix(shade, mix(treeS, vec3(0.018, 0.085, 0.085), gap), wooded);
  // snow: white, and blue-grey where the light does not reach
  light = mix(light, vec3(1.07, 1.06, 1.03) * (0.94 + 0.08 * tone), snow);
  shade = mix(shade, mix(vec3(0.6, 0.65, 0.72), vec3(0.7, 0.74, 0.8), tone), snow);

  // in the sun, a face that meets it squarely is a tone brighter than one it glances along
  light *= mix(1.0, mix(0.72, 1.05, smoothstep(0.25, 0.6, ndl)), rocky);
  // out of it a face still turns: lighter where it looks up at the sky, a tone deeper where it looks away
  shade *= mix(1.0, mix(0.78, 1.16, smoothstep(0.1, 0.5, dot(Nf, normalize(vec3(-L.x, 1.1, -L.z))))), rocky);

  // (a painter keeps more of the far country's colour than the air would)
  float thin = mix(1.0, 0.3, smoothstep(0.25, 2.6, alt));
  // mist lying at the feet of each range, so one stands clear of the next
  float lie = noise(vec2(az * 9.0, 1.0)) - 0.5;
  float mist = smoothstep(10.5, 14.5, r) * (1.0 - smoothstep(0.2, 0.85, alt + lie * 0.4)) * 0.2;
  mist = max(mist, smoothstep(25.0, 29.0, r) * (1.0 - smoothstep(0.6, 1.9, alt + lie * 0.6)) * 0.6);
  // the bank of cloud lies behind the foothills and in front of everything else
  float front = bank * smoothstep(14.0, 15.0, r);
  float land = 1.0 - front;
  // (colours as their square roots, and the air as how much of the view it has taken at a set thickness: both
  // fit eight bits, which lets the picture be half as fine again)
  oLight = vec4(sqrt(min(light, 1.0)) * land, 1.0);
  oShade = vec4(sqrt(min(shade, 1.0)) * land, lit * land);
  oAir = vec4((1.0 - exp(-t * 1000.0 * thin * 0.3 * mix(mix(0.85, 3.0, smoothstep(7.4, 12.0, r)), 1.0, rocky) * 0.00015)) * land, mist * land, front, front * bankLit);
}
`;

/**
 * @param side the afternoon sun's level direction (x, z): which side the mountains are lit from
 * @param options.width texels round the horizon; options.ground texels along a side of the height map
 */
export function createBackdrop(renderer, hero, side, { width = 8192, ground = 4096 } = {}) {
  width = Math.min(width, renderer.capabilities.maxTextureSize);
  ground = Math.min(ground, renderer.capabilities.maxTextureSize);
  const el = [THREE.MathUtils.degToRad(-2.6), THREE.MathUtils.degToRad(21.4)];
  const height = Math.round((width * (el[1] - el[0])) / (Math.PI * 2));
  const target = new THREE.WebGLRenderTarget(width, height, { count: 3, type: THREE.UnsignedByteType, format: THREE.RGBAFormat, depthBuffer: false, generateMipmaps: false });
  for (const texture of target.textures) {
    texture.minFilter = texture.magFilter = THREE.LinearFilter;
    texture.wrapS = THREE.RepeatWrapping;
    texture.wrapT = THREE.ClampToEdgeWrapping;
  }
  const heights = new THREE.WebGLRenderTarget(ground, ground, { type: THREE.HalfFloatType, format: THREE.RedFormat, depthBuffer: false, generateMipmaps: false });
  heights.texture.minFilter = heights.texture.magFilter = THREE.LinearFilter;
  heights.texture.wrapS = heights.texture.wrapT = THREE.ClampToEdgeWrapping;

  const triangle = new THREE.BufferGeometry();
  triangle.setAttribute('position', new THREE.Float32BufferAttribute([-1, -1, 0, 3, -1, 0, -1, 3, 0], 3));
  const stage = (fragmentShader, uniforms) => {
    const mesh = new THREE.Mesh(triangle, new THREE.ShaderMaterial({ glslVersion: THREE.GLSL3, vertexShader: vertex, fragmentShader, uniforms, depthTest: false, depthWrite: false }));
    mesh.frustumCulled = false;
    const scene = new THREE.Scene();
    scene.add(mesh);
    return scene;
  };
  const groundScene = stage(heightFragment, { uHero: { value: new THREE.Vector2(hero[0], hero[1]) }, uDetail: { value: ground > 2048 ? 10 : 9 } });
  const paintScene = stage(paintFragment, {
    tHeight: { value: heights.texture },
    uTexel: { value: SPAN / ground },
    uSide: { value: new THREE.Vector2(side[0], side[1]).normalize() },
    uHero: { value: new THREE.Vector2(hero[0], hero[1]) },
    uEl: { value: new THREE.Vector2(el[0], el[1]) },
    uEye: { value: 0.06 },
  });
  const camera = new THREE.Camera();
  /** Draws a scene into a target in upright strips, so no one draw holds the GPU for long. */
  const draw = (scene, into, w, h, strips) => {
    const step = Math.ceil(w / strips);
    into.scissorTest = true;
    for (let i = 0; i < strips; i++) {
      into.scissor.set(i * step, 0, step, h);
      renderer.setRenderTarget(into);
      renderer.render(scene, camera);
    }
    into.scissorTest = false;
  };
  return {
    textures: target.textures,
    el,
    /** Works the ground out and paints the whole strip, once. */
    paint() {
      const previous = renderer.getRenderTarget();
      draw(groundScene, heights, ground, ground, 8);
      draw(paintScene, target, width, height, 48);
      renderer.setRenderTarget(previous);
    },
    compile: () => Promise.all([renderer.compileAsync(groundScene, camera), renderer.compileAsync(paintScene, camera)]),
  };
}
