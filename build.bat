rmdir /s /q "./pkg"
rmdir /s /q "./www/pkg"
wasm-pack build --target web --release
move ./pkg "./www/pkg"