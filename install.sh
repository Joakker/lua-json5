#!/usr/bin/env bash

TARGET_DIR=${CARGO_TARGET_DIR:-target}

cargo build --features luajit --release --target-dir "$TARGET_DIR"

case $OSTYPE in
"linux-gnu"*)
	mv "$TARGET_DIR"/release/liblua_json5.so lua/json5.so
	strip lua/json5.so
	;;
"darwin"*)
	# Provide both just in case
	cp "$TARGET_DIR"/release/liblua_json5.dylib lua/json5.dylib
	cp lua/json5.dylib lua/json5.so
	;;
esac
