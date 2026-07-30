#!/bin/sh
set -e
cd /app/apps/server

mkdir -p /app/target /home/appuser/.cargo
chown -R appuser:appuser /app/target /home/appuser/.cargo

export HOME=/home/appuser
export CARGO_HOME=/home/appuser/.cargo

exec gosu appuser:appuser \
	cargo watch -x 'run --bin orcid-acad-mgr-server' -w src -w config -w docs
