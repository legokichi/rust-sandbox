#!/bin/bash

set -euxo pipefail

gcloud storage rm --recursive gs://duxca-litestream-sandbox/hoge.db || true

sqlx database reset -y

export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream replicate -config litestream.yml -exec "sleep 5"

