#[cfg(test)]
mod tests;

mod alphanum;
mod default;
mod helpers;
mod polynomials;
mod vecl;

/// Still useless, only test purposes for now.
fn main() {
    const VERSION: usize = 5;
    const QUALITY: vecl::ECL = vecl::ECL::Q;
    const STRING_TO_ENCODE: &[u8] = b"HELLO WORLD ME IS ERWAN";

    let res = alphanum::encode_alphanum(STRING_TO_ENCODE, VERSION, QUALITY);
    let data_codewords = &[
        67, 85, 70, 134, 87, 38, 85, 194, 119, 50, 6, 18, 6, 103, 38, 246, 246, 66, 7, 118, 134,
        242, 7, 38, 86, 22, 198, 199, 146, 6, 182, 230, 247, 119, 50, 7, 118, 134, 87, 38, 82, 6,
        134, 151, 50, 7, 70, 247, 118, 86, 194, 6, 151, 50, 16, 236, 17, 236, 17, 236, 17, 236,
    ]
    .to_vec();
    // println!("{:?}", tmp1);
    let error_codewords = polynomials::GENERATOR_POLYNOMIALS[vecl::ecc_to_ect(QUALITY, VERSION)];
    // println!("{:?}", tmp2);

    let [interleaved_data, interleaved_error] =
        polynomials::structure(&data_codewords, &error_codewords, QUALITY, VERSION);

    println!("{:#?}", interleaved_data);
    println!("{:#?}", interleaved_error);

    // let division = polynomials::division(&data_codewords, &error_codewords);
    // println!("{:?}", division);
}
