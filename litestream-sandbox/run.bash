#!/bin/bash

set -euxo pipefail

rm -f ./hoge.db

export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream restore -if-replica-exists -config ./litestream.yml ./hoge.db
./litestream replicate -exec ./target/release/litestream-sandbox -config ./litestream.yml
