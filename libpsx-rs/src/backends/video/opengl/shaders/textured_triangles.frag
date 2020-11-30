#version 450 core

uniform sampler2D tex2d;
uniform uint clut_mode;
uniform vec2 clut_coord_base;
uniform float tex_coord_base_x;
uniform uint transparency_mode;

layout(location = 0) in vec2 in_tex_coord;
layout(location = 0) out vec4 out_color;

const float WIDTH = 1024.0;
const float TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL = 1.0 / WIDTH;
const uint MAX_VALUE_5BIT = (1 << 5) - 1;

uint compressed_texel_value_16(const vec4 texel_color) {
    // 5551 RGBA format.
    uint packed_r_value_16 = uint(round(float(MAX_VALUE_5BIT) * texel_color.r));
    uint packed_g_value_16 = uint(round(float(MAX_VALUE_5BIT) * texel_color.g)) << 5;
    uint packed_b_value_16 = uint(round(float(MAX_VALUE_5BIT) * texel_color.b)) << 10;
    uint packed_a_value_16 = uint(round(texel_color.a)) << 15;
    return packed_r_value_16 | packed_g_value_16 | packed_b_value_16 | packed_a_value_16;
}

uint extract_bitfield(const uint value, const uint index, const uint size) {
    return (value >> (index * size)) & ((1 << size) - 1);
}

void discard_test(const vec4 color) {
    uint packed_value_16 = compressed_texel_value_16(color);

    for (uint i = 0; i < 3; i++) {
        if (extract_bitfield(packed_value_16, i, 5) != 0) {
            return;
        }
    }

    if (extract_bitfield(packed_value_16, 15, 1) != 0) {
        return;
    }

    discard;
}

void main() {
    // Default / error color.
    out_color = vec4(1.0, 0.0, 0.0, 1.0);

    if (clut_mode > 2) {
        // Invalid.
        return;
    }

    if (clut_mode == 2) {
        // 15-bits per texel (5551 direct / no CLUT).
        // No conversion needed, can directly output the texel.
        vec4 color = texture(tex2d, in_tex_coord);

        // Discard transparent pixels.
        //discard_test(color);

        // Done.
        out_color = color;
    } else {
        // Using CLUT mode.
        // Sample from the CLUT texture, correcting for the half-pixel offset w/ texcoords.
        float tex_coord_offset_x = in_tex_coord.x - tex_coord_base_x;
        uint texel_index = uint(floor(tex_coord_offset_x / TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL));
        vec2 tex_coord_real = vec2(tex_coord_base_x + (TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL * float(texel_index)), in_tex_coord.y);
        vec4 texel_color = texture(tex2d, tex_coord_real);

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

        // Determine which framebuffer pixel / sub-texel is being rendered (can use texcoords to determine this as it's a linear relationship).
        float sub_texel_offset = mod(tex_coord_offset_x, TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL);
        uint sub_texel_data_index = uint(floor(sub_texel_offset / (TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL / ratio)));

        // Get the appropriate bits out of the packed data and work out the CLUT pixel to use.
        uint clut_index = extract_bitfield(packed_value_16, sub_texel_data_index, ratio);
        vec2 clut_coord = vec2(
            clut_coord_base.x + (TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL * float(clut_index)),
            clut_coord_base.y
        );

        // Sample the CLUT.
        vec4 clut_color = texture(tex2d, clut_coord);

        // Discard transparent pixels.
        discard_test(clut_color);

        // Done.
        out_color = clut_color;
    }
}
