#[cfg(test)]
mod tests;

mod bitstring;
mod datamasking;
mod default;
mod encode;
mod helpers;
mod placement;
mod polynomials;
mod score;
mod vecl;
mod version;

/// Still useless, only test purposes for now.
fn main() {
    let content = String::from("https://vahan.dev");
    let quality = None;

    let mat = placement::qrcode(content, quality);
    helpers::print_matrix_with_margin(&mat);
}
