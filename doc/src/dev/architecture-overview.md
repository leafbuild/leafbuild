# Architecture overview

There are two main types of components in `leafbuild`:
- [The ones that help produce a `LfBuildsys`](#producers)
- [The ones that consume the `LfBuildsys`](#consumers)

# Producers
The components that *help produce* the `LfBuildsys` are called *producers*.
They all work together, and they are:

## `leafbuild-syntax`
Refer to [`leafbuild-syntax`](leafbuild-syntax.md)

## `leafbuild-interpreter`
Interprets the [`leafbuild-syntax`](#leafbuild-syntax) structures produced by the [parser](leafbuild-syntax.md#parser) and outputs the `LfBuildsys`. This is where most of the magic happens.
Further described in [`leafbuild-interpreter`](leafbuild-interpreter.md)

## All the middle layers
Middle layers are quite a complicated thing to explain, so you can find more about them [here](leafbuild-ml.md#what-is-a-middle-layer-in-leafbuild)

# Consumers
The components that *consume* the `LfBuildsys` are called *consumers*.

They can either dump it into a ninja.build file, makefile, or even a [clang compilation database](https://clang.llvm.org/docs/JSONCompilationDatabase.html).
