# Data Format & Ordering
Data is passed to and from the backends using PSX conventions (eg: pixel coordinates, top-left origin). 
Consult online docs for more information.

# OpenGL Context

The emulation core assumes the OpenGL context will not change outside of its operation.
If you do make changes, you will need to restore the original context afterwards.
