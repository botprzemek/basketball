#!/bin/bash
set -e

cargo install cargo-watch --locked

cd ./app && npm install || true

cd ../api && cargo build || true