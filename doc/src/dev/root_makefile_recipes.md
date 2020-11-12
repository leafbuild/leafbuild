# Root makefile recipes

They can be invoked via
```sh
cargo make <recipe_name>
# or
makers <recipe_name> 
```

## `format`
Formats all the source files.

## `fmtcheck`
Checks formatting of all the source files and exits with non-zero if they're not formatted properly.

## `clean`
Calls cargo clean, removing all built artifacts.

## `build`
Builds the `leafbuild` executable.

## `lint`
Calls our good friend `clippy` to help improve code.

## `check`
Checks the database, by first formatting, then invoking clippy and testing.
This should usually be invoked before a commit.

## `doc-build`
Invokes `mdbook` to build `doc/book` from `doc/src`.

## `doc-serve`
Invokes `mdbook` to serve the built files on [localhost:3000](http://localhost:3000).

## `doc-nuke`
Cleans up the currently-built book in `doc/book`.

## `doc-push`
Pushes the built doc to [the docs repo](https://github.com/leafbuild/docs).

## `doc-build-highlighter`
Builds the syntax highlighter from the highlight.js repo with `doc/leafbuild_highlight.js`.
More about the highlighter [here](highlighter.md).