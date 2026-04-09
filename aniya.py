import bpy
import math

# ==============================
# 1. 清空场景，准备建模
# ==============================
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# ==============================
# 2. 头部（主体，圆润造型）
# ==============================
bpy.ops.mesh.primitive_uv_sphere_add(radius=1.2, location=(0, 0, 3.5))
head = bpy.context.active_object
head.scale = (1.1, 1.0, 0.9)  # 调整卡通头型
bpy.ops.object.shade_smooth()

# ==============================
# 3. 身体（上半身+衣服）
# ==============================
# 身体主体
bpy.ops.mesh.primitive_cube_add(size=1, location=(0, 0, 2.2))
body = bpy.context.active_object
body.scale = (0.8, 0.6, 0.7)
bpy.ops.object.shade_smooth()

# 衣服（深蓝色）
body.data.materials.append(bpy.data.materials.new(name="Anya_Clothes"))
body.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (0.05, 0.08, 0.2, 1)

# 蝴蝶结
bpy.ops.mesh.primitive_cube_add(size=0.3, location=(0, 0, 2.5))
bow = bpy.context.active_object
bow.scale = (1.5, 0.3, 0.8)
bow.data.materials.append(bpy.data.materials.new(name="Anya_Bow"))
bow.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (1, 1, 1, 1)

# ==============================
# 4. 头发（粉色，蓬松造型）
# ==============================
# 头顶主体
bpy.ops.mesh.primitive_uv_sphere_add(radius=1.3, location=(0, 0, 4.2))
hair_top = bpy.context.active_object
hair_top.scale = (1.2, 1.1, 0.8)
hair_top.data.materials.append(bpy.data.materials.new(name="Anya_Hair"))
hair_top.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (1, 0.7, 0.8, 1)
bpy.ops.object.shade_smooth()

# 两侧刘海
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.8, location=(-1.1, 0, 3.8))
hair_left = bpy.context.active_object
hair_left.scale = (0.9, 0.7, 0.8)
hair_left.data.materials.append(bpy.data.materials.new(name="Anya_Hair"))
hair_left.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (1, 0.7, 0.8, 1)

bpy.ops.mesh.primitive_uv_sphere_add(radius=0.8, location=(1.1, 0, 3.8))
hair_right = bpy.context.active_object
hair_right.scale = (0.9, 0.7, 0.8)
hair_right.data.materials.append(bpy.data.materials.new(name="Anya_Hair"))
hair_right.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (1, 0.7, 0.8, 1)

# ==============================
# 5. 五官（眼睛、嘴巴，卡通风格）
# ==============================
# 左眼
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.3, location=(-0.4, 0.9, 3.7))
eye_left = bpy.context.active_object
eye_left.scale = (1.2, 0.3, 1.0)
eye_left.data.materials.append(bpy.data.materials.new(name="Anya_Eye_White"))
eye_left.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (1, 1, 1, 1)

# 左眼瞳孔（绿色）
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.15, location=(-0.4, 1.1, 3.7))
pupil_left = bpy.context.active_object
pupil_left.data.materials.append(bpy.data.materials.new(name="Anya_Eye_Green"))
pupil_left.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (0.2, 0.8, 0.6, 1)

# 右眼
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.3, location=(0.4, 0.9, 3.7))
eye_right = bpy.context.active_object
eye_right.scale = (1.2, 0.3, 1.0)
eye_right.data.materials.append(bpy.data.materials.new(name="Anya_Eye_White"))
eye_right.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (1, 1, 1, 1)

# 右眼瞳孔（绿色）
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.15, location=(0.4, 1.1, 3.7))
pupil_right = bpy.context.active_object
pupil_right.data.materials.append(bpy.data.materials.new(name="Anya_Eye_Green"))
pupil_right.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (0.2, 0.8, 0.6, 1)

# 嘴巴（微笑造型）
bpy.ops.mesh.primitive_cube_add(size=0.1, location=(0, 1.0, 3.2))
mouth = bpy.context.active_object
mouth.scale = (0.8, 0.2, 0.3)
mouth.data.materials.append(bpy.data.materials.new(name="Anya_Mouth"))
mouth.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (0.9, 0.4, 0.4, 1)

# ==============================
# 6. 手臂（拿蜡笔的造型）
# ==============================
# 右手（拿蜡笔）
bpy.ops.mesh.primitive_cylinder_add(radius=0.15, depth=0.8, location=(-0.9, 0.5, 2.8), rotation=(0, math.radians(30), 0))
arm_right = bpy.context.active_object
arm_right.data.materials.append(bpy.data.materials.new(name="Anya_Skin"))
arm_right.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (0.95, 0.85, 0.75, 1)

# 蜡笔
bpy.ops.mesh.primitive_cylinder_add(radius=0.08, depth=0.4, location=(-1.2, 0.7, 3.0), rotation=(0, math.radians(30), 0))
crayon = bpy.context.active_object
crayon.data.materials.append(bpy.data.materials.new(name="Anya_Crayon"))
crayon.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (0.9, 0.5, 0.2, 1)

# 左手
bpy.ops.mesh.primitive_cylinder_add(radius=0.15, depth=0.8, location=(0.9, 0.5, 2.8), rotation=(0, math.radians(-30), 0))
arm_left = bpy.context.active_object
arm_left.data.materials.append(bpy.data.materials.new(name="Anya_Skin"))
arm_left.data.materials[0].node_tree.nodes["Principled BSDF"].base_color = (0.95, 0.85, 0.75, 1)

# ==============================
# 7. 合并所有部件，统一父级
# ==============================
bpy.ops.object.select_all(action='SELECT')
bpy.context.view_layer.objects.active = head
bpy.ops.object.parent_set(type='OBJECT')

# ==============================
# 8. 添加灯光和相机，渲染准备
# ==============================
# 主光源
bpy.ops.object.light_add(type='SUN', location=(5, 5, 10))
sun = bpy.context.active_object
sun.data.energy = 3.5

# 补光
bpy.ops.object.light_add(type='AREA', location=(-3, -3, 5))
fill = bpy.context.active_object
fill.data.energy = 2.0

# 相机
bpy.ops.object.camera_add(location=(0, -5, 3.5), rotation=(math.radians(80), 0, 0))
camera = bpy.context.active_object
bpy.context.scene.camera = camera

# ==============================
# 9. 导出模型（可选）
# ==============================
# bpy.ops.export_scene.gltf(filepath="/tmp/anya_model.glb")
# bpy.ops.export_scene.obj(filepath="/tmp/anya_model.obj")

print("✅ 间谍过家家 阿尼亚 Blender 模型生成完成！")