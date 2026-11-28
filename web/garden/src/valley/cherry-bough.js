import * as THREE from 'three';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
import { COMMON } from '../lib/index.js';
import { TREE } from './world.js';

export async function loadCherryBough(field) {
  const { scene } = await new GLTFLoader().loadAsync('./cherry-bough.glb');
  const branch = scene.children[0];
  const original = branch.material;
  branch.material = new THREE.ShaderMaterial({
    glslVersion: THREE.GLSL3, vertexColors: true, side: THREE.DoubleSide,
    uniforms: field.uniforms,
    vertexShader: `
      ${COMMON}
      out vec2 surface;
      out vec3 pigment, at, facing;
      void main() {
        at = (modelMatrix * vec4(position, 1.)).xyz;
        float tip = smoothstep(0., 6., position.x);
        at.xz += tip * uWind.xy * 0.035 * sin(uTime * 1.1 + at.z * 0.6);
        surface = uv;
        facing = mat3(modelMatrix) * normal;
        pigment = color.rgb;
        gl_Position = projectionMatrix * viewMatrix * vec4(at, 1.);
      }`,
    fragmentShader: `
      ${COMMON}
      in vec2 surface;
      in vec3 pigment, at, facing;
      layout(location = 0) out vec4 outC;
      // Fade a repeated vein into the base pigment once it is smaller than a pixel.
      float veinLine(float phase, float width) {
        float footprint = max(fwidth(phase), 0.001);
        float d = abs(fract(phase + 0.5) - 0.5);
        return (1. - smoothstep(width, width + footprint, d)) * (1. - smoothstep(0.15, 0.5, footprint));
      }
      void main() {
        vec3 n = normalize(facing) * (gl_FrontFacing ? 1. : -1.);
        vec3 paint = pigment;
        float s = 2. * fract(surface.x) - 1., t = surface.y;
        float grain = texture(tNoise, surface * vec2(5., 9.)).g;
        float transmission = 0.;
        if (surface.x < 1.) {
          float fan = s * (3.1 + 2.6*t) + 0.18*sin(t*5. + s*2.);
          float vein = veinLine(fan, 0.045);
          float fork = veinLine(fan * 2. + 0.45 * sin(4. * t), 0.038) * smoothstep(0.35, 0.8, t);
          float folds = cos(fan * 6.283) * (1. - smoothstep(0.12, 0.5, fwidth(fan)));
          float rim = smoothstep(0.74, 1., max(t, abs(s)));
          paint *= (0.92 + 0.13 * grain + 0.065 * folds)
            * mix(vec3(1.), vec3(0.84, 0.53, 0.69), (vein + 0.3 * fork) * (1. - 0.6 * t));
          paint = mix(paint, vec3(1., 0.87, 0.80), 0.24 * rim);
          transmission = 0.28 + 0.32 * t + 0.20 * rim;
        } else if (surface.x < 3.) {
          float layer = t*68. + grain*1.4;
          float lenticel = smoothstep(0.84, 1., sin(layer*6.28)) * smoothstep(0.51, 0.72, grain);
          paint *= 0.70 + grain*0.50 + lenticel*(1.-smoothstep(0.14, 0.5, fwidth(layer)));
        } else if (surface.x < 4.) {
          float midrib = 1. - smoothstep(0.015, 0.065 + fwidth(s), abs(s));
          float ribs = veinLine(t * 6. - 2.1 * abs(s) + 0.65 * s * s, 0.045);
          paint *= 0.80 + 0.17 * grain + 0.14 * smoothstep(-0.2, 0.2, s);
          paint = mix(paint, vec3(0.48, 0.55, 0.20), 0.47 * midrib + 0.22 * ribs);
          transmission = 0.20 + 0.10 * abs(s);
        }
        float sun = dot(n, uSunDir);
        vec3 light = uSkyColor * (0.66 + 0.17*n.y) +
          uSunColor * (0.24 + 0.72*max(sun,0.) + transmission*max(-sun,0.)) * cloudAt(at.xz);
        outC = vec4(applyFog(paint * light, at), 1.);
      }`,
  });
  original.dispose();
  scene.rotation.y = -Math.PI / 2;
  scene.position.set(TREE[0], field.heightAt(...TREE) + 3.25, TREE[1]);
  scene.name = 'Low flowering cherry bough';
  return scene;
}
