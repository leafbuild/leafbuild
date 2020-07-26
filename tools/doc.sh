#!/usr/bin/env bash
if [ $# -lt 1 ] || [ "$1" != "build" ] && [ "$1" != "serve" ] && [ "$1" != "push" ] && [ "$1" != "build_highlighter" ] && [ "$1" != "prepare_commit" ];
then echo "$0: invoke as \"$0 <build|serve|push|build_highlighter|prepare_commit>\"" >&2
     exit 1
fi
case $1 in
"build")
  pushd doc || exit $?
  mdbook build
  echo "Book was generated at $PWD/book"
  popd || exit $?
  ;;
"serve")
  pushd doc || exit $?
  mdbook serve
  popd || exit $?
  ;;
"push")
  pushd doc || exit $?
  mdbook build
  cd book || exit $?
  git init
  git remote add origin https://github.com/dblanovschi/leafbuild.git
  git add -A
  git commit -a -m "Update docs"
  git branch gh-pages
  git push origin gh-pages -f
  popd || exit $?
  ;;
"build_highlighter")
  pushd doc || exit $?
  pushd hl_clone || exit $?
  cd highlight.js || exit $?
  node tools/build.js -n leafbuild rust || exit $?
  popd || exit $?
  cp hl_clone/highlight.js/build/highlight.js theme/highlight.js || exit $?
  popd || exit $?
  ;;
"prepare_commit")
  pushd doc || exit $?
  cp hl_clone/highlight.js/src/languages/leafbuild.js ./leafbuild_highlight.js
  popd || exit $?
  ;;
esac