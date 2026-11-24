"""Rebuild the Garden's cherry-bough.glb with Blender in background mode."""
import math
import random
from pathlib import Path

import bpy
from mathutils import Vector


rng = random.Random(4261)
tau = math.tau
vertices, faces, colors, surface = [], [], [], []


def vertex(point, tint, uv):
    vertices.append(tuple(point))
    colors.append((*tint, 1))
    surface.append(uv)
    return len(vertices) - 1


def grid(rows, wrapped=False):
    for row, following in zip(rows, rows[1:]):
        for i in range(len(row) if wrapped else len(row) - 1):
            j = (i + 1) % len(row)
            faces.append((row[i], row[j], following[j], following[i]))


def frame(axis):
    axis = axis.normalized()
    cross = axis.cross(Vector((0, 0, 1)) if abs(axis.z) < 0.9 else Vector((0, 1, 0))).normalized()
    return cross, axis.cross(cross).normalized()


def stem(points, radii, tint, sides=7):
    rows = []
    for k, (point, radius) in enumerate(zip(points, radii)):
        cross, up = frame(points[min(k+1, len(points)-1)] - points[max(k-1, 0)])
        rows.append([vertex(point + radius * (math.cos(tau*i/sides)*cross + math.sin(tau*i/sides)*up),
                            tint, (2 + i/sides, k/(len(points)-1))) for i in range(sides)])
    grid(rows, True)


def bulb(base, axis, radius, length, tint):
    cross, up = frame(axis)
    rows = []
    for j in range(6):
        t = j / 5
        belly = radius * (0.02 + math.sin(math.pi*t)**0.8) * (1.23 - 0.48*t)
        rows.append([vertex(base + axis*length*t + belly*(cross*math.cos(tau*i/8) + up*math.sin(tau*i/8)),
                            tuple(c*(0.48+0.52*t) for c in tint), (4.5, t)) for i in range(8)])
    grid(rows, True)


def blade(base, outward, side, normal, length, tint, leaf=False):
    rows = []
    for j in range(8):
        t = j / 7
        width = (0.018 + math.sin(math.pi*t*(1 if leaf else 0.88))**0.78) * (0.23 if leaf else 0.54)
        row = []
        for i in range(7):
            s = (i-3)/3
            notch = 0 if leaf else t**5 * (0.09*(1-abs(s))**3 + 0.16*abs(s)**3)
            lift = (0.17*t - 0.24*t*t if leaf else 0.48*t - 0.33*t*t) + width*s*s*0.31
            point = base + length*(outward*(t-notch) + side*(s*width) + normal*lift)
            mix = min(1, t*1.3)
            throat = (0.10, 0.16, 0.043) if leaf else (0.55, 0.068, 0.16)
            pigment = tuple(a*(1-mix)+b*mix for a,b in zip(throat,tint))
            row.append(vertex(point, pigment, ((3 if leaf else 0) + (s+1)*0.495, t)))
        rows.append(row)
    grid(rows)


def bloom(base, axis, radius, phase):
    across, up = frame(axis)
    tint = (1.0, rng.uniform(0.55, 0.84), rng.uniform(0.65, 0.82))
    for k in range(5):
        a = phase + k*tau/5 + rng.uniform(-0.05, 0.05)
        ray = across*math.cos(a) + up*math.sin(a)
        blade(base, ray, axis.cross(ray), axis, radius*rng.uniform(0.92,1.09), tint)
    for k in range(8):
        a = phase + k*tau/8
        ray = across*math.cos(a) + up*math.sin(a)
        tip = base + radius*(ray*0.29 + axis*rng.uniform(0.30,0.45))
        stem([base, tip], [radius*0.021, radius*0.013], (0.96, 0.69, 0.41), 4)
        stem([tip, tip + axis*radius*0.035, tip + axis*radius*0.078],
             [radius*0.032, radius*0.052, radius*0.008], (1.0, 0.54, 0.065), 5)


def spine(t):
    return Vector((5.65*t, 0.42*math.sin(5*t), 0.76*math.sin(math.pi*t)-0.65*t))


wood = (0.23, 0.095, 0.062)
steps = [k/24 for k in range(25)]
stem([spine(t) for t in steps], [0.20*(1-t)**1.2+0.009 for t in steps], wood, 14)
for k in range(12):
    start = spine(0.22 + k*0.061)
    sign = 1 if k % 2 else -1
    sweep = Vector((rng.uniform(0.35,0.95), sign*rng.uniform(0.45,1.20), rng.uniform(-0.77,0.22)))

    def twig(t):
        return start + sweep*t + Vector((0, 0.09*sign, 0.21))*math.sin(math.pi*t)

    steps = [j/8 for j in range(9)]
    stem([twig(t) for t in steps], [0.040*(1-t)+0.006 for t in steps], wood)
    for j in range(3):
        fork = twig(0.29 + 0.33*j)
        angle = rng.random()*tau
        leaf = Vector((math.cos(angle),math.sin(angle),rng.uniform(-0.4,0.35))).normalized()
        across = Vector((0,0,1)).cross(leaf).normalized()
        blade(fork, leaf, across, leaf.cross(across), rng.uniform(0.22,0.38), (0.24,0.34,0.093), True)
        for n in range(2 + (j == 2)):
            a = angle + n*2.40
            axis = Vector((math.cos(a), math.sin(a), rng.uniform(-1.2,0.65))).normalized()
            base = fork + axis*rng.uniform(0.14,0.29)
            stem([fork, base], [0.013,0.007], (0.34,0.16,0.12), 5)
            if (k+j+n) % 5 == 0:
                bulb(base, axis, 0.060, 0.14, (0.84,0.21,0.34))
            else:
                bloom(base, axis, rng.uniform(0.13,0.20), a)

bpy.ops.wm.read_factory_settings(use_empty=True)
mesh = bpy.data.meshes.new("Flowering cherry")
mesh.from_pydata(vertices, [], faces)
mesh.update()
for poly in mesh.polygons:
    poly.use_smooth = True
color = mesh.color_attributes.new(name="Pigment", type="BYTE_COLOR", domain="POINT")
for entry, tint in zip(color.data, colors):
    entry.color = tint
uv = mesh.uv_layers.new(name="Surface")
for loop in mesh.loops:
    u,v = surface[loop.vertex_index]
    uv.data[loop.index].uv = (u, 1-v)
branch = bpy.data.objects.new("Flowering cherry bough", mesh)
bpy.context.collection.objects.link(branch)
mat = bpy.data.materials.new("Cherry pigment")
mat.use_nodes = True
paint = mat.node_tree.nodes.new("ShaderNodeVertexColor")
paint.layer_name = color.name
mat.node_tree.links.new(paint.outputs["Color"], mat.node_tree.nodes.get("Principled BSDF").inputs["Base Color"])
mesh.materials.append(mat)
output = Path(__file__).resolve().parents[1] / "public" / "cherry-bough.glb"
bpy.ops.export_scene.gltf(filepath=str(output), export_animations=False)
