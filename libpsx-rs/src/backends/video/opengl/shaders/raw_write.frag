#version 450 core

uniform sampler2D framebuffer;
uniform sampler2D raw_texture;
uniform bool mask_bit_force_set;
uniform bool mask_bit_check;

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_tex_coord;
layout(location = 0) out vec4 out_color;

const float WIDTH = 1024.0;
const float HEIGHT = 512.0;
const float TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL = 1.0 / WIDTH;
const float TEXCOORD_NORMALIZED_HEIGHT_PER_PIXEL = 1.0 / HEIGHT;

bool mask_bit(const vec4 color) {
    return abs(color.a - 1.0) < 0.00001;
}

float coordinate_to_texture_coordinate_single(const float coordinate) {
    return (coordinate + 1.0) / 2.0;
}

vec2 coordinate_to_texture_coordinate(const vec2 coordinate) {
    return vec2(
        coordinate_to_texture_coordinate_single(coordinate.x),
        coordinate_to_texture_coordinate_single(coordinate.y)
    );
}

void handle_mask_bit_check() {
    if (mask_bit_check) {
        vec2 framebuffer_texture_position = coordinate_to_texture_coordinate(in_position);
        vec4 framebuffer_color = texture(framebuffer, framebuffer_texture_position);

        if (mask_bit(framebuffer_color)) {
            discard;
        }
    }
}

void handle_mask_bit_force_set() {
    if (mask_bit_force_set) {
        out_color.a = 1.0;
    }
}

void handle_render() {
    out_color = texture(raw_texture, in_tex_coord);
}

void main() {
    // Default / error color
    out_color = vec4(1.0, 0.0, 0.0, 1.0);

    handle_mask_bit_check();
    handle_render();
    handle_mask_bit_force_set();
}
