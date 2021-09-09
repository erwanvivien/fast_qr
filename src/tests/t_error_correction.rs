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

//32,179,11,120,209,114,0,102,179,176,166,154,54,98,155,231,100,0,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17
#[test]
fn error_code_computation_05_13() {
    let version = 5;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;
    let string_to_encode: String = String::from("HELLO MY NAME IS ERWAN");

    let vec = crate::encoding::alphanum::encode(string_to_encode, version, QUALITY);
    let tmp1 = vec.unwrap().to_vec();

    let tmp2 = crate::polynomials::GENERATOR_POLYNOMIALS[13];

    let div = crate::polynomials::division(&tmp1, &tmp2);
    assert_eq!(
        div,
        [190, 34, 203, 43, 239, 12, 245, 201, 206, 94, 141, 104, 128].to_vec()
    )
}
#[test]
fn error_code_computation_05_15() {
    let version = 5;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;
    let string_to_encode: String = String::from("HELLO MY NAME IS ERWAN");

    let vec = crate::encoding::alphanum::encode(string_to_encode, version, QUALITY);
    let tmp1 = vec.unwrap().to_vec();

    let tmp2 = crate::polynomials::GENERATOR_POLYNOMIALS[15];

    let div = crate::polynomials::division(&tmp1, &tmp2);
    assert_eq!(
        div,
        [242, 155, 43, 96, 52, 0, 116, 109, 2, 48, 116, 3, 88, 153, 221].to_vec()
    )
}
#[test]
fn error_code_computation_05_28() {
    let version = 5;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;
    let string_to_encode: String = String::from("HELLO MY NAME IS ERWAN");

    let vec = crate::encoding::alphanum::encode(string_to_encode, version, QUALITY);
    let tmp1 = vec.unwrap().to_vec();

    let tmp2 = crate::polynomials::GENERATOR_POLYNOMIALS[28];

    let div = crate::polynomials::division(&tmp1, &tmp2);
    assert_eq!(
        div,
        [
            70, 189, 121, 107, 1, 64, 88, 143, 59, 160, 93, 16, 73, 136, 2, 4, 235, 248, 196, 34,
            72, 83, 143, 127, 200, 210, 71, 123
        ]
        .to_vec()
    )
}

#[test]
fn error_code_computation_05_18() {
    let version = 5;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;
    let string_to_encode: String = String::from("HELLO MY NAME IS ERWAN");

    let vec = crate::encoding::alphanum::encode(string_to_encode, version, QUALITY);
    let tmp1 = vec.unwrap().to_vec();

    let tmp2 = crate::polynomials::GENERATOR_POLYNOMIALS[crate::vecl::ecc_to_ect(QUALITY, version)];

    let div = crate::polynomials::division(&tmp1, &tmp2);
    assert_eq!(
        div,
        [18, 132, 191, 150, 90, 206, 18, 95, 66, 14, 204, 41, 184, 20, 83, 189, 35, 248].to_vec()
    )
}
