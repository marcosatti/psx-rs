use log::debug;
use euclid::{Rect, Point2D, point2, rect};
use crate::backends::video::VideoBackend;
use crate::State;
use crate::types::bitfield::Bitfield;
use crate::controllers::gpu::open_gl;
use crate::resources::gpu::*;

pub unsafe fn handle_command(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;

    let b24_color_depth = stat.read_bitfield(STAT_DISPLAY_COLOR_DEPTH) !=0;
    if b24_color_depth { unimplemented!("24 bit color depth not supported yet"); }

    let fifo = &mut resources.gpu.gpu1810.gp0;
    
    if fifo.len() == 0 {
        return;
    }

    let _lock = resources.gpu.gpu1810.gp0_mutex.lock();

    let command = fifo.front();
    if let Some(&value) = command {
        let cmd = GP_CMD.extract_from(value) as u8;

        match cmd {
            0x00 => command_00(fifo.pop_front().unwrap()),
            0x01 => command_01(fifo.pop_front().unwrap()),
            0x05 => command_05(fifo.pop_front().unwrap()),
            0x06 => command_06(fifo.pop_front().unwrap()),
            0x0c => command_0c(fifo.pop_front().unwrap()),
            0x28 => {
                if fifo.len() < 5 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_28(state, value);
            },
            0x2C => {
                if fifo.len() < 9 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_2c(state, value);
            }
            0x30 => {
                if fifo.len() < 6 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_30(state, value);
            },
            0x38 => {
                if fifo.len() < 8 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_38(state, value);
            },
            0x3C => {
                if fifo.len() < 12 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_3c(state, value);
            },
            0x50 => {
                if fifo.len() < 4 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_50(state, value);
            },
            0x6F => {
                if fifo.len() < 3 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_6f(state, value);
            },
            0x80 => {
                if fifo.len() < 4 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_80(state, value);
            },
            0xA0 => {
                if fifo.len() < 3 { return; }

                let header = [fifo[0], fifo[1], fifo[2]];
                let width = Bitfield::new(0, 16).extract_from(header[2]) as usize;
                let height = Bitfield::new(16, 16).extract_from(header[2]) as usize;
                let count = width * height;
                let fifo_words = (count + 1) / 2;

                if fifo.len() < (3 + fifo_words) { return; }

                let header = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                let data = fifo.drain(0..fifo_words).collect::<Vec<_>>();
                command_a0(state, header, &data);
            },
            0xC0 => {
                if fifo.len() < 3 { return; }
                let value = [fifo.pop_front().unwrap(), fifo.pop_front().unwrap(), fifo.pop_front().unwrap()];
                command_c0(state, value);
            },
            0xE1 => command_e1(state, fifo.pop_front().unwrap()),
            0xE2 => command_e2(state, fifo.pop_front().unwrap()),
            0xE3 => command_e3(state, fifo.pop_front().unwrap()),
            0xE4 => command_e4(state, fifo.pop_front().unwrap()),
            0xE5 => command_e5(state, fifo.pop_front().unwrap()),
            0xE6 => command_e6(state, fifo.pop_front().unwrap()),
            _ => unimplemented!("Unknown GP0 command: 0x{:0X} (0x{:0X})", cmd, value),
        }
    }
}

fn command_00(_command: u32) {
    // NOP
    //debug!("NOP 00, c1 = 0x{:0X}", _command);
}

fn command_01(_command: u32) {
    // Flush cache (NOP)
    //debug!("Flush cache (NOP) 01, c1 = 0x{:0X}", _command);
}

fn command_05(_command: u32) {
    // NOP
    //debug!("NOP 05, c1 = 0x{:0X}", _command);
}

fn command_06(_command: u32) {
    // NOP
    //debug!("NOP 06, c1 = 0x{:0X}", _command);
}

fn command_0c(_command: u32) {
    // NOP
    //debug!("NOP 0C, c1 = 0x{:0X}", _command);
}

