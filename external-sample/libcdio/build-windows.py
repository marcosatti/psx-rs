import json

include_paths = [
    r'C:\Development\libcdio_release-2.1.0-1_msvc16\include',
]
header_paths = [
    r'C:\Development\libcdio_release-2.1.0-1_msvc16\include\cdio\cdio.h',
    r'C:\Development\libcdio_release-2.1.0-1_msvc16\include\cdio\util.h',
]
library_search_paths = [
    r'C:\Development\libcdio_release-2.1.0-1_msvc16\lib\x64',
]
library_names = [
    'libcdio',
]
defines = [
]
blacklist_item_regexes = [
]
whitelist_function_regexes = [
    r'cdio\w+',
]
whitelist_type_regexes = [
]
whitelist_variable_regexes = [
    r'cdio_version_string',
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
