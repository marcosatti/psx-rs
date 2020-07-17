# Data Format & Ordering
Data is passed to and from the backends using OpenGL conventions. Consult online docs for more information.

In practice, this means using normalized coordinates, texcoords, etc.

# OpenGL Context

The emulation core assumes the OpenGL context will not change outside of its operation.
If you do make changes, you will need to restore the original context afterwards.
