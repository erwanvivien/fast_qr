use qrgen::{QRCode, QRCodeError, Version, ECL};

/// Still useless, only test purposes for now.
fn main() -> Result<(), QRCodeError> {
    const MASK: Option<usize> = None;
    const VERSION: Option<Version> = None;
    const LEVEL: Option<ECL> = Some(ECL::H);

    let qrcode = QRCode::new(b"https://example.com/", LEVEL, VERSION, MASK)?;
    qrcode.print();

    Ok(())
}
