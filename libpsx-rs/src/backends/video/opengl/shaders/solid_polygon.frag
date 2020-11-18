#version 450 core

uniform vec4 color;
uniform uint transparency;

layout(location = 0) out vec4 out_color;

void main() {
    out_color = color;
}
