#[test]
fn generator_polynomial_7() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[7];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x7 + α87x6 + α229x5 + α146x4 + α149x3 + α238x2 + α102x + α21"
    )
}

#[test]
fn generator_polynomial_8() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[8];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x8 + α175x7 + α238x6 + α208x5 + α249x4 + α215x3 + α252x2 + α196x + α28"
    )
}

#[test]
fn generator_polynomial_9() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[9];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x9 + α95x8 + α246x7 + α137x6 + α231x5 + α235x4 + α149x3 + α11x2 + α123x + α36"
    )
}

#[test]
fn generator_polynomial_10() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[10];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x10 + α251x9 + α67x8 + α46x7 + α61x6 + α118x5 + α70x4 + α64x3 + α94x2 + α32x + α45"
    )
}

#[test]
fn generator_polynomial_11() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[11];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x11 + α220x10 + α192x9 + α91x8 + α194x7 + α172x6 + α177x5 + α209x4 + α116x3 + α227x2 + α10x + α55"
    )
}

#[test]
fn generator_polynomial_12() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[12];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x12 + α102x11 + α43x10 + α98x9 + α121x8 + α187x7 + α113x6 + α198x5 + α143x4 + α131x3 + α87x2 + α157x + α66"
    )
}

#[test]
fn generator_polynomial_13() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[13];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x13 + α74x12 + α152x11 + α176x10 + α100x9 + α86x8 + α100x7 + α106x6 + α104x5 + α130x4 + α218x3 + α206x2 + α140x + α78"
    )
}

#[test]
fn generator_polynomial_14() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[14];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x14 + α199x13 + α249x12 + α155x11 + α48x10 + α190x9 + α124x8 + α218x7 + α137x6 + α216x5 + α87x4 + α207x3 + α59x2 + α22x + α91"
    )
}

#[test]
fn generator_polynomial_15() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[15];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x15 + α8x14 + α183x13 + α61x12 + α91x11 + α202x10 + α37x9 + α51x8 + α58x7 + α58x6 + α237x5 + α140x4 + α124x3 + α5x2 + α99x + α105"
    )
}

#[test]
fn generator_polynomial_16() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[16];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x16 + α120x15 + α104x14 + α107x13 + α109x12 + α102x11 + α161x10 + α76x9 + α3x8 + α91x7 + α191x6 + α147x5 + α169x4 + α182x3 + α194x2 + α225x + α120"
    )
}

#[test]
fn generator_polynomial_17() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[17];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x17 + α43x16 + α139x15 + α206x14 + α78x13 + α43x12 + α239x11 + α123x10 + α206x9 + α214x8 + α147x7 + α24x6 + α99x5 + α150x4 + α39x3 + α243x2 + α163x + α136"
    )
}

#[test]
fn generator_polynomial_18() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[18];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x18 + α215x17 + α234x16 + α158x15 + α94x14 + α184x13 + α97x12 + α118x11 + α170x10 + α79x9 + α187x8 + α152x7 + α148x6 + α252x5 + α179x4 + α5x3 + α98x2 + α96x + α153"
    )
}

#[test]
fn generator_polynomial_19() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[19];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x19 + α67x18 + α3x17 + α105x16 + α153x15 + α52x14 + α90x13 + α83x12 + α17x11 + α150x10 + α159x9 + α44x8 + α128x7 + α153x6 + α133x5 + α252x4 + α222x3 + α138x2 + α220x + α171"
    )
}

#[test]
fn generator_polynomial_20() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[20];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x20 + α17x19 + α60x18 + α79x17 + α50x16 + α61x15 + α163x14 + α26x13 + α187x12 + α202x11 + α180x10 + α221x9 + α225x8 + α83x7 + α239x6 + α156x5 + α164x4 + α212x3 + α212x2 + α188x + α190"
    )
}

#[test]
fn generator_polynomial_21() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[21];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x21 + α240x20 + α233x19 + α104x18 + α247x17 + α181x16 + α140x15 + α67x14 + α98x13 + α85x12 + α200x11 + α210x10 + α115x9 + α148x8 + α137x7 + α230x6 + α36x5 + α122x4 + α254x3 + α148x2 + α175x + α210"
    )
}

#[test]
fn generator_polynomial_22() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[22];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x22 + α210x21 + α171x20 + α247x19 + α242x18 + α93x17 + α230x16 + α14x15 + α109x14 + α221x13 + α53x12 + α200x11 + α74x10 + α8x9 + α172x8 + α98x7 + α80x6 + α219x5 + α134x4 + α160x3 + α105x2 + α165x + α231"
    )
}

