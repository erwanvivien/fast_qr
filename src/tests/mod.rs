#[test]
fn version_size_01() {
    let mat = crate::default::create_matrix_from_version(1);
    let length = mat.len();
    assert_eq!(length, 21);
}
#[test]
fn version_size_03() {
    let mat = crate::default::create_matrix_from_version(3);
    let length = mat.len();
    assert_eq!(length, 29);
}
#[test]
fn version_size_22() {
    let mat = crate::default::create_matrix_from_version(22);
    let length = mat.len();
    assert_eq!(length, 105);
}
#[test]
fn version_size_40() {
    let mat = crate::default::create_matrix_from_version(40);
    let length = mat.len();
    assert_eq!(length, 177);
}

#[test]
fn position_pattern_03() {
    let mat = crate::default::create_matrix_from_version(3);
    // Top line
    assert!(mat[0][0]);
    assert!(mat[0][1]);
    assert!(mat[0][2]);
    assert!(mat[0][3]);
    assert!(mat[0][4]);
    assert!(mat[0][5]);
    assert!(mat[0][6]);

    // Left line
    assert!(mat[1][0]);
    assert!(mat[2][0]);
    assert!(mat[3][0]);
    assert!(mat[4][0]);
    assert!(mat[5][0]);
    assert!(mat[6][0]);

    // Right line
    assert!(mat[1][6]);
    assert!(mat[2][6]);
    assert!(mat[3][6]);
    assert!(mat[4][6]);
    assert!(mat[5][6]);
    assert!(mat[6][6]);

    // Bottom line
    assert!(mat[6][0]);
    assert!(mat[6][1]);
    assert!(mat[6][2]);
    assert!(mat[6][3]);
    assert!(mat[6][4]);
    assert!(mat[6][5]);
    assert!(mat[6][6]);

    // Inside square
    assert!(mat[2][2]);
    assert!(mat[2][3]);
    assert!(mat[2][4]);
    assert!(mat[3][2]);
    assert!(mat[3][3]);
    assert!(mat[3][4]);
    assert!(mat[4][2]);
    assert!(mat[4][3]);
    assert!(mat[4][4]);
}

/// Checks it's empty
#[test]
fn position_pattern_01() {
    let mat = crate::default::create_matrix_from_version(1);

    for i in 7..21 {
        for j in 7..21 {
            // Dark module
            if i == 13 && j == 8 {
                continue;
            }
            assert_eq!(mat[i][j], false);
        }
    }
}

#[test]
#[test]
fn generator_polynomial_7() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[7];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x7 + α87x6 + α229x5 + α146x4 + α149x3 + α238x2 + α102x + α21"
    )
}

#[test]
fn generator_polynomial_8() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[8];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x8 + α175x7 + α238x6 + α208x5 + α249x4 + α215x3 + α252x2 + α196x + α28"
    )
}

#[test]
fn generator_polynomial_9() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[9];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x9 + α95x8 + α246x7 + α137x6 + α231x5 + α235x4 + α149x3 + α11x2 + α123x + α36"
    )
}

#[test]
fn generator_polynomial_10() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[10];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x10 + α251x9 + α67x8 + α46x7 + α61x6 + α118x5 + α70x4 + α64x3 + α94x2 + α32x + α45"
    )
}

#[test]
fn generator_polynomial_11() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[11];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x11 + α220x10 + α192x9 + α91x8 + α194x7 + α172x6 + α177x5 + α209x4 + α116x3 + α227x2 + α10x + α55"
    )
}

#[test]
fn generator_polynomial_12() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[12];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x12 + α102x11 + α43x10 + α98x9 + α121x8 + α187x7 + α113x6 + α198x5 + α143x4 + α131x3 + α87x2 + α157x + α66"
    )
}

#[test]
fn generator_polynomial_13() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[13];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x13 + α74x12 + α152x11 + α176x10 + α100x9 + α86x8 + α100x7 + α106x6 + α104x5 + α130x4 + α218x3 + α206x2 + α140x + α78"
    )
}

#[test]
fn generator_polynomial_14() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[14];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x14 + α199x13 + α249x12 + α155x11 + α48x10 + α190x9 + α124x8 + α218x7 + α137x6 + α216x5 + α87x4 + α207x3 + α59x2 + α22x + α91"
    )
}

#[test]
fn generator_polynomial_15() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[15];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x15 + α8x14 + α183x13 + α61x12 + α91x11 + α202x10 + α37x9 + α51x8 + α58x7 + α58x6 + α237x5 + α140x4 + α124x3 + α5x2 + α99x + α105"
    )
}

