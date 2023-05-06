//! Module `qr` is the entrypoint to start making `QRCodes`

use crate::module::Module;
use core::fmt::{Debug, Formatter};
use core::ops::{Index, IndexMut};
use std::ops::RangeTo;

use crate::datamasking::Mask;
use crate::encode::Mode;
#[cfg(not(target_arch = "wasm32"))]
use crate::helpers;
use crate::{encode, Version, ECL};

/// A `QRCode` can be created using [`QRBuilder`]. Simple API for simple usage.
/// If you need to use `QRCode` directly, please file an [issue on
/// github](https://github.com/erwanvivien/fast_qr) explaining your use case.
///
/// Contains all needed information about the `QRCode`.
/// This is the main struct of the crate.
///
/// It contains the matrix of the `QRCode`, stored as a one-dimensional array.
#[derive(Clone, Debug)]
pub struct QRCode {
    /// This array length is of size `177 x 177`. It is using a fixed size
    /// array simply because of performance.
    ///
    /// # Other data type possible:
    /// - Templated Matrix was faster but crate size was huge.
    /// - Vector using `with_capacity`, really bad.
    pub data: QRCodeData,
    /// Width & Height of QRCode. If manually set, should be `version * 4 + 17`, `version` going
    /// from 1 to 40 both included.
    pub size: usize,

    /// Version of the `QRCode`, impacts the size.
    ///
    /// `None` will optimize Version according to ECL and Mode
    pub version: Option<Version>,
    /// Defines how powerful `QRCode` redundancy should be or how much percent of a QRCode can be
    /// recovered.
    ///
    /// - `ECL::L`: 7%
    /// - `ECL::M`: 15%
    /// - `ECL::Q`: 25%
    /// - `ELC::H`: 30%
    ///
    /// `None` will set ECL to Quartile (`ELC::Q`)
    pub ecl: Option<ECL>,

    /// Changes the final pattern used.
    ///
    /// None will find the best suited mask.
    pub mask: Option<Mask>,
    /// Mode defines which data is being parsed, between Numeric, AlphaNumeric & Byte.
    ///
    /// `None` will optimize Mode according to user input.
    ///
    /// ## Note
    /// Kanji mode is not supported (yet).
    pub mode: Option<Mode>,
}

