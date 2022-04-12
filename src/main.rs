use fast_qr::qr::{QRBuilder, QRCodeError};
use fast_qr::{Version, ECL};

/// Still useless, only test purposes for now.
fn main() -> Result<(), QRCodeError> {
    let qrcode = QRBuilder::new("https://example.com/".into())
        .ecl(ECL::H)
        .version(Version::V03)
        .build()?;
    qrcode.print();

    Ok(())
}
