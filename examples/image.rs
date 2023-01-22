fn main() {
    #[cfg(feature = "image")]
    {
        use fast_qr::{
            convert::{image::ImageBuilder, Builder, Shape},
            QRBuilder, Version, ECL,
        };

        let qrcode = QRBuilder::new("https://example.com/")
            .ecl(ECL::H)
            .version(Version::V03)
            .build()
            .unwrap();

        let _image = ImageBuilder::default()
            .shape(Shape::RoundedSquare)
            .fit_width(600)
            .background_color([255, 255, 255, 0]) // transparency
            .to_file(&qrcode, "out.png");
    }
}
