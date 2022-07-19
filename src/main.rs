use fast_qr::module::{Module, ModuleType};
use fast_qr::qr::{QRBuilder, QRCodeError};
use fast_qr::{default, helpers, Version, ECL};

/// Still useless, only test purposes for now.
fn main() -> Result<(), QRCodeError> {
    // let qrcode = QRBuilder::new("https://example.com/".into())
    //     .ecl(ECL::H)
    //     .version(Version::V03)
    //     .build()?;
    // qrcode.print();

    let mat = default::create_matrix::<21>(Version::V01);
    helpers::print_matrix_with_margin(&mat);

    Ok(())
}
