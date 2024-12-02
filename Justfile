build:
    CARGO_TARGET='aarch64-unknown-linux-gnu' CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER='aarch64-none-linux-gnu-gcc' cargo b -r

push:
    ./scripts/check-uncommited.sh
    git push

release name: build push
    gh release -d {{name}} create target/aarch64-unknown-linux-gnu/release/basic