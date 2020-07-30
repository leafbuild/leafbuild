# The `executable()` function

## Parameters
```leafbuild
executable (
        executable_name,
        files,
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

## Aliases
`bin()`, `binary()`