import json

include_paths = [
    r'C:\Devel\mesa\include',
]
header_paths = [
    r'C:\Devel\mesa\include\GL\glcorearb.h',
]
library_search_paths = [
    r'C:\Devel\mesa3d-20.0.1-development-pack-msvc\lib\x64\src\gallium\targets\libgl-gdi',
]
library_names = [
    'opengl32',
]
defines = [
    'GL_GLEXT_PROTOTYPES=1',
    'GL_VERSION_4_0=0',
    'GL_VERSION_4_1=0',
    'GL_VERSION_4_2=0',
    'GL_VERSION_4_3=0',
    'GL_VERSION_4_4=0',
    'GL_VERSION_4_5=0',
    'GL_VERSION_4_6=0',
]

print(json.dumps({
    'include_paths': include_paths,
    'header_paths': header_paths,
    'library_search_paths': library_search_paths,
    'library_names': library_names,
    'defines': defines,
}))
