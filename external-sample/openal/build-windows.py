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

print(json.dumps({
    'include_paths': include_paths,
    'header_paths': header_paths,
    'library_search_paths': library_search_paths,
    'library_names': library_names,
    'defines': defines,
}))
