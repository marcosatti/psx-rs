import subprocess
import json

process = subprocess.run(
    ['pkgconf', 'openal', '--cflags', '--libs'], 
    check=True, 
)

print(json.dumps({'enable': True}))