/// QRCode data, that stores exactly the right amount of data.
#[derive(Clone, Debug)]
#[cfg(feature = "fast")]
pub enum QRCodeData {
    /// Data for QRCode of version 1, size 21x21
    V01(Box<[Module; 21 * 21]>),
    /// Data for QRCode of version 2, size 25x25
    V02(Box<[Module; 25 * 25]>),
    /// Data for QRCode of version 3, size 29x29
    V03(Box<[Module; 29 * 29]>),
    /// Data for QRCode of version 4, size 33x33
    V04(Box<[Module; 33 * 33]>),
    /// Data for QRCode of version 5, size 37x37
    V05(Box<[Module; 37 * 37]>),
    /// Data for QRCode of version 6, size 41x41
    V06(Box<[Module; 41 * 41]>),
    /// Data for QRCode of version 7, size 45x45
    V07(Box<[Module; 45 * 45]>),
    /// Data for QRCode of version 8, size 49x49
    V08(Box<[Module; 49 * 49]>),
    /// Data for QRCode of version 9, size 53x53
    V09(Box<[Module; 53 * 53]>),
    /// Data for QRCode of version 10, size 57x57
    V10(Box<[Module; 57 * 57]>),
    /// Data for QRCode of version 11, size 61x61
    V11(Box<[Module; 61 * 61]>),
    /// Data for QRCode of version 12, size 65x65
    V12(Box<[Module; 65 * 65]>),
    /// Data for QRCode of version 13, size 69x69
    V13(Box<[Module; 69 * 69]>),
    /// Data for QRCode of version 14, size 73x73
    V14(Box<[Module; 73 * 73]>),
    /// Data for QRCode of version 15, size 77x77
    V15(Box<[Module; 77 * 77]>),
    /// Data for QRCode of version 16, size 81x81
    V16(Box<[Module; 81 * 81]>),
    /// Data for QRCode of version 17, size 85x85
    V17(Box<[Module; 85 * 85]>),
    /// Data for QRCode of version 18, size 89x89
    V18(Box<[Module; 89 * 89]>),
    /// Data for QRCode of version 19, size 93x93
    V19(Box<[Module; 93 * 93]>),
    /// Data for QRCode of version 20, size 97x97
    V20(Box<[Module; 97 * 97]>),
    /// Data for QRCode of version 21, size 101x101
    V21(Box<[Module; 101 * 101]>),
    /// Data for QRCode of version 22, size 105x105
    V22(Box<[Module; 105 * 105]>),
    /// Data for QRCode of version 23, size 109x109
    V23(Box<[Module; 109 * 109]>),
    /// Data for QRCode of version 24, size 113x113
    V24(Box<[Module; 113 * 113]>),
    /// Data for QRCode of version 25, size 117x117
    V25(Box<[Module; 117 * 117]>),
    /// Data for QRCode of version 26, size 121x121
    V26(Box<[Module; 121 * 121]>),
    /// Data for QRCode of version 27, size 125x125
    V27(Box<[Module; 125 * 125]>),
    /// Data for QRCode of version 28, size 129x129
    V28(Box<[Module; 129 * 129]>),
    /// Data for QRCode of version 29, size 133x133
    V29(Box<[Module; 133 * 133]>),
    /// Data for QRCode of version 30, size 137x137
    V30(Box<[Module; 137 * 137]>),
    /// Data for QRCode of version 31, size 141x141
    V31(Box<[Module; 141 * 141]>),
    /// Data for QRCode of version 32, size 145x145
    V32(Box<[Module; 145 * 145]>),
    /// Data for QRCode of version 33, size 149x149
    V33(Box<[Module; 149 * 149]>),
    /// Data for QRCode of version 34, size 153x153
    V34(Box<[Module; 153 * 153]>),
    /// Data for QRCode of version 35, size 157x157
    V35(Box<[Module; 157 * 157]>),
    /// Data for QRCode of version 36, size 161x161
    V36(Box<[Module; 161 * 161]>),
    /// Data for QRCode of version 37, size 165x165
    V37(Box<[Module; 165 * 165]>),
    /// Data for QRCode of version 38, size 169x169
    V38(Box<[Module; 169 * 169]>),
    /// Data for QRCode of version 39, size 173x173
    V39(Box<[Module; 173 * 173]>),
    /// Data for QRCode of version 40, size 177x177
    V40(Box<[Module; 177 * 177]>),
    /// Data for a non standard QRCode, basically a matrix.
    NotConform(Vec<Module>),
}

/// QRCode data, stored as a one-dimensional array of size `177 x 177`.
#[derive(Clone, Debug)]
#[cfg(not(feature = "fast"))]
pub struct QRCodeData([Module; 177 * 177]);

