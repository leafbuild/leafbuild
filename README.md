# The Leaf Build System

![Maintained](https://img.shields.io/maintenance/yes/2021.svg?style=for-the-badge)
![pipeline](https://img.shields.io/github/workflow/status/leafbuild/leafbuild/rust?label=Build&style=for-the-badge)
[![Discord server](https://img.shields.io/discord/736172943759114250?color=blue&label=discord&style=for-the-badge)](https://discord.gg/KF45NYK)
[![Gitter](https://img.shields.io/gitter/room/leafbuild/leafbuild?style=for-the-badge)](https://gitter.im/leafbuild/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

### Current version: 0.0.1-dev.1

A work in progress meta build system for C/C++ projects written in Rust.

Primary values:
- Speed. The Compile part in the Edit-Compile-Run cycle should
 take as little as possible.

- Compatibility with popular build systems out there.
 There is a lot of C/C++ code out there, and your code may
 have dependencies that use other build systems. In such
 cases, `leafbuild` should try to make sure they will work together.

- Keep the syntax as simple as *reasonable*, but don't take the simplicity
 part too far(see meson, golang).

## Contributions
See [CONTRIBUTING.md](https://github.com/leafbuild/leafbuild/blob/master/CONTRIBUTING.md)

License: BSD-3-Clause
