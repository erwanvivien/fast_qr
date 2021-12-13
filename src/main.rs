use qrgen::comptime::qrcode::{QRCode, Version, ECL};

/// Still useless, only test purposes for now.
fn main() {
    const CONTENT: &str = "https://vahan.dev/";
    const MASK: Option<usize> = None;
    const VERSION: Option<Version> = None;
    const LEVEL: Option<ECL> = Some(ECL::H);

    const QRCODE: QRCode = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);
    QRCODE.print();
}
