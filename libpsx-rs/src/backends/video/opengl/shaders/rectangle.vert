#version 450 core

layout(location = 0) in vec2 in_coord;
layout(location = 1) in vec2 in_tex_coord;
layout(location = 0) out vec2 out_coord;
layout(location = 1) out vec2 out_tex_coord;

void main() {
    gl_Position = vec4(in_coord, 0.0, 1.0);
    out_coord = in_coord;
    out_tex_coord = in_tex_coord;
}
