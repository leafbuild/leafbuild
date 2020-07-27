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
This function returns a `executable` object. The type is described [here](../../special_types/executable.md).

## Positional parameters

### library_name
> **Type**: `string`

#### Description

The name of the executable.
Should not contain `/`s.

## Aliases
`bin()`, `binary()`