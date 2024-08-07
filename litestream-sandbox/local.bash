#!/bin/bash
set -euxo pipefail
sqlx database reset -y
cargo run
