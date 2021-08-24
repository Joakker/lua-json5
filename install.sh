#!/bin/sh

cargo build --release
mv ./target/release/liblua_json5.so lua/json5.so
