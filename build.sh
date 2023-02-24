#!/usr/bin/env bash
echo "------------- sudo apt stuff"
sudo apt update
sudo apt -y install libpq-dev
sudo apt -y install libgssapi-krb5-2 
sudo apt -y install libgssapi-krb5-2:i386
sudo apt -y install rsyslog-gssapi
sudo apt -y install postgresql-devel
echo "------------- apt-cache search libpq"
apt-cache search libpq
echo "sudo ln -s  /usr/local/pgsql/lib/* /usr/lib/"
sudo ln -s  /usr/local/pgsql/lib/* /usr/lib/
echo "------------- cargo stuff"
cargo install diesel_cli --no-default-features --features postgres --verbose
RUSTFLAGS='-L /usr/local/pgsql/lib' cargo build --release
PATH="$PATH:/root/.cargo/bin"
diesel database setup
