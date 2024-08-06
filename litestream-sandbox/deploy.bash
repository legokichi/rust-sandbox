#!/bin/bash
set -euxo pipefail
NAME=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest
docker build . --tag=$NAME
docker push $NAME
gcloud run deploy --image=$NAME --region=asia-northeast1 --allow-unauthenticated litestream-sandbox
 
