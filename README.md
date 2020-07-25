# psx-rs: Playstation emulator written in Rust
[![CircleCI](https://circleci.com/gh/marcosatti/psx-rs.svg?style=svg)](https://circleci.com/gh/marcosatti/psx-rs)

Work in progress!

See BUILD.md for build details.

## Targeting
- OpenGL 3.3 (Video, Windows / Linux, mesa's llvmpipe software renderer, hardware rendering supported on Linux)
- OpenAL 1.1 (Audio, Windows / Linux, openal-soft)
- libmirage 3.2 (CDROM, Linux)
- libcdio (CDROM, Windows / Linux)

## Status
- Video starting to work, needs implementing / bug fixing.
- Audio starting to work, needs implementing / bug fixing, but you can make out the BIOS intro reasonably well.
- CDROM starting to work, needs implementing (reading sectors is working).
- Input not done at all (Hi-Z always).

## Running
The psx-rs binary is the main entry point.

A config file will need to be created at ${cwd}/workspace/config.toml. Example:
```
audio_backend = 'openal'     # 'openal' / 'none'
cdrom_backend = 'libcdio'    # 'libcdio' / 'libmirage' / 'none'
video_backend = 'opengl'     # 'opengl' / 'none'
worker_threads = 2           # Tune for your own system, it can use any number of threads.
time_delta = 15              # Number of microseconds before a hard synchronize is required.
quit_on_exception = false    # Quit automatically when an state exception occurs.
pause_on_start = true        # Pause upon starting the emulator.
internal_scale_factor = 3    # Internal scaling factor for the GPU; must be an integer.
global_bias = 1.0            # 
r3000_bias = 1.0             #
gpu_bias = 1.0               #
dmac_bias = 1.0              #
spu_bias = 1.0               # Bias' for each controller.
timers_bias = 1.0            #
cdrom_bias = 1.0             #
padmc_bias = 1.0             #
intc_bias = 1.0              #
gpu_crtc_bias = 1.0          #
```

You will need to put the BIOS (scph5501.bin) into the folder ${cwd}/workspace/bios.
Optionally, specify a path to a supported disc file by the CDROM backend as the first argument.

Keymap:
F1 => Pause / resume
F2 => Quit
F3 => Soft reset (R3000 only)
F4 => Hard reset (All state)
F10 => Save state
F11 => Load state

![BIOS Intro](/media/2019-03-18.png?raw=true "BIOS Intro")

![Reading CDROM](/media/2020-03-12.png?raw=true "Reading CDROM")
