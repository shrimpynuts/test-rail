#!/usr/bin/env bash
echo "------------- sudo apt stuff"
sudo apt update
sudo apt-get -y install postgresql postgresql-contrib libpq-dev
echo "------------- apt-cache search libpq"
apt-cache search libpq
echo "sudo ln -s  /usr/local/pgsql/lib/* /usr/lib/"
sudo ln -s  /usr/local/pgsql/lib/* /usr/lib/
echo "------------- cargo stuff"
cargo install diesel_cli --no-default-features --features postgres --verbose
RUSTFLAGS='-L /usr/local/pgsql/lib' cargo build --release
PATH="$PATH:/root/.cargo/bin"
diesel database setup
