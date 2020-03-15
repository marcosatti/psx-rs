import subprocess
import json

process = subprocess.run(
    ['pkgconf', 'gl', '--cflags', '--libs'], 
    check=True, 
)

print(json.dumps({'enable': True}))
