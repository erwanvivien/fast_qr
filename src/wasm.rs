use crate::QRCode;
#[cfg(feature = "svg")]
use crate::{convert, Version, ECL};
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

fn bool_to_u8(qr: QRCode) -> Vec<u8> {
    let dim = qr.size;
    qr.data[..dim * dim]
        .iter()
        .map(|x| u8::from(x.value()))
        .collect()
}

/// Generate a QR code from a string. All parameters are automatically set.
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[must_use]
pub fn qr(content: &str) -> Vec<u8> {
    let qrcode = QRCode::new(content.as_bytes(), None, None, None, None);
    qrcode.map(bool_to_u8).unwrap_or(Vec::new())
}

/// Configuration for the SVG output.
#[cfg(feature = "svg")]
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct SvgOptions {
    shape: convert::Shape,
    module_color: Vec<u8>,
    margin: usize,

    ecl: Option<ECL>,
    version: Option<Version>,

    background_color: Vec<u8>,

    image: String,
    image_background_color: Vec<u8>,
    image_background_shape: convert::ImageBackgroundShape,
    image_size: Option<f64>,
    image_gap: Option<f64>,
    image_position: Vec<f64>,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[cfg(feature = "svg")]
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
    pub fn shape(self, shape: convert::Shape) -> Self {
        Self { shape, ..self }
    }

    /// Updates the module color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
    pub fn module_color(self, module_color: String) -> Self {
        let code = Self::color_to_code(module_color);
        if code.len() != 4 {
            return self;
        }
        Self {
            module_color: code,
            ..self
        }
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

    /// Updates the shape of the image background. Takes an convert::ImageBackgroundShape.
    pub fn image_background_shape(
        self,
        image_background_shape: convert::ImageBackgroundShape,
    ) -> Self {
        Self {
            image_background_shape,
            ..self
        }
    }

    /// Updates the size of the image. (unit being module size).
    pub fn image_size(self, size: f64) -> Self {
        Self {
            image_size: Some(size),
            ..self
        }
    }

    /// Updates the gap between background color and the image. (unit being module size).
    pub fn image_gap(self, gap: f64) -> Self {
        Self {
            image_gap: Some(gap),
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

    /// Updates the error correction level of the QRCode (can increase the size of the QRCode)
    pub fn ecl(self, ecl: ECL) -> Self {
        Self {
            ecl: Some(ecl),
            ..self
        }
    }

    /// Forces the version of the QRCode
    pub fn version(self, version: Version) -> Self {
        Self {
            version: Some(version),
            ..self
        }
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[cfg(feature = "svg")]
impl SvgOptions {
    /// Creates a new SvgOptions object.
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            shape: convert::Shape::Square,
            module_color: vec![0, 0, 0, 255],
            margin: 4,

            ecl: None,
            version: None,

            background_color: vec![255, 255, 255, 255],

            image: String::new(),
            image_background_color: vec![255, 255, 255, 255],
            image_background_shape: convert::ImageBackgroundShape::Square,
            image_size: None,
            image_gap: None,
            image_position: vec![],
        }
    }
}

/// Generate a QR code from a string. All parameters are automatically set.
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[cfg(feature = "svg")]
pub fn qr_svg(content: &str, options: SvgOptions) -> String {
    use crate::convert::svg::SvgBuilder;
    use crate::convert::Builder;
    let qrcode = QRCode::new(content.as_bytes(), options.ecl, options.version, None, None);

    let mut builder = SvgBuilder::default();
    builder.shape(options.shape);
    builder.margin(options.margin);
    builder.background_color(options.background_color);
    builder.module_color(options.module_color);
    if !options.image.is_empty() {
        builder.image(options.image);
    }

    builder.image_background_color(options.image_background_color);
    builder.image_background_shape(options.image_background_shape);

    if let Some(image_size) = options.image_size {
        builder.image_size(image_size);
    }

    if let Some(image_gap) = options.image_gap {
        builder.image_gap(image_gap);
    }

    if options.image_position.len() == 2 {
        let x = options.image_position[0];
        let y = options.image_position[1];
        builder.image_position(x, y);
    }

    qrcode
        .map(|qrcode| builder.to_str(&qrcode))
        .unwrap_or(String::new())
}
