#!/bin/bash
set -euxo pipefail
NAME=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest
docker build . --tag=$NAME
docker push $NAME
gcloud run deploy \
  --image=$NAME \
  --region=asia-northeast1 \
  --allow-unauthenticated litestream-sandbox \
  --execution-environment=gen1 \
  --cpu=1 \
  --memory=128Mi \
  --timeout=3s \
  --concurrency=128 \
  --max-instances=1 \
  --min-instances=0 \
  --no-cpu-boost \
  --cpu-throttling
#  --use-http2 \