unsafe fn command_28(state: &State, values: [u32; 5]) {
    //debug!("Monochrome four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4]);
    
    let (r, g, b, a) = (
        Bitfield::new(0, 8).extract_from(values[0]) as u8, 
        Bitfield::new(8, 8).extract_from(values[0]) as u8, 
        Bitfield::new(16, 8).extract_from(values[0]) as u8, 
        std::u8::MAX
    );
    
    let vertices = {
        let x_bitfield = Bitfield::new(0, 16);
        let y_bitfield = Bitfield::new(16, 16);

        let mut vx = [
            x_bitfield.extract_from(values[1]) as f32,
            x_bitfield.extract_from(values[2]) as f32,
            x_bitfield.extract_from(values[3]) as f32,
            x_bitfield.extract_from(values[4]) as f32,
        ];
        for v in vx.iter_mut() { *v = ((*v as f64 - 511.0) / 511.0) as f32; }

        let mut vy = [
            y_bitfield.extract_from(values[1]) as f32,
            y_bitfield.extract_from(values[2]) as f32,
            y_bitfield.extract_from(values[3]) as f32,
            y_bitfield.extract_from(values[4]) as f32,
        ];
        for v in vy.iter_mut() { *v = (-((*v as f64 - 255.0) / 255.0)) as f32; }

        [point2(vx[0], vy[0]), point2(vx[1], vy[1]), point2(vx[2], vy[2]), point2(vx[3], vy[3])]
    };

    match state.video_backend {
        VideoBackend::OpenGl(ref backend_params) => {
            open_gl::draw_polygon_4_solid(backend_params, vertices, r, g, b, a);
        },
    }
}

fn command_2c(state: &State, values: [u32; 9]) {
    //debug!("Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8]);

    // TODO: implement this properly - need to make a shader to do this I think...
    // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

    let (r, g, b, a) = (
        Bitfield::new(0, 8).extract_from(values[0]) as u8, 
        Bitfield::new(8, 8).extract_from(values[0]) as u8, 
        Bitfield::new(16, 8).extract_from(values[0]) as u8, 
        std::u8::MAX
    );

    let texpage = Bitfield::new(16, 16).extract_from(values[4]);
    let texpage_x_base_raw = Bitfield::new(0, 4).extract_from(texpage) * 64;
    let texpage_y_base_raw = Bitfield::new(4, 1).extract_from(texpage) * 256;
    let texpage_trans = Bitfield::new(5, 2).extract_from(texpage);
    let texpage_color_fmt = Bitfield::new(7, 2).extract_from(texpage);

    let clut = Bitfield::new(16, 16).extract_from(values[2]);
    let clut_x = Bitfield::new(0, 6).extract_from(clut) * 16;
    let clut_y = Bitfield::new(6, 9).extract_from(clut);

    let vertices_raw: [Point2D<_>; 4] = {
        let x_bitfield = Bitfield::new(0, 16);
        let y_bitfield = Bitfield::new(16, 16);

        let vx = [
            x_bitfield.extract_from(values[1]) as usize,
            x_bitfield.extract_from(values[3]) as usize,
            x_bitfield.extract_from(values[5]) as usize,
            x_bitfield.extract_from(values[7]) as usize,
        ];

        let vy = [
            y_bitfield.extract_from(values[1]) as usize,
            y_bitfield.extract_from(values[3]) as usize,
            y_bitfield.extract_from(values[5]) as usize,
            y_bitfield.extract_from(values[7]) as usize,
        ];

        [point2(vx[0], vy[0]), point2(vx[1], vy[1]), point2(vx[2], vy[2]), point2(vx[3], vy[3])]
    };

    let vertices: [Point2D<_>; 4] = {
        let mut buffer = [point2(0.0, 0.0); 4];
        for i in 0..4 {
            let vertex = &vertices_raw[i];
            let vx = ((vertex.x as f64 - 511.0) / 511.0) as f32;
            let vy = (-((vertex.y as f64 - 255.0) / 255.0)) as f32;
            buffer[i] = point2(vx, vy);
        }

        buffer
    };

    let texcoords_raw: [Point2D<_>; 4] = {
        let x_bitfield = Bitfield::new(0, 8);
        let y_bitfield = Bitfield::new(8, 8);

        let tx = [
            x_bitfield.extract_from(values[2]),
            x_bitfield.extract_from(values[4]),
            x_bitfield.extract_from(values[6]),
            x_bitfield.extract_from(values[8]),
        ];

        let ty = [
            y_bitfield.extract_from(values[2]),
            y_bitfield.extract_from(values[4]),
            y_bitfield.extract_from(values[6]),
            y_bitfield.extract_from(values[8]),
        ];

        [point2(tx[0], ty[0]), point2(tx[1], ty[1]), point2(tx[2], ty[2]), point2(tx[3], ty[3])]
    };

    let texpage_x_base = texpage_x_base_raw as f32 / 1023.0;
    let texpage_y_base = (511 - texpage_y_base_raw) as f32 / 511.0;
    let texcoords: [Point2D<_>; 4] = {
        let mut buffer = [point2(0.0, 0.0); 4];
        for i in 0..4 {
            match texpage_color_fmt {
                0 => {
                    // 4 Bit -> 4 texture pixels per 16-bit framebuffer pixel.
                    // Raw texcoords specified in terms of 4-bit pixels.
                    let tx = texpage_x_base + (256.0 / 1024.0) * ((texcoords_raw[i].x as f32 / 255.0) / 4.0);
                    let ty = texpage_y_base - (256.0 / 512.0) * (texcoords_raw[i].y as f32 / 255.0);
                    buffer[i] = point2(tx, ty);
                },
                _ => unimplemented!("texpage_color_fmt not handled"),
            }
        }

        buffer
    };

    // debug!("vertices raw = {:?}", vertices_raw);
    // debug!("texcoords raw = {:?}", texcoords_raw);
    // debug!("texpage: x_base = {}, y_base = {}, trans = {}, color_fmt = {}", texpage_x_base_raw, texpage_y_base_raw, texpage_trans, texpage_color_fmt);
    // debug!("clut: x = {}, y = {}", clut_x, clut_y);

    match state.video_backend {
        VideoBackend::OpenGl(ref backend_params) => {
            open_gl::draw_polygon_4_fb_blended(backend_params, vertices, texcoords);
        },
    }
}

