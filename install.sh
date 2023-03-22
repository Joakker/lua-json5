#!/usr/bin/env bash

cargo build --release

case $OSTYPE in
"linux-gnu"*)
	mv ./target/release/liblua_json5.so lua/json5.so
	strip lua/json5.so
	;;
"darwin"*)
	mv ./target/release/liblua_json5.dylib lua/json5.dylib
	;;
esac
