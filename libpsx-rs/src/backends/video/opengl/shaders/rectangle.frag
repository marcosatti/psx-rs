#version 450 core

uniform sampler2D framebuffer;
uniform vec2 position_base; 
uniform vec3 color;
uniform uint rendering_mode;
uniform vec2 texture_position_base;
uniform vec2 texture_position_base_offset;
uniform uint clut_mode;
uniform vec2 clut_texture_position_base;
uniform uint transparency_mode;
uniform vec2 drawing_area_top_left;
uniform vec2 drawing_area_bottom_right;
uniform bool mask_bit_force_set;
uniform bool mask_bit_check;

layout(location = 0) in vec2 in_position;
layout(location = 0) out vec4 out_color;

/////////////////
/// Constants ///
/////////////////

const uint MAX_VALUE_5BIT = (1 << 5) - 1;
const float NORMALIZED_WIDTH_PER_PIXEL = 2.0 / 1024.0;
const float NORMALIZED_HEIGHT_PER_PIXEL = 2.0 / 512.0;
const float TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL = 1.0 / 1024.0;
const float TEXCOORD_NORMALIZED_HEIGHT_PER_PIXEL = 1.0 / 512.0;
const uint RENDERING_MODE_SHADED = 0;
const uint RENDERING_MODE_TEXTURE_BLENDING = 1;
const uint RENDERING_MODE_RAW_TEXTURE = 2;
const uint TRANSPARENCY_MODE_OPAQUE = 0;
const uint TRANSPARENCY_MODE_AVERAGE = 1;
const uint TRANSPARENCY_MODE_ADDITIVE = 2;
const uint TRANSPARENCY_MODE_DIFFERENCE = 3;
const uint TRANSPARENCY_MODE_QUARTER = 4;
const uint CLUT_MODE_DIRECT = 0;
const uint CLUT_MODE_BIT4 = 1;
const uint CLUT_MODE_BIT8 = 2;

/////////////////////////
/// Utility functions ///
/////////////////////////

vec2 coordinate_to_texture_coordinate(const vec2 coordinate) {
    return vec2((coordinate.x + 1.0) / 2.0, (coordinate.y + 1.0) / 2.0);
}

vec2 size_to_texture_size(const vec2 size) {
    return vec2(size.x / 2.0, size.y / 2.0);
}

bool alpha_set(const vec4 texture_color) {
    return abs(texture_color.a - 1.0) < 0.00001;
}

uint extract_bitfield(const uint value, const uint index, const uint size) {
    return (value >> (index * size)) & ((1 << size) - 1);
}

uint texel_value_5551(const vec4 texture_color) {
    // 5551 RGBA format.
    uint packed_r_value_16 = uint(round(float(MAX_VALUE_5BIT) * texture_color.r));
    uint packed_g_value_16 = uint(round(float(MAX_VALUE_5BIT) * texture_color.g)) << 5;
    uint packed_b_value_16 = uint(round(float(MAX_VALUE_5BIT) * texture_color.b)) << 10;
    uint packed_a_value_16 = uint(round(texture_color.a)) << 15;
    return packed_r_value_16 | packed_g_value_16 | packed_b_value_16 | packed_a_value_16;
}

////////////////////
/// Shader logic ///
////////////////////

void handle_drawing_area_clipping();
void handle_mask_bit_check();
void handle_mask_bit_force_set();
void handle_transparency();
void handle_clut();
void handle_render();

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

    handle_drawing_area_clipping();
    handle_mask_bit_check();
    handle_render();
    handle_mask_bit_force_set();
}

void handle_drawing_area_clipping() {
    if ((in_position.x < drawing_area_top_left.x) || 
        (in_position.x > (drawing_area_bottom_right.x + NORMALIZED_WIDTH_PER_PIXEL)) || 
        (in_position.y < (drawing_area_bottom_right.y - NORMALIZED_HEIGHT_PER_PIXEL)) ||
        (in_position.y > drawing_area_top_left.y)) 
    {
        discard;
    }
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
    if (rendering_mode == RENDERING_MODE_SHADED) {
        out_color = vec4(color.rgb, 0.0);
        handle_transparency();
    } else {
        handle_clut();

        if (rendering_mode == RENDERING_MODE_TEXTURE_BLENDING) {
            out_color.rgb = out_color.rgb * color.rgb / 0.5;
        }

        if (alpha_set(out_color)) {
            handle_transparency();
        }
    }
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
    uint size = 16;
    float ratio = 1.0;
    if (clut_mode == CLUT_MODE_BIT4) {
        // 4-bits per sub-texel; 4 sub-texels per texel.
        ratio = 4.0;
        size = 4;
    } else if (clut_mode == CLUT_MODE_BIT8) {
        // 8-bits per sub-texel; 2 sub-texels per texel.
        ratio = 2.0;
        size = 8;
    }

    vec2 texture_position = texture_position_base;
    texture_position.x += texture_position_base_offset.x / ratio;
    texture_position.y -= texture_position_base_offset.y;
    vec2 offset = size_to_texture_size(in_position - position_base);
    texture_position.x += offset.x / ratio;
    texture_position.y += offset.y;
    offset.x = offset.x / ratio;
    vec4 texture_color = texture(framebuffer, texture_position);

    if (clut_mode == CLUT_MODE_DIRECT) {
        out_color = texture_color;
    } else {
        // Determine which framebuffer pixel / sub-texel is being rendered.
        float sub_texel_offset = mod(offset.x, TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL);
        uint sub_texel_data_index = uint(floor(sub_texel_offset / (TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL / ratio)));

        // Get the appropriate bits out of the packed data and work out the CLUT pixel to use. 
        // Apply a half-pixel offset to ensure we sample the correct texel.
        uint raw_value = texel_value_5551(texture_color);
        uint clut_index = extract_bitfield(raw_value, sub_texel_data_index, size);
        vec2 clut_texture_offset = vec2(TEXCOORD_NORMALIZED_WIDTH_PER_PIXEL * (0.5 + float(clut_index)), -0.5 * TEXCOORD_NORMALIZED_HEIGHT_PER_PIXEL);
        vec2 clut_texture_position = clut_texture_position_base + clut_texture_offset;

        // Sample the CLUT.
        out_color = texture(framebuffer, clut_texture_position);
    }
    
    // Check for a fully-transparent texel.
    if (texel_value_5551(out_color) == 0) {
        discard;
    }
}

void handle_mask_bit_force_set() {
    if (mask_bit_force_set) {
        out_color.a = 1.0;
    }
}
