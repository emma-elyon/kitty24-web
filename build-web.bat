@cls
@cargo build --release --target wasm32-unknown-unknown --lib
@wasm-opt -O -o web/kitty24.wasm target/wasm32-unknown-unknown/release/kitty24.wasm
@REM `wasm-opt` further optimizes the binary after rustc has compiled it.
@REM If this is not desired, simply copy the .wasm-file directly:
@REM @cp target/wasm32-unknown-unknown/release/kitty24.wasm web/