#[test]
fn generator_polynomial_23() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[23];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x23 + α171x22 + α102x21 + α146x20 + α91x19 + α49x18 + α103x17 + α65x16 + α17x15 + α193x14 + α150x13 + α14x12 + α25x11 + α183x10 + α248x9 + α94x8 + α164x7 + α224x6 + α192x5 + α1x4 + α78x3 + α56x2 + α147x + α253"
    )
}

#[test]
fn generator_polynomial_24() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[24];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x24 + α229x23 + α121x22 + α135x21 + α48x20 + α211x19 + α117x18 + α251x17 + α126x16 + α159x15 + α180x14 + α169x13 + α152x12 + α192x11 + α226x10 + α228x9 + α218x8 + α111x7 + α0x6 + α117x5 + α232x4 + α87x3 + α96x2 + α227x + α21"
    )
}

#[test]
fn generator_polynomial_25() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[25];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x25 + α231x24 + α181x23 + α156x22 + α39x21 + α170x20 + α26x19 + α12x18 + α59x17 + α15x16 + α148x15 + α201x14 + α54x13 + α66x12 + α237x11 + α208x10 + α99x9 + α167x8 + α144x7 + α182x6 + α95x5 + α243x4 + α129x3 + α178x2 + α252x + α45"
    )
}

#[test]
fn generator_polynomial_26() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[26];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x26 + α173x25 + α125x24 + α158x23 + α2x22 + α103x21 + α182x20 + α118x19 + α17x18 + α145x17 + α201x16 + α111x15 + α28x14 + α165x13 + α53x12 + α161x11 + α21x10 + α245x9 + α142x8 + α13x7 + α102x6 + α48x5 + α227x4 + α153x3 + α145x2 + α218x + α70"
    )
}

#[test]
fn generator_polynomial_27() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[27];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x27 + α79x26 + α228x25 + α8x24 + α165x23 + α227x22 + α21x21 + α180x20 + α29x19 + α9x18 + α237x17 + α70x16 + α99x15 + α45x14 + α58x13 + α138x12 + α135x11 + α73x10 + α126x9 + α172x8 + α94x7 + α216x6 + α193x5 + α157x4 + α26x3 + α17x2 + α149x + α96"
    )
}

#[test]
fn generator_polynomial_28() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[28];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x28 + α168x27 + α223x26 + α200x25 + α104x24 + α224x23 + α234x22 + α108x21 + α180x20 + α110x19 + α190x18 + α195x17 + α147x16 + α205x15 + α27x14 + α232x13 + α201x12 + α21x11 + α43x10 + α245x9 + α87x8 + α42x7 + α195x6 + α212x5 + α119x4 + α242x3 + α37x2 + α9x + α123"
    )
}

#[test]
fn generator_polynomial_29() {
    let poly = crate::tests::GENERATOR_POLYNOMIALS[29];
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "α0x29 + α156x28 + α45x27 + α183x26 + α29x25 + α151x24 + α219x23 + α54x22 + α96x21 + α249x20 + α24x19 + α136x18 + α5x17 + α241x16 + α175x15 + α189x14 + α28x13 + α75x12 + α234x11 + α150x10 + α148x9 + α23x8 + α9x7 + α202x6 + α162x5 + α68x4 + α250x3 + α140x2 + α24x + α151"
    )
}

#[test]
fn generator1_l() {
    let version = crate::version::Version::V1;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[7])
}

#[test]
fn generator1_m() {
    let version = crate::version::Version::V1;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[10])
}

#[test]
fn generator1_q() {
    let version = crate::version::Version::V1;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[13])
}

#[test]
fn generator1_h() {
    let version = crate::version::Version::V1;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[17])
}

#[test]
fn generator2_l() {
    let version = crate::version::Version::V2;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[10])
}

#[test]
fn generator2_m() {
    let version = crate::version::Version::V2;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[16])
}

#[test]
fn generator2_q() {
    let version = crate::version::Version::V2;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator2_h() {
    let version = crate::version::Version::V2;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator3_l() {
    let version = crate::version::Version::V3;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[15])
}

#[test]
fn generator3_m() {
    let version = crate::version::Version::V3;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator3_q() {
    let version = crate::version::Version::V3;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[18])
}

#[test]
fn generator3_h() {
    let version = crate::version::Version::V3;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator4_l() {
    let version = crate::version::Version::V4;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[20])
}

#[test]
fn generator4_m() {
    let version = crate::version::Version::V4;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[18])
}

#[test]
fn generator4_q() {
    let version = crate::version::Version::V4;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator4_h() {
    let version = crate::version::Version::V4;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[16])
}

