# Building

## Dependencies
- Python 3 [build dependency]
- SDL2 [optional (see remarks)]
- OpenGL [optional (see remarks)]
- OpenAL (openal-soft) [optional]
- libmirage [optional]
- libcdio [optional]

## General
The relevant *-sys projects expects Python to be available through the shell and invokes the respective external build script (for example, ./external/openal/build.py) to find the needed headers, libraries, etc. The build process looks for these scripts in a hardcoded 'external/{library name}' folder.

There are 2 types of build scripts that are looked for within each external library directory:
- check.py: to enable the feature.
- build.py: to gather build information.

Easiest way to get this working is on a Linux environment, just install the dependencies listed and it should just work. 
Check the external-sample folder and copy over the relevant structure required. The Linux example build scripts use pkgconf to find the dependencies.

## Remarks
SDL2 strictly speaking is not required to use libpsx-rs, but the executable psx-rs does expect it to be available. You can omit this dependency if you build your own wrapper. 

For testing the core library, psx-rs-cli is included. It provides no video, audio, or CDROM functionality, but can be used to build an executable without any external dependencies.

opengl-sys is always enabled as OpenGL is handled by SDL2 when initialized (at runtime). If OpenGL is not available at runtime, then the emulator will panic.
