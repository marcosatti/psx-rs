import subprocess
import json

try:
    process = subprocess.run(
        ['pkgconf', 'openal', '--cflags', '--libs'], 
        capture_output=True, 
        check=True, 
    )
    enable = True
except subprocess.CalledProcessError:
    enable = False

print(json.dumps({'enable': enable}))
