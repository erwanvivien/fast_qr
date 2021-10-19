#[rustfmt::skip]
#[test]
fn version_format_l_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V5);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, false, true, true, true, true, true, false, false, false, true, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V5(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V3);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, false, false, true, false, true, true, true, true, false, false, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V3(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V6);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, true, true, false, true, true, false, true, false, true, false, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V6(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V3);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, true, false, false, false, true, false, false, true, true, true, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V3(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V6);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, false, true, true, false, false, false, true, false, true, true, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V6(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V6);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, false, false, true, true, false, false, false, true, true, false, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V6(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V6);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, true, true, false, false, false, true, false, false, false, false, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V6(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V5);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::L);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, true, false, false, true, false, true, true, true, false, true, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V5(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V1);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, false, true, false, false, false, false, false, true, false, false, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V1(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V4);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, false, false, false, true, false, false, true, false, false, true, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V4(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V2);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, true, true, true, false, false, true, true, true, true, true, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V2(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V6);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, true, false, true, true, false, true, false, false, true, false, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V6(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V1);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, false, true, false, true, true, true, true, true, true, false, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V1(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V2);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, false, false, false, false, true, true, false, false, true, true, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V2(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V1);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, true, true, true, true, true, false, false, true, false, true, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V1(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_m_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V3);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::M);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, true, false, true, false, true, false, true, false, false, false, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V3(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V4);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, false, true, false, true, false, true, false, true, true, true, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V4(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V2);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, false, false, false, false, false, true, true, false, true, false, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V2(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V4);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, true, true, true, true, false, false, true, true, false, false, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V4(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V5);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, true, false, true, false, false, false, false, false, false, true, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V5(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V1);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, false, true, false, false, true, false, true, true, false, true, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V1(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V5);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, false, false, false, true, true, false, false, false, false, false, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V5(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V2);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, true, true, true, false, true, true, false, true, true, false, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V2(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_q_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V1);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::Q);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, true, false, true, true, true, true, true, false, true, true, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V1(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V6);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, false, true, true, false, true, false, false, false, true, false, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V6(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V2);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, false, false, true, true, true, false, true, true, true, true, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V2(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V4);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, true, true, false, false, true, true, true, false, false, true, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V4(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V3);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, true, false, false, true, true, true, false, true, false, false, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V3(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V2);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, false, true, true, true, false, true, true, false, false, false, true, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V2(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V4);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, false, false, true, false, false, true, false, true, false, true, false, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V4(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V2);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, true, true, false, true, false, false, false, false, true, true, false, false];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V2(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}

#[rustfmt::skip]
#[test]
fn version_format_h_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V1);
    const LEVEL: Option<crate::vecl::ECL> = Some(crate::vecl::ECL::H);

    const QRCODE: Option<crate::qrcode::QRCode> =
        crate::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, true, false, false, false, false, false, true, true, true, false, true, true];

    if let Some(q) = QRCODE {
        if let crate::qrcode::QRCode::V1(mat) = q {
            let l = mat.len();
            let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }
    }
}
