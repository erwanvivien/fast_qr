//! Converts [`QRCode`] to an image
//!
//! ```rust
//! use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilde::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _img = ImageBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .fit_width(600)
//!     .to_file(&qrcode, "out.png");
//!
//! #     std::fs::remove_file("out.png");
//! #     Ok(())
//! # }
//! ```

use std::io;

use crate::QRCode;

use super::{svg::SvgBuilder, Builder, Shape};

use resvg::tiny_skia::{self, Pixmap};
use resvg::usvg;

/// Builder for image, refer to [`SvgBuilder`] for more information
pub struct ImageBuilder {
    fit_height: Option<u32>,
    fit_width: Option<u32>,
    svg_builder: SvgBuilder,
}

#[derive(Debug)]
/// Error when converting to svg
pub enum ImageError {
    /// Error while writing to file
    IoError(io::Error),
    /// Error while creating svg
    ImageError(String),
}

/// Creates an ImageBuilder instance, which contains an [`SvgBuilder`]
impl Default for ImageBuilder {
    fn default() -> Self {
        ImageBuilder {
            fit_height: None,
            fit_width: None,
            svg_builder: Default::default(),
        }
    }
}

impl Builder for ImageBuilder {
    fn margin(&mut self, margin: usize) -> &mut Self {
        self.svg_builder.margin(margin);
        self
    }

    fn module_color(&mut self, module_color: [u8; 4]) -> &mut Self {
        self.svg_builder.module_color(module_color);
        self
    }

    fn background_color(&mut self, background_color: [u8; 4]) -> &mut Self {
        self.svg_builder.background_color(background_color);
        self
    }

    fn shape(&mut self, shape: Shape) -> &mut Self {
        self.svg_builder.shape(shape);
        self
    }

    fn image(&mut self, image: String) -> &mut Self {
        self.svg_builder.image(image);
        self
    }

    fn image_background_color(&mut self, image_background_color: [u8; 4]) -> &mut Self {
        self.svg_builder
            .image_background_color(image_background_color);
        self
    }

    fn image_background_shape(
        &mut self,
        image_background_shape: super::ImageBackgroundShape,
    ) -> &mut Self {
        self.svg_builder
            .image_background_shape(image_background_shape);
        self
    }

    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self {
        self.svg_builder.image_size(image_size, gap);
        self
    }

    fn image_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.svg_builder.image_position(x, y);
        self
    }

    fn shape_color(&mut self, shape: Shape, color: [u8; 4]) -> &mut Self {
        self.svg_builder.shape_color(shape, color);
        self
    }
}

impl ImageBuilder {
    /// Add a max-height boundary
    pub fn fit_height(&mut self, height: u32) -> &mut Self {
        self.fit_height = Some(height);
        self
    }

    /// Add a max-width boundary
    pub fn fit_width(&mut self, width: u32) -> &mut Self {
        self.fit_width = Some(width);
        self
    }

    // From https://github.com/RazrFalcon/resvg/blob/master/tests/integration/main.rs
    /// Return a pixmap containing the svg for a QRCode
    pub fn to_pixmap(&self, qr: &QRCode) -> Pixmap {
        let opt = usvg::Options {
            font_family: "Noto Sans".to_string(),
            ..usvg::Options::default()
        };

        // Do not unwrap on the from_data line, because panic will poison GLOBAL_OPT.
        let tree = {
            let svg_data = self.svg_builder.to_str(qr);
            let tree = usvg::Tree::from_data(svg_data.as_bytes(), &opt);
            tree.unwrap()
        };

        let fit_to = match (self.fit_width, self.fit_height) {
            (Some(w), Some(h)) => usvg::FitTo::Size(w, h),
            (Some(w), None) => usvg::FitTo::Width(w),
            (None, Some(h)) => usvg::FitTo::Height(h),
            _ => usvg::FitTo::Original,
        };

        let size = fit_to.fit_to(tree.size.to_screen_size()).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
        resvg::render(
            &tree,
            fit_to,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .unwrap();

        pixmap
    }

    /// Saves the image for a QRCode to a file
    pub fn to_file(&self, qr: &QRCode, file: &str) -> Result<(), ImageError> {
        let out = self.to_pixmap(qr);

        out.save_png(file).unwrap();

        Ok(())
    }
}
