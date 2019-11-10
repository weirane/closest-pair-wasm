#!/bin/bash
set -xe

wasm-pack build -- --verbose
cd www
npm install
npm run build

cd dist
git init
git config user.name "Travis CI"
git config user.email "travis@travis-ci.org"
git checkout --orphan=gh-pages
git add --all
git commit --message "Auto deploy from Travis CI build $TRAVIS_BUILD_NUMBER"
git remote add deploy https://$GH_TOKEN@github.com/weirane/closest-pair-wasm.git
git push --force deploy gh-pages
