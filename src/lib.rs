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

pub use crate::datamasking::Mask;
pub use crate::ecl::ECL;
pub use crate::encode::Mode;
pub use crate::module::{Module, ModuleType};
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
#[cfg(not(feature = "wasm-bindgen"))]
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
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
