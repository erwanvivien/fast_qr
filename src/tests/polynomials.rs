/// Contains all possible generator polynomials (to compule error codewords)
pub const GENERATOR_POLYNOMIALS: [&'static [u8]; 31] = [
    &[0],
    &[0, 0],
    &[0, 25, 1],
    &[0, 198, 199, 3],
    &[0, 75, 249, 78, 6],
    &[0, 113, 164, 166, 119, 10],
    &[0, 166, 0, 134, 5, 176, 15],
    &[0, 87, 229, 146, 149, 238, 102, 21],
    &[0, 175, 238, 208, 249, 215, 252, 196, 28],
    &[0, 95, 246, 137, 231, 235, 149, 11, 123, 36],
    &[0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45],
    &[0, 220, 192, 91, 194, 172, 177, 209, 116, 227, 10, 55],
    &[0, 102, 43, 98, 121, 187, 113, 198, 143, 131, 87, 157, 66],
    &[
        0, 74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78,
    ],
    &[
        0, 199, 249, 155, 48, 190, 124, 218, 137, 216, 87, 207, 59, 22, 91,
    ],
    &[
        0, 8, 183, 61, 91, 202, 37, 51, 58, 58, 237, 140, 124, 5, 99, 105,
    ],
    &[
        0, 120, 104, 107, 109, 102, 161, 76, 3, 91, 191, 147, 169, 182, 194, 225, 120,
    ],
    &[
        0, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136,
    ],
    &[
        0, 215, 234, 158, 94, 184, 97, 118, 170, 79, 187, 152, 148, 252, 179, 5, 98, 96, 153,
    ],
    &[
        0, 67, 3, 105, 153, 52, 90, 83, 17, 150, 159, 44, 128, 153, 133, 252, 222, 138, 220, 171,
    ],
    &[
        0, 17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164, 212, 212, 188,
        190,
    ],
    &[
        0, 240, 233, 104, 247, 181, 140, 67, 98, 85, 200, 210, 115, 148, 137, 230, 36, 122, 254,
        148, 175, 210,
    ],
    &[
        0, 210, 171, 247, 242, 93, 230, 14, 109, 221, 53, 200, 74, 8, 172, 98, 80, 219, 134, 160,
        105, 165, 231,
    ],
    &[
        0, 171, 102, 146, 91, 49, 103, 65, 17, 193, 150, 14, 25, 183, 248, 94, 164, 224, 192, 1,
        78, 56, 147, 253,
    ],
    &[
        0, 229, 121, 135, 48, 211, 117, 251, 126, 159, 180, 169, 152, 192, 226, 228, 218, 111, 0,
        117, 232, 87, 96, 227, 21,
    ],
    &[
        0, 231, 181, 156, 39, 170, 26, 12, 59, 15, 148, 201, 54, 66, 237, 208, 99, 167, 144, 182,
        95, 243, 129, 178, 252, 45,
    ],
    &[
        0, 173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142, 13,
        102, 48, 227, 153, 145, 218, 70,
    ],
    &[
        0, 79, 228, 8, 165, 227, 21, 180, 29, 9, 237, 70, 99, 45, 58, 138, 135, 73, 126, 172, 94,
        216, 193, 157, 26, 17, 149, 96,
    ],
    &[
        0, 168, 223, 200, 104, 224, 234, 108, 180, 110, 190, 195, 147, 205, 27, 232, 201, 21, 43,
        245, 87, 42, 195, 212, 119, 242, 37, 9, 123,
    ],
    &[
        0, 156, 45, 183, 29, 151, 219, 54, 96, 249, 24, 136, 5, 241, 175, 189, 28, 75, 234, 150,
        148, 23, 9, 202, 162, 68, 250, 140, 24, 151,
    ],
    &[
        0, 41, 173, 145, 152, 216, 31, 179, 182, 50, 48, 110, 86, 239, 96, 222, 125, 42, 173, 226,
        193, 224, 130, 156, 37, 251, 216, 238, 40, 192, 180,
    ],
];

