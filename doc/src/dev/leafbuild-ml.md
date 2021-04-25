# `leafbuild-ml`

# What is a middle layer in `leafbuild`?
Most C/C++ build systems are isolated, in the sense that you cannot
use multiple build systems in the same codebase, and when you have
a dependency that you *need* to build which uses another build system
than the one you use, it gets a lot harder to get them to talk to each other.

So a middle layer is a layer that goes between `leafbuild` and *some* other
build systems. Read this document carefully if you want to write your own.

# Conventions
1. Across [build system boundaries][build_system_boundary], the build directory at
least somewhat resembles the directory structure of the source directory.Across
[build system boundaries][build_system_boundary], output directories
of sibling projects & modules are *always* siblings, output directories of
parent / child projects and modules have to at least somewhat resemble
that parent / child relationship. More about this [here](#output-directory-resemblance).
2. Across [build system boundaries][build_system_boundary], the build system that
manages the outer directory is responsible for creating a directory for the build
system that manages the inner directory in it's own designated space.
3. If you recognize an added subdirectory as your own, you should handle it.
*Only and only* otherwise should you invoke the middle layers to handle it.

# Output directory resemblance
TBD

# How `leafbuild-ml` works
It simply exposes a trait: `MiddleLayer` that other crates implement, and perform
a little linker magic to make `leafbuild-ml` aware of those implementations.

The `MiddleLayer` trait is pretty simple:

```rust,norun,noplayground
{{#include ../../../crates/leafbuild-ml/src/ml.rs}}
```

[build_system_boundary]: terminology.md#build-system-boundary
