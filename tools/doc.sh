#!/usr/bin/env bash
if [ $# -lt 1 ] || [ "$1" != "build" ] && [ "$1" != "serve" ] && [ "$1" != "push" ];
then echo "$0: invoke as \"$0 <build|serve|push>\"" >&2
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
esac