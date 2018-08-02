# containerization

## x86_64

musl static linking + alpine(libsqlite3)

* https://pkgs.alpinelinux.org/package/edge/main/x86/sqlite-dev
* https://hub.docker.com/r/library/alpine/tags/

```sh
docker build --tag x64server:latest --file Dockerfile-x64 .
docker run \
  --rm \
  -v `pwd`:/conf \
  -e DATABASE_URL=/conf/file:test.db \
  -e HOST_URL=0.0.0.0:3000 \
  -p 3000:3000 \
  x64server
```

## armv6

libsqlite3 static linking + raspbian(glibc)

* https://github.com/japaric/cross/pull/158/files
* https://hub.docker.com/r/raspbian/stretch/tags/


```
docker build --tag legokichi/piserver:raspbian .
docker push legokichi/piserver:raspbian
```

```sh
docker pull legokichi/piserver:raspbian
docker run -d \
  --name=piserver \
  --restart=always \
  -v `pwd`:/conf \
  -e DATABASE_URL=/conf/file:test.db \
  -e HOST_URL=0.0.0.0:3000 \
  -p 3000:3000 \
  legokichi/piserver:raspbian
docker logs piserver
docker stop piserver
```

-------------------------------------

# memo

## init

* https://github.com/diesel-rs/diesel/tree/master/diesel_cli

```sh
sudo apt install libsqlite3-dev
cargo install diesel_cli -f --no-default-features --features "postgres sqlite"
cargo install-update -a 
```

## migration

* https://github.com/diesel-rs/diesel/tree/master/examples/sqlite/getting_started_step_3

```sh
diesel migration generate blog
```

```sh
diesel setup
//diesel database reset
diesel migration run
```

`diesel.toml` + `diesel migration run` = `diesel print-schema | tee db/src/schema.rs`

## run as docker

```sh
docker run -ti --rm armhf/alpine bash server
```


## fmt

```sh
rustup component add rustfmt-preview --toolchain nightly
cargo +nightly fmt
```

## clippy

```sh
rustup update nightly
cargo +nightly install --force clippy
cargo +nightly clippy
```

## cross


```sh
docker run -it --rm -v $(pwd):/source dlecan/rust-crosscompiler-arm:stable \
  env CC=arm-linux-gnueabihf-gcc \
      CC_arm_unknown_linux_gnu=arm-linux-gnueabihf-gcc-with-link-search \
      CC_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-6 \
      AR_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-ar-6 \
      cargo build --release

docker run -it --rm -v $(pwd):/source dlecan/rust-crosscompiler-arm:stable \
  env env CC=arm-linux-gnueabihf-gcc \
      CC_arm_unknown_linux_gnu=arm-linux-gnueabihf-gcc-with-link-search \
  cargo build --release
```

see

* https://github.com/dlecan/rust-crosscompiler-arm/blob/master/arm/Dockerfile
* https://github.com/dlecan/rust-crosscompiler-arm/tree/master/arm/include

```sh
docker run -ti debian:jessie bash

rustup toolchain install nightly-arm-unknown-linux-gnueabihf
rustup target add arm-unknown-linux-gnueabihf

sudo dpkg --add-architecture armhf
sudo apt update
sudo apt install libsqlite3-dev:armhf
cargo build --target=arm-unknown-linux-gnueabihf
sudo dpkg --remove-architecture armhf
suod apt purge ".*:armhf"
```

