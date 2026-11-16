// GLSL shared by everything that draws the field: where a point is on the maps, the ground under it, the wind,
// the light. Every material includes COMMON and binds the same uniform objects (shared.js), so one change to the
// wind or the sun moves the whole picture.

export const COMMON = /* glsl */ `
precision highp float;
precision highp sampler2D;

uniform float uTime;
uniform vec4 uWind;          // direction x, z (unit), speed m/s, gust 0..1
uniform vec2 uWindScroll;    // how far the gust field has travelled, metres
uniform vec2 uExtentMin;     // corner of the ground the field knows (height, growth)
uniform float uExtentInv;    // 1 / its side
uniform float uHeightRes;    // texels across tHeight
uniform vec2 uStateMin;      // corner of the part that reacts and remembers
uniform float uStateInv;     // 1 / its side
uniform sampler2D tHeight;   // R32F, metres
uniform sampler2D tMap;      // dry, growth, wet, flowers
uniform sampler2D tState;    // bend x, bend z (128 = none), cut height in metres (255 = uncut), spare
uniform sampler2D tFire;     // heat, char, smoulder, ash
uniform sampler2D tGlow;     // where fire is (2 m cells, spread out), its light, spare, spare
uniform sampler2D tNoise;    // smooth, tiling: four different fields
uniform sampler2D tNoiseHi;  // the same fields at 16 bits, for reading stretched wide (wind, clouds)
uniform sampler2D tWhite;    // white noise, tiling, nearest

uniform vec3 uSunDir;        // towards the sun
uniform vec3 uSunColor;
uniform vec3 uSkyColor;      // light from above
uniform vec3 uBounceColor;   // light from below
uniform vec3 uFogColor;
uniform vec3 uFogSunColor;   // the haze towards the sun
uniform float uFogDensity;
uniform vec3 uFireColor;     // the colour fire throws on things, times its flicker
uniform vec3 uCamPos;
uniform vec4 uPushers[6];    // things standing in the grass: centre xyz, radius (0 for none)
uniform sampler2D tShade;      // tree shade, 0 to 1
uniform sampler2D tFloor;      // painted farmland; alpha where there is any
uniform vec3 uHeroEye;
uniform sampler2D tSlope;      // the ground's slope: see index.js
uniform sampler2D tStrokes;    // brushwork: tone in red (half is none), stones in blue
uniform vec4 uFloor;
uniform float uFloorInv;
uniform float uCloudShade;   // how dark a cloud's shadow is, 0 for none
uniform vec4 uWalker;        // someone walking in the open: x, z, how dark their shadow, how long (0 for nobody)
uniform vec4 uLamp;          // a flame carried through the field: where it is, how bright (0 for none)

// The carried flame's light at p, in the same measure as tGlow's.
float lampAt(vec3 p) {
  vec3 d = p - uLamp.xyz;
  return uLamp.w / (1.0 + dot(d, d) * 0.9);
}

const float TAU = 6.28318531;

vec4 hash42(vec2 p) {
  vec4 p4 = fract(vec4(p.xyxy) * vec4(0.1031, 0.1030, 0.0973, 0.1099));
  p4 += dot(p4, p4.wzxy + 33.33);
  return fract((p4.xxyz + p4.yzzw) * p4.zywx);
}

uvec3 pcg3d(uvec3 v) {
  v = v * 1664525u + 1013904223u;
  v.x += v.y * v.z; v.y += v.z * v.x; v.z += v.x * v.y;
  v ^= v >> 16u;
  v.x += v.y * v.z; v.y += v.z * v.x; v.z += v.x * v.y;
  return v;
}

// Four random numbers for a cell of the world, the same every frame and on every machine.
vec4 cellRand(vec2 cell, float salt) {
  uvec3 a = pcg3d(uvec3(ivec3(int(cell.x) + 65536, int(cell.y) + 65536, int(salt))));
  uvec3 b = pcg3d(a + 7u);
  return vec4(vec3(a >> 8u), float(b.x >> 8u)) / 16777216.0;
}

float hash12(vec2 p) {
  vec3 p3 = fract(vec3(p.xyx) * 0.1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}

vec2 extentUv(vec2 xz) { return (xz - uExtentMin) * uExtentInv; }
vec2 stateUv(vec2 xz) { return (xz - uStateMin) * uStateInv; }

// The ground's height, read the same way the CPU reads it (field.heightAt): bilinear between texel centres.
float groundAt(vec2 xz) {
  vec2 p = extentUv(xz) * uHeightRes - 0.5;
  vec2 i = floor(p);
  vec2 f = p - i;
  ivec2 lo = ivec2(0);
  ivec2 hi = ivec2(int(uHeightRes) - 1);
  ivec2 a = clamp(ivec2(i), lo, hi);
  ivec2 b = clamp(ivec2(i) + 1, lo, hi);
  float h00 = texelFetch(tHeight, a, 0).r;
  float h10 = texelFetch(tHeight, ivec2(b.x, a.y), 0).r;
  float h01 = texelFetch(tHeight, ivec2(a.x, b.y), 0).r;
  float h11 = texelFetch(tHeight, b, 0).r;
  return mix(mix(h00, h10, f.x), mix(h01, h11, f.x), f.y);
}

// 0 in a lull to 1 in a gust: two fields riding the wind, the broad one faster.
float gustAt(vec2 xz) {
  vec2 a = (xz - uWindScroll) / 61.0;
  vec2 b = (xz - uWindScroll * 0.62) / 17.0;
  float g = texture(tNoiseHi, a).r * 0.68 + texture(tNoiseHi, b + 0.37).g * 0.32;
  return smoothstep(0.26, 0.78, g);
}

// 1 in the open, less under a cloud or a tree. The clouds drift with the wind, slowly.
float openAt(vec2 xz) {
  // (summer cumulus: shadows the size of fields, crossing the valley at the wind's pace)
  vec2 p = (xz - uWindScroll * 2.6) / 3400.0;
  float c = texture(tNoiseHi, p).b * 0.7 + texture(tNoiseHi, p * 2.7 + 0.19).a * 0.3;
  float open = 1.0 - uCloudShade * smoothstep(0.47, 0.6, c);
  float tree = texture(tShade, extentUv(xz)).r;
  if (tree > 0.004) {
    // under a tree the light comes through the leaves in spots, and the spots move as the leaves do
    float s1 = texture(tNoise, xz / 2.7 + uTime * vec2(0.011, 0.008)).g;
    float s2 = texture(tNoise, xz / 0.9 - uTime * vec2(0.013, 0.017)).a;
    float spot = mix(smoothstep(0.53, 0.66, s1 * 0.62 + s2 * 0.38), 0.3, smoothstep(50.0, 150.0, distance(xz, uCamPos.xz)));
    open *= 1.0 - tree * (0.88 - 0.8 * spot);
  }
  return open;
}

// The shadow of someone walking: a body's width, lying away from the sun as long as the hour makes it, with the
// clean edge a painted shadow has.
float walkerShade(vec2 xz) {
  if (uWalker.w <= 0.0) return 1.0;
  vec2 d = xz - uWalker.xy;
  vec2 away = -normalize(uSunDir.xz + vec2(1e-5));
  float along = clamp(dot(d, away), 0.0, uWalker.w);
  float r = length(d - away * along);
  // (narrower towards the head)
  float wide = mix(0.36, 0.2, along / uWalker.w);
  return 1.0 - uWalker.z * (1.0 - smoothstep(wide, wide + 0.1, r));
}

// What the ground gets of the sun: the open sky, less anyone's shadow.
float cloudAt(vec2 xz) {
  return openAt(xz) * walkerShade(xz);
}

// Distance haze, warmer towards the sun.
vec3 applyFog(vec3 c, vec3 world) {
  vec3 d = world - uCamPos;
  float dist = length(d);
  // the haze lies in the valleys: what stands above it is seen through less of it
  float thin = mix(1.0, 0.3, smoothstep(250.0, 2400.0, world.y));
  float f = 1.0 - exp(-dist * uFogDensity * thin);
  float toSun = pow(max(dot(d / max(dist, 1e-3), uSunDir), 0.0), 6.0);
  return mix(c, mix(uFogColor, uFogSunColor, toSun), clamp(f, 0.0, 1.0));
}

// What grass is, as colour: green where it's lush, straw where it's dry. s is 0 at the root and 1 at the tip.
vec3 grassColor(float dry, float s) {
  vec3 green = mix(vec3(0.035, 0.135, 0.05), vec3(0.25, 0.43, 0.085), s);
  vec3 straw = mix(vec3(0.085, 0.090, 0.030), vec3(0.420, 0.400, 0.150), s);
  return mix(green, straw, dry);
}

// Broad patches of yellower and bluer green, as a painter lays a meadow in: the same wherever grass is drawn.
vec3 meadowTint(vec2 xz) {
  float a = texture(tNoiseHi, xz / 520.0).a;
  float b = texture(tNoiseHi, xz / 37.0 + 0.23).r;
  float w = smoothstep(0.3, 0.7, a * 0.6 + b * 0.4);
  vec3 t = mix(vec3(0.8, 0.96, 1.1), vec3(1.2, 1.06, 0.76), w);
  // here and there a patch of something darker and bluer: clover, sedge
  float c = texture(tNoiseHi, xz / 13.0 + 0.61).g;
  return t * mix(vec3(1.0), vec3(0.72, 0.87, 0.96), smoothstep(0.6, 0.72, c));
}

// Grass stands taller in some places than in others: 1 is its usual height.
float tallAt(vec2 xz) {
  float a = texture(tNoiseHi, xz / 23.0 + 0.47).b;
  float b = texture(tNoiseHi, xz / 7.0 + 0.11).r;
  return mix(0.72, 1.5, smoothstep(0.34, 0.68, a * 0.7 + b * 0.3));
}

// Which way the ground faces at a place on the field.
vec3 landNormal(vec2 xz) {
  vec2 s = (texture(tSlope, extentUv(xz)).rg - 0.5) * 0.5;
  return normalize(vec3(-s.x, 1.0, -s.y));
}

// How a slope stands to the sun, as a painter would have it: 0.5 for level ground, more where it is turned to the
// light and less where it is turned away. Slopes are made three times what they are, or gentle ground shows nothing.
float slopeRel(vec3 N) {
  vec3 n = normalize(vec3(N.x * 3.2, N.y, N.z * 3.2));
  vec3 L = normalize(vec3(uSunDir.x, max(uSunDir.y, 0.3), uSunDir.z));
  return clamp(0.5 + (dot(n, L) - L.y) * 1.5, 0.0, 1.0);
}

// How much of the sun reaches ground that stands so: a hollow is lit by the sky.
float slopeSun(float rel) {
  return mix(0.3, 1.0, smoothstep(0.04, 0.5, rel));
}

// The meadow as it is painted: three greens, chosen by tone (0 in deep shade, 1 where the light is full).
vec3 meadowGreen(float tone) {
  vec3 c = mix(vec3(0.05, 0.25, 0.07), vec3(0.15, 0.46, 0.085), smoothstep(0.2, 0.5, tone));
  return mix(c, vec3(0.3, 0.57, 0.12), smoothstep(0.56, 0.9, tone));
}

// The light on it at this hour. shade is how open the place is to the sun (cloudAt's measure).
vec3 meadowLight(float shade) {
  vec3 sun = mix(uSunColor, vec3(dot(uSunColor, vec3(0.333))), 0.3) * 0.42 + uSkyColor * 0.35;
  float lum = dot(uSkyColor, vec3(0.333));
  vec3 sky = mix(vec3(0.4, 0.74, 0.95), uSkyColor / max(lum, 0.01) * 0.62, 0.35) * (lum * 1.9 + 0.06);
  return mix(sky * 0.85, sun, smoothstep(0.25, 0.95, shade));
}

// A slope turned to the sun is lighter than one turned away, seen from far enough off to read as a slope.
float slopeLight(vec3 N) {
  return mix(0.7, 1.12, smoothstep(-0.1, 0.75, dot(N, uSunDir)));
}

// A meadow seen from off is brushwork: dabs of lighter and darker green at every size, the light ones leaning
// yellow and the dark ones blue, and (where stones is 1) a grey stone here and there.
// Where a point on the land is in the picture seen from the start of the walk: radians right of the valley's line,
// and radians below the level of the eye.
vec2 heroAngle(vec3 world) {
  vec2 d = world.xz - uHeroEye.xz;
  return vec2(atan(dot(d, vec2(-uFloor.y, uFloor.x)), dot(d, uFloor.xy)), atan(uHeroEye.y - world.y, length(d)));
}

// Cells: how far a point is from the edge of the cell it is in, and two numbers that are the cell's own.
vec3 paintCells(vec2 x) {
  vec2 n = floor(x);
  vec2 f = x - n;
  float d1 = 8.0;
  float d2 = 8.0;
  vec2 id = n;
  for (int j = -1; j <= 1; j++) {
    for (int i = -1; i <= 1; i++) {
      vec2 c = vec2(float(i), float(j));
      vec2 r = c + hash42(n + c).xy - f;
      float d = dot(r, r);
      if (d < d1) { d2 = d1; d1 = d; id = n + c; }
      else if (d < d2) d2 = d;
    }
  }
  vec4 h = hash42(id + 0.37);
  return vec3(sqrt(d2) - sqrt(d1), h.z, h.w);
}

// The far hills' recipe (backdrop.js), on the meadow. q is heroAngle there and w how much it changes across a
// pixel (a fragment shader's fwidth of it).
vec3 heroPaint(vec3 g, vec3 world, vec2 q, vec2 w, float dist) {
  float r = distance(world.xz, uHeroEye.xz);
  // not at your feet, where the blades are the brushwork; not past the plain; not behind the start, where the
  // picture's two edges meet
  float k = smoothstep(70.0, 150.0, dist) * (1.0 - smoothstep(2200.0, 3600.0, r)) * (1.0 - smoothstep(2.1, 2.8, abs(q.x)));
  if (k <= 0.0) return g;
  float px = max(w.x, w.y);
  // soft patches of three greens at two sizes, and long strokes of them lying level
  float soft = k * (1.0 - smoothstep(0.0015, 0.005, px));
  float v = texture(tNoiseHi, q / 0.42 + 0.2).r * 0.4 + texture(tNoiseHi, q / 0.15 + 0.6).g * 0.3 + texture(tNoiseHi, vec2(q.x / 0.5, q.y / 0.07) + 0.4).b * 0.3;
  g *= mix(vec3(1.0), vec3(0.7, 0.87, 1.0), (1.0 - smoothstep(0.42, 0.48, v)) * soft * 0.9);
  g *= mix(vec3(1.0), vec3(1.34, 1.15, 0.8), smoothstep(0.53, 0.59, v) * soft * 0.9);
  // clumps of rough grass, gathered into masses as the woods on the hills are
  float coarse = k * (1.0 - smoothstep(0.3, 0.7, px * 70.0)) * smoothstep(0.016, 0.03, px * 70.0);
  vec3 t = paintCells(q * 70.0);
  float mass = smoothstep(0.48, 0.56, texture(tNoiseHi, q / 0.3 + 0.7).a);
  g *= mix(vec3(1.0), vec3(0.62, 0.82, 0.9), smoothstep(0.1, 0.2, t.x) * step(t.y, 0.34 * mass + 0.04) * coarse);
  // dabs: a darker tuft, a fleck of sun, a few flowers
  float fine = k * (1.0 - smoothstep(0.3, 0.7, px * 220.0)) * smoothstep(0.05, 0.09, px * 220.0);
  vec3 c = paintCells(q * 220.0);
  float dab = smoothstep(0.1, 0.22, c.x) * fine;
  g *= mix(vec3(1.0), vec3(0.72, 0.85, 0.92), dab * step(0.8, c.y));
  g *= mix(vec3(1.0), vec3(1.26, 1.13, 0.84), dab * step(c.y, 0.15));
  g = mix(g, c.z > 0.985 ? vec3(0.88, 0.86, 0.78) : vec3(0.86, 0.8, 0.4), smoothstep(0.3, 0.42, c.x) * fine * step(0.965, c.z) * 0.6);
  return g;
}

// Where the long strokes on the land lie: lines of level ground, wandered a little so they are not a map's.
float strokeCoord(vec3 world) {
  return (world.y + 7.0 * texture(tNoiseHi, world.xz / 270.0 + 0.4).g) / 5.5;
}

// On level ground there is no lie of the land to follow, so a second set of strokes lies across the valley, wandering.
float strokeCoord2(vec2 xz) {
  return (dot(xz, uFloor.xy) + 70.0 * texture(tNoiseHi, xz / 430.0 + 0.7).a) / 31.0 + 57.0;
}

// Long thin strokes that follow the lie of the land, a few of them, some lighter than the grass and some deeper:
// what a painter draws on a far meadow to show how it lies. x is strokeCoord there and w how much it changes
// across a pixel (a fragment shader's fwidth of it), which keeps a stroke the same few pixels thick anywhere.
vec3 landStrokes(vec3 g, vec2 xz, float dist, float x, float w) {
  float far = smoothstep(70.0, 170.0, dist) * (1.0 - smoothstep(2600.0, 4600.0, dist));
  float line = 1.0 - smoothstep(1.5, 3.2, abs(fract(x) - 0.5) / max(w, 1e-5));
  float id = floor(x);
  // only here and there, and each for a different length
  float here = smoothstep(0.45, 0.53, texture(tNoiseHi, xz / 340.0 + id * 0.37).b);
  float k = line * here * far;
  // most are pale, as a track or a line of sunlit grass is; some are a deeper green
  return fract(id * 0.618) < 0.62 ? mix(g, vec3(0.5, 0.64, 0.22), k * 0.6) : g * mix(vec3(1.0), vec3(0.62, 0.8, 0.86), k);
}

vec3 paintGround(vec3 g, vec2 xz, float dist, float stones) {
  // at your feet, the brush's dabs, the same the blades take their tone from; gone by a hundred metres
  float nearK = 1.0 - smoothstep(40.0, 110.0, dist);
  float t = ((texture(tStrokes, xz / 6.0).r - 0.5) * 0.5 + 0.7 * (texture(tStrokes, xz.yx / 27.0 + 0.37).r - 0.5) + 0.9 * (texture(tStrokes, xz / 95.0 + 0.11).r - 0.5)) * 0.7 * nearK;
  float farK = 1.0 - nearK;
  // beyond, one slow change of green across the land,
  t += (texture(tNoiseHi, xz / 1100.0 + 0.2).r - 0.5) * 0.5 * farK;
  g *= (1.0 + t * 0.5) * mix(vec3(0.9, 1.0, 1.1), vec3(1.1, 1.03, 0.84), clamp(0.5 + t * 1.4, 0.0, 1.0));
  // and on it, as on the far hills, soft patches at two sizes: a sunlit yellower green here, a deeper bluer one
  // there. They differ in colour more than in tone and their edges are soft, which is what keeps them paint and
  // not blotches.
  float v = texture(tNoiseHi, xz / 520.0 + 0.2).g * 0.58 + texture(tNoiseHi, xz / 170.0 + 0.61).b * 0.42;
  g *= mix(vec3(1.0), vec3(1.26, 1.13, 0.76), smoothstep(0.5, 0.62, v) * farK * 0.4);
  g *= mix(vec3(1.0), vec3(0.76, 0.92, 1.06), (1.0 - smoothstep(0.36, 0.48, v)) * farK * 0.4);
  // a few tufts of longer grass standing darker in it: short dashes lying across the view, on the nearer ground only
  vec2 q = vec2(dot(xz, vec2(-uFloor.y, uFloor.x)) / 130.0, dot(xz, uFloor.xy) / 64.0);
  float tuft = 1.0 - smoothstep(0.13, 0.2, texture(tStrokes, q).r);
  g *= mix(vec3(1.0), vec3(0.7, 0.86, 0.9), tuft * farK * (1.0 - smoothstep(350.0, 800.0, dist)) * 0.6);
  // (a far meadow in the sun is a lighter, yellower green than the grass at your feet, which you see into)
  return g * mix(vec3(1.0), vec3(1.16, 1.1, 0.9), 1.0 - nearK);
}

vec3 paintGroundOld(vec3 g, vec2 xz, float dist, float stones) {
  float a = texture(tStrokes, xz / 6.0).r - 0.5;
  float b = texture(tStrokes, xz.yx / 27.0 + 0.37).r - 0.5;
  float c = texture(tStrokes, xz / 121.0 + 0.71).r - 0.5;
  float e = texture(tStrokes, xz.yx / 470.0 + 0.13).r - 0.5;
  float t = a * (1.0 - smoothstep(50.0, 200.0, dist)) + b * 0.9 * (1.0 - smoothstep(500.0, 1700.0, dist)) + c * 1.5 + e * 1.3;
  // past the blades the meadow is a painting: mid is how much of one
  float mid = smoothstep(40.0, 120.0, dist) * (1.0 - smoothstep(2500.0, 5000.0, dist));
  // its greens are laid in as separate paints with an edge between one and the next, not blended
  float s = t * 3.0 + 0.5;
  t = mix(t, (floor(s) + smoothstep(0.38, 0.62, fract(s)) - 0.5) / 3.0, mid * 0.85);
  g *= (1.0 + t * 0.85) * mix(vec3(0.86, 1.0, 1.16), vec3(1.14, 1.04, 0.78), clamp(0.5 + t * 1.4, 0.0, 1.0));
  // Flat ground seen from a hill is squeezed twentyfold from front to back, so a mark must be long and lie across
  // the view to be seen at all: the meadow is painted in bands, as it is in the films. Broad bands of deeper and
  // of sunlit grass, thin dark and light streaks along them, and drifts of flowers lying the same way.
  vec2 across = vec2(-uFloor.y, uFloor.x);
  vec2 q1 = vec2(dot(xz, across) / 1300.0, dot(xz, uFloor.xy) / 250.0);
  vec2 q2 = vec2(dot(xz, across * 0.86 + uFloor.xy * 0.5) / 900.0, dot(xz, uFloor.xy * 0.86 - across * 0.5) / 170.0) + 0.43;
  float l1 = texture(tStrokes, q1).r;
  float l2 = texture(tStrokes, q2).r;
  float reach = mid * (1.0 - smoothstep(1400.0, 3000.0, dist));
  float lo = min(l1, l2);
  float hi = max(l1, l2);
  g *= mix(vec3(1.0), vec3(0.8, 0.9, 0.96), (1.0 - smoothstep(0.42, 0.46, lo)) * reach);
  g *= mix(vec3(1.0), vec3(0.72, 0.84, 0.9), (1.0 - smoothstep(0.26, 0.32, lo)) * reach);
  g *= mix(vec3(1.0), vec3(1.08, 1.06, 0.94), smoothstep(0.55, 0.59, hi) * reach);
  g *= mix(vec3(1.0), vec3(1.12, 1.08, 0.9), smoothstep(0.7, 0.76, hi) * reach);
  float bloom = texture(tStrokes, q1 * vec2(1.7, 1.3) + 0.61).r;
  float drift = smoothstep(0.46, 0.58, texture(tNoiseHi, xz / 310.0 + 0.3).g);
  vec3 petal = mix(vec3(0.85, 0.82, 0.62), vec3(0.86, 0.66, 0.2), step(0.5, texture(tNoiseHi, xz / 170.0).a));
  g = mix(g, petal, max(smoothstep(0.55, 0.6, bloom) * 0.3, smoothstep(0.7, 0.76, bloom) * 0.7) * drift * reach * (1.0 - smoothstep(350.0, 800.0, dist)));
  float rock = texture(tStrokes, xz / 31.0 + 0.19).b * stones * (1.0 - smoothstep(500.0, 1100.0, dist));
  return mix(g, vec3(0.5, 0.5, 0.47), smoothstep(0.35, 0.6, rock));
}

// The farmland's paint at a place, and in alpha how much of the ground there is farmland.
vec4 floorAt(vec2 xz) {
  vec2 uv = (vec2(dot(xz, uFloor.xy), dot(xz, vec2(-uFloor.y, uFloor.x))) - uFloor.zw) * uFloorInv;
  return texture(tFloor, clamp(uv, 0.0, 1.0));
}
`;

