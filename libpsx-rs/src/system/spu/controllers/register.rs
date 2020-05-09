use crate::system::types::State;
use crate::system::spu::types::*;
use crate::system::spu::constants::*;
use crate::system::spu::controllers::dac::voice::*;
use crate::types::bitfield::Bitfield;
use crate::{utilities::bool_to_flag, types::memory::*};

pub fn handle_control(state: &State, controller_state: &mut ControllerState) {
    let mut write_fn = |value| {
        controller_state.enabled = CONTROL_ENABLE.extract_from(value) > 0;
        
        controller_state.muted = CONTROL_UNMUTE.extract_from(value) == 0;
        
        let transfer_mode = match CONTROL_TRANSFER_MODE.extract_from(value) {
            0 => TransferMode::Stop,
            1 => TransferMode::ManualWrite,
            2 => TransferMode::DmaWrite,
            3 => TransferMode::DmaRead,
            _ => unreachable!("Invalid transfer mode"),
        };
        
        controller_state.transfer_state.current_transfer_mode = transfer_mode;
        state.spu.stat.write_bitfield(STAT_DATA_BUSY_FLAG, bool_to_flag(transfer_mode != TransferMode::Stop) as u16);

        let header = Bitfield::new(0, 5);
        state.spu.stat.write_bitfield(header, header.extract_from(value));

        log::debug!("Acknowledged CTRL write");
    };

    state.spu.control.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => value,
            LatchKind::Write => { write_fn(value); value },
        }
    });
}

pub fn handle_data_transfer_address(state: &State, controller_state: &mut ControllerState) {
    let mut write_fn = |value| {
        controller_state.transfer_state.current_transfer_address = value as usize * 8;
        log::debug!("Acknowledged DTA write");
    };

    state.spu.data_transfer_address.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => value,
            LatchKind::Write => { write_fn(value); value },
        }
    });
}

pub fn handle_key_on(state: &State, controller_state: &mut ControllerState) {
    // Initializes voice state (starts ADSR envelope).
    // Copies start address to current address (internal).
    // Copies start Address to repeat address register.
    // Clears the corresponding voice status bit.

    let mut write_fn = |value| {
        for voice_id in 0..24 {
            let voice_bitfield = Bitfield::new(voice_id, 1);

            if voice_bitfield.extract_from(value) > 0 {
                let mut voice_state = VoiceState::new();
                let start_address = get_saddr(state, voice_id).read_u16() as usize * 8;
                voice_state.current_address = start_address;
                *get_voice_state(controller_state, voice_id) = voice_state;
                state.spu.voice_channel_status.write_bitfield(voice_bitfield, 0);
                log::debug!("Acknowledged KEYON write");
            }
        }
    };

    state.spu.voice_key_on.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => {
                log::warn!("Read from KeyOn occurred, but there is no handler");
                value
            },
            LatchKind::Write => { write_fn(value); value },
        }
    });
}

pub fn handle_key_off(state: &State, controller_state: &mut ControllerState) {
    // Changes voice ADSR phase to the release state.

    let mut write_fn = |value| {
        for voice_id in 0..24 {
            let voice_bitfield = Bitfield::new(voice_id, 1);

            if voice_bitfield.extract_from(value) > 0 {
                get_voice_state(controller_state, voice_id).adsr_state.phase = AdsrPhase::Release;
            }

            log::debug!("Acknowledged KEYOFF write");
        }
    };

    state.spu.voice_key_off.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => {
                log::warn!("Read from KeyOff occurred, but there is no handler");
                value
            },
            LatchKind::Write => { write_fn(value); value },
        }
    });
}