#[test]
fn generator_polynomials() {
    let poly = GENERATOR_POLYNOMIALS[7];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x7 + ??87x6 + ??229x5 + ??146x4 + ??149x3 + ??238x2 + ??102x + ??21"
    );

    let poly = GENERATOR_POLYNOMIALS[8];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x8 + ??175x7 + ??238x6 + ??208x5 + ??249x4 + ??215x3 + ??252x2 + ??196x + ??28"
    );

    let poly = GENERATOR_POLYNOMIALS[9];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x9 + ??95x8 + ??246x7 + ??137x6 + ??231x5 + ??235x4 + ??149x3 + ??11x2 + ??123x + ??36"
    );

    let poly = GENERATOR_POLYNOMIALS[10];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x10 + ??251x9 + ??67x8 + ??46x7 + ??61x6 + ??118x5 + ??70x4 + ??64x3 + ??94x2 + ??32x + ??45"
    );

    let poly = GENERATOR_POLYNOMIALS[11];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x11 + ??220x10 + ??192x9 + ??91x8 + ??194x7 + ??172x6 + ??177x5 + ??209x4 + ??116x3 + ??227x2 + ??10x + ??55"
    );

    let poly = GENERATOR_POLYNOMIALS[12];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x12 + ??102x11 + ??43x10 + ??98x9 + ??121x8 + ??187x7 + ??113x6 + ??198x5 + ??143x4 + ??131x3 + ??87x2 + ??157x + ??66"
    );

    let poly = GENERATOR_POLYNOMIALS[13];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x13 + ??74x12 + ??152x11 + ??176x10 + ??100x9 + ??86x8 + ??100x7 + ??106x6 + ??104x5 + ??130x4 + ??218x3 + ??206x2 + ??140x + ??78"
    );

    let poly = GENERATOR_POLYNOMIALS[14];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x14 + ??199x13 + ??249x12 + ??155x11 + ??48x10 + ??190x9 + ??124x8 + ??218x7 + ??137x6 + ??216x5 + ??87x4 + ??207x3 + ??59x2 + ??22x + ??91"
    );

    let poly = GENERATOR_POLYNOMIALS[15];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x15 + ??8x14 + ??183x13 + ??61x12 + ??91x11 + ??202x10 + ??37x9 + ??51x8 + ??58x7 + ??58x6 + ??237x5 + ??140x4 + ??124x3 + ??5x2 + ??99x + ??105"
    );

    let poly = GENERATOR_POLYNOMIALS[16];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x16 + ??120x15 + ??104x14 + ??107x13 + ??109x12 + ??102x11 + ??161x10 + ??76x9 + ??3x8 + ??91x7 + ??191x6 + ??147x5 + ??169x4 + ??182x3 + ??194x2 + ??225x + ??120"
    );

    let poly = GENERATOR_POLYNOMIALS[17];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x17 + ??43x16 + ??139x15 + ??206x14 + ??78x13 + ??43x12 + ??239x11 + ??123x10 + ??206x9 + ??214x8 + ??147x7 + ??24x6 + ??99x5 + ??150x4 + ??39x3 + ??243x2 + ??163x + ??136"
    );

    let poly = GENERATOR_POLYNOMIALS[18];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x18 + ??215x17 + ??234x16 + ??158x15 + ??94x14 + ??184x13 + ??97x12 + ??118x11 + ??170x10 + ??79x9 + ??187x8 + ??152x7 + ??148x6 + ??252x5 + ??179x4 + ??5x3 + ??98x2 + ??96x + ??153"
    );

    let poly = GENERATOR_POLYNOMIALS[19];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x19 + ??67x18 + ??3x17 + ??105x16 + ??153x15 + ??52x14 + ??90x13 + ??83x12 + ??17x11 + ??150x10 + ??159x9 + ??44x8 + ??128x7 + ??153x6 + ??133x5 + ??252x4 + ??222x3 + ??138x2 + ??220x + ??171"
    );

    let poly = GENERATOR_POLYNOMIALS[20];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x20 + ??17x19 + ??60x18 + ??79x17 + ??50x16 + ??61x15 + ??163x14 + ??26x13 + ??187x12 + ??202x11 + ??180x10 + ??221x9 + ??225x8 + ??83x7 + ??239x6 + ??156x5 + ??164x4 + ??212x3 + ??212x2 + ??188x + ??190"
    );

    let poly = GENERATOR_POLYNOMIALS[21];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x21 + ??240x20 + ??233x19 + ??104x18 + ??247x17 + ??181x16 + ??140x15 + ??67x14 + ??98x13 + ??85x12 + ??200x11 + ??210x10 + ??115x9 + ??148x8 + ??137x7 + ??230x6 + ??36x5 + ??122x4 + ??254x3 + ??148x2 + ??175x + ??210"
    );

    let poly = GENERATOR_POLYNOMIALS[22];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x22 + ??210x21 + ??171x20 + ??247x19 + ??242x18 + ??93x17 + ??230x16 + ??14x15 + ??109x14 + ??221x13 + ??53x12 + ??200x11 + ??74x10 + ??8x9 + ??172x8 + ??98x7 + ??80x6 + ??219x5 + ??134x4 + ??160x3 + ??105x2 + ??165x + ??231"
    );

    let poly = GENERATOR_POLYNOMIALS[23];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x23 + ??171x22 + ??102x21 + ??146x20 + ??91x19 + ??49x18 + ??103x17 + ??65x16 + ??17x15 + ??193x14 + ??150x13 + ??14x12 + ??25x11 + ??183x10 + ??248x9 + ??94x8 + ??164x7 + ??224x6 + ??192x5 + ??1x4 + ??78x3 + ??56x2 + ??147x + ??253"
    );

    let poly = GENERATOR_POLYNOMIALS[24];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x24 + ??229x23 + ??121x22 + ??135x21 + ??48x20 + ??211x19 + ??117x18 + ??251x17 + ??126x16 + ??159x15 + ??180x14 + ??169x13 + ??152x12 + ??192x11 + ??226x10 + ??228x9 + ??218x8 + ??111x7 + ??0x6 + ??117x5 + ??232x4 + ??87x3 + ??96x2 + ??227x + ??21"
    );

    let poly = GENERATOR_POLYNOMIALS[25];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x25 + ??231x24 + ??181x23 + ??156x22 + ??39x21 + ??170x20 + ??26x19 + ??12x18 + ??59x17 + ??15x16 + ??148x15 + ??201x14 + ??54x13 + ??66x12 + ??237x11 + ??208x10 + ??99x9 + ??167x8 + ??144x7 + ??182x6 + ??95x5 + ??243x4 + ??129x3 + ??178x2 + ??252x + ??45"
    );

    let poly = GENERATOR_POLYNOMIALS[26];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x26 + ??173x25 + ??125x24 + ??158x23 + ??2x22 + ??103x21 + ??182x20 + ??118x19 + ??17x18 + ??145x17 + ??201x16 + ??111x15 + ??28x14 + ??165x13 + ??53x12 + ??161x11 + ??21x10 + ??245x9 + ??142x8 + ??13x7 + ??102x6 + ??48x5 + ??227x4 + ??153x3 + ??145x2 + ??218x + ??70"
    );

    let poly = GENERATOR_POLYNOMIALS[27];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x27 + ??79x26 + ??228x25 + ??8x24 + ??165x23 + ??227x22 + ??21x21 + ??180x20 + ??29x19 + ??9x18 + ??237x17 + ??70x16 + ??99x15 + ??45x14 + ??58x13 + ??138x12 + ??135x11 + ??73x10 + ??126x9 + ??172x8 + ??94x7 + ??216x6 + ??193x5 + ??157x4 + ??26x3 + ??17x2 + ??149x + ??96"
    );

    let poly = GENERATOR_POLYNOMIALS[28];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x28 + ??168x27 + ??223x26 + ??200x25 + ??104x24 + ??224x23 + ??234x22 + ??108x21 + ??180x20 + ??110x19 + ??190x18 + ??195x17 + ??147x16 + ??205x15 + ??27x14 + ??232x13 + ??201x12 + ??21x11 + ??43x10 + ??245x9 + ??87x8 + ??42x7 + ??195x6 + ??212x5 + ??119x4 + ??242x3 + ??37x2 + ??9x + ??123"
    );

    let poly = GENERATOR_POLYNOMIALS[29];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "??0x29 + ??156x28 + ??45x27 + ??183x26 + ??29x25 + ??151x24 + ??219x23 + ??54x22 + ??96x21 + ??249x20 + ??24x19 + ??136x18 + ??5x17 + ??241x16 + ??175x15 + ??189x14 + ??28x13 + ??75x12 + ??234x11 + ??150x10 + ??148x9 + ??23x8 + ??9x7 + ??202x6 + ??162x5 + ??68x4 + ??250x3 + ??140x2 + ??24x + ??151"
    )
}
mod generators {
    use super::GENERATOR_POLYNOMIALS;

    #[test]
    fn generator1() {
        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[7]);

        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[10]);

        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[13]);

        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[17]);
    }

    #[test]
    fn generator2() {
        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[10]);

        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[16]);

        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator3() {
        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[15]);

        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);
    }

    #[test]
    fn generator4() {
        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[16]);
    }

    #[test]
    fn generator5() {
        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);
    }

    #[test]
    fn generator6() {
        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[16]);

        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator7() {
        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);
    }

    #[test]
    fn generator8() {
        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);
    }

    #[test]
    fn generator9() {
        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator10() {
        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator11() {
        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator12() {
        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator13() {
        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);
    }

    #[test]
    fn generator14() {
        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator15() {
        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator16() {
        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator17() {
        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator18() {
        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator19() {
        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);
    }

    #[test]
    fn generator20() {
        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator21() {
        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator22() {
        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator23() {
        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator24() {
        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator25() {
        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator26() {
        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator27() {
        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator28() {
        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator29() {
        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator30() {
        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator31() {
        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator32() {
        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator33() {
        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator34() {
        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator35() {
        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator36() {
        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator37() {
        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator38() {
        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator39() {
        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator40() {
        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }
}
