#[cfg(feature = "svg")]
fn main() {
    use fast_qr::{
        convert::{svg::SvgBuilder, Builder, Shape},
        QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let _svg = SvgBuilder::default()
        .shape(Shape::RoundedSquare)
        .to_file(&qrcode, "svg.svg");
}

#[cfg(not(feature = "svg"))]
fn main() {
    eprintln!("Please enable the `svg` feature.")
}
