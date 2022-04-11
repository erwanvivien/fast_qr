//! # Example
//! ```rust
//! use fast_qr::{ECL, Version};
//!
//! const MASK: Option<usize> = None;
//! const VERSION: Option<Version> = None;
//! const LEVEL: Option<ECL> = Some(ECL::H);
//!
//! let qrcode = QRCode::new(b"https://example.com/", LEVEL, VERSION, MASK)?;
//! qrcode.print();
//! ```
//!
//!
//! <svg viewBox='0 0 37 37' xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="148px">
//!  <path d='M4,4h1v1h-1M5,4h1v1h-1M6,4h1v1h-1M7,4h1v1h-1M8,4h1v1h-1M9,4h1v1h-1M10,4h1v1h-1M12,4h1v1h-1M13,4h1v1h-1M17,4h1v1h-1M19,4h1v1h-1M22,4h1v1h-1M24,4h1v1h-1M26,4h1v1h-1M27,4h1v1h-1M28,4h1v1h-1M29,4h1v1h-1M30,4h1v1h-1M31,4h1v1h-1M32,4h1v1h-1M4,5h1v1h-1M10,5h1v1h-1M12,5h1v1h-1M13,5h1v1h-1M14,5h1v1h-1M17,5h1v1h-1M19,5h1v1h-1M22,5h1v1h-1M23,5h1v1h-1M26,5h1v1h-1M32,5h1v1h-1M4,6h1v1h-1M6,6h1v1h-1M7,6h1v1h-1M8,6h1v1h-1M10,6h1v1h-1M12,6h1v1h-1M14,6h1v1h-1M16,6h1v1h-1M18,6h1v1h-1M19,6h1v1h-1M23,6h1v1h-1M24,6h1v1h-1M26,6h1v1h-1M28,6h1v1h-1M29,6h1v1h-1M30,6h1v1h-1M32,6h1v1h-1M4,7h1v1h-1M6,7h1v1h-1M7,7h1v1h-1M8,7h1v1h-1M10,7h1v1h-1M13,7h1v1h-1M15,7h1v1h-1M18,7h1v1h-1M21,7h1v1h-1M23,7h1v1h-1M26,7h1v1h-1M28,7h1v1h-1M29,7h1v1h-1M30,7h1v1h-1M32,7h1v1h-1M4,8h1v1h-1M6,8h1v1h-1M7,8h1v1h-1M8,8h1v1h-1M10,8h1v1h-1M16,8h1v1h-1M17,8h1v1h-1M18,8h1v1h-1M19,8h1v1h-1M20,8h1v1h-1M22,8h1v1h-1M23,8h1v1h-1M24,8h1v1h-1M26,8h1v1h-1M28,8h1v1h-1M29,8h1v1h-1M30,8h1v1h-1M32,8h1v1h-1M4,9h1v1h-1M10,9h1v1h-1M12,9h1v1h-1M13,9h1v1h-1M14,9h1v1h-1M15,9h1v1h-1M16,9h1v1h-1M19,9h1v1h-1M22,9h1v1h-1M26,9h1v1h-1M32,9h1v1h-1M4,10h1v1h-1M5,10h1v1h-1M6,10h1v1h-1M7,10h1v1h-1M8,10h1v1h-1M9,10h1v1h-1M10,10h1v1h-1M12,10h1v1h-1M13,10h1v1h-1M14,10h1v1h-1M15,10h1v1h-1M16,10h1v1h-1M17,10h1v1h-1M18,10h1v1h-1M19,10h1v1h-1M20,10h1v1h-1M21,10h1v1h-1M22,10h1v1h-1M23,10h1v1h-1M24,10h1v1h-1M25,10h1v1h-1M26,10h1v1h-1M27,10h1v1h-1M28,10h1v1h-1M29,10h1v1h-1M30,10h1v1h-1M31,10h1v1h-1M32,10h1v1h-1M12,11h1v1h-1M13,11h1v1h-1M15,11h1v1h-1M16,11h1v1h-1M17,11h1v1h-1M18,11h1v1h-1M19,11h1v1h-1M6,12h1v1h-1M7,12h1v1h-1M8,12h1v1h-1M10,12h1v1h-1M12,12h1v1h-1M20,12h1v1h-1M22,12h1v1h-1M23,12h1v1h-1M24,12h1v1h-1M25,12h1v1h-1M26,12h1v1h-1M27,12h1v1h-1M30,12h1v1h-1M31,12h1v1h-1M32,12h1v1h-1M9,13h1v1h-1M10,13h1v1h-1M11,13h1v1h-1M12,13h1v1h-1M13,13h1v1h-1M14,13h1v1h-1M15,13h1v1h-1M16,13h1v1h-1M18,13h1v1h-1M20,13h1v1h-1M25,13h1v1h-1M26,13h1v1h-1M27,13h1v1h-1M28,13h1v1h-1M29,13h1v1h-1M30,13h1v1h-1M32,13h1v1h-1M4,14h1v1h-1M6,14h1v1h-1M7,14h1v1h-1M9,14h1v1h-1M10,14h1v1h-1M12,14h1v1h-1M13,14h1v1h-1M14,14h1v1h-1M15,14h1v1h-1M16,14h1v1h-1M17,14h1v1h-1M18,14h1v1h-1M19,14h1v1h-1M20,14h1v1h-1M22,14h1v1h-1M24,14h1v1h-1M25,14h1v1h-1M26,14h1v1h-1M27,14h1v1h-1M4,15h1v1h-1M6,15h1v1h-1M8,15h1v1h-1M9,15h1v1h-1M10,15h1v1h-1M11,15h1v1h-1M12,15h1v1h-1M13,15h1v1h-1M15,15h1v1h-1M16,15h1v1h-1M18,15h1v1h-1M20,15h1v1h-1M21,15h1v1h-1M22,15h1v1h-1M25,15h1v1h-1M26,15h1v1h-1M27,15h1v1h-1M29,15h1v1h-1M31,15h1v1h-1M5,16h1v1h-1M7,16h1v1h-1M9,16h1v1h-1M10,16h1v1h-1M11,16h1v1h-1M12,16h1v1h-1M14,16h1v1h-1M17,16h1v1h-1M24,16h1v1h-1M25,16h1v1h-1M27,16h1v1h-1M30,16h1v1h-1M31,16h1v1h-1M32,16h1v1h-1M5,17h1v1h-1M6,17h1v1h-1M8,17h1v1h-1M9,17h1v1h-1M10,17h1v1h-1M12,17h1v1h-1M16,17h1v1h-1M18,17h1v1h-1M20,17h1v1h-1M23,17h1v1h-1M24,17h1v1h-1M25,17h1v1h-1M26,17h1v1h-1M28,17h1v1h-1M29,17h1v1h-1M31,17h1v1h-1M32,17h1v1h-1M4,18h1v1h-1M5,18h1v1h-1M7,18h1v1h-1M9,18h1v1h-1M10,18h1v1h-1M12,18h1v1h-1M13,18h1v1h-1M14,18h1v1h-1M16,18h1v1h-1M19,18h1v1h-1M20,18h1v1h-1M22,18h1v1h-1M24,18h1v1h-1M26,18h1v1h-1M27,18h1v1h-1M4,19h1v1h-1M6,19h1v1h-1M7,19h1v1h-1M8,19h1v1h-1M10,19h1v1h-1M12,19h1v1h-1M13,19h1v1h-1M16,19h1v1h-1M21,19h1v1h-1M22,19h1v1h-1M24,19h1v1h-1M28,19h1v1h-1M29,19h1v1h-1M31,19h1v1h-1M5,20h1v1h-1M6,20h1v1h-1M8,20h1v1h-1M9,20h1v1h-1M10,20h1v1h-1M13,20h1v1h-1M14,20h1v1h-1M16,20h1v1h-1M19,20h1v1h-1M20,20h1v1h-1M25,20h1v1h-1M29,20h1v1h-1M30,20h1v1h-1M31,20h1v1h-1M4,21h1v1h-1M6,21h1v1h-1M7,21h1v1h-1M8,21h1v1h-1M10,21h1v1h-1M12,21h1v1h-1M14,21h1v1h-1M16,21h1v1h-1M17,21h1v1h-1M19,21h1v1h-1M20,21h1v1h-1M24,21h1v1h-1M25,21h1v1h-1M26,21h1v1h-1M27,21h1v1h-1M28,21h1v1h-1M29,21h1v1h-1M31,21h1v1h-1M32,21h1v1h-1M4,22h1v1h-1M7,22h1v1h-1M8,22h1v1h-1M10,22h1v1h-1M13,22h1v1h-1M15,22h1v1h-1M17,22h1v1h-1M19,22h1v1h-1M20,22h1v1h-1M21,22h1v1h-1M23,22h1v1h-1M26,22h1v1h-1M27,22h1v1h-1M29,22h1v1h-1M4,23h1v1h-1M6,23h1v1h-1M9,23h1v1h-1M10,23h1v1h-1M11,23h1v1h-1M13,23h1v1h-1M14,23h1v1h-1M15,23h1v1h-1M16,23h1v1h-1M19,23h1v1h-1M20,23h1v1h-1M21,23h1v1h-1M23,23h1v1h-1M24,23h1v1h-1M26,23h1v1h-1M28,23h1v1h-1M31,23h1v1h-1M4,24h1v1h-1M6,24h1v1h-1M7,24h1v1h-1M9,24h1v1h-1M10,24h1v1h-1M12,24h1v1h-1M14,24h1v1h-1M15,24h1v1h-1M16,24h1v1h-1M17,24h1v1h-1M18,24h1v1h-1M19,24h1v1h-1M20,24h1v1h-1M22,24h1v1h-1M23,24h1v1h-1M24,24h1v1h-1M25,24h1v1h-1M26,24h1v1h-1M27,24h1v1h-1M28,24h1v1h-1M30,24h1v1h-1M10,25h1v1h-1M12,25h1v1h-1M16,25h1v1h-1M18,25h1v1h-1M20,25h1v1h-1M21,25h1v1h-1M22,25h1v1h-1M24,25h1v1h-1M28,25h1v1h-1M29,25h1v1h-1M32,25h1v1h-1M4,26h1v1h-1M5,26h1v1h-1M6,26h1v1h-1M7,26h1v1h-1M8,26h1v1h-1M9,26h1v1h-1M10,26h1v1h-1M14,26h1v1h-1M16,26h1v1h-1M17,26h1v1h-1M18,26h1v1h-1M19,26h1v1h-1M21,26h1v1h-1M22,26h1v1h-1M23,26h1v1h-1M24,26h1v1h-1M26,26h1v1h-1M28,26h1v1h-1M4,27h1v1h-1M10,27h1v1h-1M13,27h1v1h-1M14,27h1v1h-1M15,27h1v1h-1M16,27h1v1h-1M17,27h1v1h-1M19,27h1v1h-1M20,27h1v1h-1M22,27h1v1h-1M23,27h1v1h-1M24,27h1v1h-1M28,27h1v1h-1M29,27h1v1h-1M4,28h1v1h-1M6,28h1v1h-1M7,28h1v1h-1M8,28h1v1h-1M10,28h1v1h-1M12,28h1v1h-1M13,28h1v1h-1M16,28h1v1h-1M20,28h1v1h-1M21,28h1v1h-1M22,28h1v1h-1M24,28h1v1h-1M25,28h1v1h-1M26,28h1v1h-1M27,28h1v1h-1M28,28h1v1h-1M29,28h1v1h-1M30,28h1v1h-1M32,28h1v1h-1M4,29h1v1h-1M6,29h1v1h-1M7,29h1v1h-1M8,29h1v1h-1M10,29h1v1h-1M12,29h1v1h-1M13,29h1v1h-1M15,29h1v1h-1M16,29h1v1h-1M17,29h1v1h-1M18,29h1v1h-1M22,29h1v1h-1M23,29h1v1h-1M24,29h1v1h-1M25,29h1v1h-1M27,29h1v1h-1M29,29h1v1h-1M30,29h1v1h-1M4,30h1v1h-1M6,30h1v1h-1M7,30h1v1h-1M8,30h1v1h-1M10,30h1v1h-1M12,30h1v1h-1M13,30h1v1h-1M14,30h1v1h-1M16,30h1v1h-1M18,30h1v1h-1M20,30h1v1h-1M21,30h1v1h-1M22,30h1v1h-1M23,30h1v1h-1M24,30h1v1h-1M25,30h1v1h-1M26,30h1v1h-1M27,30h1v1h-1M28,30h1v1h-1M30,30h1v1h-1M31,30h1v1h-1M4,31h1v1h-1M10,31h1v1h-1M13,31h1v1h-1M18,31h1v1h-1M19,31h1v1h-1M20,31h1v1h-1M21,31h1v1h-1M26,31h1v1h-1M28,31h1v1h-1M29,31h1v1h-1M31,31h1v1h-1M4,32h1v1h-1M5,32h1v1h-1M6,32h1v1h-1M7,32h1v1h-1M8,32h1v1h-1M9,32h1v1h-1M10,32h1v1h-1M14,32h1v1h-1M15,32h1v1h-1M16,32h1v1h-1M17,32h1v1h-1M18,32h1v1h-1M19,32h1v1h-1M22,32h1v1h-1M26,32h1v1h-1M28,32h1v1h-1M30,32h1v1h-1' />
//! </svg>
pub use crate::ecl::ECL;
pub use crate::version::Version;
use std::fmt::{Debug, Formatter};

