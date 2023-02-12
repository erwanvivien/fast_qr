#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
//! # Easy to use fast QRCode generator
//!
//! More examples can be found on [GitHub](https://github.com/erwanvivien/fast_qr/tree/master/examples).
//!
//! ## Converts [`QRCode`] to Unicode
//!
//! ```rust
//! # use fast_qr::convert::ConvertError;
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilder::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let str = qrcode.to_str(); // .print() exists
//! println!("{}", str);
//!
//! #     Ok(())
//! # }
//! ```
//!
//! ## Converts [`QRCode`] to SVG
//!
//! ```rust
//! # use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilder::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _svg = SvgBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .to_file(&qrcode, "out.svg");
//! #     std::fs::remove_file("out.svg");
//!
//! #     Ok(())
//! # }
//! ```
//!
//! ## Converts [`QRCode`] to an image
//!
//! ```rust
//! # use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilder::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _img = ImageBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .background_color([255, 255, 255, 0]) // transparency
//!     .fit_width(600)
//!     .to_file(&qrcode, "out.png");
//! #     std::fs::remove_file("out.png");
//!
//! #     Ok(())
//! # }
//! ```

use convert::{ImageBackgroundShape, Shape};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use crate::datamasking::Mask;
pub use crate::ecl::ECL;
pub use crate::module::{Module, ModuleType};
#[cfg(target_arch = "wasm32")]
use crate::qr::QRCodeError;
pub use crate::qr::{QRBuilder, QRCode};
pub use crate::version::Version;

mod compact;
#[doc(hidden)]
pub mod datamasking;

pub mod convert;
mod default;
mod ecl;
mod encode;
mod hardcode;
#[cfg(not(target_arch = "wasm32"))]
mod helpers;
mod module;
mod placement;
mod polynomials;
#[macro_use]
pub mod qr;
mod score;
mod version;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
fn bool_to_u8(qr: &QRCode) -> Vec<u8> {
    let dim = qr.size;
    qr.data[..dim * dim]
        .iter()
        .map(|x| x.value() as u8)
        .collect()
    // qr.data.iter().flatten().map(|x| x.value() as u8).collect()
}

#[cfg(target_arch = "wasm32")]
fn qr_generate(qr: Result<QRCode, QRCodeError>) -> Vec<u8> {
    if let Ok(qr) = qr {
        bool_to_u8(&qr)
    } else {
        Vec::new()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
/// Generate a QR code from a string. All parameters are automatically set.
pub fn qr(content: &str) -> Vec<u8> {
    let qrcode = QRCode::new(content.as_bytes(), None, None, None);
    qr_generate(qrcode)
}

/// Configuration for the SVG output.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SvgOptions {
    shape: Shape,
    margin: usize,

    background_color: Vec<u8>,

    image: String,
    image_background_color: Vec<u8>,
    image_background_shape: ImageBackgroundShape,
    image_size: Vec<f64>,
    image_position: Vec<f64>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl SvgOptions {
    fn color_to_code(color: String) -> Vec<u8> {
        let mut color = color;
        if color.starts_with('#') {
            color.remove(0);
        }
        let color = color.as_bytes();
        let color = color.chunks_exact(2);
        let color = color.map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap());

        let mut color = color.collect::<Vec<u8>>();
        if color.len() == 3 {
            color.push(255);
        }

        color
    }

    /// Updates the shape of the QRCode modules.
    pub fn shape(self, shape: Shape) -> Self {
        Self { shape, ..self }
    }

    /// Updates the margin of the QRCode.
    pub fn margin(self, margin: usize) -> Self {
        Self { margin, ..self }
    }

    /// Updates the background color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
    pub fn background_color(self, background_color: String) -> Self {
        let code = Self::color_to_code(background_color);
        if code.len() != 4 {
            return self;
        }
        Self {
            background_color: code,
            ..self
        }
    }

    /// Updates the image of the QRCode. Takes base64 or a url.
    pub fn image(self, image: String) -> Self {
        Self { image, ..self }
    }

    /// Updates the background color of the image. Takes a string in the format `#RRGGBB[AA]`.
    pub fn image_background_color(self, image_background_color: String) -> Self {
        let code = Self::color_to_code(image_background_color);
        if code.len() != 4 {
            return self;
        }

        Self {
            image_background_color: code,
            ..self
        }
    }

    /// Updates the shape of the image background. Takes an ImageBackgroundShape.
    pub fn image_background_shape(self, image_background_shape: ImageBackgroundShape) -> Self {
        Self {
            image_background_shape,
            ..self
        }
    }

    /// Updates the size of the image. Takes a size and a gap (unit being module size).
    pub fn image_size(self, size: f64, gap: f64) -> Self {
        Self {
            image_size: vec![size, gap],
            ..self
        }
    }

    /// Updates the position of the image. Takes an array [x, y] (unit being module size).
    pub fn image_position(self, image_position: Vec<f64>) -> Self {
        if image_position.len() != 2 {
            return self;
        }

        Self {
            image_position,
            ..self
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl SvgOptions {
    #[wasm_bindgen(constructor)]
    /// Creates a new SvgOptions object.
    pub fn new() -> Self {
        Self {
            shape: Shape::Square,
            margin: 4,

            background_color: vec![255, 255, 255, 255],

            image: String::new(),
            image_background_color: vec![255, 255, 255, 255],
            image_background_shape: ImageBackgroundShape::Square,
            image_size: vec![],
            image_position: vec![],
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "svg"))]
#[wasm_bindgen]
/// Generate a QR code from a string. All parameters are automatically set.
pub fn qr_svg(content: &str, options: SvgOptions) -> String {
    use crate::convert::svg::SvgBuilder;
    use crate::convert::Builder;
    let qrcode = QRCode::new(content.as_bytes(), None, None, None);

    let mut builder = SvgBuilder::default();
    builder.shape(options.shape);
    builder.margin(options.margin);
    if let Ok(background_color) = options.background_color.try_into() {
        builder.background_color(background_color);
    }
    if !options.image.is_empty() {
        builder.image(options.image);
    }

    if let Ok(image_background_color) = options.image_background_color.try_into() {
        builder.image_background_color(image_background_color);
    }
    builder.image_background_shape(options.image_background_shape);

    if options.image_size.len() == 2 {
        let size = options.image_size[0];
        let gap = options.image_size[1];
        builder.image_size(size, gap);
    }

    if options.image_size.len() == 2 {
        let x = options.image_position[0];
        let y = options.image_position[1];
        builder.image_position(x, y);
    }

    qrcode
        .map(|qrcode| builder.to_str(&qrcode))
        .unwrap_or(String::new())
}
