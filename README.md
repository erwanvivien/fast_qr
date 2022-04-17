# Fast_QR

`fast_qr` generates QRCodes approximately 5-6x faster than `qrcode` crate.
This test was done using `time` tool and generating 10k QRCodes of version 40.

You can create a QR as

- [x] Raw matrix
- [x] Vectorized image
- [ ] Image

### Example
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
