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
blocklist_item_regexes = [
]
allowlist_function_regexes = [
    r'cdio\w+',
]
allowlist_type_regexes = [
]
allowlist_variable_regexes = [
    r'cdio_version_string',
]

print(json.dumps({
    'include_paths': include_paths,
    'header_paths': header_paths,
    'library_search_paths': library_search_paths,
    'library_names': library_names,
    'defines': defines,
    'blocklist_item_regexes': blocklist_item_regexes,
    'allowlist_function_regexes': allowlist_function_regexes,
    'allowlist_type_regexes': allowlist_type_regexes,
    'allowlist_variable_regexes': allowlist_variable_regexes,
}))