fn command_30(state: &State, values: [u32; 6]) {
    //debug!("Shaded three-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5]);

    let colors = {
        let r_bitfield = Bitfield::new(0, 8);
        let g_bitfield = Bitfield::new(8, 8);
        let b_bitfield = Bitfield::new(16, 8);

        let mut vertex_colors: [(u8, u8, u8, u8); 3] = [(0, 0, 0, 0); 3];
        for i in 0..3 {
            let value = values[i * 2];
            vertex_colors[i] = (r_bitfield.extract_from(value) as u8, g_bitfield.extract_from(value) as u8, b_bitfield.extract_from(value) as u8, std::u8::MAX);
        }

        vertex_colors
    };
    
    let vertices = {
        let x_bitfield = Bitfield::new(0, 16);
        let y_bitfield = Bitfield::new(16, 16);

        let mut vx = [
            x_bitfield.extract_from(values[1]) as f32,
            x_bitfield.extract_from(values[3]) as f32,
            x_bitfield.extract_from(values[5]) as f32,
        ];
        for v in vx.iter_mut() { *v = ((*v as f64 - 511.0) / 511.0) as f32; }

        let mut vy = [
            y_bitfield.extract_from(values[1]) as f32,
            y_bitfield.extract_from(values[3]) as f32,
            y_bitfield.extract_from(values[5]) as f32,
        ];
        for v in vy.iter_mut() { *v = (-((*v as f64 - 255.0) / 255.0)) as f32; }

        [point2(vx[0], vy[0]), point2(vx[1], vy[1]), point2(vx[2], vy[2])]
    };

    match state.video_backend {
        VideoBackend::OpenGl(ref backend_params) => {
            open_gl::draw_polygon_3_shaded(backend_params, vertices, colors);
        },
    }
}

