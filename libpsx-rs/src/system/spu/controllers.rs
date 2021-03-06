pub(crate) mod backend_dispatch;
pub(crate) mod dac;
pub(crate) mod register;
pub(crate) mod transfer;

use crate::{
    audio::AudioBackend,
    system::{
        spu::{
            constants::*,
            controllers::{
                dac::*,
                register::*,
                transfer::*,
            },
            types::ControllerState,
        },
        types::{
            ControllerContext,
            ControllerResult,
            Event,
            State,
        },
    },
};

pub(crate) fn run(context: &ControllerContext, event: Event) -> ControllerResult<()> {
    match event {
        Event::Time(time) => run_time(context.state, context.audio_backend, time),
    }
}

fn run_time(state: &State, audio_backend: &AudioBackend, duration: f32) -> ControllerResult<()> {
    let controller_state = &mut state.spu.controller_state.lock();
    controller_state.clock += duration;
    controller_state.dac_state.clock += duration;

    if state.spu.voice_channel_fm.read_u32() > 0 {
        return Err(format!("Pitch modulation not implemented"));
    }

    if state.spu.voice_channel_noise.read_u32() > 0 {
        return Err(format!("Noise generation not implemented"));
    }

    loop {
        let mut handled = false;

        if controller_state.clock > CLOCK_SPEED_PERIOD {
            handle_tick(state, controller_state)?;
            controller_state.clock -= CLOCK_SPEED_PERIOD;
            handled = true;
        }

        if controller_state.dac_state.clock > SAMPLE_RATE_PERIOD {
            handle_dac_tick(state, audio_backend, controller_state)?;
            controller_state.dac_state.clock -= SAMPLE_RATE_PERIOD;
            handled = true;
        }

        if !handled {
            break;
        }
    }

    Ok(())
}

fn handle_tick(state: &State, controller_state: &mut ControllerState) -> ControllerResult<()> {
    handle_control(state, controller_state)?;
    handle_transfer(state, controller_state)?;
    handle_data_transfer_address(state, controller_state)?;
    handle_key_on(state, controller_state)?;
    handle_key_off(state, controller_state)?;

    Ok(())
}

fn handle_dac_tick(state: &State, audio_backend: &AudioBackend, controller_state: &mut ControllerState) -> ControllerResult<()> {
    for voice_id in 0..VOICES_COUNT {
        handle_dac(state, controller_state, audio_backend, voice_id)?;
    }

    Ok(())
}