mod bitstring;
mod datamasking;

mod default;
pub mod ecl;
mod encode;
mod hardcode;
mod helpers;
mod placement;
mod polynomials;
mod score;
pub mod version;

#[cfg(test)]
mod test;

/// Enum containing all 40 QRCode versions
///
/// From `QRCode::V01` to `QRCode::V40` (both included)
#[derive(Debug)]
pub enum QRCode {
    V01(Box<[[bool; 21]; 21]>),
    V02(Box<[[bool; 25]; 25]>),
    V03(Box<[[bool; 29]; 29]>),
    V04(Box<[[bool; 33]; 33]>),
    V05(Box<[[bool; 37]; 37]>),
    V06(Box<[[bool; 41]; 41]>),
    V07(Box<[[bool; 45]; 45]>),
    V08(Box<[[bool; 49]; 49]>),
    V09(Box<[[bool; 53]; 53]>),
    V10(Box<[[bool; 57]; 57]>),
    V11(Box<[[bool; 61]; 61]>),
    V12(Box<[[bool; 65]; 65]>),
    V13(Box<[[bool; 69]; 69]>),
    V14(Box<[[bool; 73]; 73]>),
    V15(Box<[[bool; 77]; 77]>),
    V16(Box<[[bool; 81]; 81]>),
    V17(Box<[[bool; 85]; 85]>),
    V18(Box<[[bool; 89]; 89]>),
    V19(Box<[[bool; 93]; 93]>),
    V20(Box<[[bool; 97]; 97]>),
    V21(Box<[[bool; 101]; 101]>),
    V22(Box<[[bool; 105]; 105]>),
    V23(Box<[[bool; 109]; 109]>),
    V24(Box<[[bool; 113]; 113]>),
    V25(Box<[[bool; 117]; 117]>),
    V26(Box<[[bool; 121]; 121]>),
    V27(Box<[[bool; 125]; 125]>),
    V28(Box<[[bool; 129]; 129]>),
    V29(Box<[[bool; 133]; 133]>),
    V30(Box<[[bool; 137]; 137]>),
    V31(Box<[[bool; 141]; 141]>),
    V32(Box<[[bool; 145]; 145]>),
    V33(Box<[[bool; 149]; 149]>),
    V34(Box<[[bool; 153]; 153]>),
    V35(Box<[[bool; 157]; 157]>),
    V36(Box<[[bool; 161]; 161]>),
    V37(Box<[[bool; 165]; 165]>),
    V38(Box<[[bool; 169]; 169]>),
    V39(Box<[[bool; 173]; 173]>),
    V40(Box<[[bool; 177]; 177]>),
}

