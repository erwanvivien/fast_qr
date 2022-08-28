#![warn(missing_docs)]
//! # Example
//! ```rust
//! use fast_qr::{ECL, Version, QRBuilder};
//!
//! let qrcode = QRBuilder::new("https://example.com/".into())
//!     .ecl(ECL::H)
//!     .version(Version::V03)
//!     .build();
//! // It is preferable to check qrcode result before
//! qrcode.unwrap().print();
//! ```
//!
//! <svg viewBox='0 0 37 37' xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="148px">
//!  <path d='M4,4h1v1h-1M5,4h1v1h-1M6,4h1v1h-1M7,4h1v1h-1M8,4h1v1h-1M9,4h1v1h-1M10,4h1v1h-1M12,4h1v1h-1M13,4h1v1h-1M17,4h1v1h-1M19,4h1v1h-1M22,4h1v1h-1M24,4h1v1h-1M26,4h1v1h-1M27,4h1v1h-1M28,4h1v1h-1M29,4h1v1h-1M30,4h1v1h-1M31,4h1v1h-1M32,4h1v1h-1M4,5h1v1h-1M10,5h1v1h-1M12,5h1v1h-1M13,5h1v1h-1M14,5h1v1h-1M17,5h1v1h-1M19,5h1v1h-1M22,5h1v1h-1M23,5h1v1h-1M26,5h1v1h-1M32,5h1v1h-1M4,6h1v1h-1M6,6h1v1h-1M7,6h1v1h-1M8,6h1v1h-1M10,6h1v1h-1M12,6h1v1h-1M14,6h1v1h-1M16,6h1v1h-1M18,6h1v1h-1M19,6h1v1h-1M23,6h1v1h-1M24,6h1v1h-1M26,6h1v1h-1M28,6h1v1h-1M29,6h1v1h-1M30,6h1v1h-1M32,6h1v1h-1M4,7h1v1h-1M6,7h1v1h-1M7,7h1v1h-1M8,7h1v1h-1M10,7h1v1h-1M13,7h1v1h-1M15,7h1v1h-1M18,7h1v1h-1M21,7h1v1h-1M23,7h1v1h-1M26,7h1v1h-1M28,7h1v1h-1M29,7h1v1h-1M30,7h1v1h-1M32,7h1v1h-1M4,8h1v1h-1M6,8h1v1h-1M7,8h1v1h-1M8,8h1v1h-1M10,8h1v1h-1M16,8h1v1h-1M17,8h1v1h-1M18,8h1v1h-1M19,8h1v1h-1M20,8h1v1h-1M22,8h1v1h-1M23,8h1v1h-1M24,8h1v1h-1M26,8h1v1h-1M28,8h1v1h-1M29,8h1v1h-1M30,8h1v1h-1M32,8h1v1h-1M4,9h1v1h-1M10,9h1v1h-1M12,9h1v1h-1M13,9h1v1h-1M14,9h1v1h-1M15,9h1v1h-1M16,9h1v1h-1M19,9h1v1h-1M22,9h1v1h-1M26,9h1v1h-1M32,9h1v1h-1M4,10h1v1h-1M5,10h1v1h-1M6,10h1v1h-1M7,10h1v1h-1M8,10h1v1h-1M9,10h1v1h-1M10,10h1v1h-1M12,10h1v1h-1M13,10h1v1h-1M14,10h1v1h-1M15,10h1v1h-1M16,10h1v1h-1M17,10h1v1h-1M18,10h1v1h-1M19,10h1v1h-1M20,10h1v1h-1M21,10h1v1h-1M22,10h1v1h-1M23,10h1v1h-1M24,10h1v1h-1M25,10h1v1h-1M26,10h1v1h-1M27,10h1v1h-1M28,10h1v1h-1M29,10h1v1h-1M30,10h1v1h-1M31,10h1v1h-1M32,10h1v1h-1M12,11h1v1h-1M13,11h1v1h-1M15,11h1v1h-1M16,11h1v1h-1M17,11h1v1h-1M18,11h1v1h-1M19,11h1v1h-1M6,12h1v1h-1M7,12h1v1h-1M8,12h1v1h-1M10,12h1v1h-1M12,12h1v1h-1M20,12h1v1h-1M22,12h1v1h-1M23,12h1v1h-1M24,12h1v1h-1M25,12h1v1h-1M26,12h1v1h-1M27,12h1v1h-1M30,12h1v1h-1M31,12h1v1h-1M32,12h1v1h-1M9,13h1v1h-1M10,13h1v1h-1M11,13h1v1h-1M12,13h1v1h-1M13,13h1v1h-1M14,13h1v1h-1M15,13h1v1h-1M16,13h1v1h-1M18,13h1v1h-1M20,13h1v1h-1M25,13h1v1h-1M26,13h1v1h-1M27,13h1v1h-1M28,13h1v1h-1M29,13h1v1h-1M30,13h1v1h-1M32,13h1v1h-1M4,14h1v1h-1M6,14h1v1h-1M7,14h1v1h-1M9,14h1v1h-1M10,14h1v1h-1M12,14h1v1h-1M13,14h1v1h-1M14,14h1v1h-1M15,14h1v1h-1M16,14h1v1h-1M17,14h1v1h-1M18,14h1v1h-1M19,14h1v1h-1M20,14h1v1h-1M22,14h1v1h-1M24,14h1v1h-1M25,14h1v1h-1M26,14h1v1h-1M27,14h1v1h-1M4,15h1v1h-1M6,15h1v1h-1M8,15h1v1h-1M9,15h1v1h-1M10,15h1v1h-1M11,15h1v1h-1M12,15h1v1h-1M13,15h1v1h-1M15,15h1v1h-1M16,15h1v1h-1M18,15h1v1h-1M20,15h1v1h-1M21,15h1v1h-1M22,15h1v1h-1M25,15h1v1h-1M26,15h1v1h-1M27,15h1v1h-1M29,15h1v1h-1M31,15h1v1h-1M5,16h1v1h-1M7,16h1v1h-1M9,16h1v1h-1M10,16h1v1h-1M11,16h1v1h-1M12,16h1v1h-1M14,16h1v1h-1M17,16h1v1h-1M24,16h1v1h-1M25,16h1v1h-1M27,16h1v1h-1M30,16h1v1h-1M31,16h1v1h-1M32,16h1v1h-1M5,17h1v1h-1M6,17h1v1h-1M8,17h1v1h-1M9,17h1v1h-1M10,17h1v1h-1M12,17h1v1h-1M16,17h1v1h-1M18,17h1v1h-1M20,17h1v1h-1M23,17h1v1h-1M24,17h1v1h-1M25,17h1v1h-1M26,17h1v1h-1M28,17h1v1h-1M29,17h1v1h-1M31,17h1v1h-1M32,17h1v1h-1M4,18h1v1h-1M5,18h1v1h-1M7,18h1v1h-1M9,18h1v1h-1M10,18h1v1h-1M12,18h1v1h-1M13,18h1v1h-1M14,18h1v1h-1M16,18h1v1h-1M19,18h1v1h-1M20,18h1v1h-1M22,18h1v1h-1M24,18h1v1h-1M26,18h1v1h-1M27,18h1v1h-1M4,19h1v1h-1M6,19h1v1h-1M7,19h1v1h-1M8,19h1v1h-1M10,19h1v1h-1M12,19h1v1h-1M13,19h1v1h-1M16,19h1v1h-1M21,19h1v1h-1M22,19h1v1h-1M24,19h1v1h-1M28,19h1v1h-1M29,19h1v1h-1M31,19h1v1h-1M5,20h1v1h-1M6,20h1v1h-1M8,20h1v1h-1M9,20h1v1h-1M10,20h1v1h-1M13,20h1v1h-1M14,20h1v1h-1M16,20h1v1h-1M19,20h1v1h-1M20,20h1v1h-1M25,20h1v1h-1M29,20h1v1h-1M30,20h1v1h-1M31,20h1v1h-1M4,21h1v1h-1M6,21h1v1h-1M7,21h1v1h-1M8,21h1v1h-1M10,21h1v1h-1M12,21h1v1h-1M14,21h1v1h-1M16,21h1v1h-1M17,21h1v1h-1M19,21h1v1h-1M20,21h1v1h-1M24,21h1v1h-1M25,21h1v1h-1M26,21h1v1h-1M27,21h1v1h-1M28,21h1v1h-1M29,21h1v1h-1M31,21h1v1h-1M32,21h1v1h-1M4,22h1v1h-1M7,22h1v1h-1M8,22h1v1h-1M10,22h1v1h-1M13,22h1v1h-1M15,22h1v1h-1M17,22h1v1h-1M19,22h1v1h-1M20,22h1v1h-1M21,22h1v1h-1M23,22h1v1h-1M26,22h1v1h-1M27,22h1v1h-1M29,22h1v1h-1M4,23h1v1h-1M6,23h1v1h-1M9,23h1v1h-1M10,23h1v1h-1M11,23h1v1h-1M13,23h1v1h-1M14,23h1v1h-1M15,23h1v1h-1M16,23h1v1h-1M19,23h1v1h-1M20,23h1v1h-1M21,23h1v1h-1M23,23h1v1h-1M24,23h1v1h-1M26,23h1v1h-1M28,23h1v1h-1M31,23h1v1h-1M4,24h1v1h-1M6,24h1v1h-1M7,24h1v1h-1M9,24h1v1h-1M10,24h1v1h-1M12,24h1v1h-1M14,24h1v1h-1M15,24h1v1h-1M16,24h1v1h-1M17,24h1v1h-1M18,24h1v1h-1M19,24h1v1h-1M20,24h1v1h-1M22,24h1v1h-1M23,24h1v1h-1M24,24h1v1h-1M25,24h1v1h-1M26,24h1v1h-1M27,24h1v1h-1M28,24h1v1h-1M30,24h1v1h-1M10,25h1v1h-1M12,25h1v1h-1M16,25h1v1h-1M18,25h1v1h-1M20,25h1v1h-1M21,25h1v1h-1M22,25h1v1h-1M24,25h1v1h-1M28,25h1v1h-1M29,25h1v1h-1M32,25h1v1h-1M4,26h1v1h-1M5,26h1v1h-1M6,26h1v1h-1M7,26h1v1h-1M8,26h1v1h-1M9,26h1v1h-1M10,26h1v1h-1M14,26h1v1h-1M16,26h1v1h-1M17,26h1v1h-1M18,26h1v1h-1M19,26h1v1h-1M21,26h1v1h-1M22,26h1v1h-1M23,26h1v1h-1M24,26h1v1h-1M26,26h1v1h-1M28,26h1v1h-1M4,27h1v1h-1M10,27h1v1h-1M13,27h1v1h-1M14,27h1v1h-1M15,27h1v1h-1M16,27h1v1h-1M17,27h1v1h-1M19,27h1v1h-1M20,27h1v1h-1M22,27h1v1h-1M23,27h1v1h-1M24,27h1v1h-1M28,27h1v1h-1M29,27h1v1h-1M4,28h1v1h-1M6,28h1v1h-1M7,28h1v1h-1M8,28h1v1h-1M10,28h1v1h-1M12,28h1v1h-1M13,28h1v1h-1M16,28h1v1h-1M20,28h1v1h-1M21,28h1v1h-1M22,28h1v1h-1M24,28h1v1h-1M25,28h1v1h-1M26,28h1v1h-1M27,28h1v1h-1M28,28h1v1h-1M29,28h1v1h-1M30,28h1v1h-1M32,28h1v1h-1M4,29h1v1h-1M6,29h1v1h-1M7,29h1v1h-1M8,29h1v1h-1M10,29h1v1h-1M12,29h1v1h-1M13,29h1v1h-1M15,29h1v1h-1M16,29h1v1h-1M17,29h1v1h-1M18,29h1v1h-1M22,29h1v1h-1M23,29h1v1h-1M24,29h1v1h-1M25,29h1v1h-1M27,29h1v1h-1M29,29h1v1h-1M30,29h1v1h-1M4,30h1v1h-1M6,30h1v1h-1M7,30h1v1h-1M8,30h1v1h-1M10,30h1v1h-1M12,30h1v1h-1M13,30h1v1h-1M14,30h1v1h-1M16,30h1v1h-1M18,30h1v1h-1M20,30h1v1h-1M21,30h1v1h-1M22,30h1v1h-1M23,30h1v1h-1M24,30h1v1h-1M25,30h1v1h-1M26,30h1v1h-1M27,30h1v1h-1M28,30h1v1h-1M30,30h1v1h-1M31,30h1v1h-1M4,31h1v1h-1M10,31h1v1h-1M13,31h1v1h-1M18,31h1v1h-1M19,31h1v1h-1M20,31h1v1h-1M21,31h1v1h-1M26,31h1v1h-1M28,31h1v1h-1M29,31h1v1h-1M31,31h1v1h-1M4,32h1v1h-1M5,32h1v1h-1M6,32h1v1h-1M7,32h1v1h-1M8,32h1v1h-1M9,32h1v1h-1M10,32h1v1h-1M14,32h1v1h-1M15,32h1v1h-1M16,32h1v1h-1M17,32h1v1h-1M18,32h1v1h-1M19,32h1v1h-1M22,32h1v1h-1M26,32h1v1h-1M28,32h1v1h-1M30,32h1v1h-1' />
//! </svg>
//!
//! # Example SVG
//! ```rust
//! use fast_qr::{ECL, Version, QRBuilder};
//! use fast_qr::convert::svg::{SvgBuilder, Shape};
//!
//! let qrcode = QRBuilder::new("https://example.com/".into())
//!     .ecl(ECL::H)
//!     .version(Version::V03)
//!     .build();
//!
//! let svg = SvgBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .to_str(&qrcode.unwrap());
//!
//! println!("{}", svg);
//! ```

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub use crate::datamasking::Mask;
pub use crate::ecl::ECL;
use crate::qr::QRCodeError;
pub use crate::qr::{QRBuilder, QRCode};
pub use crate::version::Version;

mod compact;
mod datamasking;

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
