import json
import subprocess

include_paths = [
]
header_paths = [
    '/usr/include/GL/glcorearb.h',
]
library_search_paths = [
]
library_names = [
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

process = subprocess.run(
    ['pkgconf', 'gl', '--cflags', '--libs'], 
    check=True, 
    capture_output=True, 
    text=True
)
stdout = process.stdout
stdout = stdout.split()

for token in stdout:
    if token.startswith('-L'):
        library_search_paths.append(token[2:])
    elif token.startswith('-l'):
        library_names.append(token[2:])
    elif token.startswith('-I'):
        include_paths.append(token[2:])

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
