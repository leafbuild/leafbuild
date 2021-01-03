# Architecture overview

There are two main types of components in `leafbuild`:
- [The ones that help produce a `LfBuildsys`](#producers)
- [The ones that consume the `LfBuildsys`](#consumers)

# Producers
The components that *help produce* the `LfBuildsys` are called *producers*.
They all work together, and they are:
## `leafbuild-ast`
Holds the ast structures produced by the [`leafbuild-parser`](#leafbuild-parser).
Further described in [`leafbuild-ast`](leafbuild-ast.md).

## `leafbuild-parser`
Holds the logic to parse input `build.leaf` files and produces [`leafbuild-ast`](#leafbuild-ast) structures.
Further described in [`leafbuild-parser`](leafbuild-parser.md)

## `leafbuild-interpreter`
Interprets the [`leafbuild-ast`](#leafbuild-ast) structures produced by [`leafbuild-parser`](#leafbuild-parser)
and outputs the `LfBuildsys`. This is where most of the magic happens.
Further described in [`leafbuild-interpreter`](leafbuild-interpreter.md)

## All the middle layers
Middle layers are quite a complicated thing to explain, so you can find more about them [here](leafbuild-ml.md#what-is-a-middle-layer-in-leafbuild)

# Consumers
The components that *consume* the `LfBuildsys` are called *consumers*.
