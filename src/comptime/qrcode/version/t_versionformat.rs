#[test]
fn version_format_l_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> =
        Some(crate::comptime::qrcode::version::Version::V05);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> =
        Some(crate::comptime::qrcode::ecl::ECL::L);

    let q = crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [
        true, true, true, false, true, true, true, true, true, false, false, false, true, false,
        false,
    ];

    if let crate::comptime::qrcode::QRCode::V05(mat) = q {
        let l = mat.len();
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp, EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp, EXPECTED);
    } else {
        assert_eq!(true, false);
    }
}

#[rustfmt::skip]
#[test]
fn version_format_l_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V03);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, false, false, true, false, true, true, true, true, false, false, true, true];

    
        if let crate::comptime::qrcode::QRCode::V03(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V06);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, true, true, false, true, true, false, true, false, true, false, true, false];

    
        if let crate::comptime::qrcode::QRCode::V06(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V03);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, true, false, false, false, true, false, false, true, true, true, false, true];

    
        if let crate::comptime::qrcode::QRCode::V03(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V06);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, false, true, true, false, false, false, true, false, true, true, true, true];

    
        if let crate::comptime::qrcode::QRCode::V06(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V06);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, false, false, true, true, false, false, false, true, true, false, false, false];

    
        if let crate::comptime::qrcode::QRCode::V06(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V06);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, true, true, false, false, false, true, false, false, false, false, false, true];

    
        if let crate::comptime::qrcode::QRCode::V06(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V05);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, true, false, false, true, false, true, true, true, false, true, true, false];

    
        if let crate::comptime::qrcode::QRCode::V05(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V01);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, false, true, false, false, false, false, false, true, false, false, true, false];

    
        if let crate::comptime::qrcode::QRCode::V01(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V04);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, false, false, false, true, false, false, true, false, false, true, false, true];

    
        if let crate::comptime::qrcode::QRCode::V04(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V02);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, true, true, true, false, false, true, true, true, true, true, false, false];

    
        if let crate::comptime::qrcode::QRCode::V02(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V06);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, true, false, true, true, false, true, false, false, true, false, true, true];

    
        if let crate::comptime::qrcode::QRCode::V06(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V01);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, false, true, false, true, true, true, true, true, true, false, false, true];

    
        if let crate::comptime::qrcode::QRCode::V01(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V02);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, false, false, false, false, true, true, false, false, true, true, true, false];

    
        if let crate::comptime::qrcode::QRCode::V02(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V01);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, true, true, true, true, true, false, false, true, false, true, true, true];

    
        if let crate::comptime::qrcode::QRCode::V01(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V03);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, true, false, true, false, true, false, true, false, false, false, false, false];

    
        if let crate::comptime::qrcode::QRCode::V03(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V04);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, false, true, false, true, false, true, false, true, true, true, true, true];

    
        if let crate::comptime::qrcode::QRCode::V04(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V02);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, false, false, false, false, false, true, true, false, true, false, false, false];

    
        if let crate::comptime::qrcode::QRCode::V02(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V04);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, true, true, true, true, false, false, true, true, false, false, false, true];

    
        if let crate::comptime::qrcode::QRCode::V04(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V05);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, true, false, true, false, false, false, false, false, false, true, true, false];

    
        if let crate::comptime::qrcode::QRCode::V05(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V01);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, false, true, false, false, true, false, true, true, false, true, false, false];

    
        if let crate::comptime::qrcode::QRCode::V01(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V05);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, false, false, false, true, true, false, false, false, false, false, true, true];

    
        if let crate::comptime::qrcode::QRCode::V05(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V02);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, true, true, true, false, true, true, false, true, true, false, true, false];

    
        if let crate::comptime::qrcode::QRCode::V02(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V01);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, true, false, true, true, true, true, true, false, true, true, false, true];

    
        if let crate::comptime::qrcode::QRCode::V01(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V06);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, false, true, true, false, true, false, false, false, true, false, false, true];

    
        if let crate::comptime::qrcode::QRCode::V06(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V02);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, false, false, true, true, true, false, true, true, true, true, true, false];

    
        if let crate::comptime::qrcode::QRCode::V02(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V04);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, true, true, false, false, true, true, true, false, false, true, true, true];

    
        if let crate::comptime::qrcode::QRCode::V04(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V03);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, true, false, false, true, true, true, false, true, false, false, false, false];

    
        if let crate::comptime::qrcode::QRCode::V03(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V02);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, false, true, true, true, false, true, true, false, false, false, true, false];

    
        if let crate::comptime::qrcode::QRCode::V02(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V04);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, false, false, true, false, false, true, false, true, false, true, false, true];

    
        if let crate::comptime::qrcode::QRCode::V04(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V02);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, true, true, false, true, false, false, false, false, true, true, false, false];

    
        if let crate::comptime::qrcode::QRCode::V02(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V01);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, true, false, false, false, false, false, true, true, true, false, true, true];

    
        if let crate::comptime::qrcode::QRCode::V01(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask0_version23() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V23);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, false, true, true, true, true, true, false, false, false, true, false, false];
    let mut expected2: [bool; 18] = [false, true, false, true, true, true, false, true, true, true, true, true, true, false, true, true, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V23(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask1_version29() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V29);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, false, false, true, false, true, true, true, true, false, false, true, true];
    let mut expected2: [bool; 18] = [false, true, true, true, false, true, false, false, true, true, false, false, true, true, true, true, true, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V29(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask2_version40() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V40);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, true, true, false, true, true, false, true, false, true, false, true, false];
    let mut expected2: [bool; 18] = [true, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V40(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask3_version8() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V08);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, true, true, false, false, false, true, false, false, true, true, true, false, true];
    let mut expected2: [bool; 18] = [false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V08(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask4_version36() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V36);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, false, true, true, false, false, false, true, false, true, true, true, true];
    let mut expected2: [bool; 18] = [true, false, false, true, false, false, true, false, true, true, false, false, false, false, true, false, true, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V36(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask5_version22() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V22);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, false, false, true, true, false, false, false, true, true, false, false, false];
    let mut expected2: [bool; 18] = [false, true, false, true, true, false, true, false, false, false, true, true, false, false, true, false, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V22(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask6_version10() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V10);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, true, true, false, false, false, true, false, false, false, false, false, true];
    let mut expected2: [bool; 18] = [false, false, true, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V10(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_l_mask7_version17() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V17);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::L);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, true, false, true, false, false, true, false, true, true, true, false, true, true, false];
    let mut expected2: [bool; 18] = [false, true, false, false, false, true, false, true, false, false, false, true, false, true, true, true, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V17(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask0_version14() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V14);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, false, true, false, false, false, false, false, true, false, false, true, false];
    let mut expected2: [bool; 18] = [false, false, true, true, true, false, false, true, true, false, false, false, false, false, true, true, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V14(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask1_version30() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V30);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, false, false, false, true, false, false, true, false, false, true, false, true];
    let mut expected2: [bool; 18] = [false, true, true, true, true, false, true, true, false, true, false, true, true, true, false, true, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V30(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask2_version37() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V37);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, true, true, true, false, false, true, true, true, true, true, false, false];
    let mut expected2: [bool; 18] = [true, false, false, true, false, true, false, true, false, false, false, false, true, false, true, true, true, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V37(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask3_version22() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V22);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, true, true, false, true, true, false, true, false, false, true, false, true, true];
    let mut expected2: [bool; 18] = [false, true, false, true, true, false, true, false, false, false, true, true, false, false, true, false, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V22(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask4_version31() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V31);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, false, true, false, true, true, true, true, true, true, false, false, true];
    let mut expected2: [bool; 18] = [false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V31(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask5_version13() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V13);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, false, false, false, false, true, true, false, false, true, true, true, false];
    let mut expected2: [bool; 18] = [false, false, true, true, false, true, true, false, false, false, false, true, false, false, false, true, true, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V13(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask6_version22() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V22);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, true, true, true, true, true, false, false, true, false, true, true, true];
    let mut expected2: [bool; 18] = [false, true, false, true, true, false, true, false, false, false, true, true, false, false, true, false, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V22(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_m_mask7_version7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V07);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::M);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [true, false, false, true, false, true, false, true, false, true, false, false, false, false, false];
    let mut expected2: [bool; 18] = [false, false, false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V07(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask0_version20() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V20);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, false, true, false, true, false, true, false, true, true, true, true, true];
    let mut expected2: [bool; 18] = [false, true, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V20(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask1_version33() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V33);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, false, false, false, false, false, true, true, false, true, false, false, false];
    let mut expected2: [bool; 18] = [true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V33(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask2_version24() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V24);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, true, true, true, true, false, false, true, true, false, false, false, true];
    let mut expected2: [bool; 18] = [false, true, true, false, false, false, true, true, true, false, true, true, false, false, false, true, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V24(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask3_version18() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V18);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, true, true, false, true, false, false, false, false, false, false, true, true, false];
    let mut expected2: [bool; 18] = [false, true, false, false, true, false, true, false, true, false, false, false, false, true, false, true, true, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V18(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask4_version31() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V31);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, false, true, false, false, true, false, true, true, false, true, false, false];
    let mut expected2: [bool; 18] = [false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V31(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask5_version17() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V17);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, false, false, false, true, true, false, false, false, false, false, true, true];
    let mut expected2: [bool; 18] = [false, true, false, false, false, true, false, true, false, false, false, true, false, true, true, true, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V17(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask6_version11() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V11);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, true, true, true, false, true, true, false, true, true, false, true, false];
    let mut expected2: [bool; 18] = [false, false, true, false, true, true, true, false, true, true, true, true, true, true, false, true, true, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V11(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_q_mask7_version15() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V15);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::Q);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, true, false, true, false, true, true, true, true, true, false, true, true, false, true];
    let mut expected2: [bool; 18] = [false, false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V15(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask0_version35() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(0);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V35);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, false, true, true, false, true, false, false, false, true, false, false, true];
    let mut expected2: [bool; 18] = [true, false, false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V35(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask1_version15() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(1);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V15);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, false, false, true, true, true, false, true, true, true, true, true, false];
    let mut expected2: [bool; 18] = [false, false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V15(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask2_version15() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(2);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V15);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, true, true, false, false, true, true, true, false, false, true, true, true];
    let mut expected2: [bool; 18] = [false, false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V15(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask3_version7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(3);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V07);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, true, true, false, false, true, true, true, false, true, false, false, false, false];
    let mut expected2: [bool; 18] = [false, false, false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V07(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask4_version7() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(4);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V07);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, false, true, true, true, false, true, true, false, false, false, true, false];
    let mut expected2: [bool; 18] = [false, false, false, true, true, true, true, true, false, false, true, false, false, true, false, true, false, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V07(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask5_version20() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(5);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V20);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, false, false, true, false, false, true, false, true, false, true, false, true];
    let mut expected2: [bool; 18] = [false, true, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V20(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask6_version20() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(6);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V20);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, true, true, false, true, false, false, false, false, true, true, false, false];
    let mut expected2: [bool; 18] = [false, true, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, false];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V20(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}

