# Building

## Dependencies
- Python 3 [build dependency]
- SDL2 [optional (see remarks)]
- OpenGL (mesa) [optional (see remarks)]
- OpenAL (openal-soft) [optional]
- libmirage [optional]

## General
Easiest way to get this working is on a Linux environment, just install the dependencies listed and it should just work. It uses pkgconf to find the dependencies.
Windows works but is a bit tricky (manual setup)... good luck.

The *-sys projects expects Python to be available through the shell and invokes the respective external build script (for example, ./external/opengl/build.py) to find the needed headers, libraries, etc. Check the external-sample folder and copy over the relevant structure required. 

There are 2 types of build scripts that are looked for within each external library directory:
- check.py: to enable the feature.
- build.py: to gather build information.

## Remarks
opengl-sys is currently set up NOT to dynamically find the needed function calls (ie: does not use glXGetProcAddress, wglGetProcAddress etc), as it assumes mesa is used - specifically the software renderer llvmpipe, which supports OpenGL 3.3 fully (and exports all needed symbols already). If when building it complains it can't find or link to symbols, then you are probably not using mesa.

opengl-sys and SDL2 both are required to do anything useful with the emulator. If run without them, the emulator will shortly panic upon start, as it needs to render to a screen buffer. It's only really useful to exclude them if testing the build process (ie: just building libpsx-rs).
