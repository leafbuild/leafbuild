## The `gh-pages` Branch and Documentation
The `gh-pages` branch was created from the `doc/book` directory.
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

[master_branch]: https://github.com/leafbuild/leafbuild/tree/master