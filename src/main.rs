use qrgen::ecl::ECL;
use qrgen::qrcode::QRCode;
use qrgen::version::Version;

/// Still useless, only test purposes for now.
fn main() {
    const CONTENT: &str = "https://vahan.dev/";
    const MASK: Option<usize> = None;
    const VERSION: Option<Version> = None;
    const LEVEL: Option<ECL> = Some(ECL::H);

    const QRCODE: Option<QRCode> = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    if let Some(q) = QRCODE {
        q.print();
    }
}
