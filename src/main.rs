#[cfg(test)]
mod tests;

mod bitstring;
mod datamasking;
mod default;
mod encode;
mod helpers;
mod placement;
mod polynomials;
mod qrcode;
mod score;
mod vecl;
mod version;

/// Still useless, only test purposes for now.
fn main() {
    const content: &str = "A";
    const qrcode: Option<qrcode::QRCode> =
        qrcode::QRCode::new(content.as_bytes(), vecl::ECL::H, None);

    // if let Some(q) = qrcode {
    //     q.print();
    // }

    match qrcode.unwrap() {
        qrcode::QRCode::V1(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V2(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V3(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V4(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V5(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V6(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V7(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V8(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V9(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V10(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V11(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V12(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V13(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V14(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V15(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V16(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V17(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V18(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V19(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V20(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V21(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V22(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V23(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V24(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V25(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V26(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V27(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V28(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V29(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V30(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V31(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V32(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V33(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V34(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V35(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V36(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V37(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V38(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V39(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
        qrcode::QRCode::V40(matrix) => println!("{:?}\n{}", &matrix, score::matrix_score(&matrix)),
    }
}