#[test]
fn generator_polynomial_16() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[16];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x16 + α120x15 + α104x14 + α107x13 + α109x12 + α102x11 + α161x10 + α76x9 + α3x8 + α91x7 + α191x6 + α147x5 + α169x4 + α182x3 + α194x2 + α225x + α120"
    )
}

#[test]
fn generator_polynomial_17() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[17];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x17 + α43x16 + α139x15 + α206x14 + α78x13 + α43x12 + α239x11 + α123x10 + α206x9 + α214x8 + α147x7 + α24x6 + α99x5 + α150x4 + α39x3 + α243x2 + α163x + α136"
    )
}

#[test]
fn generator_polynomial_18() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[18];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x18 + α215x17 + α234x16 + α158x15 + α94x14 + α184x13 + α97x12 + α118x11 + α170x10 + α79x9 + α187x8 + α152x7 + α148x6 + α252x5 + α179x4 + α5x3 + α98x2 + α96x + α153"
    )
}

#[test]
fn generator_polynomial_19() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[19];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x19 + α67x18 + α3x17 + α105x16 + α153x15 + α52x14 + α90x13 + α83x12 + α17x11 + α150x10 + α159x9 + α44x8 + α128x7 + α153x6 + α133x5 + α252x4 + α222x3 + α138x2 + α220x + α171"
    )
}

#[test]
fn generator_polynomial_20() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[20];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x20 + α17x19 + α60x18 + α79x17 + α50x16 + α61x15 + α163x14 + α26x13 + α187x12 + α202x11 + α180x10 + α221x9 + α225x8 + α83x7 + α239x6 + α156x5 + α164x4 + α212x3 + α212x2 + α188x + α190"
    )
}

#[test]
fn generator_polynomial_21() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[21];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x21 + α240x20 + α233x19 + α104x18 + α247x17 + α181x16 + α140x15 + α67x14 + α98x13 + α85x12 + α200x11 + α210x10 + α115x9 + α148x8 + α137x7 + α230x6 + α36x5 + α122x4 + α254x3 + α148x2 + α175x + α210"
    )
}

#[test]
fn generator_polynomial_22() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[22];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x22 + α210x21 + α171x20 + α247x19 + α242x18 + α93x17 + α230x16 + α14x15 + α109x14 + α221x13 + α53x12 + α200x11 + α74x10 + α8x9 + α172x8 + α98x7 + α80x6 + α219x5 + α134x4 + α160x3 + α105x2 + α165x + α231"
    )
}

#[test]
fn generator_polynomial_23() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[23];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x23 + α171x22 + α102x21 + α146x20 + α91x19 + α49x18 + α103x17 + α65x16 + α17x15 + α193x14 + α150x13 + α14x12 + α25x11 + α183x10 + α248x9 + α94x8 + α164x7 + α224x6 + α192x5 + α1x4 + α78x3 + α56x2 + α147x + α253"
    )
}

#[test]
fn generator_polynomial_24() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[24];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x24 + α229x23 + α121x22 + α135x21 + α48x20 + α211x19 + α117x18 + α251x17 + α126x16 + α159x15 + α180x14 + α169x13 + α152x12 + α192x11 + α226x10 + α228x9 + α218x8 + α111x7 + α0x6 + α117x5 + α232x4 + α87x3 + α96x2 + α227x + α21"
    )
}

#[test]
fn generator_polynomial_25() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[25];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x25 + α231x24 + α181x23 + α156x22 + α39x21 + α170x20 + α26x19 + α12x18 + α59x17 + α15x16 + α148x15 + α201x14 + α54x13 + α66x12 + α237x11 + α208x10 + α99x9 + α167x8 + α144x7 + α182x6 + α95x5 + α243x4 + α129x3 + α178x2 + α252x + α45"
    )
}

#[test]
fn generator_polynomial_26() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[26];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x26 + α173x25 + α125x24 + α158x23 + α2x22 + α103x21 + α182x20 + α118x19 + α17x18 + α145x17 + α201x16 + α111x15 + α28x14 + α165x13 + α53x12 + α161x11 + α21x10 + α245x9 + α142x8 + α13x7 + α102x6 + α48x5 + α227x4 + α153x3 + α145x2 + α218x + α70"
    )
}

