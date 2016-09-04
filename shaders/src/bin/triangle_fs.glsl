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

layout(location = 0) out vec4 f_color;

void main() {
	vec4 top = vec4(1.0, 0.0, 1.0, 1.0);
	vec4 bottom = vec4(1.0, 1.0, 0.0, 1.0);
	f_color = vec4(mix(bottom, top, gl_FragCoord.y));
}
