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

    let mut _img = ImageBuilder::default()
        // Can have many shapes and custom shapes
        .shape(Shape::Command(|y, x, _| format!("M{},{}h1v.5h-1", x, y)))
        .shape_color(Shape::Circle, [255, 0, 0, 255])
        .fit_width(600)
        .background_color([255, 255, 255, 255])
        .to_file(&qrcode, "out.png");
}

#[cfg(not(all(feature = "image", feature = "svg")))]
fn main() {
    eprintln!("Please enable the `image` and `svg` features.")
}
