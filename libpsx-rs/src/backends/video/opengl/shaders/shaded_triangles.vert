#version 450 core

layout(location = 0) in vec2 in_coord;
layout(location = 1) in vec4 in_color;
layout(location = 0) out vec4 out_color;

void main() {
    gl_Position = vec4(in_coord, 0.0, 1.0);
    out_color = in_color;
}