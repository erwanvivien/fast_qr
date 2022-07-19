use fast_qr::module::{Module, ModuleType};
use fast_qr::qr::{QRBuilder, QRCodeError};
use fast_qr::{default, helpers, Version, ECL};

/// Still useless, only tests purposes for now.
fn main() -> Result<(), QRCodeError> {
    let qrcode = QRBuilder::new("https://example.com/".into())
        .ecl(ECL::H)
        .version(Version::V03)
        .build()?;
    qrcode.print();

    Ok(())
}
