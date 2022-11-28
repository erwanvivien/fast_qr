#[cfg(feature = "image")]
use fast_qr::convert::image::ImageBuilder;
#[cfg(feature = "svg")]
use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use fast_qr::{Version, ECL};

fn main() {
    let qrcode = QRBuilder::new("https://example.com/".into())
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    qrcode.print();

    #[cfg(feature = "svg")]
    let _svg = SvgBuilder::default()
        .shape(Shape::RoundedSquare)
        .to_file(&qrcode, "out.svg");

    #[cfg(feature = "image")]
    let _image = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .fit_width(600)
        .to_file(&qrcode, "out.png");
}
