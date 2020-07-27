# About `libleafcore`, a useful library in creating C/C++ meta build systems 

Contains the code that generates Ninja files and Makefiles,
deals with the specifics of C/C++ compilers, and has the `Stack<T>`
data structure for dealing with stacks.

## The generators
The module containing all the code related to the generators is [`src/generators`][src_generators_link].

The leaf build system supports `make` and `ninja`, each with it's own directory
and submodule inside the `generators` module.

## The compilers
The module containing all the code related to the compilers is [`src/compilers`][src_compilers_link].

Supported C compilers:
- gcc
- clang

Supported C++ compilers:
- gcc
- clang

[src_generators_link]: https://github.com/leafbuild/leafbuild/tree/master/libleafcore/src/generators
[src_compilers_link]: https://github.com/leafbuild/leafbuild/tree/master/libleafcore/src/compilers