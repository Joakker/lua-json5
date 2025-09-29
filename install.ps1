$TARGET_DIR = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "target" }

cargo build --features luajit --release --target-dir "$TARGET_DIR"
Move-Item -Path "$TARGET_DIR\release\lua_json5.dll" -Destination lua\json5.dll
