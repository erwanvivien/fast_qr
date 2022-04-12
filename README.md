# Fast_QR

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
```

Or find a bundled version in the latest release
