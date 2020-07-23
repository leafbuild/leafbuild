#!/usr/bin/env bash
if [ $# -lt 2 ] || [ "$1" != "build" ] && [ "$1" != "serve" ];
then echo "$0: invoke as \"$0 <build|serve>\"" >&2
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
esac