impl QRCodeData {
    /// Create a new QRCodeData with the given size.
    #[cfg(feature = "fast")]
    pub fn new(size: usize) -> Self {
        const DEFAULT_MODULE: Module = Module::data(Module::LIGHT);
        match size {
            21 => QRCodeData::V01(Box::new([DEFAULT_MODULE; 21 * 21])),
            25 => QRCodeData::V02(Box::new([DEFAULT_MODULE; 25 * 25])),
            29 => QRCodeData::V03(Box::new([DEFAULT_MODULE; 29 * 29])),
            33 => QRCodeData::V04(Box::new([DEFAULT_MODULE; 33 * 33])),
            37 => QRCodeData::V05(Box::new([DEFAULT_MODULE; 37 * 37])),
            41 => QRCodeData::V06(Box::new([DEFAULT_MODULE; 41 * 41])),
            45 => QRCodeData::V07(Box::new([DEFAULT_MODULE; 45 * 45])),
            49 => QRCodeData::V08(Box::new([DEFAULT_MODULE; 49 * 49])),
            53 => QRCodeData::V09(Box::new([DEFAULT_MODULE; 53 * 53])),
            57 => QRCodeData::V10(Box::new([DEFAULT_MODULE; 57 * 57])),
            61 => QRCodeData::V11(Box::new([DEFAULT_MODULE; 61 * 61])),
            65 => QRCodeData::V12(Box::new([DEFAULT_MODULE; 65 * 65])),
            69 => QRCodeData::V13(Box::new([DEFAULT_MODULE; 69 * 69])),
            73 => QRCodeData::V14(Box::new([DEFAULT_MODULE; 73 * 73])),
            77 => QRCodeData::V15(Box::new([DEFAULT_MODULE; 77 * 77])),
            81 => QRCodeData::V16(Box::new([DEFAULT_MODULE; 81 * 81])),
            85 => QRCodeData::V17(Box::new([DEFAULT_MODULE; 85 * 85])),
            89 => QRCodeData::V18(Box::new([DEFAULT_MODULE; 89 * 89])),
            93 => QRCodeData::V19(Box::new([DEFAULT_MODULE; 93 * 93])),
            97 => QRCodeData::V20(Box::new([DEFAULT_MODULE; 97 * 97])),
            101 => QRCodeData::V21(Box::new([DEFAULT_MODULE; 101 * 101])),
            105 => QRCodeData::V22(Box::new([DEFAULT_MODULE; 105 * 105])),
            109 => QRCodeData::V23(Box::new([DEFAULT_MODULE; 109 * 109])),
            113 => QRCodeData::V24(Box::new([DEFAULT_MODULE; 113 * 113])),
            117 => QRCodeData::V25(Box::new([DEFAULT_MODULE; 117 * 117])),
            121 => QRCodeData::V26(Box::new([DEFAULT_MODULE; 121 * 121])),
            125 => QRCodeData::V27(Box::new([DEFAULT_MODULE; 125 * 125])),
            129 => QRCodeData::V28(Box::new([DEFAULT_MODULE; 129 * 129])),
            133 => QRCodeData::V29(Box::new([DEFAULT_MODULE; 133 * 133])),
            137 => QRCodeData::V30(Box::new([DEFAULT_MODULE; 137 * 137])),
            141 => QRCodeData::V31(Box::new([DEFAULT_MODULE; 141 * 141])),
            145 => QRCodeData::V32(Box::new([DEFAULT_MODULE; 145 * 145])),
            149 => QRCodeData::V33(Box::new([DEFAULT_MODULE; 149 * 149])),
            153 => QRCodeData::V34(Box::new([DEFAULT_MODULE; 153 * 153])),
            157 => QRCodeData::V35(Box::new([DEFAULT_MODULE; 157 * 157])),
            161 => QRCodeData::V36(Box::new([DEFAULT_MODULE; 161 * 161])),
            165 => QRCodeData::V37(Box::new([DEFAULT_MODULE; 165 * 165])),
            169 => QRCodeData::V38(Box::new([DEFAULT_MODULE; 169 * 169])),
            173 => QRCodeData::V39(Box::new([DEFAULT_MODULE; 173 * 173])),
            177 => QRCodeData::V40(Box::new([DEFAULT_MODULE; 177 * 177])),
            size => QRCodeData::NotConform(vec![DEFAULT_MODULE; size * size]),
        }
    }

    /// Create a new QRCodeData with the given size.
    #[cfg(not(feature = "fast"))]
    pub const fn new(size: usize) -> Self {
        Self([Module::data(Module::LIGHT); 177 * 177])
    }
}

#[cfg(feature = "fast")]
impl AsRef<[Module]> for QRCodeData {
    fn as_ref(&self) -> &[Module] {
        match self {
            QRCodeData::V01(data) => &data[..],
            QRCodeData::V02(data) => &data[..],
            QRCodeData::V03(data) => &data[..],
            QRCodeData::V04(data) => &data[..],
            QRCodeData::V05(data) => &data[..],
            QRCodeData::V06(data) => &data[..],
            QRCodeData::V07(data) => &data[..],
            QRCodeData::V08(data) => &data[..],
            QRCodeData::V09(data) => &data[..],
            QRCodeData::V10(data) => &data[..],
            QRCodeData::V11(data) => &data[..],
            QRCodeData::V12(data) => &data[..],
            QRCodeData::V13(data) => &data[..],
            QRCodeData::V14(data) => &data[..],
            QRCodeData::V15(data) => &data[..],
            QRCodeData::V16(data) => &data[..],
            QRCodeData::V17(data) => &data[..],
            QRCodeData::V18(data) => &data[..],
            QRCodeData::V19(data) => &data[..],
            QRCodeData::V20(data) => &data[..],
            QRCodeData::V21(data) => &data[..],
            QRCodeData::V22(data) => &data[..],
            QRCodeData::V23(data) => &data[..],
            QRCodeData::V24(data) => &data[..],
            QRCodeData::V25(data) => &data[..],
            QRCodeData::V26(data) => &data[..],
            QRCodeData::V27(data) => &data[..],
            QRCodeData::V28(data) => &data[..],
            QRCodeData::V29(data) => &data[..],
            QRCodeData::V30(data) => &data[..],
            QRCodeData::V31(data) => &data[..],
            QRCodeData::V32(data) => &data[..],
            QRCodeData::V33(data) => &data[..],
            QRCodeData::V34(data) => &data[..],
            QRCodeData::V35(data) => &data[..],
            QRCodeData::V36(data) => &data[..],
            QRCodeData::V37(data) => &data[..],
            QRCodeData::V38(data) => &data[..],
            QRCodeData::V39(data) => &data[..],
            QRCodeData::V40(data) => &data[..],
            QRCodeData::NotConform(data) => &data[..],
        }
    }
}

