use fast_qr::{QRBuilder, Version, ECL};

fn main() {
    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    qrcode.print();
}
