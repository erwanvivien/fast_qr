use crate::{hardcode, polynomials, Version, ECL};

#[test]
fn error_code_computation_01() {
    let version = Version::V05;
    let quality = ECL::Q;

    let vec = [67, 85, 70, 134, 87, 38, 85, 194, 119, 50, 6, 18, 6, 103, 38];
    let generator_polynomials = hardcode::get_polynomial(version, quality);

    let div = polynomials::division(&vec, generator_polynomials, 0, vec.len());
    assert_eq!(
        div[255 - generator_polynomials.len() + 1..],
        [213, 199, 11, 45, 115, 247, 241, 223, 229, 248, 154, 117, 154, 111, 86, 161, 111, 39]
    )
}

#[test]
fn error_code_computation_02() {
    let version = Version::V05;
    let quality = ECL::Q;

    let vec = [
        246, 246, 66, 7, 118, 134, 242, 7, 38, 86, 22, 198, 199, 146, 6,
    ];

    let generator_polynomials = hardcode::get_polynomial(version, quality);

    let div = polynomials::division(&vec, generator_polynomials, 0, vec.len());

    assert_eq!(
        div[255 - generator_polynomials.len() + 1..],
        [87, 204, 96, 60, 202, 182, 124, 157, 200, 134, 27, 129, 209, 17, 163, 163, 120, 133]
    )
}

#[test]
fn error_code_computation_03() {
    let version = Version::V05;
    let quality = ECL::Q;

    let vec = [
        182, 230, 247, 119, 50, 7, 118, 134, 87, 38, 82, 6, 134, 151, 50, 7,
    ];
    let generator_polynomials = hardcode::get_polynomial(version, quality);

    let div = polynomials::division(&vec, generator_polynomials, 0, vec.len());

    assert_eq!(
        div[255 - generator_polynomials.len() + 1..],
        [148, 116, 177, 212, 76, 133, 75, 242, 238, 76, 195, 230, 189, 10, 108, 240, 192, 141]
    )
}

#[test]
fn error_code_computation_04() {
    let version = Version::V05;
    let quality = ECL::Q;

    let vec = [
        70, 247, 118, 86, 194, 6, 151, 50, 16, 236, 17, 236, 17, 236, 17, 236,
    ];
    let generator_polynomials = hardcode::get_polynomial(version, quality);

    let div = polynomials::division(&vec, generator_polynomials, 0, vec.len());

    assert_eq!(
        div[255 - generator_polynomials.len() + 1..],
        [235, 159, 5, 173, 24, 147, 59, 33, 106, 40, 255, 172, 82, 2, 131, 32, 178, 236]
    )
}

#[test]
fn error_code_computation_821043386() {
    let tmp1 = [
        29, 10, 145, 40, 0, 90, 126, 137, 221, 186, 137, 39, 208, 250, 199, 176, 202, 124, 200, 85,
        63, 254,
    ];
    let tmp2 = [
        0, 156, 45, 183, 29, 151, 219, 54, 96, 249, 24, 136, 5, 241, 175, 189, 28, 75, 234, 150,
        148, 23, 9, 202, 162, 68, 250, 140, 24, 151,
    ];
    let div = polynomials::division(&tmp1, &tmp2, 0, tmp1.len());
    assert_eq!(
        div[255 - 29..],
        ([
            0, 85, 37, 253, 234, 217, 13, 16, 62, 107, 80, 72, 22, 66, 240, 139, 57, 109, 195, 68,
            121, 32, 206, 196, 117, 252, 175, 189, 167
        ])
    )
}

#[test]
fn error_code_computation_struct_31_0() {
    let tmp1 = [28, 195, 100, 36, 175, 11, 35, 243, 28, 137, 59, 182, 193];
    let tmp2 = [
        0, 173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142, 13,
        102, 48, 227, 153, 145, 218, 70,
    ];
    let div = polynomials::division(&tmp1, &tmp2, 0, tmp1.len());
    assert_eq!(
        div[255 - 26..],
        ([
            68, 150, 68, 205, 197, 78, 104, 100, 177, 0, 185, 7, 178, 106, 110, 170, 101, 222, 45,
            74, 31, 75, 3, 126, 216, 208
        ])
    )
}

#[test]
fn error_code_computation_struct_31_1() {
    let tmp1 = [35, 37, 251, 189, 8, 169, 15, 34, 59, 137, 187, 114, 134];
    let tmp2 = [
        0, 173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142, 13,
        102, 48, 227, 153, 145, 218, 70,
    ];
    let div = polynomials::division(&tmp1, &tmp2, 0, tmp1.len());
    assert_eq!(
        div[255 - 26..],
        ([
            246, 74, 169, 24, 210, 247, 165, 59, 102, 186, 144, 234, 202, 247, 84, 191, 166, 28,
            140, 190, 219, 81, 72, 34, 159, 0
        ])
    )
}