```sh
: https://github.com/japaric/rust-cross/issues/42
: https://github.com/dlecan/rust-crosscompiler-arm/tree/master/arm/include/

docker run -ti debian:jessie bash

cd ~/

apt-get update -y
apt-get install -y binutils curl wget zip git vim build-essential

dpkg --add-architecture armhf
apt update
apt install libsqlite3-dev:armhf

export RASPBERRY_PI_TOOLS_COMMIT_ID=5caa7046982f0539cf5380f94da04b31129ed521
wget https://github.com/raspberrypi/tools/archive/$RASPBERRY_PI_TOOLS_COMMIT_ID.zip -O pi-tools.zip
unzip pi-tools.zip
mv `pwd`/tools-$RASPBERRY_PI_TOOLS_COMMIT_ID pi-tools
export PATH=`pwd`/pi-tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
export PATH=`pwd`/pi-tools/arm-bcm2708/arm-linux-gnueabihf/libexec/gcc/arm-linux-gnueabihf/4.8.3:$PATH

curl https://sh.rustup.rs -sSf | sh -s -- -y --verbose
export PATH=~/.cargo/bin:$PATH
rustup install nightly
rustup toolchain install nightly-arm-unknown-linux-gnueabihf
rustup target add arm-unknown-linux-gnueabihf

git clone https://github.com/legokichi/rust-sandbox.git
cd rust-sandbox/diesel-server/

mkdir -p .cargo
cat <<'EOF' | tee .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc-with-link-search"
EOF

cat <<'EOF' | tee arm-linux-gnueabihf-g++-with-link-search
#!/bin/bash

# /!\ Same config for gcc

arm-linux-gnueabihf-g++ \
        -isystem/usr/include/arm-linux-gnueabihf \
        -isystem/usr/include \
        -L/usr/lib/arm-linux-gnueabihf \
        -L/usr/lib \
        $@
EOF
chmod +x arm-linux-gnueabihf-g++-with-link-search
mv arm-linux-gnueabihf-g++-with-link-search /usr/local/sbin/

cat <<'EOF' | tee arm-linux-gnueabihf-gcc-with-link-search
#!/bin/bash

# /!\ Same config for g++

arm-linux-gnueabihf-gcc \
        -isystem/usr/include/arm-linux-gnueabihf \
        -isystem/usr/include \
        -L/usr/lib/arm-linux-gnueabihf \
        -L/usr/lib \
        $@
EOF
chmod +x arm-linux-gnueabihf-gcc-with-link-search
mv arm-linux-gnueabihf-gcc-with-link-search /usr/local/sbin/

cat <<'EOF' | tee fixQualifiedLibraryPaths.sh
#!/bin/bash
#This script is ugly, feel free to fix it

if [ "$#" -ne 2 ]; then
    echo "usage ./cmd target-rootfs target-toolchain"
    exit -1
fi

#passed args
ROOTFS=$1
TOOLCHAIN=$2

if [ -x $TOOLCHAIN ]; then
    echo "Passed valid toolchain"
    MACHINE=$($TOOLCHAIN -dumpmachine)
    DEB_MULTI_ARCH_MADNESS=$ROOTFS/usr/lib/$MACHINE
fi

CURRENTDIR=$PWD

function adjustSymLinks
{
    echo "Adjusting the symlinks in $1 to be relative"
    cd $1
    find . -maxdepth 1 -type l | while read i;
    do qualifies=$(file $i | sed -e "s/.*\`\(.*\)'/\1/g" | grep ^/lib)
    if [ -n "$qualifies" ]; then
    newPath=$(file $i | sed -e "s/.*\`\(.*\)'/\1/g" | sed -e "s,\`,,g" | sed -e "s,',,g" | sed -e "s,^/lib,$2/lib,g");
    echo $i
    echo $newPath;
    #sudo rm $i;
    rm $i;
    #sudo ln -s $newPath $i;
    ln -s $newPath $i;
    fi
    done
}

adjustSymLinks $ROOTFS/usr/lib "../.."

if [ -n "$DEB_MULTI_ARCH_MADNESS" -a -d "$DEB_MULTI_ARCH_MADNESS" ]; then
    echo "Debian multiarch dir exists, adjusting"
    adjustSymLinks $DEB_MULTI_ARCH_MADNESS "../../.."
fi

cd $CURRENTDIR
EOF
chmod +x fixQualifiedLibraryPaths.sh
mv fixQualifiedLibraryPaths.sh /usr/local/sbin/
fixQualifiedLibraryPaths.sh / arm-linux-gnueabihf-gcc

export CC=arm-linux-gnueabihf-gcc-with-link-search
export CXX=arm-linux-gnueabihf-g++-with-link-search
export OBJCOPY=arm-linux-gnueabihf-objcopy


cargo build --target=arm-unknown-linux-gnueabihf
readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/debug/server
```