#[rustfmt::skip]
#[test]
fn version_format_h_mask7_version17() {
    const CONTENT: &str = "4";
    const MASK: Option<usize> = Some(7);
    const VERSION: Option<crate::comptime::qrcode::version::Version> = Some(crate::comptime::qrcode::version::Version::V17);
    const LEVEL: Option<crate::comptime::qrcode::ecl::ECL> = Some(crate::comptime::qrcode::ecl::ECL::H);

    let q=
        crate::comptime::qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    const EXPECTED: [bool; 15] = [false, false, false, true, false, false, false, false, false, true, true, true, false, true, true];
    let mut expected2: [bool; 18] = [false, true, false, false, false, true, false, true, false, false, false, true, false, true, true, true, false, true];
expected2.reverse();

    
        if let crate::comptime::qrcode::QRCode::V17(mat) = q {
            let l = mat.len();
            #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
            assert_eq!(tmp, EXPECTED);

            #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
            assert_eq!(tmp, EXPECTED);

            let tmp2 = [
                mat[l - 11][0], mat[l - 10][0], mat[l - 9][0],
                mat[l - 11][1], mat[l - 10][1], mat[l - 9][1],
                mat[l - 11][2], mat[l - 10][2], mat[l - 9][2],
                mat[l - 11][3], mat[l - 10][3], mat[l - 9][3],
                mat[l - 11][4], mat[l - 10][4], mat[l - 9][4],
                mat[l - 11][5], mat[l - 10][5], mat[l - 9][5],
            ];
            assert_eq!(tmp2, expected2);

            let tmp2 = [
                mat[0][l - 11], mat[0][l - 10], mat[0][l - 9],
                mat[1][l - 11], mat[1][l - 10], mat[1][l - 9],
                mat[2][l - 11], mat[2][l - 10], mat[2][l - 9],
                mat[3][l - 11], mat[3][l - 10], mat[3][l - 9],
                mat[4][l - 11], mat[4][l - 10], mat[4][l - 9],
                mat[5][l - 11], mat[5][l - 10], mat[5][l - 9],
            ];
            assert_eq!(tmp2, expected2);


        } else {
            assert_eq!(true, false);
        }}
