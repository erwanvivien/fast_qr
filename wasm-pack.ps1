$CARGO_MODE="--release"
$TARGET_PATH="release"
$BUILD_STD_FEATURES="panic_immediate_abort"

Write-Host "Building with cargo mode: ${CARGO_MODE}"

$OUTPUT_DIR="pkg"

cargo +nightly build ${CARGO_MODE} `
    --target wasm32-unknown-unknown `
    -Z "build-std=std,panic_abort" `
    -Z "build-std-features=${BUILD_STD_FEATURES}" `
    --features svg,wasm-bindgen

& wasm-bindgen --out-dir "${OUTPUT_DIR}" `
    --web target/wasm32-unknown-unknown/${TARGET_PATH}/fast_qr.wasm

& wasm-opt `
  -O2 `
  "${OUTPUT_DIR}/fast_qr_bg.wasm" `
  -o "${OUTPUT_DIR}/fast_qr_bg.wasm"

& Write-Host "Done!"
