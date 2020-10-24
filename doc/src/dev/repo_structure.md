# Repository Structure

```tree
.
├── doc
├── buildsys-utils
└── src
```

- The `doc` folder contains the documentation.
- The `buildsys-utils` contains a crate mostly about toolchains, with all the flags you can pass to them.
- The `src` folder contains all the source code of the build system, except the parts that directly deal
with the toolchains, handled by the `buildsys-utils`.