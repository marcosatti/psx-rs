#version 450 core

uniform sampler2D framebuffer;

layout(location = 0) in vec2 in_texture_position;
layout(location = 0) out vec4 out_color;

void main() {
    out_color = texture(framebuffer, in_texture_position);
}
