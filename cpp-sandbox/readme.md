# hello-rs

rust + c++17 compile example

## vscode

* https://qiita.com/skitoy4321/items/0bf6826f948720bed821
* https://github.com/rust-lang-nursery/rls/blob/master/README.md

```sh
rustup self update
rustup update nightly
rustup component add rls-preview --toolchain nightly
rustup component add rust-analysis --toolchain nightly
rustup component add rust-src --toolchain nightly
cargo install racer
cargo install cargo-update
cargo install cargo-tree
cargo install clippy
```

## build

* cpp demangling - http://www.wagavulin.jp/entry/2017/02/09/215036
* http://mmi.hatenablog.com/entry/2017/02/28/213656
* https://github.com/alexcrichton/cc-rs
* https://qiita.com/kjunichi/items/31aef0cf3f4f7fe6dc32

```sh
sudo add-apt-repository ppa:ubuntu-toolchain-r/test
sudo apt-get update
sudo apt-get install -y g++-7 binutils gdb
```

```sh
env CXX=/usr/bin/g++-7 cargo build
env CXX=/usr/bin/g++-7 cargo run
```

## opencv

```sh
sudo apt-get install \
  libavcodec-dev libavformat-dev libswscale-dev libavresample-dev \
  libtbb2 libtbb-dev libeigen3-dev libopenblas-dev liblapacke-dev \
  gstreamer1.0-tools \
  gstreamer1.0-alsa \
  gstreamer1.0-libav \
  gstreamer1.0-libav-dbg \
  gstreamer1.0-clutter \
  gstreamer1.0-nice \
  gstreamer1.0-pulseaudio \
  gstreamer1.0-crystalhd \
  gstreamer1.0-fluendo-mp3 \
  gstreamer1.0-plugins-base \
  gstreamer1.0-plugins-base-apps \
  gstreamer1.0-plugins-base-dbg \
  gstreamer1.0-plugins-good \
  gstreamer1.0-plugins-good-dbg \
  gstreamer1.0-plugins-ugly \
  gstreamer1.0-plugins-good \
  gstreamer1.0-plugins-bad \
  gstreamer1.0-plugins-bad-dbg \
  gstreamer1.0-plugins-bad-faad \
  gstreamer1.0-plugins-bad-videoparsers \
  gstreamer1.0-plugins-ugly \
  gstreamer1.0-plugins-ugly-dbg \
  gstreamer1.0-vaapi \
  libgstreamer1.0-dev \
  libgstreamer1.0-0-dbg \
  libgstreamer-plugins-base1.0-dev \
  libgstreamer-plugins-good1.0-dev \
  libgstreamer-plugins-bad1.0-dev
```

```sh
wget https://github.com/opencv/opencv/archive/2.4.11.zip
unzip 2.4.11.zip
cd opencv-2.4.11
mkdir -p build
cd build
cmake \
  -DCMAKE_BUILD_TYPE=RELEASE \
  -DBUILD_DOCS=OFF \
  -DBUILD_EXAMPLES=OFF \
  -DBUILD_TESTS=OFF \
  -DBUILD_PERF_TESTS=OFF \
  -DBUILD_WITH_DEBUG_INFO=OFF \
  -DBUILD_SHARED_LIBS=ON \
  -DWITH_OPENCL=ON \
  -DWITH_OPENGL=ON \
  -DWITH_CUDA=OFF \
  -DWITH_TBB=ON \
  ../
make -j
sudo make install
```

```sh
wget https://github.com/opencv/opencv/archive/3.1.0.zip
unzip 3.1.0.zip
cd opencv-3.1.0
mkdir -p build
cd build
cmake \
  -DCMAKE_BUILD_TYPE=RELEASE \
  -DBUILD_DOCS=OFF \
  -DBUILD_EXAMPLES=OFF \
  -DBUILD_TESTS=OFF \
  -DBUILD_PERF_TESTS=OFF \
  -DBUILD_WITH_DEBUG_INFO=OFF \
  -DBUILD_SHARED_LIBS=ON \
  -DWITH_OPENCL=ON \
  -DWITH_OPENGL=ON \
  -DWITH_CUDA=OFF \
  -DWITH_TBB=ON \
  ../
make -j
sudo make install

```

## zmq

```sh
sudo apt-get install libzmq3-dev
```