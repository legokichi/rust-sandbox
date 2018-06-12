


```sh
sudo apt -y install gcc-6-arm-linux-gnueabihf
cat <<'EOF' > .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc-6"
EOF
rustup toolchain install stable-arm-unknown-linux-gnueabihf
rustup target add arm-unknown-linux-gnueabihf
env CC_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-6  \
    AR_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-ar-6
    cargo build -v --target=arm-unknown-linux-gnueabihf --release
scp ./target/arm-unknown-linux-gnueabihf/release/server pi@raspberrypi.local:/home/pi/
```