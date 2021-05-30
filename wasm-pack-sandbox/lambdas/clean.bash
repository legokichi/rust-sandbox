#!/bin/bash

set -uvx

for folder in $(find . -maxdepth 1 -type d -not -path "." -not -path ".."); do
  pushd "${folder}";
  npm run clean;
  rm -rf node_modules
  popd;
done

