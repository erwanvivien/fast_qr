#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
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

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use crate::datamasking::Mask;
pub use crate::ecl::ECL;
#[cfg(target_arch = "wasm32")]
use crate::qr::QRCodeError;
pub use crate::qr::{QRBuilder, QRCode};
pub use crate::version::Version;

mod compact;
#[doc(hidden)]
pub mod datamasking;

#[cfg(not(target_arch = "wasm32"))]
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
        .map(|x| u8::from(x.value()))
        .collect()
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
#[must_use]
/// Generate a QR code from a string. All parameters are automatically set.
pub fn qr(content: &str) -> Vec<u8> {
    let qrcode = QRCode::new(content.as_bytes(), None, None, None);
    qr_generate(qrcode)
}
