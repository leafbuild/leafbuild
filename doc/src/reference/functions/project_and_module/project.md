# The `project()` function

## Parameters
```leafbuild
project (
        project_name,
        languages: languages,
        authors: authors,
        [maintainers: maintainers],
        [license: license],
)
```

## Description
Tells the build system that the folder we are in should be treated
as a project, which is basically a module with more metadata as far
as the build system is concerned.

## Returns
This function doesn't return anything and is allowed only as a
standalone function call(Not allowed in expressions).

## Positional parameters

### project_name
> **Type**: `string`

The name of the project.

## Kw parameters

### languages
> **Type**: `string` or array of `string`s, that are valid languages;
> see [this](../../../supported_languages.md) for the list of supported
> languages and how to configure them.

The language or languages required to build this project.

> **NOTE**: Unlike the module, here the list of languages is required

### authors
> **Type**: `author` or array of `author`s; see
> [the author type](../../special_types/author.md)
> on how to build an author object.

The list of authors of the project. This is also mandatory.

### maintainers (optional)

> **Type**: `maintainer` or array of `maintainer`s; see
> [the maintainer type](../../special_types/maintainer.md)
> on how to build a maintainer object.

#### Description
The list of maintainers of the project.

#### Default value
The default value is taken from the parent module/project.

### license (optional)
> **Type**: `string` or array of `string`s

#### Description
The license or licenses of this project.