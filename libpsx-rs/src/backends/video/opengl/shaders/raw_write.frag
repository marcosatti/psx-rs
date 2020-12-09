#version 450 core

uniform sampler2D framebuffer;
uniform sampler2D raw_texture;
uniform bool mask_bit_force_set;
uniform bool mask_bit_check;

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_texture_position;
layout(location = 0) out vec4 out_color;

/////////////////////////
/// Utility functions ///
/////////////////////////

bool alpha_set(const vec4 texture_color) {
    return abs(texture_color.a - 1.0) < 0.00001;
}

vec2 coordinate_to_texture_coordinate(const vec2 coordinate) {
    return vec2((coordinate.x + 1.0) / 2.0, (coordinate.y + 1.0) / 2.0);
}

////////////////////
/// Shader logic ///
////////////////////

void handle_mask_bit_check();
void handle_mask_bit_force_set();
void handle_render();

void main() {
    // Default / error color
    out_color = vec4(1.0, 0.0, 0.0, 1.0);

    handle_mask_bit_check();
    handle_render();
    handle_mask_bit_force_set();
}

void handle_mask_bit_check() {
    if (mask_bit_check) {
        vec2 framebuffer_texture_position = coordinate_to_texture_coordinate(in_position);
        vec4 framebuffer_color = texture(framebuffer, framebuffer_texture_position);

        if (alpha_set(framebuffer_color)) {
            discard;
        }
    }
}

void handle_render() {
    out_color = texture(raw_texture, in_texture_position);
}

void handle_mask_bit_force_set() {
    if (mask_bit_force_set) {
        out_color.a = 1.0;
    }
}
