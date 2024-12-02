build:
    cargo b -r

push:
    ./scripts/check-uncommited.sh
    git push

release name: build push
    gh release -d {{name}} create target/aarch64-unknown-linux-gnu/release/basic