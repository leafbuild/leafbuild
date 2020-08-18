# The `add_subdirectory()` function

## Parameters
```leafbuild
add_subdirectory(
        subdirectory_path
)
```

## Description
Given a subdirectory with a `build.leaf` file present, it executes
the instructions in it and returns a map with the
[frame-returned values]()<sup>[need docs]</sup>.

## Positional parameters
### subdirectory_path
> **Type**: `string`

#### Description
The path to the subdirectory that contains the `build.leaf` file,
relative to the current directory.

## Examples

With the following structure:
```
.
├── build.leaf
└── subdir1
    ├── build.leaf
    └── subdir2
        └── build.leaf
```

To invoke `subdir1/build.leaf` from the root `build.leaf`:

```leafbuild
// in root build.leaf
add_subdirectory('subdir1')
```

To invoke `subdir1/subdir2/build.leaf` from the root `build.leaf`:
```leafbuild
// in root build.leaf
add_subdirectory('subdir1/subdir2')
```

To invoke `subdir1/subdir2/build.leaf` from `subdir1/build.leaf`:
```leafbuild
// in 'subdir1/build.leaf'
add_subdirectory('subdir2')
```

## Alias
`subdir()`