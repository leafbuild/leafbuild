# Project Model

Let's start with the simplest part of a leaf build system: a module.

## Module

In a nutshell, a module is any folder that has a `build.leaf` file directly below it.

## Project

A project is a module that contains some extra metadata.
See the [kwargs of the `project()` function]() to find out more.

The metadata present in the project should apply to all of its submodules.

# The `build.leaf` file
