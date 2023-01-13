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

#[derive(PartialEq, Eq, Ord, PartialOrd)]
/// Different possible Shapes
pub enum Shape {
    /// Square Shape
    Square,
    /// Circle Shape
    Circle,
    /// RoundedSquare Shape
    RoundedSquare,
    /// Vertical Shape
    Vertical,
    /// Horizontal Shape
    Horizontal,
    /// Diamond Shape
    Diamond,
}

/// Contains possible errors for a convertion
#[derive(Debug)]
pub enum ConvertError {
    /// Contains error message for a SVG convertion
    #[cfg(feature = "svg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
    Svg(String),
    /// Contains error message for an Image convertion
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
            SvgError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
impl From<ImageError> for ConvertError {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::ImageError(image_err) => Self::Image(image_err),
            ImageError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

/// Converts an array of pixel color to it's hexadecimal representation
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

/// Trait for `SvgBuilder` and `ImageBuilder`
pub trait Builder {
    /// Updates the margin of the builder
    fn margin(&mut self, margin: usize) -> &mut Self;
    /// Updates the module color of the builder
    fn module_color(&mut self, module_color: [u8; 4]) -> &mut Self;
    /// Updates the background color of the builder
    fn background_color(&mut self, background_color: [u8; 4]) -> &mut Self;
    /// Updates the shape of the builder
    fn shape(&mut self, shape: Shape) -> &mut Self;
}
