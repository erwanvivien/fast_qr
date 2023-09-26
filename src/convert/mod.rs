//! Converts a [`crate::QRCode`] to image or SVG you will need to activate associated feature flag

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;

#[cfg(feature = "svg")]
use svg::SvgError;

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "image")]
use image::ImageError;

mod module_shape;
pub use module_shape::{ModuleFunction, ModuleShape};

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
use wasm_bindgen::prelude::*;

/// Different possible image background shapes
#[cfg_attr(feature = "wasm-bindgen", repr(C), wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum ImageBackgroundShape {
    /// Square shape
    Square,
    /// Circle shape
    Circle,
    /// Rounded square shape
    RoundedSquare,
}

/// Contains possible errors for a conversion
#[derive(Debug)]
pub enum ConvertError {
    /// Contains error message for a SVG conversion
    #[cfg(feature = "svg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
    Svg(String),
    /// Contains error message for an Image conversion
    #[cfg(feature = "image")]
    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    Image(String),
    /// Contains error message if a file write failed
    Io(std::io::Error),
}

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
impl From<SvgError> for ConvertError {
    fn from(err: SvgError) -> Self {
        match err {
            SvgError::SvgError(svg_err) => Self::Svg(svg_err),
            #[cfg(not(feature = "wasm-bindgen"))]
            SvgError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
impl From<ImageError> for ConvertError {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::EncodingError(image_err) => Self::Image(image_err),
            ImageError::ImageError(image_err) => Self::Image(image_err),
            ImageError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

/// Converts an array of pixel color to it's hexadecimal representation
/// # Example
/// ```rust
/// # use fast_qr::convert::rgba2hex;
/// let color = [0, 0, 0, 255];
/// assert_eq!(&rgba2hex(color), "#000000");
/// ```
#[must_use]
pub fn rgba2hex(color: [u8; 4]) -> String {
    let mut hex = String::with_capacity(9);

    hex.push('#');
    hex.push_str(&format!("{:02x}", color[0]));
    hex.push_str(&format!("{:02x}", color[1]));
    hex.push_str(&format!("{:02x}", color[2]));
    if color[3] != 255 {
        hex.push_str(&format!("{:02x}", color[3]));
    }

    hex
}

/// Allows to take String, string slices, arrays or slices of u8 (3 or 4) to create a [Color]
pub struct Color(pub String);

impl Color {
    /// Returns the contained color
    #[must_use]
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Color {
    fn from(color: String) -> Self {
        Self(color)
    }
}

impl From<&str> for Color {
    fn from(color: &str) -> Self {
        Self(color.to_string())
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self(rgba2hex(color))
    }
}

impl From<[u8; 3]> for Color {
    fn from(color: [u8; 3]) -> Self {
        Self::from([color[0], color[1], color[2], 255])
    }
}

impl From<&[u8]> for Color {
    fn from(color: &[u8]) -> Self {
        if color.len() == 3 {
            Self::from([color[0], color[1], color[2]])
        } else if color.len() == 4 {
            Self::from([color[0], color[1], color[2], color[3]])
        } else {
            panic!("Invalid color length");
        }
    }
}

impl From<Vec<u8>> for Color {
    fn from(color: Vec<u8>) -> Self {
        Self::from(&color[..])
    }
}

/// Trait for `SvgBuilder` and `ImageBuilder`
pub trait Builder {
    /// Updates margin (default: 4)
    fn margin(&mut self, margin: usize) -> &mut Self;
    /// Updates module color (default: #000000)
    fn module_color<C: Into<Color>>(&mut self, module_color: C) -> &mut Self;
    /// Updates background color (default: #FFFFFF)
    fn background_color<C: Into<Color>>(&mut self, background_color: C) -> &mut Self;
    /// Adds a shape to the shapes list
    fn module_shape(&mut self, shape: ModuleShape) -> &mut Self;
    /// Add a shape to the shapes list with a specific color
    fn module_shape_color<C: Into<Color>>(&mut self, shape: ModuleShape, color: C) -> &mut Self;

    // Manages the image part

    /// Provides the image path or an base64 encoded image
    fn image(&mut self, image: String) -> &mut Self;
    /// Updates the image background color (default: #FFFFFF)
    fn image_background_color<C: Into<Color>>(&mut self, image_background_color: C) -> &mut Self;
    /// Updates the image background shape (default: Square)
    fn image_background_shape(&mut self, image_background_shape: ImageBackgroundShape)
        -> &mut Self;
    /// Updates the image size and the gap between the image and the [`crate::QRCode`]
    /// Default is around 30% of the [`crate::QRCode`] size
    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self;
    /// Updates the image position, anchor is the center of the image. Default is the center of the [`crate::QRCode`]
    fn image_position(&mut self, x: f64, y: f64) -> &mut Self;
}
