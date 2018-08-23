
```sh
docker run -d \
  --rm \
  -e RABBITMQ_ERLANG_COOKIE="SWQOKODSQALRPCLNMEQG" \
  -e RABBITMQ_DEFAULT_USER="rabbitmq" \
  -e RABBITMQ_DEFAULT_PASS="rabbitmq" \
  -e RABBITMQ_DEFAULT_VHOST="/" \
  --net=host \
  rabbitmq:3.7-management


env RUST_LOG=lapin_stream=trace,lapin_futures=trace,tokio=trace cargo watch -x run
```