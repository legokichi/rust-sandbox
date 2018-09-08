```bash
go get -u github.com/FiloSottile/mkcert
$(go env GOPATH)/bin/mkcert -install
$(go env GOPATH)/bin/mkcert localhost
docker run --net host --rm -d redis
export GITHUB_CLIENT_ID=***
export GITHUB_CLIENT_SECRET=***
env RUST_LOG=actix-redis-cookie=trace,actix_web=trace,actix_redis=trace \
    cargo watch -x run
```