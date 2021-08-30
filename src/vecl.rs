//! Contains all different levels of quality.
//! And allows to find easily max bits per version/quality pair

/// Error Correction Coding has 4 levels
#[allow(dead_code)]
pub enum ECL {
    /// Low, 7%
    L,
    /// Medium, 15%
    M,
    /// Quartile, 25%
    Q,
    /// High, 30%
    H,
}

/// Contains the max bits for any version of `LOW` ECC
const L_DATABITS: [u16; 41] = [
    0, 152, 272, 440, 640, 864, 1088, 1248, 1552, 1856, 2192, 2592, 2960, 3424, 3688, 4184, 4712,
    5176, 5768, 6360, 6888, 7456, 8048, 8752, 9392, 10208, 10960, 11744, 12248, 13048, 13880,
    14744, 15640, 16568, 17528, 18448, 19472, 20528, 21616, 22496, 23648,
];

/// Contains the max bits for any version of `MED` ECC
const M_DATABITS: [u16; 41] = [
    0, 128, 224, 352, 512, 688, 864, 992, 1232, 1456, 1728, 2032, 2320, 2672, 2920, 3320, 3624,
    4056, 4504, 5016, 5352, 5712, 6256, 6880, 7312, 8000, 8496, 9024, 9544, 10136, 10984, 11640,
    12328, 13048, 13800, 14496, 15312, 15936, 16816, 17728, 18672,
];

/// Contains the max bits for any version of `QAR` ECC
const Q_DATABITS: [u16; 41] = [
    0, 104, 176, 272, 384, 496, 608, 704, 880, 1056, 1232, 1440, 1648, 1952, 2088, 2360, 2600,
    2936, 3176, 3560, 3880, 4096, 4544, 4912, 5312, 5744, 6032, 6464, 6968, 7288, 7880, 8264, 8920,
    9368, 9848, 10288, 10832, 11408, 12016, 12656, 13328,
];

/// Contains the max bits for any version of `HIG` ECC
const H_DATABITS: [u16; 41] = [
    0, 72, 128, 208, 288, 368, 480, 528, 688, 800, 976, 1120, 1264, 1440, 1576, 1784, 2024, 2264,
    2504, 2728, 3080, 3248, 3536, 3712, 4112, 4304, 4768, 5024, 5288, 5608, 5960, 6344, 6760, 7208,
    7688, 7888, 8432, 8768, 9136, 9776, 10208,
];

/// Fetches the right array to retrieve numer of databits
pub const fn ecc_to_databits(quality: ECL, version: usize) -> u16 {
    return match quality {
        ECL::L => L_DATABITS[version],
        ECL::M => M_DATABITS[version],
        ECL::Q => Q_DATABITS[version],
        ECL::H => H_DATABITS[version],
    };
}

/// Contains the number of Data Codewords for a version at level L
const L_ECT: [usize; 41] = [
    0, 7, 10, 15, 20, 26, 18, 20, 24, 30, 18, 20, 24, 26, 30, 22, 24, 28, 30, 28, 28, 28, 28, 30,
    30, 26, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
];

/// Contains the number of Data Codewords for a version at level M
const M_ECT: [usize; 41] = [
    0, 10, 16, 26, 18, 24, 16, 18, 22, 22, 26, 30, 22, 22, 24, 24, 28, 28, 26, 26, 26, 26, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
];

/// Contains the number of Data Codewords for a version at level Q
const Q_ECT: [usize; 41] = [
    0, 13, 22, 18, 26, 18, 24, 18, 22, 20, 24, 28, 26, 24, 20, 30, 24, 28, 28, 26, 30, 28, 30, 30,
    30, 30, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
];

/// Contains the number of Data Codewords for a version at level H
const H_ECT: [usize; 41] = [
    0, 17, 28, 22, 16, 22, 28, 26, 26, 24, 28, 24, 28, 22, 24, 24, 30, 28, 28, 26, 28, 30, 24, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
];

/// Fetches the right array to retrieve numer of error correction code words
pub const fn ecc_to_ect(quality: ECL, version: usize) -> usize {
    return match quality {
        ECL::L => L_ECT[version],
        ECL::M => M_ECT[version],
        ECL::Q => Q_ECT[version],
        ECL::H => H_ECT[version],
    };
}
