# Building

## Dependencies
- SDL2
- OpenGL (mesa)
- OpenAL (openal-soft)
- libmirage

## General
There is no proper build system in place. Easiest way to get this working is on a Linux environment, just install the dependencies listed and it should just work. Windows works but is a bit tricky - for OpenGL you need mesa sources, and built mesa bins/libs (you can find prebuilt packages). Using the Windows OpenGL implementation (& ICD's) won't work (see remarks). Eventually OpenGL support will be proper, but for development work now it's easier this way.

## Remarks
opengl-sys is currently set up NOT to dynamically find the needed function calls (ie: does not use glXGetProcAddress, wglGetProcAddress etc), as it assumes mesa is used - specifically the software renderer llvmpipe, which supports OpenGL 3.3 fully (and exports all needed symbols already). If when building it complains it can't find or link to symbols, then you are probably not using mesa.
