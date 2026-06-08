#!/bin/bash
set -e

cd ./app && npm install

cd ../api && cargo build

cargo install cargo-watch --locked