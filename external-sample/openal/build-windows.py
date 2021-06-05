import json

include_paths = [
    r'C:\Development\openal-soft-1.20.1-bin\include',
]
header_paths = [
    r'C:\Development\openal-soft-1.20.1-bin\include\AL\al.h',
    r'C:\Development\openal-soft-1.20.1-bin\include\AL\alc.h',
]
library_search_paths = [
    r'C:\Development\openal-soft-1.20.1-bin\libs\Win64',
]
library_names = [
    'OpenAL32',
]
defines = [
]
blocklist_item_regexes = [
]
allowlist_function_regexes = [
    r'al\w+',
]
allowlist_type_regexes = [
    r'AL\w+',
]
allowlist_variable_regexes = [
    r'AL\w+'
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
