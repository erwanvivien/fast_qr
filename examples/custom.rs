#[cfg(all(feature = "image", feature = "svg"))]
fn main() {
    use fast_qr::{
        convert::{image::ImageBuilder, Builder, Shape},
        ModuleType, QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let mut _img = ImageBuilder::default()
        // Can have many shapes and custom shapes
        .shape(Shape::Command(|y, x, cell| {
            if cell.module_type() == ModuleType::Data {
                Shape::Square(y, x, cell)
            } else {
                String::new()
            }
        }))
        .shape_color(
            Shape::Command(|y, x, cell| {
                if cell.module_type() != ModuleType::Data {
                    // Works thanks to Deref
                    Shape::Circle(y, x, cell)
                } else {
                    String::new()
                }
            }),
            [255, 0, 0, 255],
        )
        .fit_width(600)
        .background_color([255, 255, 255, 255])
        .to_file(&qrcode, "out.png");
}

#[cfg(not(all(feature = "image", feature = "svg")))]
fn main() {
    eprintln!("Please enable the `image` and `svg` features.")
}
