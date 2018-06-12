# my rust dev env

## build

```sh
sudo docker build \
  --tag $(whoami)/rust-docker \
  --build-arg user_id=$(id -u) \
  --build-arg group_id=$(id -g) \
  .
```

## run

```sh
sudo docker run \
  --rm -ti \
  --name $(whoami)-rust-docker \
  -v=$(pwd):/source \
  --workdir=/source \
  --net=host \
  $(whoami)/rust-docker zsh
```

## attach

```sh
sudo docker attach $(whoami)-rust-docker
```

