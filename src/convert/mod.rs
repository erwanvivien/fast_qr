//! Converts a [`crate::QRCode`] to image or SVG you will need to activate associated feature flag

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;
use core::ops::Deref;

#[cfg(feature = "svg")]
use svg::SvgError;

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "image")]
use image::ImageError;

use crate::Module;

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
pub type ModuleFunction = fn(usize, usize, Module) -> String;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// TODO: Find a way to use the same enum for wasm and not wasm
// Current bug being that wasm_bindgen & #[cfg(not(target_arch = "wasm32"))] are not compatible(?)
/// Different possible Shapes to represent modules in a QRCode
#[cfg(target_arch = "wasm32")]
#[repr(C)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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

/// Different possible Shapes to represent modules in a QRCode
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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
    /// Custom Shape with a function / closure
    /// # Example
    /// ```rust
    /// use fast_qr::convert::Shape;
    /// let command_function = |y, x, cell| {
    ///     if x % 2 == 0 {
    ///         // Works thanks to Deref
    ///         Shape::Square(y, x, cell)
    ///     } else {
    ///         // Rectangle
    ///         format!("M{},{}h1v.5h-1", x, y)
    ///     }
    /// };
    /// let command = Shape::Command(command_function);
    /// ```
    ///
    /// <svg viewBox="0 0 37 37" xmlns="http://www.w3.org/2000/svg" width="250px">
    ///     <rect width="37px" height="37px" fill="#ffffff" />
    ///     <path
    ///         d="M4,4h1v1h-1M4,5h1v1h-1M4,6h1v1h-1M4,7h1v1h-1M4,8h1v1h-1M4,9h1v1h-1M4,10h1v1h-1M4,12h1v1h-1M4,13h1v1h-1M4,17h1v1h-1M4,19h1v1h-1M4,22h1v1h-1M4,24h1v1h-1M4,26h1v1h-1M4,27h1v1h-1M4,28h1v1h-1M4,29h1v1h-1M4,30h1v1h-1M4,31h1v1h-1M4,32h1v1h-1M5,4h1v.5h-1M5,10h1v.5h-1M5,12h1v.5h-1M5,13h1v.5h-1M5,14h1v.5h-1M5,17h1v.5h-1M5,19h1v.5h-1M5,22h1v.5h-1M5,23h1v.5h-1M5,26h1v.5h-1M5,32h1v.5h-1M6,4h1v1h-1M6,6h1v1h-1M6,7h1v1h-1M6,8h1v1h-1M6,10h1v1h-1M6,12h1v1h-1M6,14h1v1h-1M6,16h1v1h-1M6,18h1v1h-1M6,19h1v1h-1M6,23h1v1h-1M6,24h1v1h-1M6,26h1v1h-1M6,28h1v1h-1M6,29h1v1h-1M6,30h1v1h-1M6,32h1v1h-1M7,4h1v.5h-1M7,6h1v.5h-1M7,7h1v.5h-1M7,8h1v.5h-1M7,10h1v.5h-1M7,13h1v.5h-1M7,15h1v.5h-1M7,18h1v.5h-1M7,21h1v.5h-1M7,23h1v.5h-1M7,26h1v.5h-1M7,28h1v.5h-1M7,29h1v.5h-1M7,30h1v.5h-1M7,32h1v.5h-1M8,4h1v1h-1M8,6h1v1h-1M8,7h1v1h-1M8,8h1v1h-1M8,10h1v1h-1M8,16h1v1h-1M8,17h1v1h-1M8,18h1v1h-1M8,19h1v1h-1M8,20h1v1h-1M8,22h1v1h-1M8,23h1v1h-1M8,24h1v1h-1M8,26h1v1h-1M8,28h1v1h-1M8,29h1v1h-1M8,30h1v1h-1M8,32h1v1h-1M9,4h1v.5h-1M9,10h1v.5h-1M9,12h1v.5h-1M9,13h1v.5h-1M9,14h1v.5h-1M9,15h1v.5h-1M9,16h1v.5h-1M9,19h1v.5h-1M9,22h1v.5h-1M9,26h1v.5h-1M9,32h1v.5h-1M10,4h1v1h-1M10,5h1v1h-1M10,6h1v1h-1M10,7h1v1h-1M10,8h1v1h-1M10,9h1v1h-1M10,10h1v1h-1M10,12h1v1h-1M10,14h1v1h-1M10,16h1v1h-1M10,18h1v1h-1M10,20h1v1h-1M10,22h1v1h-1M10,24h1v1h-1M10,26h1v1h-1M10,27h1v1h-1M10,28h1v1h-1M10,29h1v1h-1M10,30h1v1h-1M10,31h1v1h-1M10,32h1v1h-1M11,12h1v.5h-1M11,13h1v.5h-1M11,15h1v.5h-1M11,16h1v.5h-1M11,17h1v.5h-1M11,18h1v.5h-1M11,19h1v.5h-1M12,6h1v1h-1M12,7h1v1h-1M12,8h1v1h-1M12,10h1v1h-1M12,12h1v1h-1M12,20h1v1h-1M12,22h1v1h-1M12,23h1v1h-1M12,24h1v1h-1M12,25h1v1h-1M12,26h1v1h-1M12,27h1v1h-1M12,30h1v1h-1M12,31h1v1h-1M12,32h1v1h-1M13,9h1v.5h-1M13,11h1v.5h-1M13,12h1v.5h-1M13,13h1v.5h-1M13,14h1v.5h-1M13,15h1v.5h-1M13,16h1v.5h-1M13,18h1v.5h-1M13,20h1v.5h-1M13,25h1v.5h-1M13,26h1v.5h-1M13,27h1v.5h-1M13,28h1v.5h-1M13,29h1v.5h-1M13,30h1v.5h-1M13,32h1v.5h-1M14,4h1v1h-1M14,6h1v1h-1M14,7h1v1h-1M14,9h1v1h-1M14,10h1v1h-1M14,12h1v1h-1M14,13h1v1h-1M14,14h1v1h-1M14,15h1v1h-1M14,16h1v1h-1M14,17h1v1h-1M14,18h1v1h-1M14,19h1v1h-1M14,20h1v1h-1M14,22h1v1h-1M14,24h1v1h-1M14,25h1v1h-1M14,26h1v1h-1M14,27h1v1h-1M15,4h1v.5h-1M15,6h1v.5h-1M15,8h1v.5h-1M15,9h1v.5h-1M15,11h1v.5h-1M15,12h1v.5h-1M15,13h1v.5h-1M15,15h1v.5h-1M15,16h1v.5h-1M15,18h1v.5h-1M15,20h1v.5h-1M15,21h1v.5h-1M15,22h1v.5h-1M15,25h1v.5h-1M15,26h1v.5h-1M15,27h1v.5h-1M15,29h1v.5h-1M15,31h1v.5h-1M16,5h1v1h-1M16,7h1v1h-1M16,9h1v1h-1M16,10h1v1h-1M16,11h1v1h-1M16,12h1v1h-1M16,14h1v1h-1M16,17h1v1h-1M16,24h1v1h-1M16,25h1v1h-1M16,27h1v1h-1M16,30h1v1h-1M16,31h1v1h-1M16,32h1v1h-1M17,5h1v.5h-1M17,6h1v.5h-1M17,8h1v.5h-1M17,9h1v.5h-1M17,12h1v.5h-1M17,16h1v.5h-1M17,18h1v.5h-1M17,20h1v.5h-1M17,23h1v.5h-1M17,24h1v.5h-1M17,25h1v.5h-1M17,26h1v.5h-1M17,28h1v.5h-1M17,29h1v.5h-1M17,31h1v.5h-1M17,32h1v.5h-1M18,4h1v1h-1M18,5h1v1h-1M18,7h1v1h-1M18,9h1v1h-1M18,10h1v1h-1M18,12h1v1h-1M18,13h1v1h-1M18,14h1v1h-1M18,16h1v1h-1M18,19h1v1h-1M18,20h1v1h-1M18,22h1v1h-1M18,24h1v1h-1M18,26h1v1h-1M18,27h1v1h-1M19,4h1v.5h-1M19,6h1v.5h-1M19,7h1v.5h-1M19,8h1v.5h-1M19,12h1v.5h-1M19,13h1v.5h-1M19,16h1v.5h-1M19,21h1v.5h-1M19,22h1v.5h-1M19,24h1v.5h-1M19,28h1v.5h-1M19,29h1v.5h-1M19,31h1v.5h-1M20,5h1v1h-1M20,6h1v1h-1M20,8h1v1h-1M20,9h1v1h-1M20,10h1v1h-1M20,13h1v1h-1M20,14h1v1h-1M20,16h1v1h-1M20,19h1v1h-1M20,20h1v1h-1M20,25h1v1h-1M20,29h1v1h-1M20,30h1v1h-1M20,31h1v1h-1M21,4h1v.5h-1M21,6h1v.5h-1M21,7h1v.5h-1M21,8h1v.5h-1M21,12h1v.5h-1M21,14h1v.5h-1M21,16h1v.5h-1M21,17h1v.5h-1M21,19h1v.5h-1M21,20h1v.5h-1M21,24h1v.5h-1M21,25h1v.5h-1M21,26h1v.5h-1M21,27h1v.5h-1M21,28h1v.5h-1M21,29h1v.5h-1M21,31h1v.5h-1M21,32h1v.5h-1M22,4h1v1h-1M22,7h1v1h-1M22,8h1v1h-1M22,10h1v1h-1M22,13h1v1h-1M22,15h1v1h-1M22,17h1v1h-1M22,19h1v1h-1M22,20h1v1h-1M22,21h1v1h-1M22,23h1v1h-1M22,26h1v1h-1M22,27h1v1h-1M22,29h1v1h-1M23,4h1v.5h-1M23,6h1v.5h-1M23,9h1v.5h-1M23,11h1v.5h-1M23,13h1v.5h-1M23,14h1v.5h-1M23,15h1v.5h-1M23,16h1v.5h-1M23,19h1v.5h-1M23,20h1v.5h-1M23,21h1v.5h-1M23,23h1v.5h-1M23,24h1v.5h-1M23,26h1v.5h-1M23,28h1v.5h-1M23,31h1v.5h-1M24,4h1v1h-1M24,6h1v1h-1M24,7h1v1h-1M24,9h1v1h-1M24,10h1v1h-1M24,12h1v1h-1M24,14h1v1h-1M24,15h1v1h-1M24,16h1v1h-1M24,17h1v1h-1M24,18h1v1h-1M24,19h1v1h-1M24,20h1v1h-1M24,22h1v1h-1M24,23h1v1h-1M24,24h1v1h-1M24,25h1v1h-1M24,26h1v1h-1M24,27h1v1h-1M24,28h1v1h-1M24,30h1v1h-1M25,12h1v.5h-1M25,16h1v.5h-1M25,18h1v.5h-1M25,20h1v.5h-1M25,21h1v.5h-1M25,22h1v.5h-1M25,24h1v.5h-1M25,28h1v.5h-1M25,29h1v.5h-1M25,32h1v.5h-1M26,4h1v1h-1M26,5h1v1h-1M26,6h1v1h-1M26,7h1v1h-1M26,8h1v1h-1M26,9h1v1h-1M26,10h1v1h-1M26,14h1v1h-1M26,16h1v1h-1M26,17h1v1h-1M26,18h1v1h-1M26,19h1v1h-1M26,21h1v1h-1M26,22h1v1h-1M26,23h1v1h-1M26,24h1v1h-1M26,26h1v1h-1M26,28h1v1h-1M27,4h1v.5h-1M27,10h1v.5h-1M27,13h1v.5h-1M27,14h1v.5h-1M27,15h1v.5h-1M27,16h1v.5h-1M27,17h1v.5h-1M27,19h1v.5h-1M27,20h1v.5h-1M27,22h1v.5h-1M27,23h1v.5h-1M27,24h1v.5h-1M27,28h1v.5h-1M27,29h1v.5h-1M28,4h1v1h-1M28,6h1v1h-1M28,7h1v1h-1M28,8h1v1h-1M28,10h1v1h-1M28,12h1v1h-1M28,13h1v1h-1M28,16h1v1h-1M28,20h1v1h-1M28,21h1v1h-1M28,22h1v1h-1M28,24h1v1h-1M28,25h1v1h-1M28,26h1v1h-1M28,27h1v1h-1M28,28h1v1h-1M28,29h1v1h-1M28,30h1v1h-1M28,32h1v1h-1M29,4h1v.5h-1M29,6h1v.5h-1M29,7h1v.5h-1M29,8h1v.5h-1M29,10h1v.5h-1M29,12h1v.5h-1M29,13h1v.5h-1M29,15h1v.5h-1M29,16h1v.5h-1M29,17h1v.5h-1M29,18h1v.5h-1M29,22h1v.5h-1M29,23h1v.5h-1M29,24h1v.5h-1M29,25h1v.5h-1M29,27h1v.5h-1M29,29h1v.5h-1M29,30h1v.5h-1M30,4h1v1h-1M30,6h1v1h-1M30,7h1v1h-1M30,8h1v1h-1M30,10h1v1h-1M30,12h1v1h-1M30,13h1v1h-1M30,14h1v1h-1M30,16h1v1h-1M30,18h1v1h-1M30,20h1v1h-1M30,21h1v1h-1M30,22h1v1h-1M30,23h1v1h-1M30,24h1v1h-1M30,25h1v1h-1M30,26h1v1h-1M30,27h1v1h-1M30,28h1v1h-1M30,30h1v1h-1M30,31h1v1h-1M31,4h1v.5h-1M31,10h1v.5h-1M31,13h1v.5h-1M31,18h1v.5h-1M31,19h1v.5h-1M31,20h1v.5h-1M31,21h1v.5h-1M31,26h1v.5h-1M31,28h1v.5h-1M31,29h1v.5h-1M31,31h1v.5h-1M32,4h1v1h-1M32,5h1v1h-1M32,6h1v1h-1M32,7h1v1h-1M32,8h1v1h-1M32,9h1v1h-1M32,10h1v1h-1M32,14h1v1h-1M32,15h1v1h-1M32,16h1v1h-1M32,17h1v1h-1M32,18h1v1h-1M32,19h1v1h-1M32,22h1v1h-1M32,26h1v1h-1M32,28h1v1h-1M32,30h1v1h-1"
    ///         fill="#000000" />
    /// </svg>
    Command(ModuleFunction),
}
impl From<Shape> for usize {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Square => 0,
            Shape::Circle => 1,
            Shape::RoundedSquare => 2,
            Shape::Vertical => 3,
            Shape::Horizontal => 4,
            Shape::Diamond => 5,
            #[cfg(not(target_arch = "wasm32"))]
            Shape::Command(_) => 6,
        }
    }
}