#[cfg(feature = "fast")]
impl AsMut<[Module]> for QRCodeData {
    fn as_mut(&mut self) -> &mut [Module] {
        match self {
            QRCodeData::V01(data) => &mut data[..],
            QRCodeData::V02(data) => &mut data[..],
            QRCodeData::V03(data) => &mut data[..],
            QRCodeData::V04(data) => &mut data[..],
            QRCodeData::V05(data) => &mut data[..],
            QRCodeData::V06(data) => &mut data[..],
            QRCodeData::V07(data) => &mut data[..],
            QRCodeData::V08(data) => &mut data[..],
            QRCodeData::V09(data) => &mut data[..],
            QRCodeData::V10(data) => &mut data[..],
            QRCodeData::V11(data) => &mut data[..],
            QRCodeData::V12(data) => &mut data[..],
            QRCodeData::V13(data) => &mut data[..],
            QRCodeData::V14(data) => &mut data[..],
            QRCodeData::V15(data) => &mut data[..],
            QRCodeData::V16(data) => &mut data[..],
            QRCodeData::V17(data) => &mut data[..],
            QRCodeData::V18(data) => &mut data[..],
            QRCodeData::V19(data) => &mut data[..],
            QRCodeData::V20(data) => &mut data[..],
            QRCodeData::V21(data) => &mut data[..],
            QRCodeData::V22(data) => &mut data[..],
            QRCodeData::V23(data) => &mut data[..],
            QRCodeData::V24(data) => &mut data[..],
            QRCodeData::V25(data) => &mut data[..],
            QRCodeData::V26(data) => &mut data[..],
            QRCodeData::V27(data) => &mut data[..],
            QRCodeData::V28(data) => &mut data[..],
            QRCodeData::V29(data) => &mut data[..],
            QRCodeData::V30(data) => &mut data[..],
            QRCodeData::V31(data) => &mut data[..],
            QRCodeData::V32(data) => &mut data[..],
            QRCodeData::V33(data) => &mut data[..],
            QRCodeData::V34(data) => &mut data[..],
            QRCodeData::V35(data) => &mut data[..],
            QRCodeData::V36(data) => &mut data[..],
            QRCodeData::V37(data) => &mut data[..],
            QRCodeData::V38(data) => &mut data[..],
            QRCodeData::V39(data) => &mut data[..],
            QRCodeData::V40(data) => &mut data[..],
            QRCodeData::NotConform(data) => &mut data[..],
        }
    }
}

impl QRCode {
    /// A default `QRCode` will have all it's fields as `None` and a default Matrix filled with `Module::LIGHT`.
    #[must_use]
    pub fn default(size: usize) -> Self {
        QRCode {
            data: QRCodeData::new(size),
            size,
            version: None,
            ecl: None,
            mask: None,
            mode: None,
        }
    }
}

impl Index<usize> for QRCode {
    type Output = [Module];

    fn index(&self, index: usize) -> &Self::Output {
        #[cfg(feature = "fast")]
        let data = self.data.as_ref();
        #[cfg(not(feature = "fast"))]
        let data = &self.data.0;

        &data[index * self.size..(index + 1) * self.size]
    }
}

impl IndexMut<usize> for QRCode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        #[cfg(feature = "fast")]
        let data = self.data.as_mut();
        #[cfg(not(feature = "fast"))]
        let data = &mut self.data.0;

        &mut data[index * self.size..(index + 1) * self.size]
    }
}

impl Index<RangeTo<usize>> for QRCode {
    type Output = [Module];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        #[cfg(feature = "fast")]
        let data = self.data.as_ref();
        #[cfg(not(feature = "fast"))]
        let data = &self.data.0;

