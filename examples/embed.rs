#[cfg(all(feature = "image", feature = "svg"))]
fn main() {
    use fast_qr::{
        convert::{image::ImageBuilder, Builder, ImageBackgroundShape, Shape},
        QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let mut image = ImageBuilder::default();
    image
        .shape(Shape::RoundedSquare)
        .fit_width(600)
        .background_color([255, 255, 255, 255]) // transparency
        // New: embed an image
        .image("./awesome.jpg")
        .image_background_color([165, 34, 247, 255])
        .image_background_shape(ImageBackgroundShape::Square);

    let _img = image.to_file(&qrcode, "out.png");
}

#[cfg(not(all(feature = "image", feature = "svg")))]
fn main() {
    eprintln!("Please enable the `image` and `svg` features.")
}
