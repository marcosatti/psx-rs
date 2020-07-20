# Building

## Dependencies
- Python 3 [build dependency]
- SDL2 [optional (see remarks)]
- OpenGL (mesa) [optional (see remarks)]
- OpenAL (openal-soft) [optional]
- libmirage [optional]
- libcdio [optional]

## General
The *-sys projects expects Python to be available through the shell and invokes the respective external build script (for example, ./external/opengl/build.py) to find the needed headers, libraries, etc. The build process looks for these scripts in a hardcoded 'external/{library name}' folder.

There are 2 types of build scripts that are looked for within each external library directory:
- check.py: to enable the feature.
- build.py: to gather build information.

Easiest way to get this working is on a Linux environment, just install the dependencies listed and it should just work. 
Check the external-sample folder and copy over the relevant structure required. The Linux example build scripts use pkgconf to find the dependencies.

## Remarks
SDL2 strictly speaking is not required to use libpsx-rs, but the executable psx-rs does expect it to be available. You can omit this dependency if you build your own wrapper. 

opengl-sys is required to do anything useful with the emulator. If run without, the emulator will shortly panic upon start, as it needs to render to a screen buffer.

opengl-sys is currently set up NOT to dynamically find the needed function calls (ie: does not use glXGetProcAddress, wglGetProcAddress etc), as it assumes mesa is used - specifically the software renderer llvmpipe, which supports OpenGL 3.3 fully (and exports all needed symbols already). If when building it complains it can't find or link to symbols, then you are probably not using mesa.

For testing the core library, psx-rs-cli is included. It provides no video, audio, or CDROM functionality, but can be used to build an executable without any external dependencies.
