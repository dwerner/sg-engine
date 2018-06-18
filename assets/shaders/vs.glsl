// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout(set = 0, binding = 1) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
} uniforms;

layout(push_constant) uniform PushConstants {
    mat4 model_mat;
    uint material;
} push_constants;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec2 v_uv;
layout(location = 2) flat out uint v_material;


void main() {
// TODO : this is extremely wasteful of GPU time, and should be done once on the CPU per model instead

    mat4 worldview = uniforms.view * uniforms.world * push_constants.model_mat;
    v_normal = transpose(inverse(mat3(worldview))) * normal;
    gl_Position = uniforms.proj * worldview * vec4(position, 1.0);
    v_uv = uv;

  //  v_material = push_constants.material;
}