```console
# readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/debug/server
Attribute Section: aeabi
File Attributes
  Tag_CPU_name: "7-A"
  Tag_CPU_arch: v7
  Tag_CPU_arch_profile: Application
  Tag_ARM_ISA_use: Yes
  Tag_THUMB_ISA_use: Thumb-2
  Tag_FP_arch: VFPv3-D16
  Tag_ABI_PCS_GOT_use: GOT-indirect
  Tag_ABI_PCS_wchar_t: 4
  Tag_ABI_FP_rounding: Needed
  Tag_ABI_FP_denormal: Needed
  Tag_ABI_FP_exceptions: Needed
  Tag_ABI_FP_number_model: IEEE 754
  Tag_ABI_align_needed: 8-byte
  Tag_ABI_enum_size: int
  Tag_ABI_HardFP_use: SP and DP
  Tag_ABI_VFP_args: VFP registers
  Tag_CPU_unaligned_access: v6
  Tag_ABI_FP_16bit_format: IEEE 754
```

```sh
docker run -ti debian:stretch bash
#!/bin/bash
set -euvx

cd ~/

export CC_DIR=/opt/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin
export REAL_CC=$CC_DIR/arm-linux-gnueabihf-gcc
export CC=arm-linux-gnueabihf-gcc-with-link-search
export CXX=arm-linux-gnueabihf-g++-with-link-search
export PATH=$CC_DIR:$PATH:$HOME/.cargo/bin
export OBJCOPY=$CC_DIR/arm-linux-gnueabihf-objcopy
export PKG_CONFIG_ALLOW_CROSS=1


dpkg --add-architecture armhf

cat <<'EOF'|tee -a /etc/apt/sources.list
deb [arch=armhf] http://mirrordirector.raspbian.org/raspbian/ jessie main contrib non-free rpi
deb http://httpredir.debian.org/debian jessie main
deb http://httpredir.debian.org/debian jessie-updates main
deb http://security.debian.org jessie/updates main
EOF
apt-key adv --recv-keys --keyserver keys.gnupg.net 9165938D90FDDD2E 
apt-get update -y
apt-get install -y --no-install-recommends \
    build-essential binutils \
    ca-certificates pkg-config \
    file vim \
    curl wget zip git
apt-get install -y --no-install-recommends \
    libssl-dev libssl-dev:armhf
apt-get install -y --no-install-recommends \
    libsqlite3-dev:armhf

apt-get install -y --no-install-recommends \
    unzip
wget https://github.com/raspberrypi/tools/archive/master.zip -O pi-tools.zip
unzip -z pi-tools.zip
mv `pwd`/tools-master pi-tools


curl -sSL https://github.com/raspberrypi/tools/archive/master.tar.gz \
  | tar -zxC /opt tools-master/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64 --strip=2


cat <<'EOF' | tee arm-linux-gnueabihf-g++-with-link-search
#!/bin/bash

# /!\ Same config for gcc

arm-linux-gnueabihf-g++ \
        -isystem/usr/include/arm-linux-gnueabihf \
        -isystem/usr/include \
        -L/usr/lib/arm-linux-gnueabihf \
        -L/usr/lib \
        $@
EOF
chmod +x arm-linux-gnueabihf-g++-with-link-search
mv arm-linux-gnueabihf-g++-with-link-search /usr/local/sbin/

cat <<'EOF' | tee arm-linux-gnueabihf-gcc-with-link-search
#!/bin/bash

# /!\ Same config for g++

arm-linux-gnueabihf-gcc \
        -isystem/usr/include/arm-linux-gnueabihf \
        -isystem/usr/include \
        -L/usr/lib/arm-linux-gnueabihf \
        -L/usr/lib \
        $@
EOF
chmod +x arm-linux-gnueabihf-gcc-with-link-search
mv arm-linux-gnueabihf-gcc-with-link-search /usr/local/sbin/

cat <<'EOF' | tee fixQualifiedLibraryPaths.sh
#!/bin/bash
#This script is ugly, feel free to fix it

if [ "$#" -ne 2 ]; then
    echo "usage ./cmd target-rootfs target-toolchain"
    exit -1
fi

#passed args
ROOTFS=$1
TOOLCHAIN=$2

if [ -x $TOOLCHAIN ]; then
    echo "Passed valid toolchain"
    MACHINE=$($TOOLCHAIN -dumpmachine)
    DEB_MULTI_ARCH_MADNESS=$ROOTFS/usr/lib/$MACHINE
fi

CURRENTDIR=$PWD

function adjustSymLinks
{
    echo "Adjusting the symlinks in $1 to be relative"
    cd $1
    find . -maxdepth 1 -type l | while read i;
    do qualifies=$(file $i | sed -e "s/.*\`\(.*\)'/\1/g" | grep ^/lib)
    if [ -n "$qualifies" ]; then
    newPath=$(file $i | sed -e "s/.*\`\(.*\)'/\1/g" | sed -e "s,\`,,g" | sed -e "s,',,g" | sed -e "s,^/lib,$2/lib,g");
    echo $i
    echo $newPath;
    #sudo rm $i;
    rm $i;
    #sudo ln -s $newPath $i;
    ln -s $newPath $i;
    fi
    done
}

adjustSymLinks $ROOTFS/usr/lib "../.."

if [ -n "$DEB_MULTI_ARCH_MADNESS" -a -d "$DEB_MULTI_ARCH_MADNESS" ]; then
    echo "Debian multiarch dir exists, adjusting"
    adjustSymLinks $DEB_MULTI_ARCH_MADNESS "../../.."
fi

cd $CURRENTDIR
EOF
chmod +x fixQualifiedLibraryPaths.sh
mv fixQualifiedLibraryPaths.sh /usr/local/sbin/
fixQualifiedLibraryPaths.sh / $REAL_CC 


curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
PATH=$PATH:$HOME/.cargo/bin
rustup target add arm-unknown-linux-gnueabihf


git clone https://github.com/legokichi/rust-sandbox.git
cd rust-sandbox/diesel-server/

mkdir -p .cargo
cat <<'EOF' | tee .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc-with-link-search"
EOF

env \
      CC=arm-linux-gnueabihf-gcc-6 \
      CC_arm_unknown_linux_gnu=arm-linux-gnueabihf-gcc-with-link-search \
      CC_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-6 \
      AR_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-ar-6 \
cargo build --target=arm-unknown-linux-gnueabihf


docker run -it --rm \
  -v $(pwd):/source \
  dlecan/rust-crosscompiler-arm:stable \
    /bin/bash
apt-get update -y
apt-get install -y libsqlite3-dev:armhf
rustup target add arm-unknown-linux-gnueabihf
/usr/lib/arm-linux-gnueabihf/
cargo build
readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/debug/server


scp apricot:/home/legokichi/Github/rust-sandbox/diesel-server/target/arm-unknown-linux-gnueabihf/debug/server ./

```

