#!/usr/bin/env bash
sudo apt update
sudo apt -y install libpq-dev
sudo apt -y install libgssapi-krb5-2 
sudo apt -y install libgssapi-krb5-2:i386
cargo install diesel_cli --no-default-features --features postgres
RUSTFLAGS='-L /usr/local/pgsql/lib' cargo build --release
PATH="$PATH:/root/.cargo/bin"
diesel database setup
