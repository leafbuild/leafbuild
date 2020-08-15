# The `library()` function

## Parameters
```leafbuild
library (
        library_name,
        type,
        files,
        [internal_includes: internal_includes],
        [external_includes: external_includes],
        [language: language],
)
```

## Description
Creates a library in the current module and returns it.

## Returns
This function returns a `library` object. The type is described [here](../../special_types/library.md).

## Positional parameters

### library_name
> **Type**: `string`

#### Description

The name of the library.
Should not contain `/`s.

### type
> **Type**: `libtype`

#### Description

The type of the library.

`static` will create a static library (`libx.a` on linux or `x.dll` on windows).
`shared` will create a dynamic library (`libx.so` on linux or `x.dll` on windows).

The values come from [the prelude](../../prelude.md), so you can just use them like:

```leafbuild
library(
    /* name here */,
    static,
    /* files */,
    /* whatever else you may need */
)
```

### files
> **Type**: `string` or array of `string`s.

#### Description
The list of source files to build the library with.

## KW parameters
### external_includes
> **Type**: `string` or array of `string`s.

#### Description
The includes used by everything that needs the library.

### internal_includes
> **Type**: `string` or array of `string`s.

#### Description
Aside the `external_includes`, you can also have some includes only used when compiling
the sources of the library.

## Aliases
`lib()`