pub enum QRCodeError {
    EncodedData,
    SpecifiedVersion,
}

impl Debug for QRCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QRCodeError::EncodedData => f.write_str("Data too big to be encoded"),
            QRCodeError::SpecifiedVersion => {
                f.write_str("Specified version too low to contain data")
            }
        }
    }
}

impl QRCode {
    /// Creates a new QRCode from a ECL / version
    pub fn new(
        input: &[u8],
        ecl: Option<ECL>,
        v: Option<Version>,
        mask_nb: Option<usize>,
    ) -> Result<Self, QRCodeError> {
        use placement::create_matrix;
        use QRCode::*;
        let mode = encode::best_encoding(input);
        let mut level = ECL::Q;
        if let Some(e) = ecl {
            level = e;
        }

        let version = match Version::get(mode, level, input.len()) {
            Some(version) => version,
            None => return Err(QRCodeError::EncodedData),
        };
        let version = match v {
            Some(user_version) if user_version as usize >= version as usize => user_version,
            None => version,
            Some(_) => return Err(QRCodeError::SpecifiedVersion),
        };

        #[rustfmt::skip]
        let out = match version {
            Version::V01 => V01(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V02 => V02(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V03 => V03(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V04 => V04(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V05 => V05(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V06 => V06(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V07 => V07(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V08 => V08(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V09 => V09(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V10 => V10(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V11 => V11(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V12 => V12(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V13 => V13(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V14 => V14(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V15 => V15(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V16 => V16(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V17 => V17(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V18 => V18(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V19 => V19(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V20 => V20(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V21 => V21(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V22 => V22(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V23 => V23(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V24 => V24(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V25 => V25(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V26 => V26(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V27 => V27(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V28 => V28(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V29 => V29(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V30 => V30(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V31 => V31(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V32 => V32(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V33 => V33(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V34 => V34(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V35 => V35(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V36 => V36(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V37 => V37(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V38 => V38(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V39 => V39(Box::new(create_matrix(input, level, mode, version, mask_nb))),
            Version::V40 => V40(Box::new(create_matrix(input, level, mode, version, mask_nb))),
        };

        Ok(out)
    }

    /// Prints the matrix to the terminal
    pub fn print(&self) {
        use QRCode::*;
        match self {
            V01(matrix) => helpers::print_matrix_with_margin(matrix),
            V02(matrix) => helpers::print_matrix_with_margin(matrix),
            V03(matrix) => helpers::print_matrix_with_margin(matrix),
            V04(matrix) => helpers::print_matrix_with_margin(matrix),
            V05(matrix) => helpers::print_matrix_with_margin(matrix),
            V06(matrix) => helpers::print_matrix_with_margin(matrix),
            V07(matrix) => helpers::print_matrix_with_margin(matrix),
            V08(matrix) => helpers::print_matrix_with_margin(matrix),
            V09(matrix) => helpers::print_matrix_with_margin(matrix),
            V10(matrix) => helpers::print_matrix_with_margin(matrix),
            V11(matrix) => helpers::print_matrix_with_margin(matrix),
            V12(matrix) => helpers::print_matrix_with_margin(matrix),
            V13(matrix) => helpers::print_matrix_with_margin(matrix),
            V14(matrix) => helpers::print_matrix_with_margin(matrix),
            V15(matrix) => helpers::print_matrix_with_margin(matrix),
            V16(matrix) => helpers::print_matrix_with_margin(matrix),
            V17(matrix) => helpers::print_matrix_with_margin(matrix),
            V18(matrix) => helpers::print_matrix_with_margin(matrix),
            V19(matrix) => helpers::print_matrix_with_margin(matrix),
            V20(matrix) => helpers::print_matrix_with_margin(matrix),
            V21(matrix) => helpers::print_matrix_with_margin(matrix),
            V22(matrix) => helpers::print_matrix_with_margin(matrix),
            V23(matrix) => helpers::print_matrix_with_margin(matrix),
            V24(matrix) => helpers::print_matrix_with_margin(matrix),
            V25(matrix) => helpers::print_matrix_with_margin(matrix),
            V26(matrix) => helpers::print_matrix_with_margin(matrix),
            V27(matrix) => helpers::print_matrix_with_margin(matrix),
            V28(matrix) => helpers::print_matrix_with_margin(matrix),
            V29(matrix) => helpers::print_matrix_with_margin(matrix),
            V30(matrix) => helpers::print_matrix_with_margin(matrix),
            V31(matrix) => helpers::print_matrix_with_margin(matrix),
            V32(matrix) => helpers::print_matrix_with_margin(matrix),
            V33(matrix) => helpers::print_matrix_with_margin(matrix),
            V34(matrix) => helpers::print_matrix_with_margin(matrix),
            V35(matrix) => helpers::print_matrix_with_margin(matrix),
            V36(matrix) => helpers::print_matrix_with_margin(matrix),
            V37(matrix) => helpers::print_matrix_with_margin(matrix),
            V38(matrix) => helpers::print_matrix_with_margin(matrix),
            V39(matrix) => helpers::print_matrix_with_margin(matrix),
            V40(matrix) => helpers::print_matrix_with_margin(matrix),
        }
    }
}
