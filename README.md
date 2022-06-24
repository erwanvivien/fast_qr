![FastQR banner](/assets/banner.png)

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
wasm-opt -Os -o pkg/fast_qr_bg_small.wasm pkg/fast_qr_bg.wasm
```

Or find a bundled version in the latest release

## Benchmarks

| Version | Level | fast_qr   | qrcode    | Ratio |
|---------|-------|-----------|-----------|-------|
| V3      | High  | 97.590 us | 620.01 us | 6.35x |
| V3      | Low   | 93.605 us | 608.85 us | 6,50x |
| V10     | High  | 404.44 us | 2.5028 ms | 6,19x |
| V10     | Low   | 386.35 us | 2.3996 ms | 6,21x |
| V40     | High  | 3.2543 ms | 21.570 ms | 6,63x |
| V40     | Low   | 3.3271 ms | 21.744 ms | 6,54x |

<i>Computed using [cargo-criterion](https://crates.io/crates/cargo-criterion).</i> \
We clearly see that its invariant to level. 