#[test]
fn generator5_l() {
    let version = crate::version::Version::V5;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator5_m() {
    let version = crate::version::Version::V5;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator5_q() {
    let version = crate::version::Version::V5;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[18])
}

#[test]
fn generator5_h() {
    let version = crate::version::Version::V5;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator6_l() {
    let version = crate::version::Version::V6;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[18])
}

#[test]
fn generator6_m() {
    let version = crate::version::Version::V6;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[16])
}

#[test]
fn generator6_q() {
    let version = crate::version::Version::V6;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator6_h() {
    let version = crate::version::Version::V6;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator7_l() {
    let version = crate::version::Version::V7;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[20])
}

#[test]
fn generator7_m() {
    let version = crate::version::Version::V7;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[18])
}

#[test]
fn generator7_q() {
    let version = crate::version::Version::V7;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[18])
}

#[test]
fn generator7_h() {
    let version = crate::version::Version::V7;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator8_l() {
    let version = crate::version::Version::V8;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator8_m() {
    let version = crate::version::Version::V8;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator8_q() {
    let version = crate::version::Version::V8;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator8_h() {
    let version = crate::version::Version::V8;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator9_l() {
    let version = crate::version::Version::V9;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator9_m() {
    let version = crate::version::Version::V9;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator9_q() {
    let version = crate::version::Version::V9;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[20])
}

#[test]
fn generator9_h() {
    let version = crate::version::Version::V9;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator10_l() {
    let version = crate::version::Version::V10;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[18])
}

#[test]
fn generator10_m() {
    let version = crate::version::Version::V10;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator10_q() {
    let version = crate::version::Version::V10;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator10_h() {
    let version = crate::version::Version::V10;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator11_l() {
    let version = crate::version::Version::V11;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[20])
}

#[test]
fn generator11_m() {
    let version = crate::version::Version::V11;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator11_q() {
    let version = crate::version::Version::V11;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator11_h() {
    let version = crate::version::Version::V11;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator12_l() {
    let version = crate::version::Version::V12;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator12_m() {
    let version = crate::version::Version::V12;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator12_q() {
    let version = crate::version::Version::V12;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator12_h() {
    let version = crate::version::Version::V12;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator13_l() {
    let version = crate::version::Version::V13;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator13_m() {
    let version = crate::version::Version::V13;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator13_q() {
    let version = crate::version::Version::V13;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator13_h() {
    let version = crate::version::Version::V13;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator14_l() {
    let version = crate::version::Version::V14;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator14_m() {
    let version = crate::version::Version::V14;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator14_q() {
    let version = crate::version::Version::V14;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[20])
}

#[test]
fn generator14_h() {
    let version = crate::version::Version::V14;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator15_l() {
    let version = crate::version::Version::V15;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[22])
}

#[test]
fn generator15_m() {
    let version = crate::version::Version::V15;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator15_q() {
    let version = crate::version::Version::V15;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator15_h() {
    let version = crate::version::Version::V15;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator16_l() {
    let version = crate::version::Version::V16;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator16_m() {
    let version = crate::version::Version::V16;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator16_q() {
    let version = crate::version::Version::V16;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator16_h() {
    let version = crate::version::Version::V16;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator17_l() {
    let version = crate::version::Version::V17;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator17_m() {
    let version = crate::version::Version::V17;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator17_q() {
    let version = crate::version::Version::V17;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator17_h() {
    let version = crate::version::Version::V17;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator18_l() {
    let version = crate::version::Version::V18;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator18_m() {
    let version = crate::version::Version::V18;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator18_q() {
    let version = crate::version::Version::V18;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator18_h() {
    let version = crate::version::Version::V18;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator19_l() {
    let version = crate::version::Version::V19;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator19_m() {
    let version = crate::version::Version::V19;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator19_q() {
    let version = crate::version::Version::V19;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator19_h() {
    let version = crate::version::Version::V19;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator20_l() {
    let version = crate::version::Version::V20;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator20_m() {
    let version = crate::version::Version::V20;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator20_q() {
    let version = crate::version::Version::V20;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator20_h() {
    let version = crate::version::Version::V20;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator21_l() {
    let version = crate::version::Version::V21;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator21_m() {
    let version = crate::version::Version::V21;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator21_q() {
    let version = crate::version::Version::V21;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator21_h() {
    let version = crate::version::Version::V21;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator22_l() {
    let version = crate::version::Version::V22;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator22_m() {
    let version = crate::version::Version::V22;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator22_q() {
    let version = crate::version::Version::V22;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator22_h() {
    let version = crate::version::Version::V22;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[24])
}

#[test]
fn generator23_l() {
    let version = crate::version::Version::V23;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator23_m() {
    let version = crate::version::Version::V23;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator23_q() {
    let version = crate::version::Version::V23;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator23_h() {
    let version = crate::version::Version::V23;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator24_l() {
    let version = crate::version::Version::V24;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator24_m() {
    let version = crate::version::Version::V24;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator24_q() {
    let version = crate::version::Version::V24;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator24_h() {
    let version = crate::version::Version::V24;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator25_l() {
    let version = crate::version::Version::V25;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[26])
}

#[test]
fn generator25_m() {
    let version = crate::version::Version::V25;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator25_q() {
    let version = crate::version::Version::V25;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator25_h() {
    let version = crate::version::Version::V25;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator26_l() {
    let version = crate::version::Version::V26;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator26_m() {
    let version = crate::version::Version::V26;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator26_q() {
    let version = crate::version::Version::V26;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator26_h() {
    let version = crate::version::Version::V26;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator27_l() {
    let version = crate::version::Version::V27;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator27_m() {
    let version = crate::version::Version::V27;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator27_q() {
    let version = crate::version::Version::V27;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator27_h() {
    let version = crate::version::Version::V27;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator28_l() {
    let version = crate::version::Version::V28;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator28_m() {
    let version = crate::version::Version::V28;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator28_q() {
    let version = crate::version::Version::V28;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator28_h() {
    let version = crate::version::Version::V28;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator29_l() {
    let version = crate::version::Version::V29;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator29_m() {
    let version = crate::version::Version::V29;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator29_q() {
    let version = crate::version::Version::V29;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator29_h() {
    let version = crate::version::Version::V29;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator30_l() {
    let version = crate::version::Version::V30;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator30_m() {
    let version = crate::version::Version::V30;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator30_q() {
    let version = crate::version::Version::V30;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator30_h() {
    let version = crate::version::Version::V30;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator31_l() {
    let version = crate::version::Version::V31;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator31_m() {
    let version = crate::version::Version::V31;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator31_q() {
    let version = crate::version::Version::V31;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator31_h() {
    let version = crate::version::Version::V31;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator32_l() {
    let version = crate::version::Version::V32;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator32_m() {
    let version = crate::version::Version::V32;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator32_q() {
    let version = crate::version::Version::V32;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator32_h() {
    let version = crate::version::Version::V32;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator33_l() {
    let version = crate::version::Version::V33;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator33_m() {
    let version = crate::version::Version::V33;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator33_q() {
    let version = crate::version::Version::V33;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator33_h() {
    let version = crate::version::Version::V33;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator34_l() {
    let version = crate::version::Version::V34;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator34_m() {
    let version = crate::version::Version::V34;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator34_q() {
    let version = crate::version::Version::V34;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator34_h() {
    let version = crate::version::Version::V34;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator35_l() {
    let version = crate::version::Version::V35;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator35_m() {
    let version = crate::version::Version::V35;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator35_q() {
    let version = crate::version::Version::V35;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator35_h() {
    let version = crate::version::Version::V35;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator36_l() {
    let version = crate::version::Version::V36;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator36_m() {
    let version = crate::version::Version::V36;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator36_q() {
    let version = crate::version::Version::V36;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator36_h() {
    let version = crate::version::Version::V36;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator37_l() {
    let version = crate::version::Version::V37;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator37_m() {
    let version = crate::version::Version::V37;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator37_q() {
    let version = crate::version::Version::V37;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator37_h() {
    let version = crate::version::Version::V37;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator38_l() {
    let version = crate::version::Version::V38;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator38_m() {
    let version = crate::version::Version::V38;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator38_q() {
    let version = crate::version::Version::V38;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator38_h() {
    let version = crate::version::Version::V38;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator39_l() {
    let version = crate::version::Version::V39;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator39_m() {
    let version = crate::version::Version::V39;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator39_q() {
    let version = crate::version::Version::V39;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator39_h() {
    let version = crate::version::Version::V39;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator40_l() {
    let version = crate::version::Version::V40;
    let gen = version.get_polynomial(crate::vecl::ECL::L);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator40_m() {
    let version = crate::version::Version::V40;
    let gen = version.get_polynomial(crate::vecl::ECL::M);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[28])
}

#[test]
fn generator40_q() {
    let version = crate::version::Version::V40;
    let gen = version.get_polynomial(crate::vecl::ECL::Q);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}

#[test]
fn generator40_h() {
    let version = crate::version::Version::V40;
    let gen = version.get_polynomial(crate::vecl::ECL::H);

    assert_eq!(gen, crate::tests::GENERATOR_POLYNOMIALS[30])
}
