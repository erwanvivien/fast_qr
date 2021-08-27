#[cfg(test)]
mod tests;

mod alphanum;
mod default;
mod ecc;
mod generator_polynomial;
mod helpers;

/// Still useless, only test purposes for now.
fn main() {
    let version = 1;
    let mat = default::create_matrix_from_version(version);

    let string_to_encode = b"HELLO WORLD";
    // print_matrix(&mat);
    helpers::print_matrix_with_margin(&mat);

    let res = alphanum::encode_alphanum(string_to_encode, version, ecc::ECC::M);
    println!("{} =", res);
    println!("{:?}", alphanum::alphanum_to_binary(&res));

    println!(
        "{:?} {}",
        generator_polynomial::generator(4),
        generator_polynomial::generated_to_string(&generator_polynomial::generator(4))
    )
}
