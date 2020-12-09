#version 450 core

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_texture_position;
layout(location = 0) out vec2 out_texture_position;

void main() {
    gl_Position = vec4(in_position, 0.0, 1.0);
    out_texture_position = in_texture_position;
}
