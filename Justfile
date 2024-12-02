set dotenv-load

build:
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER='aarch64-none-linux-gnu-gcc' cargo b -r --target aarch64-unknown-linux-gnu

send-unchecked: build
    scp target/aarch64-unknown-linux-gnu/release/basic $IP:/home/yato/pi-basic/basic

send: build send-unchecked