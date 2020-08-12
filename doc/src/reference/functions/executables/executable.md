# The `executable()` function

## Parameters
```leafbuild
executable (
        executable_name,
        files,
        [include_dirs: include_dirs],
)
```

## Description
Creates a library in the current module and returns it.

## Returns
This function returns an `executable` object. The type is described [here](../../special_types/executable.md).

## Positional parameters

### executable_name
> **Type**: `string`

#### Description

The name of the executable.
Should not contain `/`s.

### files
> **Type**: `string` or array of `string`s.

#### Description
The list of source files to build the executable with.

## Kw parameters
### include_dirs
> **Type**: `string` or array of `string`s.

#### Description
The list of include directories all objects that make this executable should use during compilation.

## Aliases
`bin()`, `binary()`