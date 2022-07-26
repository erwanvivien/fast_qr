use fast_qr::convert::svg::{SvgBuilder, SvgError};
use fast_qr::qr::{QRBuilder, QRCodeError};
use fast_qr::{Version, ECL};

#[derive(Debug)]
enum Error {
    QrCodeError(QRCodeError),
    SvgError(SvgError),
}

impl From<QRCodeError> for Error {
    fn from(e: QRCodeError) -> Error {
        Error::QrCodeError(e)
    }
}

impl From<SvgError> for Error {
    fn from(e: SvgError) -> Error {
        Error::SvgError(e)
    }
}

fn main() -> Result<(), Error> {
    let qrcode = QRBuilder::new("https://example.com/".into())
        .ecl(ECL::H)
        .version(Version::V03)
        .build()?;

    qrcode.print();

    // SvgBuilder::new().to_file(qrcode, "qrcode.svg")?;
    let _svg = SvgBuilder::new().to_str(&qrcode);
    // println!("{}", _svg);

    Ok(())
}
