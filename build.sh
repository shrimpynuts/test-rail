#!/usr/bin/env bash
sudo apt-get update
sudo apt-get -y install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
RUSTFLAGS='-L /usr/local/pgsql/lib' cargo build --release
PATH="$PATH:/root/.cargo/bin"
diesel database setup