impl From<String> for Shape {
    fn from(shape: String) -> Self {
        match shape.to_lowercase().as_str() {
            "square" => Shape::Square,
            "circle" => Shape::Circle,
            "rounded_square" => Shape::RoundedSquare,
            "vertical" => Shape::Vertical,
            "horizontal" => Shape::Horizontal,
            "diamond" => Shape::Diamond,

            _ => Shape::Square,
        }
    }
}

impl From<Shape> for &str {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Square => "square",
            Shape::Circle => "circle",
            Shape::RoundedSquare => "rounded_square",
            Shape::Vertical => "vertical",
            Shape::Horizontal => "horizontal",
            Shape::Diamond => "diamond",
            #[cfg(not(target_arch = "wasm32"))]
            Shape::Command(_) => "command",
        }
    }
}

impl Shape {
    pub(crate) fn square(y: usize, x: usize, _: Module) -> String {
        format!("M{},{}h1v1h-1", x, y)
    }

    pub(crate) fn circle(y: usize, x: usize, _: Module) -> String {
        format!("M{},{}a.5,.5 0 1,1 0,-.1", x + 1, y as f32 + 0.5f32)
    }

    pub(crate) fn rounded_square(y: usize, x: usize, _: Module) -> String {
        format!("M{0}.2,{1}.2 {0}.8,{1}.2 {0}.8,{1}.8 {0}.2,{1}.8z", x, y)
    }

