#[cfg(feature = "svg")]
fn main() {
    use fast_qr::{
        convert::{svg::SvgBuilder, Builder, Shape},
        QRBuilder, Version, ECL,
    };

    let tmp = SvgBuilder::default();
    let qrcode = QRBuilder::new("https://example.com/").build().unwrap();
    tmp.to_file(&qrcode, "tmp.svg");
}

#[cfg(not(feature = "svg"))]
fn main() {
    compile_error!("Please enable the `svg` features.")
}
