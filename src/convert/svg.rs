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

use crate::QRCode;

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
            image: None,

            image_background_color: [255; 4],
            image_background_shape: ImageBackgroundShape::Square,
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
}

impl SvgBuilder {
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
            let size_offset = match self.image_background_shape {
                ImageBackgroundShape::Circle => 2f64,
                ImageBackgroundShape::Square => 0f64,
                ImageBackgroundShape::RoundedSquare => 0f64,
            };

            let max_size = (n as f64 / 3f64).floor();
            let placed_coord = max_size + self.margin as f64 + 1f64;
            dbg!(max_size, placed_coord);

            let max_size = max_size - size_offset;

            out.push_str(&format!(
                r#"<rect x="{0:.2}" y="{0:.2}" width="{1:.2}" height="{1:.2}" fill="white"/>"#,
                placed_coord - 1f64,
                max_size + 2f64 + size_offset,
            ));
            match self.image_background_shape {
                ImageBackgroundShape::Square => {
                    out.push_str(&format!(
                        r#"<rect x="{0:.2}" y="{0:.2}" width="{1:.2}" height="{1:.2}" fill="{2}"/>"#,
                        placed_coord - 1f64,
                        max_size + 2f64 + size_offset,
                        rgba2hex(self.image_background_color)
                    ));
                }
                ImageBackgroundShape::Circle => {
                    out.push_str(&format!(
                        r#"<rect x="{0:.2}" y="{0:.2}" width="{1:.2}" height="{1:.2}" fill="{2}" rx="1000px"/>"#,
                        placed_coord - 1f64,
                        max_size + 2f64 + size_offset,
                        rgba2hex(self.image_background_color)
                    ));
                }
                ImageBackgroundShape::RoundedSquare => {
                    out.push_str(&format!(
                        r#"<rect x="{0:.2}" y="{0:.2}" width="{1:.2}" height="{1:.2}" fill="{2}" rx="1px"/>"#,
                        placed_coord - 1f64,
                        max_size + 2f64 + size_offset,
                        rgba2hex(self.image_background_color)
                    ));
                }
            }
            out.push_str(&format!(
                r#"<image x="{0:.2}" y="{0:.2}" width="{1:.2}" height="{1:.2}" href="{2}" />"#,
                placed_coord + size_offset / 2f64,
                max_size,
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
