//! Wrappers to create QRCode
//!
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

use crate::module::Matrix;
use std::fmt::{Debug, Formatter};

use crate::{encode, helpers, Version, ECL};

/// Enum containing all 40 QRCode versions
///
/// From `QRCode::V01` to `QRCode::V40` (both included)
#[derive(Debug)]
pub enum QRCode {
    /// QRCode Version n°01, being 21x21
    V01(Box<Matrix<21>>),
    /// QRCode Version n°02, being 25x25
    V02(Box<Matrix<25>>),
    /// QRCode Version n°03, being 29x29
    V03(Box<Matrix<29>>),
    /// QRCode Version n°04, being 33x33
    V04(Box<Matrix<33>>),
    /// QRCode Version n°05, being 37x37
    V05(Box<Matrix<37>>),
    /// QRCode Version n°06, being 41x41
    V06(Box<Matrix<41>>),
    /// QRCode Version n°07, being 45x45
    V07(Box<Matrix<45>>),
    /// QRCode Version n°08, being 49x49
    V08(Box<Matrix<49>>),
    /// QRCode Version n°09, being 53x53
    V09(Box<Matrix<53>>),
    /// QRCode Version n°10, being 57x57
    V10(Box<Matrix<57>>),
    /// QRCode Version n°11, being 61x61
    V11(Box<Matrix<61>>),
    /// QRCode Version n°12, being 65x65
    V12(Box<Matrix<65>>),
    /// QRCode Version n°13, being 69x69
    V13(Box<Matrix<69>>),
    /// QRCode Version n°14, being 73x73
    V14(Box<Matrix<73>>),
    /// QRCode Version n°15, being 77x77
    V15(Box<Matrix<77>>),
    /// QRCode Version n°16, being 81x81
    V16(Box<Matrix<81>>),
    /// QRCode Version n°17, being 85x85
    V17(Box<Matrix<85>>),
    /// QRCode Version n°18, being 89x89
    V18(Box<Matrix<89>>),
    /// QRCode Version n°19, being 93x93
    V19(Box<Matrix<93>>),
    /// QRCode Version n°20, being 97x97
    V20(Box<Matrix<97>>),
    /// QRCode Version n°21, being 101x101
    V21(Box<Matrix<101>>),
    /// QRCode Version n°22, being 105x105
    V22(Box<Matrix<105>>),
    /// QRCode Version n°23, being 109x109
    V23(Box<Matrix<109>>),
    /// QRCode Version n°24, being 113x113
    V24(Box<Matrix<113>>),
    /// QRCode Version n°25, being 117x117
    V25(Box<Matrix<117>>),
    /// QRCode Version n°26, being 121x121
    V26(Box<Matrix<121>>),
    /// QRCode Version n°27, being 125x125
    V27(Box<Matrix<125>>),
    /// QRCode Version n°28, being 129x129
    V28(Box<Matrix<129>>),
    /// QRCode Version n°29, being 133x133
    V29(Box<Matrix<133>>),
    /// QRCode Version n°30, being 137x137
    V30(Box<Matrix<137>>),
    /// QRCode Version n°31, being 141x141
    V31(Box<Matrix<141>>),
    /// QRCode Version n°32, being 145x145
    V32(Box<Matrix<145>>),
    /// QRCode Version n°33, being 149x149
    V33(Box<Matrix<149>>),
    /// QRCode Version n°34, being 153x153
    V34(Box<Matrix<153>>),
    /// QRCode Version n°35, being 157x157
    V35(Box<Matrix<157>>),
    /// QRCode Version n°36, being 161x161
    V36(Box<Matrix<161>>),
    /// QRCode Version n°37, being 165x165
    V37(Box<Matrix<165>>),
    /// QRCode Version n°38, being 169x169
    V38(Box<Matrix<169>>),
    /// QRCode Version n°39, being 173x173
    V39(Box<Matrix<173>>),
    /// QRCode Version n°40, being 177x177
    V40(Box<Matrix<177>>),
}

