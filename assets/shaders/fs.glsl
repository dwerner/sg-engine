#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_450pack : enable

layout(set = 0, binding = 0) uniform sampler2D tex;

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec2 v_uv;

layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.5, 0.5, 0.5);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    vec3 dark_color = vec3(0.7, 0.7, 0.7);
    vec3 regular_color = vec3(1.0, 1.0, 1.0);

    f_color = texture(tex, v_uv);// * vec4(mix(dark_color, regular_color, brightness), 1.0);
    //f_color = vec4(1.0, 1.0, 1.0, 1.0);
}
