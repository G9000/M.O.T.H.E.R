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
      void main() {
        vec3 n = normalize(facing) * (gl_FrontFacing ? 1. : -1.);
        vec3 paint = pigment;
        float s = 2. * fract(surface.x) - 1., t = surface.y;
        float grain = texture(tNoise, surface * vec2(5., 9.)).g;
        float transmission = 0.;
        if (surface.x < 1.) {
          float fan = s * (3.1 + 2.6*t) + 0.18*sin(t*5. + s*2.);
          float width = max(fwidth(fan), 0.001);
          float vein = (1. - smoothstep(0.02, 0.065 + width, abs(fract(fan+0.5)-0.5)))
            * (1. - smoothstep(0.1, 0.4, width));
          paint *= (0.93 + 0.12*grain) * mix(vec3(1.), vec3(0.86,0.60,0.72), vein*(1.-0.65*t));
          transmission = 0.40 + 0.25*t;
        } else if (surface.x < 3.) {
          float layer = t*68. + grain*1.4;
          float lenticel = smoothstep(0.84, 1., sin(layer*6.28)) * smoothstep(0.51, 0.72, grain);
          paint *= 0.70 + grain*0.50 + lenticel*(1.-smoothstep(0.14, 0.5, fwidth(layer)));
        } else if (surface.x < 4.) {
          paint *= 0.87 + 0.19*grain + 0.24*(1.-smoothstep(0., 0.10, abs(s)));
          transmission = 0.18;
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
