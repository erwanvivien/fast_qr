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
const L_ECT: [u16; 41] = [
    0, 19, 34, 55, 80, 108, 136, 156, 194, 232, 274, 324, 370, 428, 461, 523, 589, 647, 721, 795,
    861, 932, 1006, 1094, 1174, 1276, 1370, 1468, 1531, 1631, 1735, 1843, 1955, 2071, 2191, 2306,
    2434, 2566, 2702, 2812, 2956,
];

/// Contains the number of Data Codewords for a version at level M
const M_ECT: [u16; 41] = [
    0, 16, 28, 44, 64, 86, 108, 124, 154, 182, 216, 254, 290, 334, 365, 415, 453, 507, 563, 627,
    669, 714, 782, 860, 914, 1000, 1062, 1128, 1193, 1267, 1373, 1455, 1541, 1631, 1725, 1812,
    1914, 1992, 2102, 2216, 2334,
];

/// Contains the number of Data Codewords for a version at level Q
const Q_ECT: [u16; 41] = [
    0, 13, 22, 34, 48, 62, 76, 88, 110, 132, 154, 180, 206, 244, 261, 295, 325, 367, 397, 445, 485,
    512, 568, 614, 664, 718, 754, 808, 871, 911, 985, 1033, 1115, 1171, 1231, 1286, 1354, 1426,
    1502, 1582, 1666,
];

/// Contains the number of Data Codewords for a version at level H
const H_ECT: [u16; 41] = [
    0, 9, 16, 26, 36, 46, 60, 66, 86, 100, 122, 140, 158, 180, 197, 223, 253, 283, 313, 341, 385,
    406, 442, 464, 514, 538, 596, 628, 661, 701, 745, 793, 845, 901, 961, 986, 1054, 1096, 1142,
    1222, 1276,
];

/// Fetches the right array to retrieve numer of error correction code words
pub const fn ecc_to_ect(quality: ECL, version: usize) -> u16 {
    return match quality {
        ECL::L => L_ECT[version],
        ECL::M => M_ECT[version],
        ECL::Q => Q_ECT[version],
        ECL::H => H_ECT[version],
    };
}
