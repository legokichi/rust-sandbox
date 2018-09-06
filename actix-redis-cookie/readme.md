```bash
go get -u github.com/FiloSottile/mkcert
$(go env GOPATH)/bin/mkcert -install
$(go env GOPATH)/bin/mkcert localhost
docker run --net host redis  
```