import json
import subprocess

include_paths = [
]
header_paths = [
    '/usr/include/libmirage-3.2/mirage/mirage.h',
]
library_search_paths = [
]
library_names = [
]
defines = [
]
blacklist_item_regexes = [
]
whitelist_function_regexes = [
    r'mirage\w+',
    r'g_\w+',
]
whitelist_type_regexes = [
    r'GObject',
    r'GError',
    r'Mirage\w+',
]
whitelist_variable_regexes = [
    r'MIRAGE\w+',
    r'mirage\w+',
]

process = subprocess.run(
    ['pkgconf', 'libmirage', '--cflags', '--libs'], 
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
