fn main() {
    #[cfg(feature = "svg")]
    {
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
            .to_file(&qrcode, "out.svg");
    }
}
