import json

include_paths = [
    r'C:\Devel\openal-soft-1.20.1-bin\include',
]
header_paths = [
    r'C:\Devel\openal-soft-1.20.1-bin\include\AL\al.h',
    r'C:\Devel\openal-soft-1.20.1-bin\include\AL\alc.h',
]
library_search_paths = [
    r'C:\Devel\openal-soft-1.20.1-bin\libs\Win64',
]
library_names = [
    'OpenAL32',
]
defines = [
]
blacklist_item_regexes = [
]
whitelist_function_regexes = [
    r'al\w+',
]
whitelist_type_regexes = [
    r'AL\w+',
]
whitelist_variable_regexes = [
    r'AL\w+'
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
