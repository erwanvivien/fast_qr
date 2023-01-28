//! Converts [`QRCode`] to SVG
//!
//! ```rust
//! use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilde::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _svg = SvgBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .to_file(&qrcode, "out.svg");
//!
//! #     std::fs::remove_file("out.svg");
//! #     Ok(())
//! # }
//! ```

use std::fs::File;
use std::io;
use std::io::Write;

use crate::{QRCode, Version};

use super::{rgba2hex, Builder, ImageBackgroundShape, Shape};

/// Builder for svg, can set shape, margin, background_color, dot_color
pub struct SvgBuilder {
    /// The shape for each module, default is square
    shape: Shape,
    /// The margin for the svg, default is 4
    margin: usize,
    /// The background color for the svg, default is #FFFFFF
    background_color: [u8; 4],
    /// The color for each module, default is #000000
    dot_color: [u8; 4],

    image: Option<&'static str>,
    image_background_color: [u8; 4],
    image_background_shape: ImageBackgroundShape,
    image_size: Option<(f64, f64)>,
}

#[derive(Debug)]
/// Error when converting to svg
pub enum SvgError {
    /// Error while writing to file
    IoError(io::Error),
    /// Error while creating svg
    SvgError(String),
}

/// Creates a Builder instance
impl Default for SvgBuilder {
    fn default() -> Self {
        SvgBuilder {
            background_color: [255; 4],
            dot_color: [0, 0, 0, 255],
            margin: 4,
            shape: Shape::Square,

            // Image Embedding
            image: None,
            image_background_color: [255; 4],
            image_background_shape: ImageBackgroundShape::Square,
            image_size: None,
        }
    }
}

impl Builder for SvgBuilder {
    /// Changes margin (default: 4)
    fn margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }

    /// Changes module color (default: #000000)
    fn module_color(&mut self, dot_color: [u8; 4]) -> &mut Self {
        self.dot_color = dot_color;
        self
    }

    /// Changes background color (default: #FFFFFF)
    fn background_color(&mut self, background_color: [u8; 4]) -> &mut Self {
        self.background_color = background_color;
        self
    }

    /// Changes shape (default: Square)
    fn shape(&mut self, shape: Shape) -> &mut Self {
        self.shape = shape;
        self
    }

    fn image(&mut self, image: &'static str) -> &mut Self {
        self.image = Some(image);
        self
    }

    fn image_background_color(&mut self, image_background_color: [u8; 4]) -> &mut Self {
        self.image_background_color = image_background_color;
        self
    }

    fn image_background_shape(
        &mut self,
        image_background_shape: ImageBackgroundShape,
    ) -> &mut Self {
        self.image_background_shape = image_background_shape;
        self
    }

    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self {
        self.image_size = Some((image_size, gap));
        self
    }
}

impl SvgBuilder {
    fn image_placement(
        image_background_shape: ImageBackgroundShape,
        margin: usize,
        n: usize,
    ) -> (f64, f64, f64) {
        use ImageBackgroundShape::{Circle, RoundedSquare, Square};

        #[rustfmt::skip]
        const SQUARE: [(f64, f64); 40] = [
            (5f64, 8f64),   (9f64, 8f64),   (9f64, 10f64),  (11f64, 11f64), (13f64, 12f64),
            (13f64, 14f64), (15f64, 15f64), (17f64, 16f64), (17f64, 18f64), (19f64, 19f64),
            (21f64, 20f64), (21f64, 22f64), (23f64, 23f64), (25f64, 24f64), (25f64, 26f64),
            (27f64, 27f64), (29f64, 28f64), (29f64, 30f64), (31f64, 31f64), (33f64, 32f64),
            (33f64, 34f64), (35f64, 35f64), (37f64, 36f64), (37f64, 38f64), (39f64, 39f64),
            (41f64, 40f64), (41f64, 42f64), (43f64, 43f64), (45f64, 44f64), (45f64, 46f64),
            (47f64, 47f64), (49f64, 48f64), (49f64, 50f64), (51f64, 51f64), (53f64, 52f64),
            (53f64, 54f64), (55f64, 55f64), (57f64, 56f64), (57f64, 58f64), (59f64, 59f64),
        ];
        const ROUNDED_SQUARE: [(f64, f64); 40] = SQUARE;
        const CIRCLE: [(f64, f64); 40] = SQUARE;

        // Using hardcoded values
        let version = Version::from_n(n) as usize;
        let (border_size, placed_coord) = match image_background_shape {
            Square => SQUARE[version],
            RoundedSquare => ROUNDED_SQUARE[version],
            Circle => CIRCLE[version],
        };

        // Allows for a module gap between the image and the border
        let gap = match image_background_shape {
            Square | RoundedSquare => 2f64,
            Circle => 3f64,
        };
        // Make the image border bigger for bigger versions
        let gap = gap * (version + 10) as f64 / 10f64;

        (border_size, placed_coord + margin as f64, border_size - gap)
    }

