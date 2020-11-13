import json

include_paths = [
    r'C:\Development\mesa\include',
]
header_paths = [
    r'C:\Development\mesa\include\GL\glcorearb.h',
]
library_search_paths = [
    r'C:\Development\mesa3d-20.2.2-development-pack-msvc\lib\x64\src\gallium\targets\libgl-gdi',
]
library_names = [
    'opengl32',
]
defines = [
    'GL_GLEXT_PROTOTYPES=1',
    'GL_VERSION_4_6=0',
]
blacklist_item_regexes = [
]
whitelist_function_regexes = [
    r'gl\w+',
]
whitelist_type_regexes = [
    r'GL\w+',
]
whitelist_variable_regexes = [
    r'GL\w+'
]

print(json.dumps({
    'include_paths': include_paths,
    'header_paths': header_paths,
    'library_search_paths': library_search_paths,
    'library_names': library_names,
    'defines': defines,
    'blacklist_item_regexes': blacklist_item_regexes,
    'whitelist_function_regexes': whitelist_function_regexes,
    'whitelist_type_regexes': whitelist_type_regexes,
    'whitelist_variable_regexes': whitelist_variable_regexes,
}))