// The light on a thin leaf. N faces the viewer; light from behind it comes through, yellower.
export const LEAF_LIGHT = /* glsl */ `
// up: 0 at the ground to 1 at the top of the sward. shade: cloudAt there. glow: tGlow's light there.
vec3 leafLight(vec3 albedo, vec3 N, vec3 V, float up, float gloss, float shade, float glow) {
  float ndl = dot(N, uSunDir);
  // low sun reaches the tips and little else: the blades in front stand in the way
  float reach = pow(clamp(up, 0.0, 1.0), mix(3.0, 0.55, clamp(uSunDir.y * 2.2, 0.0, 1.0)));
  float sun = shade * mix(0.05, 1.0, reach);
  vec3 through = albedo * vec3(1.0, 1.16, 0.36) + vec3(0.012, 0.02, 0.0);
  vec3 c = albedo * uSunColor * max(ndl, 0.0) * sun;
  c += through * uSunColor * max(-ndl, 0.0) * 0.72 * sun;
  vec3 H = normalize(uSunDir + V);
  c += uSunColor * pow(abs(dot(N, H)), 42.0) * gloss * sun;
  float ao = mix(0.22, 1.0, clamp(up, 0.0, 1.0));
  c += albedo * mix(uBounceColor, uSkyColor, N.y * 0.5 + 0.5) * ao;
  c += albedo * uFireColor * glow * (0.35 + 0.65 * up);
  return c;
}
`;

export const FIRE_RAMP = /* glsl */ `
// dark red, red, orange, yellow, white-hot, in linear light (from Patch's fire block)
vec3 fireRamp(float T) {
  vec3 c = mix(vec3(0.05, 0.002, 0.0), vec3(0.42, 0.028, 0.002), smoothstep(0.0, 0.25, T));
  c = mix(c, vec3(1.1, 0.13, 0.008), smoothstep(0.25, 0.45, T));
  c = mix(c, vec3(1.6, 0.34, 0.018), smoothstep(0.45, 0.65, T));
  c = mix(c, vec3(2.25, 0.9, 0.07), smoothstep(0.65, 0.85, T));
  c = mix(c, vec3(5.0, 3.6, 1.6), smoothstep(0.85, 1.1, T));
  return c;
}
`;
