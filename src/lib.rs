//! Enum containing all 40 QRCode versions

pub use crate::ecl::ECL;
pub use crate::version::Version;

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
#[doc(hidden)]
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

impl QRCode {
    /// Creates a new QRCode from a ECL / version
    ///
    /// # Example
    /// ```txt
    /// const QRCODE2: Option<QRCode> = QRCode::new(
    ///     "Hello, world!".as_bytes(),
    ///     Some(ecl::ECL::H),
    ///     Some(version::Version::V2),
    ///     None,
    /// );
    /// ```
    pub fn new(input: &[u8], ecl: Option<ECL>, v: Option<Version>, mask_nb: Option<usize>) -> Self {
        use placement::create_matrix;
        use QRCode::*;
        let mode = encode::best_encoding(input);
        let mut level = ECL::Q;
        if let Some(e) = ecl {
            level = e;
        }
        let version = match Version::get(mode, level, input.len()) {
            Some(version) => version,
            None => panic!("Version doesn't contain a valid version"),
        };
        let version = match v {
            Some(user_version) if user_version as usize >= version as usize => user_version,
            None => version,
            Some(_) => panic!("Specified version is not a valid version"),
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

        out
    }
    /// Prints the matrix
    ///
    /// # Example
    /// ```txt
    /// const CONTENT: &str = "https://vahan.dev/";
    /// const LEVEL: Option<ecl::ECL> = Some(ecl::ECL::H);
    ///
    /// const QRCODE: Option<QRCode> =
    ///     QRCode::new(CONTENT.as_bytes(), LEVEL, None, None);
    ///
    /// if let Some(q) = QRCODE {
    ///     q.print();
    /// }
    /// ```
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