fn command_38(state: &State, values: [u32; 8]) {
    //debug!("Shaded four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7]);
    
    let colors = {
        let r_bitfield = Bitfield::new(0, 8);
        let g_bitfield = Bitfield::new(8, 8);
        let b_bitfield = Bitfield::new(16, 8);

        let mut vertex_colors: [(u8, u8, u8, u8); 4] = [(0, 0, 0, 0); 4];
        for i in 0..4 {
            let value = values[i * 2];
            vertex_colors[i] = (r_bitfield.extract_from(value) as u8, g_bitfield.extract_from(value) as u8, b_bitfield.extract_from(value) as u8, std::u8::MAX);
        }

        vertex_colors
    };
    
    let vertices = {
        let x_bitfield = Bitfield::new(0, 16);
        let y_bitfield = Bitfield::new(16, 16);

        let mut vx = [
            x_bitfield.extract_from(values[1]) as f32,
            x_bitfield.extract_from(values[3]) as f32,
            x_bitfield.extract_from(values[5]) as f32,
            x_bitfield.extract_from(values[7]) as f32,
        ];
        for v in vx.iter_mut() { *v = ((*v as f64 - 511.0) / 511.0) as f32; }

        let mut vy = [
            y_bitfield.extract_from(values[1]) as f32,
            y_bitfield.extract_from(values[3]) as f32,
            y_bitfield.extract_from(values[5]) as f32,
            y_bitfield.extract_from(values[7]) as f32,
        ];
        for v in vy.iter_mut() { *v = (-((*v as f64 - 255.0) / 255.0)) as f32; }

        [point2(vx[0], vy[0]), point2(vx[1], vy[1]), point2(vx[2], vy[2]), point2(vx[3], vy[3])]
    };

    match state.video_backend {
        VideoBackend::OpenGl(ref backend_params) => {
            open_gl::draw_polygon_4_shaded(backend_params, vertices, colors);
        },
    }
}

