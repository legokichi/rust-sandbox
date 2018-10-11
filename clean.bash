#!/bin/bash

set -uvx

for folder in $(find . -maxdepth 1 -type d); do
  pushd "${folder}";
  cargo clean;
  popd;
done