systemd systemctl setting

```sh
sudo touch /etc/systemd/system/torhttp.service
cat<<'EOF'>>/etc/systemd/system/torhttp.service
[Unit]
Description=Tor Hidden HTTP Service
After=network.target

[Service]
Type=simple
WorkingDirectory=/home/pi/github/torhttp/
ExecStart=/home/pi/github/torhttp/server
Restart=always

[Install]
WantedBy=multi-user.target

EOF
sudo systemctl daemon-reload
sudo systemctl restart torhttp
```

```sh
docker run -it --rm \
  -v $(pwd):/source \
  dlecan/rust-crosscompiler-arm:stable \
    /bin/bash
apt-get update -y
apt-get install -y libsqlite3-dev:armhf
rustup target add arm-unknown-linux-gnueabihf
cargo build --release
readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/debug/server
```

```console
# readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/debug/server
Attribute Section: aeabi
File Attributes
  Tag_CPU_name: "6"
  Tag_CPU_arch: v6
  Tag_ARM_ISA_use: Yes
  Tag_THUMB_ISA_use: Thumb-1
  Tag_FP_arch: VFPv2
  Tag_ABI_PCS_GOT_use: GOT-indirect
  Tag_ABI_PCS_wchar_t: 4
  Tag_ABI_FP_rounding: Needed
  Tag_ABI_FP_denormal: Needed
  Tag_ABI_FP_exceptions: Needed
  Tag_ABI_FP_number_model: IEEE 754
  Tag_ABI_align_needed: 8-byte
  Tag_ABI_enum_size: int
  Tag_ABI_HardFP_use: SP and DP
  Tag_ABI_VFP_args: VFP registers
  Tag_CPU_unaligned_access: v6
  Tag_ABI_FP_16bit_format: IEEE 754
  Tag_DIV_use: Not allowed
```

