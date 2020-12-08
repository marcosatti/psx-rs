#version 450 core

uniform sampler2D framebuffer;
uniform uint rendering_mode;
uniform vec2 texture_position_base;
uniform uint clut_mode;
uniform vec2 clut_texture_position_base;
uniform uint transparency_mode;
uniform bool mask_bit_force_set;
uniform bool mask_bit_check;

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec3 in_color;
layout(location = 2) in vec2 in_texture_position_offset;
layout(location = 0) out vec4 out_color;

const uint MAX_VALUE_5BIT = (1 << 5) - 1;
const float WIDTH = 1024.0;
const float TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL = 1.0 / WIDTH;
const uint RENDERING_MODE_SHADED = 0;
const uint RENDERING_MODE_TEXTURE_BLENDING = 1;
const uint RENDERING_MODE_RAW_TEXTURE = 2;
const uint TRANSPARENCY_MODE_OPAQUE = 0;
const uint TRANSPARENCY_MODE_AVERAGE = 1;
const uint TRANSPARENCY_MODE_ADDITIVE = 2;
const uint TRANSPARENCY_MODE_DIFFERENCE = 3;
const uint TRANSPARENCY_MODE_QUARTER = 4;
const uint CLUT_MODE_DIRECT = 0;
const uint CLUT_MODE_BIT4 = 0;
const uint CLUT_MODE_BIT8 = 1;

bool mask_bit(const vec4 texture_color) {
    return abs(texture_color.a - 1.0) < 0.00001;
}

bool transparency_flag(const vec4 texture_color) {
    return mask_bit(texture_color);
}

float coordinate_to_texture_coordinate_single(const float coordinate) {
    return (coordinate + 1.0) / 2.0;
}

float size_to_texture_size_single(const float size) {
    return size / 2.0;
}

vec2 coordinate_to_texture_coordinate(const vec2 coordinate) {
    return vec2(
        coordinate_to_texture_coordinate_single(coordinate.x),
        coordinate_to_texture_coordinate_single(coordinate.y)
    );
}

vec2 size_to_texture_size(const vec2 size) {
    return vec2(
        size_to_texture_size_single(size.x),
        size_to_texture_size_single(size.y)
    );
}

uint compressed_texel_value_16(const vec4 texture_color) {
    // 5551 RGBA format.
    uint packed_r_value_16 = uint(round(float(MAX_VALUE_5BIT) * texture_color.r));
    uint packed_g_value_16 = uint(round(float(MAX_VALUE_5BIT) * texture_color.g)) << 5;
    uint packed_b_value_16 = uint(round(float(MAX_VALUE_5BIT) * texture_color.b)) << 10;
    uint packed_a_value_16 = uint(round(texture_color.a)) << 15;
    return packed_r_value_16 | packed_g_value_16 | packed_b_value_16 | packed_a_value_16;
}

uint extract_bitfield(const uint value, const uint index, const uint size) {
    return (value >> (index * size)) & ((1 << size) - 1);
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

bool is_texture_color_fully_transparent(const vec4 texture_color) {
    return compressed_texel_value_16(texture_color) == 0;
}

void handle_transparency() {
    if (transparency_mode == TRANSPARENCY_MODE_OPAQUE) {
        return;
    } 

    vec2 framebuffer_texture_position = coordinate_to_texture_coordinate(in_position);
    vec4 framebuffer_color = texture(framebuffer, framebuffer_texture_position);
    
    float background_alpha = 0.0;
    float foreground_alpha = 0.0;

    if (transparency_mode == TRANSPARENCY_MODE_AVERAGE) {
        background_alpha = 0.5;
        foreground_alpha = 0.5;
    } else if (transparency_mode == TRANSPARENCY_MODE_ADDITIVE) {
        background_alpha = 1.0;
        foreground_alpha = 1.0;
    } else if (transparency_mode == TRANSPARENCY_MODE_DIFFERENCE) {
        background_alpha = 1.0;
        foreground_alpha = -1.0;
    } else if (transparency_mode == TRANSPARENCY_MODE_QUARTER) {
        background_alpha = 1.0;
        foreground_alpha = 0.25;
    }

    out_color.rgb = (background_alpha * framebuffer_color.rgb) + (foreground_alpha * out_color.rgb);
    out_color.rgb = clamp(out_color.rgb, 0.0, 1.0);
}

void handle_clut() {
    float ratio = 1.0;
    if (clut_mode == CLUT_MODE_BIT4) {
        // 4-bits per sub-texel; 4 sub-texels per texel.
        ratio = 4.0;
    } else if (clut_mode == CLUT_MODE_BIT8) {
        // 8-bits per sub-texel; 2 sub-texels per texel.
        ratio = 8.0;
    }

    vec2 offset = in_texture_position_offset - texture_position_base;
    offset.x = offset.x / ratio;
    vec2 texture_position = texture_position_base + offset;
    vec4 texture_color = texture(framebuffer, texture_position);

    if (is_texture_color_fully_transparent(texture_color)) {
        discard;
    }

    if (clut_mode == CLUT_MODE_DIRECT) {
        out_color = texture_color;
    } else {
        uint packed_value_16 = compressed_texel_value_16(texture_color);

        // Determine which framebuffer pixel / sub-texel is being rendered.
        float sub_texel_offset = mod(offset.x, TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL);
        uint sub_texel_data_index = uint(floor(sub_texel_offset / (TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL / ratio)));

        // Get the appropriate bits out of the packed data and work out the CLUT pixel to use.
        uint clut_index = extract_bitfield(packed_value_16, sub_texel_data_index, uint(ratio));
        vec2 clut_texture_offset = vec2(TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL * float(clut_index), 0.0);
        vec2 clut_texture_position = clut_texture_position_base + clut_texture_offset;

        // Sample the CLUT.
        vec4 clut_texture_color = texture(framebuffer, clut_texture_position);

        if (is_texture_color_fully_transparent(clut_texture_color)) {
            discard;
        }

        out_color = clut_texture_color;
    }
}

void handle_render() {
    if (rendering_mode == RENDERING_MODE_SHADED) {
        out_color = vec4(in_color.rgb, 0.0);
        handle_transparency();
    } else {
        handle_clut();

        if (rendering_mode == RENDERING_MODE_TEXTURE_BLENDING) {
            // TODO: Whats the proper formula for this??? 
            out_color.rgb = mix(out_color.rgb, in_color.rgb, 0.5);
        }

        if (transparency_flag(out_color)) {
            handle_transparency();
        }
    }
}

void main() {
    // Default / error color
    out_color = vec4(1.0, 0.0, 0.0, 1.0);

    if (rendering_mode > 2) {
        return;
    }

    if (clut_mode > 2) {
        return;
    }

    if (transparency_mode > 4) {
        return;
    }

    handle_mask_bit_check();
    handle_render();
    handle_mask_bit_force_set();
}
