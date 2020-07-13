use crate::Core;
use crate::backends::video::VideoBackend;

const SAVE_STATE_DEFAULT_NAME: &'static str = "save_state_default.bin.zst";

pub fn save_state(core: &Core, name: Option<&str>) -> Result<(), String> {
    log::warn!("GPU framebuffer serialization incomplete, use with caution");

    let name = name.unwrap_or(SAVE_STATE_DEFAULT_NAME);
    let mut path = core.config.workspace_path.join(r"saves/");
    std::fs::create_dir_all(&path).unwrap();
    path = path.join(name);
    let file = std::fs::File::create(path).map_err(|e| format!("Unable to create save state file: {}", e))?;

    {
        let file = file.try_clone().map_err(|e| format!("Unable to create save state file: {}", e))?;
        let zstd_stream = zstd::Encoder::new(file, 0).map_err(|e| format!("Unable to make zstd stream: {}", e))?;
        let zstd_stream = zstd_stream.auto_finish();
        bincode::serialize_into(zstd_stream, &core.state).map_err(|e| format!("Error occurred serializing machine state: {}", e))?;
    }

    file.sync_all().map_err(|e| format!("Error writing to save file: {}", e))
}

pub fn load_state(core: &mut Core, name: Option<&str>) -> Result<(), String> {
    let name = name.unwrap_or(SAVE_STATE_DEFAULT_NAME);
    let path = core.config.workspace_path.join(r"saves/").join(name);
    let file = std::fs::File::open(path).map_err(|e| format!("Unable to open save state file: {}", e))?;

    let zstd_stream = zstd::Decoder::new(file).map_err(|e| format!("Unable to make zstd stream: {}", e))?;
    let mut state = bincode::deserialize_from(zstd_stream).map_err(|e| format!("Error occurred deserializing machine state: {}", e).to_owned())?;
    std::mem::swap(core.state.as_mut(), &mut state);

    Ok(())
}

fn serialize_gpu_framebuffer(video_backend: &VideoBackend) -> Vec<u8> {
    match video_backend {
        VideoBackend::None => {
            log::warn!("Cannot serialize GPU framebuffer as there is no active backend; returning empty data");
            Vec::new()
        },
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => serialize_gpu_framebuffer_opengl(backend_params),
        _ => unimplemented!(),
    }
}

#[cfg(opengl)]
fn serialize_gpu_framebuffer_opengl(backend_params: &crate::backends::video::opengl::BackendParams) -> Vec<u8> {
    use crate::system::gpu::constants::*;

    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        glFinish();
        glReadBuffer(GL_COLOR_ATTACHMENT0);
        let mut buffer: Vec<u8> = vec![0; VRAM_WIDTH_16B * VRAM_HEIGHT_LINES];
        glReadPixels(0, 0, VRAM_WIDTH_16B as GLint, VRAM_HEIGHT_LINES as GLint, GL_RGBA, GL_UNSIGNED_BYTE, buffer.as_mut_ptr() as *mut std::ffi::c_void);
        buffer
    }
}
