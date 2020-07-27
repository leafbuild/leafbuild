# About `libleaf`, the library that manages everything

`libleaf` is the library that manages everything, from parsing
and interpreting `build.leaf` files all the way to setting up
the build directory.

## The `grammar` Module

This is the module responsible for lexing and parsing a `build.leaf`
file and giving out an AST.

The lexer is written manually and can be found [here][lexer_link].

The parser is generated with [LALRPOP](https://github.com/lalrpop/lalrpop),
and can be found [here][lalrpop_parser_link].


## The `handle` Module

This module defines a data structure that will live for as long as a project is
interpreted. It stores all the [environment frames](#the-envframe-structure).

## The `interpreter` Module

This module is responsible for taking an AST as produced by the [grammar module](#the-grammar-module)
and interpreting it.

It defines a few important structures and traits, detailed below.

### The `EnvFrame` Structure

The environment frame structure keeps track of variables, their values, and the libraries,
executables, and other values the frame exposes to the outside.

An environment frame is created and pushed onto the environment stack
for each file the interpreter is called on.

When we are done with a file, we pop the environment frame associated with it off the stack.
It essentially works like a stack frame.

### The `ValueTypeMarker` Trait

Implemented only on types that can be used as values and stored in variables.

Here are the types that implement it:
- `i32`, `i64`, `u32` and `u64`
- `String`

The implementations of it can be found in the `interpreter::types` module.

`Box<T> where T: ValueTypeMarker` also implements the `ValueTypeMarker` trait,
calling the functions on the inner value; see below why.

### The `Value<T>` structure

`T` is anything that implements the `ValueTypeMarker` trait, as long as it is sized.

It is responsible for managing a Value.

`Value<Box<dyn ValueTypeMarker>>` is the way to pass an unknown-type value, and 
that is why `Box<T> where T: ValueTypeMarker` implements the `ValueTypeMarker` trait.

### The `CallExecutor` structure

It is basically a wrapper over a closure responsible for interpreting a function or method call
from the source file; it also contains its name as a field.

The closure signature looks like this:
```rust
Fn(
    &AstFuncCallArgs,
    &mut EnvFrame,
    Option<&Value<Box<dyn ValueTypeMarker>>>,
) -> Value<Box<dyn ValueTypeMarker>>
```

The parameters are:
1. The function arguments' ASTs
2. The frame we are currently in
3. The base value. This is a `Some(&value)` if we are calling this as a method on `Value`
or `None` if we are calling this as a function.

### The `CallExecutorPool` structure

It is a collection of `CallExecutor`s associated with a type or functions that can be
called from the global scope.

[lexer_link]: https://github.com/leafbuild/leafbuild/blob/master/libleaf/src/grammar/lexer.rs
[lalrpop_parser_link]: https://github.com/leafbuild/leafbuild/blob/master/libleaf/src/grammar/leafparser.lalrpop