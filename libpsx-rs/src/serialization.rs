use crate::Core;
use crate::system::types::State;
use crate::backends::video::VideoBackend;
use serde::{
    Deserialize,
    Serialize,
};

const SAVE_STATE_DEFAULT_NAME: &'static str = "save_state_default.bin.zst";

#[derive(Serialize, Deserialize)]
struct SaveState {
    state: Box<State>,
    gpu_framebuffer: Vec<u8>,
}

pub fn save_state(core: &Core, name: Option<&str>) -> Result<(), String> {
    log::warn!("Might have bugs with backend serialization, use with caution");

    let name = name.unwrap_or(SAVE_STATE_DEFAULT_NAME);
    let mut path = core.config.workspace_path.join(r"saves/");
    std::fs::create_dir_all(&path).unwrap();
    path = path.join(name);
    let file = std::fs::File::create(path).map_err(|e| format!("Unable to create save state file: {}", e))?;

    {
        let state = SaveState {
            state: core.state.clone(),
            gpu_framebuffer: read_gpu_framebuffer(&core.config.video_backend),
        };

        let file = file.try_clone().map_err(|e| format!("Unable to create save state file: {}", e))?;
        let zstd_stream = zstd::Encoder::new(file, 0).map_err(|e| format!("Unable to make zstd stream: {}", e))?;
        let zstd_stream = zstd_stream.auto_finish();
        bincode::serialize_into(zstd_stream, &state).map_err(|e| format!("Error occurred serializing machine state: {}", e))?;
    }

    file.sync_all().map_err(|e| format!("Error writing to save file: {}", e))
}

pub fn load_state(core: &mut Core, name: Option<&str>) -> Result<(), String> {
    let name = name.unwrap_or(SAVE_STATE_DEFAULT_NAME);
    let path = core.config.workspace_path.join(r"saves/").join(name);
    let file = std::fs::File::open(path).map_err(|e| format!("Unable to open save state file: {}", e))?;

    let zstd_stream = zstd::Decoder::new(file).map_err(|e| format!("Unable to make zstd stream: {}", e))?;
    let mut save_state: SaveState = bincode::deserialize_from(zstd_stream).map_err(|e| format!("Error occurred deserializing machine state: {}", e))?;

    std::mem::swap(&mut core.state, &mut save_state.state);
    write_gpu_framebuffer(&core.config.video_backend, &save_state.gpu_framebuffer);

    Ok(())
}

fn read_gpu_framebuffer(video_backend: &VideoBackend) -> Vec<u8> {
    match video_backend {
        VideoBackend::None => {
            log::warn!("Cannot serialize GPU framebuffer as there is no active backend; returning empty data");
            Vec::new()
        },
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => opengl::read_gpu_framebuffer(backend_params),
        _ => unimplemented!(),
    }
}

fn write_gpu_framebuffer(video_backend: &VideoBackend, data: &[u8]) {
    match video_backend {
        VideoBackend::None => log::warn!("Cannot deserialize GPU framebuffer as there is no active backend"),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => opengl::write_gpu_framebuffer(backend_params, data),
        _ => unimplemented!(),
    }
}

#[cfg(opengl)]
mod opengl {
    pub(crate) fn read_gpu_framebuffer(backend_params: &crate::backends::video::opengl::BackendParams) -> Vec<u8> {
        use opengl_sys::*;
        use crate::system::gpu::constants::*;
    
        let (_context_guard, _context) = backend_params.context.guard();
    
        unsafe {
            glFinish();
            glBindTexture(GL_TEXTURE_2D, crate::backends::video::opengl::rendering::SCENE_TEXTURE);
            let mut buffer: Vec<u8> = vec![0; VRAM_WIDTH_16B * VRAM_HEIGHT_LINES * 4];
            glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_BYTE, buffer.as_mut_ptr() as *mut std::ffi::c_void);
            buffer
        }
    }

    pub(crate) fn write_gpu_framebuffer(backend_params: &crate::backends::video::opengl::BackendParams, data: &[u8]) {
        use opengl_sys::*;
        use crate::system::gpu::constants::*;
    
        let size = VRAM_WIDTH_16B * VRAM_HEIGHT_LINES * 4;
        assert!(data.len() == size);

        let (_context_guard, _context) = backend_params.context.guard();
        
        unsafe {
            glFinish();
            glBindTexture(GL_TEXTURE_2D, crate::backends::video::opengl::rendering::SCENE_TEXTURE);
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, VRAM_WIDTH_16B as GLint, VRAM_HEIGHT_LINES as GLint, 0, GL_RGBA, GL_UNSIGNED_BYTE, data.as_ptr() as *const std::ffi::c_void);
        }
    }
}