#[test]
fn generator_polynomial_27() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[27];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x27 + α79x26 + α228x25 + α8x24 + α165x23 + α227x22 + α21x21 + α180x20 + α29x19 + α9x18 + α237x17 + α70x16 + α99x15 + α45x14 + α58x13 + α138x12 + α135x11 + α73x10 + α126x9 + α172x8 + α94x7 + α216x6 + α193x5 + α157x4 + α26x3 + α17x2 + α149x + α96"
    )
}

#[test]
fn generator_polynomial_28() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[28];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x28 + α168x27 + α223x26 + α200x25 + α104x24 + α224x23 + α234x22 + α108x21 + α180x20 + α110x19 + α190x18 + α195x17 + α147x16 + α205x15 + α27x14 + α232x13 + α201x12 + α21x11 + α43x10 + α245x9 + α87x8 + α42x7 + α195x6 + α212x5 + α119x4 + α242x3 + α37x2 + α9x + α123"
    )
}

#[test]
fn generator_polynomial_29() {
    let poly = crate::polynomials::GENERATOR_POLYNOMIALS[29];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x29 + α156x28 + α45x27 + α183x26 + α29x25 + α151x24 + α219x23 + α54x22 + α96x21 + α249x20 + α24x19 + α136x18 + α5x17 + α241x16 + α175x15 + α189x14 + α28x13 + α75x12 + α234x11 + α150x10 + α148x9 + α23x8 + α9x7 + α202x6 + α162x5 + α68x4 + α250x3 + α140x2 + α24x + α151"
    )
}

#[test]
fn error_code_computation_01() {
    let version = 5;
    let quality = crate::vecl::ECL::Q;

    let vec: Vec<u8> = [67, 85, 70, 134, 87, 38, 85, 194, 119, 50, 6, 18, 6, 103, 38].to_vec();
    let nb_error_codes = crate::vecl::ecc_to_ect(quality, version);

    let generator_polynomials = crate::polynomials::GENERATOR_POLYNOMIALS[nb_error_codes];

    let div = crate::polynomials::division(&vec, generator_polynomials);

    assert_eq!(
        div,
        [213, 199, 11, 45, 115, 247, 241, 223, 229, 248, 154, 117, 154, 111, 86, 161, 111, 39]
            .to_vec()
    )
}

#[test]
fn error_code_computation_02() {
    let version = 5;
    let quality = crate::vecl::ECL::Q;

    let vec: Vec<u8> = [
        246, 246, 66, 7, 118, 134, 242, 7, 38, 86, 22, 198, 199, 146, 6,
    ]
    .to_vec();
    let nb_error_codes = crate::vecl::ecc_to_ect(quality, version);

    let generator_polynomials = crate::polynomials::GENERATOR_POLYNOMIALS[nb_error_codes];

    let div = crate::polynomials::division(&vec, generator_polynomials);

    assert_eq!(
        div,
        [87, 204, 96, 60, 202, 182, 124, 157, 200, 134, 27, 129, 209, 17, 163, 163, 120, 133]
            .to_vec()
    )
}

#[test]
fn error_code_computation_03() {
    let version = 5;
    let quality = crate::vecl::ECL::Q;

    let vec: Vec<u8> = [
        182, 230, 247, 119, 50, 7, 118, 134, 87, 38, 82, 6, 134, 151, 50, 7,
    ]
    .to_vec();
    let nb_error_codes = crate::vecl::ecc_to_ect(quality, version);

    let generator_polynomials = crate::polynomials::GENERATOR_POLYNOMIALS[nb_error_codes];

    let div = crate::polynomials::division(&vec, generator_polynomials);

    assert_eq!(
        div,
        [148, 116, 177, 212, 76, 133, 75, 242, 238, 76, 195, 230, 189, 10, 108, 240, 192, 141]
            .to_vec()
    )
}

#[test]
fn error_code_computation_04() {
    let version = 5;
    let quality = crate::vecl::ECL::Q;

    let vec: Vec<u8> = [
        70, 247, 118, 86, 194, 6, 151, 50, 16, 236, 17, 236, 17, 236, 17, 236,
    ]
    .to_vec();
    let nb_error_codes = crate::vecl::ecc_to_ect(quality, version);

    let generator_polynomials = crate::polynomials::GENERATOR_POLYNOMIALS[nb_error_codes];

    let div = crate::polynomials::division(&vec, generator_polynomials);

    assert_eq!(
        div,
        [235, 159, 5, 173, 24, 147, 59, 33, 106, 40, 255, 172, 82, 2, 131, 32, 178, 236].to_vec()
    )
}
