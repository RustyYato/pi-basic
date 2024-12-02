build:
    cargo b -r

release name: build
    gh release -d {{name        }} create target/aarch64-unknown-linux-gnu/release/basic