/// Contains different error when QRCode could not be created
pub enum QRCodeError {
    /// If data if too big to be encoded (refer to [character-capacities](https://www.thonky.com/qr-code-tutorial/character-capacities))
    EncodedData,
    /// Specified version too low to contain data
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

/// Builder for `QRCode` struct
pub struct QRBuilder {
    input: String,
    ecl: Option<ECL>,
    // mode: Option<Mode>,
    version: Option<Version>,
    mask_nb: Option<usize>,
}

impl QRBuilder {
    /// Creates an instance of QRBuilder with default parameters
    pub fn new(input: String) -> QRBuilder {
        QRBuilder {
            input,
            mask_nb: None,
            // mode: None,
            version: None,
            ecl: None,
        }
    }

    // pub fn mode(&mut self, mode: Mode) -> &mut Self {
    //     self.mode = Some(mode);
    //     self
    // }

    /// Changes the Encoding Level
    pub fn ecl(&mut self, ecl: ECL) -> &mut Self {
        self.ecl = Some(ecl);
        self
    }

    /// Changes the version
    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = Some(version);
        self
    }

    /// Changes the mask, should very rarely be used
    pub fn mask_nb(&mut self, mask_nb: usize) -> &mut Self {
        self.mask_nb = Some(mask_nb);
        self
    }

    /// Computes a QRCode with given parameters
    pub fn build(&self) -> Result<QRCode, QRCodeError> {
        QRCode::new(self.input.as_bytes(), self.ecl, self.version, self.mask_nb)
    }
}

macro_rules! match_matrix {
    // Takes a matrix and a function that takes a matrix
    ($matrix:expr, $func:expr) => {
        match $matrix {
            QRCode::V01(matrix) => $func(matrix),
            QRCode::V02(matrix) => $func(matrix),
            QRCode::V03(matrix) => $func(matrix),
            QRCode::V04(matrix) => $func(matrix),
            QRCode::V05(matrix) => $func(matrix),
            QRCode::V06(matrix) => $func(matrix),
            QRCode::V07(matrix) => $func(matrix),
            QRCode::V08(matrix) => $func(matrix),
            QRCode::V09(matrix) => $func(matrix),
            QRCode::V10(matrix) => $func(matrix),
            QRCode::V11(matrix) => $func(matrix),
            QRCode::V12(matrix) => $func(matrix),
            QRCode::V13(matrix) => $func(matrix),
            QRCode::V14(matrix) => $func(matrix),
            QRCode::V15(matrix) => $func(matrix),
            QRCode::V16(matrix) => $func(matrix),
            QRCode::V17(matrix) => $func(matrix),
            QRCode::V18(matrix) => $func(matrix),
            QRCode::V19(matrix) => $func(matrix),
            QRCode::V20(matrix) => $func(matrix),
            QRCode::V21(matrix) => $func(matrix),
            QRCode::V22(matrix) => $func(matrix),
            QRCode::V23(matrix) => $func(matrix),
            QRCode::V24(matrix) => $func(matrix),
            QRCode::V25(matrix) => $func(matrix),
            QRCode::V26(matrix) => $func(matrix),
            QRCode::V27(matrix) => $func(matrix),
            QRCode::V28(matrix) => $func(matrix),
            QRCode::V29(matrix) => $func(matrix),
            QRCode::V30(matrix) => $func(matrix),
            QRCode::V31(matrix) => $func(matrix),
            QRCode::V32(matrix) => $func(matrix),
            QRCode::V33(matrix) => $func(matrix),
            QRCode::V34(matrix) => $func(matrix),
            QRCode::V35(matrix) => $func(matrix),
            QRCode::V36(matrix) => $func(matrix),
            QRCode::V37(matrix) => $func(matrix),
            QRCode::V38(matrix) => $func(matrix),
            QRCode::V39(matrix) => $func(matrix),
            QRCode::V40(matrix) => $func(matrix),
        }
    };
}

impl QRCode {
    /// Creates a new QRCode from a ECL / version
    pub fn new(
        input: &[u8],
        ecl: Option<ECL>,
        v: Option<Version>,
        mask_nb: Option<usize>,
    ) -> Result<Self, QRCodeError> {
        use crate::placement::create_matrix;
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

    /// Prints the QRCode to the terminal
    pub fn print(&self) {
        match_matrix!(self, helpers::print_matrix_with_margin);
    }
}
