#!/usr/bin/env bash
echo "------------- sudo apt stuff"
sudo apt update
sudo apt -y install libpq-dev
sudo apt -y install libgssapi-krb5-2 
sudo apt -y install libgssapi-krb5-2:i386
sudo apt -y install rsyslog-gssapi
sudo apt -y install postgresql-devel
echo "------------- cargo stuff"
cargo install diesel_cli --no-default-features --features postgres --verbose
RUSTFLAGS='-L /usr/local/pgsql/lib' cargo build --release
PATH="$PATH:/root/.cargo/bin"
diesel database setup
