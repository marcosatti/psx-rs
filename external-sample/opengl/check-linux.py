import subprocess
import json

process = subprocess.run(
    ['pkgconf', 'gl', '--cflags', '--libs'], 
    capture_output=True, 
    check=True, 
)

print(json.dumps({'enable': True}))
