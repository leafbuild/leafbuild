## The `docs` Repo and Documentation
The `docs` repo was created from the `doc/book` directory.
The `doc/book` directory is generated from `doc/src` and `doc/book.toml` by [mdbook](https://github.com/rust-lang/mdBook).

If you want to see the changes after you made them, make sure you have mdbook installed. 

Running `tools/doc.sh build` from the root of the repo will automatically
generate `doc/book` with all of its contents.

If you run `tools/doc.sh serve` from the root of the repo, you will have a
local instance of the doc site at [http://localhost:3000](http://localhost:3000).

After you are happy with the changes, submit a PR on
[the `master` branch][master_branch],
and mention you changed the documentation, so the site can be rebuilt.

Please use reference-style links for all links to the main repo.

You can use `rust` and `leafbuild` for syntax highlighting.
Also the `leafbuild` language declaration for `highlight.js` can be found [here][leafbuild_highlight]. 

[master_branch]: https://gitlab.com/leafbuild/leafbuild/tree/master
[leafbuild_highlight]: https://gitlab.com/leafbuild/leafbuild/blob/master/doc/leafbuild_highlight.js