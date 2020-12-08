#version 450 core

uniform sampler2D framebuffer;

layout(location = 0) in vec2 in_tex_coord;
layout(location = 0) out vec4 out_color;

void main() {
    out_color = texture(framebuffer, in_tex_coord);
}
