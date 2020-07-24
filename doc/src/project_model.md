# Project Model

Let's start with the simplest part of a leaf build system: a module.

## Module

In a nutshell, a module is any folder that has a `build.leaf` file directly below it.

## Project

A project is a module that contains some extra metadata.
See the [kwargs of the `project()` function]() to find out more.

# The `build.leaf` file

A `build.leaf` file should always start with a call to either
[the `project()` function](reference/functions/project_and_module/project.md) or
[the `module()` function](reference/functions/project_and_module/module.md).

 