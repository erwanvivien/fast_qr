<div style="display: flex; justify-content: center">
  <img src="assets/banner.svg"  alt="Example qr for website example.com"/>
</div>

`fast_qr` is approximately 6.3 times faster than `qrcode`, see [benchmarks](#benchmarks)

You can create a QR as

- [x] Raw matrix, well suited for custom usage
- [x] Vectorized image, well suited for web usage
- [ ] Image, well suited for mobile / print usage

### Usage

```rust
use fast_qr::{ECL, Version, QRBuilder};

let qrcode = QRBuilder::new("https://example.com/".into())
    .ecl(ECL::H)
    .version(Version::V03)
    .build();

// It is preferable to check qrcode result before
qrcode.unwrap().print();
```

<div style="display: flex; justify-content: center">
  <img src="assets/example.com.svg"  alt="Example qr for website example.com"/>
</div>

## Build WASM

```bash
wasm-pack build --target web # All ready in ./pkg
wasm-opt -Os -o pkg/fast_qr_bg.wasm pkg/fast_qr_bg.wasm # Optimizes wasm module size
```

Or find a bundled version in the latest release

## Benchmarks

According to the following benchmarks, `fast_qr` is approximately 9-10x faster than `qrcode`.

| Benchmark    |   Lower   | Estimate  |   Upper   |                          |
|:-------------|:---------:|:---------:|:---------:|--------------------------|
| V03H/qrcode  | 471.38 us | 472.47 us | 473.57 us |                          |
| V03H/fast_qr | 46.447 us | 46.573 us | 46.710 us | fast_qr is 10.14x faster |
| V10H/qrcode  | 2.0083 ms | 2.0121 ms | 2.0160 ms |                          |
| V10H/fast_qr | 196.96 us | 197.30 us | 197.62 us | fast_qr is 10.20x faster |
| V40H/qrcode  | 17.316 ms | 17.339 ms | 17.361 ms |                          |
| V40H/fast_qr | 1.9863 ms | 1.9898 ms | 1.9934 ms | fast_qr is 8.71x faster  |

More benchmarks can be found in [/benches folder](https://github.com/erwanvivien/fast_qr/tree/master/benches).
