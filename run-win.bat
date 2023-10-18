@cls
@cargo clean --release --target x86_64-pc-windows-msvc
@cargo build --release --target x86_64-pc-windows-msvc
@cargo run --release --target x86_64-pc-windows-msvc