    /// Return a string containing the svg for a qr code
    pub fn to_str(&self, qr: &QRCode) -> String {
        let n: usize = qr.size;

        let mut out = String::with_capacity(11 * n * n / 2);
        out.push_str(&format!(
            r#"<svg viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
            self.margin * 2 + n
        ));

        out.push_str(&format!(
            r#"<rect width="{0}px" height="{0}px" fill="{1}"/><path d=""#,
            self.margin * 2 + n,
            rgba2hex(self.background_color)
        ));

        for i in 0..qr.size {
            let line = &qr[i];
            for (j, &cell) in line.iter().enumerate() {
                if !cell.value() {
                    continue;
                }

                let current = match self.shape {
                    Shape::Square => format!("M{},{}h1v1h-1", j + self.margin, i + self.margin),
                    Shape::Circle => format!(
                        "M{},{}a.5,.5 0 1,1 0,-.1",
                        j + self.margin + 1,
                        (i + self.margin) as f64 + 0.5f64
                    ),
                    Shape::RoundedSquare => format!(
                        "M{0}.2,{1}.2 {0}.8,{1}.2 {0}.8,{1}.8 {0}.2,{1}.8z",
                        j + self.margin,
                        i + self.margin,
                    ),
                    Shape::Horizontal => {
                        format!("M{}.1,{}h1v.8h-1", j + self.margin, i + self.margin)
                    }
                    Shape::Vertical => {
                        format!("M{},{}.1h.8v1h-.8", j + self.margin, i + self.margin)
                    }
                    Shape::Diamond => {
                        format!(
                            "M{}.5,{}l.5,.5l-.5,.5l-.5,-.5z",
                            j + self.margin,
                            i + self.margin
                        )
                    }
                };

                out.push_str(&current);
            }
        }

        if self.shape == Shape::RoundedSquare {
            out.push_str(&format!(
                r##"" stroke-width=".3" stroke-linejoin="round" stroke="{}"##,
                rgba2hex(self.dot_color)
            ));
        }

        out.push_str(&format!(r#"" fill="{}"/>"#, rgba2hex(self.dot_color)));

        if let Some(image) = self.image {
            let (mut border_size, mut placed_coord, mut image_size) =
                Self::image_placement(self.image_background_shape, self.margin, n);

            if let Some((override_size, gap)) = self.image_size {
                border_size = override_size + gap * 2f64;
                placed_coord = (self.margin * 2 + n) as f64 - border_size;
                placed_coord /= 2f64;
                image_size = override_size;
            }

            out.push_str(&format!(
                r#"<rect x="{0:.2}" y="{0:.2}" width="{1:.2}" height="{1:.2}" fill="{2}"/>"#,
                placed_coord,
                border_size,
                rgba2hex(self.background_color)
            ));
            let format = match self.image_background_shape {
                ImageBackgroundShape::Square => {
                    r#"<rect x="{0}" y="{0}" width="{1}" height="{1}" fill="{2}"/>"#
                }
                ImageBackgroundShape::Circle => {
                    r#"<rect x="{0}" y="{0}" width="{1}" height="{1}" fill="{2}" rx="1000px"/>"#
                }
                ImageBackgroundShape::RoundedSquare => {
                    r#"<rect x="{0}" y="{0}" width="{1}" height="{1}" fill="{2}" rx="1px"/>"#
                }
            };

            let format = format
                .replace("{0}", &placed_coord.to_string())
                .replace("{1}", &border_size.to_string())
                .replace("{2}", &rgba2hex(self.image_background_color));

            out.push_str(&format);

            out.push_str(&format!(
                r#"<image x="{0:.2}" y="{0:.2}" width="{1:.2}" height="{1:.2}" href="{2}" />"#,
                placed_coord + (border_size - image_size) / 2f64,
                image_size,
                image
            ));
        }

        out.push_str("</svg>");
        out
    }

    /// Saves the svg for a qr code to a file
    pub fn to_file(&self, qr: &QRCode, file: &str) -> Result<(), SvgError> {
        let out = self.to_str(qr);

        let mut f = File::create(file).map_err(SvgError::IoError)?;
        f.write_all(out.as_bytes()).map_err(SvgError::IoError)?;

        Ok(())
    }
}
