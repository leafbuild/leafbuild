#!/usr/bin/env bash
if [ $# -lt 1 ] || [ "$1" != "build" ] && [ "$1" != "serve" ] && [ "$1" != "push" ] && [ "$1" != "build_highlighter" ] && [ "$1" != "nuke" ];
then echo "$0: invoke as \"$0 <build|serve|push|build_highlighter|nuke>\"" >&2
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

  mkdir -p .public
  mv ./* .nojekyll .public
  mv .public public

  cp ../pages.gitlab-ci.yml ./.gitlab-ci.yml

  git init
  git remote add origin git@gitlab.com:leafbuild/docs.git
  git add -A
  git commit -S -a -m "Update docs"
  git push -f origin master

  popd || exit $?
  ;;
"build_highlighter")
  pushd doc || exit $?
  cp ./leafbuild_highlight.js hl_clone/highlight.js/src/languages/leafbuild.js
  pushd hl_clone || exit $?
  cd highlight.js || exit $?
  node tools/build.js -n leafbuild rust || exit $?
  popd || exit $?
  cp hl_clone/highlight.js/build/highlight.js theme/highlight.js || exit $?
  popd || exit $?
  ;;
"nuke")
  rm -rf doc/book
  ;;
esac