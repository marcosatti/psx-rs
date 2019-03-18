# psx-rs: Playstation emulator written in Rust

Work in progress!

See BUILD.md for build details.

## Targeting
- OpenGL 3.3 (mesa's llvmpipe software renderer, hardware rendering supported on Linux)
- OpenAL 1.1 (openal-soft)

## Status
- Video starting to work, mostly just needs to be implemented.
- Audio starting to work (ADPCM decoded ok), but comes out garbled due to missing ADSR and interpolation most likely. You can kind of make out the intro sound at least...
- Input not done at all.

## Running
You will need to put the BIOS (scph5501.bin) into the folder ${cwd}/workspace/bios .

![BIOS Intro](/media/2019-03-18.png?raw=true "BIOS Intro")
