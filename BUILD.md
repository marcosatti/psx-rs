# Building

## Dependencies
- SDL2
- OpenGL (mesa)
- OpenAL (openal-soft)

## General
There is no proper build system in place. Easiest way to get this working is on a Linux environment, just install the dependencies listed and it should just work. Windows works but is a bit tricky - for OpenGL you need mesa sources, and built mesa bins/libs (you can find prebuilt packages). Using the Windows OpenGL implementation (& ICD's) won't work (see remarks).

Eventually OpenGL support will be proper, but for development work now it's easier this way.

See the openal-sys/build.rs and opengl-sys/build.rs for compile options.

## Remarks
The openal-sys and opengl-sys packages are Rust bindings of the respective API's.

opengl-sys is currently set up NOT to dynamically find the needed function calls (ie: does not use glXGetProcAddress, wglGetProcAddress etc), as it assumes mesa is used - specifically the software renderer llvmpipe, which supports OpenGL 3.3 fully (and exports all needed symbols already). If when building it complains it can't find or link to symbols, then you are probably not using mesa.
