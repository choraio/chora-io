#!/usr/bin/env sh

# abort on errors
set -e

# build
sh ./scripts/build_pro.sh

# change to build directory
cd static

# git init and commit
git init
git add -A
git commit -m 'publish'

# push to gh-pages branch
git push https://github.com/choraio/chora-web master:gh-pages -f

# clean up
rm -rf .git
