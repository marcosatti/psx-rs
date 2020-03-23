# psx-rs: Playstation emulator written in Rust
[![CircleCI](https://circleci.com/gh/marcosatti/psx-rs.svg?style=svg)](https://circleci.com/gh/marcosatti/psx-rs)

Work in progress!

See BUILD.md for build details.

## Targeting
- OpenGL 3.3 (Video, mesa's llvmpipe software renderer, hardware rendering supported on Linux)
- OpenAL 1.1 (Audio, openal-soft)
- libmirage 3.2 (CDROM)

## Status
- Video starting to work, needs implementing / bug fixing.
- Audio starting to work, needs implementing / bug fixing, but you can make out the BIOS intro reasonably well.
- Input not done at all (Hi-Z always).

## Running
You will need to put the BIOS (scph5501.bin) into the folder ${cwd}/workspace/bios.
Specify a path to a supported disc file by libmirage as the first argument (or disable the code manually for no disc).

![BIOS Intro](/media/2019-03-18.png?raw=true "BIOS Intro")

![Reading CDROM](/media/2020-03-12.png?raw=true "Reading CDROM")
