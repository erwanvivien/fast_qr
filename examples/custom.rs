#[cfg(all(feature = "image", feature = "svg"))]
fn main() {
    use fast_qr::{
        convert::{image::ImageBuilder, Builder, Shape},
        QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let mut image = ImageBuilder::default();
    image
        // Can have many shapes and custom shapes
        .shape(Shape::Command(|y, x| format!("M{},{}h1v.5h-1", x, y)))
        .shape(Shape::Circle)
        .fit_width(600)
        .background_color([255, 255, 255, 255]);

    let _img = image.to_file(&qrcode, "out.png");
}

#[cfg(not(all(feature = "image", feature = "svg")))]
fn main() {
    eprintln!("Please enable the `image` and `svg` features.")
}