    pub(crate) fn horizontal(y: usize, x: usize, _: Module) -> String {
        format!("M{},{}.1h1v.8h-1", x, y)
    }

    pub(crate) fn vertical(y: usize, x: usize, _: Module) -> String {
        format!("M{}.1,{}h.8v1h-.8", x, y)
    }

    pub(crate) fn diamond(y: usize, x: usize, _: Module) -> String {
        format!("M{}.5,{}l.5,.5l-.5,.5l-.5,-.5z", x, y)
    }

    const FUNCTIONS: [ModuleFunction; 6] = [
        Shape::square,
        Shape::circle,
        Shape::rounded_square,
        Shape::vertical,
        Shape::horizontal,
        Shape::diamond,
    ];
}

impl Deref for Shape {
    type Target = ModuleFunction;

    fn deref(&self) -> &Self::Target {
        let index: usize = (*self).into();
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Command(func) => func,
            _ => &Self::FUNCTIONS[index],
        }
    }
}

/// Different possible image background shapes
#[cfg_attr(target_arch = "wasm32", repr(C), wasm_bindgen)]
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
            #[cfg(not(target_arch = "wasm32"))]
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
    fn shape(&mut self, shape: Shape) -> &mut Self;
    /// Add a shape to the shapes list with a specific color
    fn shape_color<C: Into<Color>>(&mut self, shape: Shape, color: C) -> &mut Self;

    // Manages the image part

    /// Provides the image path or an base64 encoded image
    fn image(&mut self, image: String) -> &mut Self;
    /// Updates the image background color (default: #FFFFFF)
    fn image_background_color<C: Into<Color>>(&mut self, image_background_color: C) -> &mut Self;
    /// Updates the image background shape (default: Square)
    fn image_background_shape(&mut self, image_background_shape: ImageBackgroundShape)
        -> &mut Self;
    /// Updates the image size and the gap between the image and the QRCode
    /// Default is around 30% of the QRCode size
    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self;
    /// Updates the image position, anchor is the center of the image. Default is the center of the QRCode
    fn image_position(&mut self, x: f64, y: f64) -> &mut Self;
}
