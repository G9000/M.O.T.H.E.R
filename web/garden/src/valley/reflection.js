import * as THREE from 'three';
import { TARN } from './world.js';

// Only the four glasshouse meshes enter this pass. The water shader mixes the
// result with its existing sky, ripples and Fresnel term at the point of reflection.
export function poolReflection(renderer, house, water, quality) {
  const mirror = new THREE.PerspectiveCamera();
  const scene = new THREE.Scene();
  scene.add(house.clone(true));
  const target = new THREE.WebGLRenderTarget(1, 1);
  const uniforms = water.material.uniforms;
  uniforms.tGlasshouse.value = target.texture;
  const longEdge = quality === 'high' ? 640 : quality === 'medium' ? 384 : 256;
  const frustum = new THREE.Frustum(), projection = new THREE.Matrix4();
  const bounds = new THREE.Sphere(new THREE.Vector3(TARN.at[0], TARN.level, TARN.at[1]), 10);
  const lastView = new THREE.Matrix4(), lastLens = new THREE.Matrix4();
  const forward = new THREE.Vector3(), savedEye = new THREE.Vector3(), colour = new THREE.Color();
  let lastHour = -100;
  return (camera, hour) => {
    projection.multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse);
    const inView = frustum.setFromProjectionMatrix(projection).intersectsSphere(bounds);
    uniforms.uMirrorVisible.value = Number(inView);
    if (!inView || (lastView.equals(camera.matrixWorld) && lastLens.equals(camera.projectionMatrix) && Math.abs(hour - lastHour) < 0.025)) return;

    lastView.copy(camera.matrixWorld);
    lastLens.copy(camera.projectionMatrix);
    lastHour = hour;
    target.setSize(Math.max(1, Math.round(longEdge * Math.min(1, camera.aspect))),
      Math.max(1, Math.round(longEdge * Math.min(1, 1 / camera.aspect))));
    mirror.position.copy(camera.position);
    mirror.position.y = 2 * TARN.level - mirror.position.y;
    camera.getWorldDirection(forward);
    forward.y *= -1;
    mirror.up.set(0, 1, 0).applyQuaternion(camera.quaternion);
    mirror.up.y *= -1;
    mirror.lookAt(forward.add(mirror.position));
    mirror.projectionMatrix.copy(camera.projectionMatrix);
    mirror.updateMatrixWorld();
    uniforms.uMirrorProjection.value.multiplyMatrices(mirror.projectionMatrix, mirror.matrixWorldInverse);

    const previous = renderer.getRenderTarget(), alpha = renderer.getClearAlpha();
    renderer.getClearColor(colour);
    savedEye.copy(uniforms.uCamPos.value);
    try {
      uniforms.uCamPos.value.copy(mirror.position);
      renderer.setRenderTarget(target);
      renderer.setClearColor(0, 0);
      renderer.clear();
      renderer.render(scene, mirror);
    } finally {
      uniforms.uCamPos.value.copy(savedEye);
      renderer.setClearColor(colour, alpha);
      renderer.setRenderTarget(previous);
    }
  };
}
