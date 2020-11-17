#version 450 core

uniform sampler2D tex2d;
in vec2 texcoord;
out vec4 color;

void main() {
    color = texture(tex2d, texcoord);
}
