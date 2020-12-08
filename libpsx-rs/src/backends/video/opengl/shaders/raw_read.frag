#version 450 core

uniform sampler2D framebuffer;

layout(location = 0) in vec2 in_tex_coord;
layout(location = 0) out vec4 out_color;

void main() {
    // Default / error color
    out_color = vec4(1.0, 0.0, 0.0, 1.0);
    
    out_color = texture(framebuffer, in_tex_coord);
}
