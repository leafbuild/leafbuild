# Terminology
Some of the terminology used.

## Build system boundary
Refers to when a child directory of a module / project is managed by a different build system.

Example:
```text
.
└── outer
    ├── build.leaf
    └── inner
        └── CMakeLists.txt
```
And with `inner` subdir-ed from `outer`
```leafbuild
// outer/build.leaf
project('outer')

subdir('inner')
```
Here `outer` is managed by `leafbuild` while `inner` is managed by `cmake`.
We call `outer` the outer directory and `inner` the inner directory across
the `("outer", "outer/inner")` build system boundary.
