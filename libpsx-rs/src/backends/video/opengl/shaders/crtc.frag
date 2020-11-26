#version 450 core

uniform sampler2D tex2d;

layout(location = 0) in vec2 in_tex_coord;
layout(location = 0) out vec4 out_color;

void main() {
    out_color = texture(tex2d, in_tex_coord);
}