        &data[..index.end]
    }
}

/// Contains different error when [`QRCode`] could not be created
pub enum QRCodeError {
    /// If data if too large to be encoded (refer to Table 7-11 of the spec or [an online table](https://fast-qr.com/blog/tables/ecl))
    EncodedData,
    /// Specified version too small to contain data
    SpecifiedVersion,
}

// We don't want to use `std::error::Error` on wasm32
impl std::error::Error for QRCodeError {}

impl std::fmt::Display for QRCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            QRCodeError::EncodedData => f.write_str("Data too big to be encoded"),
            QRCodeError::SpecifiedVersion => {
                f.write_str("Specified version too low to contain data")
            }
        }
    }
}

impl Debug for QRCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            QRCodeError::EncodedData => f.write_str("Data too big to be encoded"),
            QRCodeError::SpecifiedVersion => {
                f.write_str("Specified version too low to contain data")
            }
        }
    }
}

impl QRCode {
    /// Creates a new `QRCode` from a ECL / version
    ///
    /// # Errors
    /// - `QRCodeError::EncodedData` if `input` is too large to be encoded
    /// - `QRCodeError::SpecifiedVersion` if specified `version` is too small to contain data
    pub(crate) fn new(
        input: &[u8],
        ecl: Option<ECL>,
        v: Option<Version>,
        mut mask: Option<Mask>,
    ) -> Result<Self, QRCodeError> {
        use crate::placement::create_matrix;

        let mode = encode::best_encoding(input);
        let level = ecl.unwrap_or(ECL::Q);

        let version = match Version::get(mode, level, input.len()) {
            Some(version) => version,
            None => return Err(QRCodeError::EncodedData),
        };
        let version = match v {
            Some(user_version) if user_version as usize >= version as usize => user_version,
            None => version,
            Some(_) => return Err(QRCodeError::SpecifiedVersion),
        };

        let out = create_matrix(input, level, mode, version, &mut mask);
        Ok(out)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prints the `QRCode` to the terminal
    #[must_use]
    pub fn to_str(&self) -> String {
        helpers::print_matrix_with_margin(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prints the `QRCode` to the terminal
    pub fn print(&self) {
        println!("{}", helpers::print_matrix_with_margin(self));
    }
}

/// Builder struct, makes it easier to create a [`QRCode`].
///
/// # Example
/// ```rust
/// use fast_qr::QRBuilder;
/// use fast_qr::{Mask, ECL, Version};
///
/// // Creates a `QRCode` with a forced `version`, `ecl` and/or `mask`
/// let input = String::from("Hello World!");
/// let qr = QRBuilder::new(input)
///     // .version(Version::V05)
///     // .ecl(ECL::H)
///     // .mask(Mask::Checkerboard)
///     .build();
/// ```
pub struct QRBuilder {
    input: Vec<u8>,
    ecl: Option<ECL>,
    // mode: Option<Mode>,
    version: Option<Version>,
    mask: Option<Mask>,
}

impl QRBuilder {
    /// Creates an instance of `QRBuilder` with default parameters
    #[must_use]
    pub fn new<I: Into<Vec<u8>>>(input: I) -> QRBuilder {
        QRBuilder {
            input: input.into(),
            mask: None,
            // mode: None,
            version: None,
            ecl: None,
        }
    }

    // pub fn mode(&mut self, mode: Mode) -> &mut Self {
    //     self.mode = Some(mode);
    //     self
    // }

    /// Forces the Encoding Level
    pub fn ecl(&mut self, ecl: ECL) -> &mut Self {
        self.ecl = Some(ecl);
        self
    }

    /// Forces the version
    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = Some(version);
        self
    }

    /// Forces the mask, should very rarely be used
    pub fn mask(&mut self, mask: Mask) -> &mut Self {
        self.mask = Some(mask);
        self
    }

    /// Computes a [`QRCode`] with given parameters
    ///
    /// # Errors
    /// - `QRCodeError::EncodedData` if `input` is too large to be encoded. See [an online table](https://fast-qr.com/blog/tables/ecl) for more info.
    /// - `QRCodeError::SpecifiedVersion` if specified `version` is too small to contain data
    pub fn build(&self) -> Result<QRCode, QRCodeError> {
        QRCode::new(&self.input, self.ecl, self.version, self.mask)
    }
}
