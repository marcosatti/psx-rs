#version 450 core

uniform uint transparency_mode;

layout(location = 0) in vec4 in_color;
layout(location = 0) out vec4 out_color;

bool transparency_flag(const vec4 color) {
    return abs(in_color.a - 1.0) < 0.00001;
}

void main() {
    // Default / error color.
    out_color = vec4(1.0, 0.0, 0.0, 1.0);

    if (transparency_mode > 4) {
        // Invalid.
        return;
    }

    if ((transparency_mode == 0) || (!transparency_flag(in_color))) {
        // Opaque.
        out_color = in_color;
    } else if (transparency_mode == 1) {
        // Average.
    } else if (transparency_mode == 2) {

    } else if (transparency_mode == 3) {

    } else if (transparency_mode == 4) {

    }
}