fn command_3c(_state: &State, values: [u32; 12]) {
    debug!("Shaded Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}, c10 = 0x{:0X}, c11 = 0x{:0X}, c12 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8], values[9], values[10], values[11]);
}

fn command_50(_state: &State, values: [u32; 4]) {
    debug!("Shaded line, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", values[0], values[1], values[2], values[3]);
}

fn command_6f(_state: &State, values: [u32; 3]) {
    debug!("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", values[0], values[1], values[2]);
}

fn command_80(_state: &State, values: [u32; 4]) {
    debug!("Copy Rectangle (VRAM to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", values[0], values[1], values[2], values[3]);
}

fn command_a0(state: &State, values: [u32; 3], data: &[u32]) {
    //debug!("Copy Rectangle (CPU to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, data len = {}", values[0], values[1], values[2], data.len());
    
    let start_x = Bitfield::new(0, 16).extract_from(values[1]);
    let start_y = Bitfield::new(16, 16).extract_from(values[1]);
    
    let width = Bitfield::new(0, 16).extract_from(values[2]) as usize;
    let height = Bitfield::new(16, 16).extract_from(values[2]) as usize;

    let count = width * height;
    let fifo_words = (count + 1) / 2;
    if data.len() != fifo_words {
        panic!("Data does not contain the required data length, need = {}, given = {}", fifo_words, data.len());
    }

    let rect: Rect<f32> = rect(
        ((start_x as i32) - 511) as f32 / 511.0,
        (-(start_y as i32 + height as i32) + 255) as f32 / 255.0,
        (width as f32 / 1024.0) * 2.0, 
        (height as f32 / 512.0) * 2.0, 
    );

    // TODO: This is not a proper way to implement this command - the halfwords do not strictly represent pixels (16-bit colors / 5-5-5-1 colors).
    // However, the command addresses the VRAM (and incoming data) as 16-bit units through the coordinates given.
    // Ie: they could be 24-bit pixel data... gives correct result for now. Hope this makes sense.... :/

    let mut data_rgb = Vec::with_capacity(data.len() * 2 * 3);
    for i in 0..data.len() {
        let color_bitfields = [Bitfield::new(0, 16), Bitfield::new(16, 16)];
        for field in color_bitfields.iter() {
            let packed_16 = field.extract_from(data[i]);
            let r = (Bitfield::new(0, 5).extract_from(packed_16) * 255 / 31) as u8;
            let g = (Bitfield::new(5, 5).extract_from(packed_16) * 255 / 31) as u8;
            let b = (Bitfield::new(10, 5).extract_from(packed_16) * 255 / 31) as u8;
            let _mask = Bitfield::new(15, 1).extract_from(packed_16) != 0;
            data_rgb.push(r);
            data_rgb.push(g);
            data_rgb.push(b);
        }
    }

    match state.video_backend {
        VideoBackend::OpenGl(ref backend_params) => {
            open_gl::draw_rectangle_textured(backend_params, rect, width, height, &data_rgb);
        },
    }
}

unsafe fn command_c0(state: &State, values: [u32; 3]) {
    //debug!("Copy Rectangle (VRAM to CPU), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", values[0], values[1], values[2]);
    let width = Bitfield::new(0, 16).extract_from(values[2]) as usize;
    let height = Bitfield::new(16, 16).extract_from(values[2]) as usize;
    let count = width * height;
    let fifo_words = (count + 1) / 2;

    debug!("Pushing {} words of 0xFFFF_FFFF to GPUREAD", fifo_words);

    let resources = &mut *state.resources;
    let _lock = resources.gpu.gpu1810.read_mutex.lock();
    let read = &mut resources.gpu.gpu1810.read;
    for _ in 0..fifo_words {
        read.push_back(0xFFFF_FFFF);
    }
}

pub unsafe fn command_e1(state: &State, command: u32) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_TEXPAGEX, Bitfield::new(0, 4).extract_from(command));
    stat.write_bitfield(STAT_TEXPAGEY, Bitfield::new(4, 1).extract_from(command));
    stat.write_bitfield(STAT_TRANSPARENCY, Bitfield::new(5, 2).extract_from(command));
    stat.write_bitfield(STAT_TEXPAGE_COLORS, Bitfield::new(7, 2).extract_from(command));
    stat.write_bitfield(STAT_DITHER, Bitfield::new(9, 1).extract_from(command));
    stat.write_bitfield(STAT_DRAW_DISPLAY, Bitfield::new(10, 1).extract_from(command));
    stat.write_bitfield(STAT_TEXTURE_DISABLE, Bitfield::new(11, 1).extract_from(command));
    resources.gpu.textured_rect_x_flip = Bitfield::new(12, 1).extract_from(command) != 0;
    resources.gpu.textured_rect_y_flip = Bitfield::new(13, 1).extract_from(command) != 0;
}

pub unsafe fn command_e2(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.texture_window_mask_x = Bitfield::new(0, 5).extract_from(command) as usize;
    resources.gpu.texture_window_mask_y = Bitfield::new(5, 5).extract_from(command) as usize;
    resources.gpu.texture_window_offset_x = Bitfield::new(10, 5).extract_from(command) as usize;
    resources.gpu.texture_window_offset_y = Bitfield::new(15, 5).extract_from(command) as usize;
}

pub unsafe fn command_e3(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.drawing_area_x1 = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.drawing_area_y1 = Bitfield::new(10, 9).extract_from(command) as usize;
}

pub unsafe fn command_e4(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.drawing_area_x2 = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.drawing_area_y2 = Bitfield::new(10, 9).extract_from(command) as usize;
}

pub unsafe fn command_e5(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.drawing_offset_x = Bitfield::new(0, 11).extract_from(command) as usize;
    resources.gpu.drawing_offset_y = Bitfield::new(11, 11).extract_from(command) as usize;
}

pub unsafe fn command_e6(state: &State, command: u32) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_DRAW_MASK, Bitfield::new(0, 1).extract_from(command));
    stat.write_bitfield(STAT_DRAW_PIXELS, Bitfield::new(1, 1).extract_from(command));
}
