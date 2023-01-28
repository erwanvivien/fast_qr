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

/// Converts a position to a module svg
/// # Example
///
/// For the square shape, the svg is `M{x},{y}h1v1h-1`
///
/// ```rust
/// fn square(y: usize, x: usize) -> String {
///     format!("M{},{}h1v1h-1", x, y)
/// }
/// ```
pub type ModuleFunction = fn(usize, usize) -> String;

/// Builder for svg, can set shape, margin, background_color, dot_color
pub struct SvgBuilder {
    /// The shape for each module, default is square
    commands: Vec<ModuleFunction>,
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
    image_position: Option<(f64, f64)>,
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
            commands: Vec::new(),

            // Image Embedding
            image: None,
            image_background_color: [255; 4],
            image_background_shape: ImageBackgroundShape::Square,
            image_size: None,
            image_position: None,
        }
    }
}

impl Builder for SvgBuilder {
    fn margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }

    fn module_color(&mut self, dot_color: [u8; 4]) -> &mut Self {
        self.dot_color = dot_color;
        self
    }

    fn background_color(&mut self, background_color: [u8; 4]) -> &mut Self {
        self.background_color = background_color;
        self
    }

    fn shape(&mut self, shape: Shape) -> &mut Self {
        let command: ModuleFunction = match shape {
            Shape::Square => square,
            Shape::Circle => circle,
            Shape::RoundedSquare => rounded_square,
            Shape::Vertical => vertical,
            Shape::Horizontal => horizontal,
            Shape::Diamond => diamond,
            Shape::Command(command) => command,
        };

        self.commands.push(command);
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

    fn image_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.image_position = Some((x, y));
        self
    }
}

fn square(y: usize, x: usize) -> String {
    format!("M{},{}h1v1h-1", x, y)
}

fn circle(y: usize, x: usize) -> String {
    format!("M{},{}a.5,.5 0 1,1 0,-.1", x + 1, y as f32 + 0.5f32)
}

fn rounded_square(y: usize, x: usize) -> String {
    format!("M{0}.2,{1}.2 {0}.8,{1}.2 {0}.8,{1}.8 {0}.2,{1}.8z", x, y)
}

fn horizontal(y: usize, x: usize) -> String {
    format!("M{},{}.1h1v.8h-1", x, y)
}

fn vertical(y: usize, x: usize) -> String {
    format!("M{}.1,{}h.8v1h-.8", x, y)
}

fn diamond(y: usize, x: usize) -> String {
    format!("M{}.5,{}l.5,.5l-.5,.5l-.5,-.5z", x, y)
}

impl SvgBuilder {
    fn image_placement(
        image_background_shape: ImageBackgroundShape,
        margin: usize,
        n: usize,
    ) -> (f64, (f64, f64), f64) {
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
        let placed_coord = placed_coord + margin as f64;
        let placed_coord = (placed_coord, placed_coord);

        (border_size, placed_coord, border_size - gap)
    }

    fn image(&self, n: usize) -> String {
        if self.image.is_none() {
            return String::new();
        }

        let image = self.image.unwrap();
        let mut out = String::with_capacity(image.len() + 100);

        let (mut border_size, mut placed_coord, mut image_size) =
            Self::image_placement(self.image_background_shape, self.margin, n);

        if let Some((override_size, gap)) = self.image_size {
            border_size = override_size + gap * 2f64;
            let mut placed_coord_x = (self.margin * 2 + n) as f64 - border_size;
            placed_coord_x /= 2f64;
            placed_coord = (placed_coord_x, placed_coord_x);
            image_size = override_size;
        }

        if let Some((x, y)) = self.image_position {
            placed_coord = (x - border_size / 2f64, y - border_size / 2f64);
        }

        out.push_str(&format!(
            r#"<rect x="{0:.2}" y="{1:.2}" width="{2:.2}" height="{2:.2}" fill="{3}"/>"#,
            placed_coord.0,
            placed_coord.1,
            border_size,
            rgba2hex(self.background_color)
        ));
        let format = match self.image_background_shape {
            ImageBackgroundShape::Square => {
                r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}"/>"#
            }
            ImageBackgroundShape::Circle => {
                r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}" rx="1000px"/>"#
            }
            ImageBackgroundShape::RoundedSquare => {
                r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}" rx="1px"/>"#
            }
        };

        let format = format
            .replace("{0}", &placed_coord.0.to_string())
            .replace("{1}", &placed_coord.1.to_string())
            .replace("{2}", &border_size.to_string())
            .replace("{3}", &rgba2hex(self.image_background_color));

        out.push_str(&format);

        out.push_str(&format!(
            r#"<image x="{0:.2}" y="{1:.2}" width="{2:.2}" height="{2:.2}" href="{3}" />"#,
            placed_coord.0 + (border_size - image_size) / 2f64,
            placed_coord.1 + (border_size - image_size) / 2f64,
            image_size,
            image
        ));

        out
    }

    fn path(&self, qr: &QRCode) -> String {
        let commands: &[ModuleFunction] = if !self.commands.is_empty() {
            &self.commands
        } else {
            &[square]
        };

        let mut paths = vec![String::with_capacity(10 * qr.size * qr.size); commands.len()];
        for path in paths.iter_mut() {
            path.push_str(r#"<path d=""#);
        }

        for y in 0..qr.size {
            let line = &qr[y];
            for (x, &cell) in line.iter().enumerate() {
                if !cell.value() {
                    continue;
                }

                for (i, command) in commands.iter().enumerate() {
                    paths[i].push_str(&command(x + self.margin, y + self.margin));
                }
            }
        }

        for (i, &command) in commands.iter().enumerate() {
            // Allows to compare if two function pointers are the same
            // This works because there is no notion of Generics for `rounded_square`
            if command as usize == rounded_square as usize {
                paths[i].push_str(&format!(
                    r##"" stroke-width=".3" stroke-linejoin="round" stroke="{}"##,
                    rgba2hex(self.dot_color)
                ));
            }

            paths[i].push_str(&format!(r#"" fill="{}"/>"#, rgba2hex(self.dot_color)));
        }

        paths.join("")
    }

    /// Return a string containing the svg for a qr code
    pub fn to_str(&self, qr: &QRCode) -> String {
        let n = qr.size;

        let mut out = String::with_capacity(11 * n * n / 2);
        out.push_str(&format!(
            r#"<svg viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
            self.margin * 2 + n
        ));

        out.push_str(&format!(
            r#"<rect width="{0}px" height="{0}px" fill="{1}"/>"#,
            self.margin * 2 + n,
            rgba2hex(self.background_color)
        ));

        out.push_str(&self.path(qr));
        out.push_str(&self.image(n));

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
