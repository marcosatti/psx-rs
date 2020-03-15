import subprocess
import json

process = subprocess.run(
    ['pkgconf', 'libmirage', '--cflags', '--libs'], 
    check=True, 
)

print(json.dumps({'enable': True}))
