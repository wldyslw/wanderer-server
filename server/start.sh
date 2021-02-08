#!/bin/sh

set -e

cd server
cargo run --bin seed # TODO: run only when --build flag passed to docker-compose
cargo run
