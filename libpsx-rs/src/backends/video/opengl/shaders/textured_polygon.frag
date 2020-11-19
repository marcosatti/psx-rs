#version 450 core

uniform sampler2D tex2d;
uniform uint clut_mode;
uniform vec2 clut_coord_base;
uniform float tex_coord_base_x;

layout(location = 0) in vec2 in_tex_coord;
layout(location = 0) out vec4 out_color;

const float WIDTH = 1024.0;
const float TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL = 1.0 / WIDTH;
const float MAX_VALUE_5BIT = 31.0;

uint compressed_texel_value_16(const vec4 texel_color) {
    // 5551 RGBA format.
    vec4 unpacked_value_16 = floor(vec4(MAX_VALUE_5BIT * texel_color.rgb, round(texel_color.a)));
    uint packed_r_value_16 = uint(unpacked_value_16.r);
    uint packed_g_value_16 = uint(unpacked_value_16.g) << 5;
    uint packed_b_value_16 = uint(unpacked_value_16.b) << 10;
    uint packed_a_value_16 = uint(unpacked_value_16.a) << 15;
    uint packed_value_16 = packed_r_value_16 | packed_g_value_16 | packed_g_value_16 | packed_a_value_16;
    return packed_value_16;
}

void main() {
    // Default / error color.
    out_color = vec4(1.0, 0.0, 0.0, 1.0);

    if (clut_mode > 2) {
        return;
    }

    // Get raw texture color.
    vec4 texel_color = texture(tex2d, in_tex_coord);

    if (clut_mode == 2) {
        // 15-bits per texel (5551 direct / no CLUT).
        // No conversion needed, can directly output the texel.
        out_color = texel_color;
    } else {
        // Using CLUT mode.
        // Convert texel back to 16-bit value.
        uint packed_value_16 = compressed_texel_value_16(texel_color);

        uint ratio = 1;
        if (clut_mode == 0) {
            // 4-bits per sub-texel; 4 sub-texels per texel.
            ratio = 4;
        } else if (clut_mode == 1) {
            // 8-bits per sub-texel; 2 sub-texels per texel.
            ratio = 8;
        }

        float tex_offset_x = in_tex_coord.x - tex_coord_base_x;
        float sub_pixel_offset = mod(tex_offset_x, TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL);
        uint sub_pixel_data_index = uint(floor(sub_pixel_offset / (TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL / ratio)));
        uint sub_pixel_index = (packed_value_16 >> (sub_pixel_data_index * ratio)) & ((1 << ratio) - 1);
        vec2 clut_coord = vec2(
            clut_coord_base.x + (TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL * float(sub_pixel_index)),
            clut_coord_base.y
        );
        out_color = texture(tex2d, clut_coord);
    }
}
